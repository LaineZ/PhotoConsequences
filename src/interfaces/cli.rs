use std::{
    fs::{self, DirEntry},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use indicatif::{MultiProgress, ProgressStyle, ProgressBar};

use crate::state_headless::StateHeadless;

pub fn cli(args: Vec<String>) -> anyhow::Result<()> {
    if args.len() > 3 {
        let project_path = Arc::new(PathBuf::from_str(&args[1])?);
        let image_path = PathBuf::from_str(&args[2])?;
        let image_export_path = Arc::new(PathBuf::from_str(&args[3])?);

        let cpus = num_cpus::get();
        let mut paths = fs::read_dir(image_path)?
            .collect::<Vec<Result<DirEntry, std::io::Error>>>();

        paths.retain(|f| f.is_ok());

        let path_size = paths.len();
        let paths_arc = Arc::new(paths);
        let mut threads = Vec::new();

        let m = MultiProgress::new();
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg:>50}",
        )
        .unwrap()
        .progress_chars("##-");


        let pb = m.add(ProgressBar::new(100));
        pb.set_style(sty.clone());

        for i in 0..cpus {
            let paths_ar = Arc::clone(&paths_arc);
            let project_path = Arc::clone(&project_path);
            let image_export_path = Arc::clone(&image_export_path);
            let pb = m.add(ProgressBar::new(100));
            pb.set_style(sty.clone());

            let m_clone = m.clone();
            threads.push(std::thread::spawn(move || {
                println!("Starting thread: {}", i);

                let chunks: Vec<_> = paths_ar.chunks((path_size / cpus) + 1).collect();
                let my_chunk = chunks[i];
                
                let mut state = StateHeadless::new();
                
                state.load_project(project_path.as_path()).unwrap();
                state.rack.block_size = 16384;

                for (idx, image_path) in my_chunk.iter().enumerate() {
                    let img_path = image_path.as_ref().unwrap();
                    let export_path = image_export_path.join(img_path.file_name());
                    //println!("Processing: {}", img_path.path().display());
                    state.load_image(img_path.path()).unwrap_or_else(|op| println!("Unable to load image: {}", op));
                    state.rack.start_process();

                    pb.set_message(format!("{}", img_path.path().display()));
                    pb.set_length(my_chunk.len() as u64);
                    pb.set_position(idx as u64);

                    while !state.rack.is_finished() {
                        state.rack.process_next();
                    }

                    state.rack.save_image(export_path.as_path()).unwrap_or_else(|op| println!("Unable to save image: {}", op));
                    m_clone.println(format!("Saved: {}", export_path.display())).unwrap();
                }
            }))
        }

        for thread in threads {
            let _ = thread.join();
        }

        println!("Processing is done!");
    } else {
        eprintln!("Not enough arguments. Exiting");
        println!("Usage: photoconsequences [project .viproj path] [input image folder pathj] [output path]");
    }
    Ok(())
}
