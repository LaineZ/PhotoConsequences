use std::{
    fs::{self, DirEntry},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use crate::state_headless::StateHeadless;

pub fn cli(args: Vec<String>) -> anyhow::Result<()> {
    if args.len() > 3 {
        let project_path = Arc::new(PathBuf::from_str(&args[1])?);
        let image_path = PathBuf::from_str(&args[2])?;
        let image_export_path = Arc::new(PathBuf::from_str(&args[3])?);

        let cpus = 2;
        let mut paths = fs::read_dir(image_path)?
            .collect::<Vec<Result<DirEntry, std::io::Error>>>();

        paths.retain(|f| f.is_ok());

        let path_size = paths.len();
        let paths_arc = Arc::new(paths);
        let mut threads = Vec::new();

        for i in 0..cpus {
            let paths_ar = Arc::clone(&paths_arc);
            let project_path = Arc::clone(&project_path);
            let image_export_path = Arc::clone(&image_export_path);
            threads.push(std::thread::spawn(move || {
                println!("Starting thread: {}", i);
                let chunks: Vec<_> = paths_ar.chunks((path_size / cpus) + 1).collect();
                let my_chunk = chunks[i];
                let mut state = StateHeadless::new();
                state.load_project(project_path.as_path()).unwrap();
                state.rack.block_size = 16384;

                for image_path in my_chunk {
                    let img_path = image_path.as_ref().unwrap();
                    let export_path = image_export_path.join(img_path.file_name());
                    println!("Processing: {}", img_path.path().display());
                    state.load_image(img_path.path()).unwrap_or_else(|op| println!("Unable to load image: {}", op));
                    state.process();
                    state.rack.save_image(export_path.as_path()).unwrap_or_else(|op| println!("Unable to save image: {}", op));
                    println!("Saved: {}", export_path.display());
                }
            }))
        }

        for thread in threads {
            let _ = thread.join();
        }
    } else {
        eprintln!("Not enough arguments. Exiting");
        println!("Usage: photoconsequences [project .viproj path] [input image folder pathj] [output path]");
    }
    Ok(())
}
