use interfaces::{cli, gui};

pub mod editor_wrapper;
pub mod egui_platform_winit;
pub mod image_generators;
pub mod image_utils;
pub mod interfaces;
pub mod models;
pub mod msgboxwrapper;
pub mod plugin_rack;
pub mod renderer;
pub mod state_headless;
pub mod ui;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("PhotoConsequences by @140bpmdubstep");
    println!("Version {}", VERSION);

    if args.len() > 2 {
        println!("Running in cli mode");
        cli::cli(args).expect("Error while running cli mode");
    } else {
        println!("Running in gui mode");
        gui::gui(args);
    }
}
