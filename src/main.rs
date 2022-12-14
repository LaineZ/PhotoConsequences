use interfaces::{cli, gui};
use log::info;

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
pub mod processing;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();

    info!("PhotoConsequences by @140bpmdubstep");
    info!("Version {}", VERSION);

    if args.len() > 2 {
        info!("Running in cli mode");
        cli::cli(args).expect("Error while running cli mode");
    } else {
        info!("Running in gui mode");
        gui::gui(args);
    }
}
