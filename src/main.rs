use interfaces::{cli, gui};

pub mod plugin_rack;
pub mod ui;
pub mod renderer;
pub mod msgboxwrapper;
pub mod ui_enums;
pub mod editor_wrapper;
pub mod image_generators;
pub mod egui_platform_winit;
pub mod state_headless;
pub mod interfaces;


pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Running in cli mode");
        cli::cli(args).unwrap();
    } else {
        println!("Running in gui mode");
        gui::gui(args);
    }
}
