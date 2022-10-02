use egui::{
    menu,
    plot::{Corner, Legend, Plot, PlotImage, Text, PlotPoint},
    vec2, Context, RichText, Color32, Align2,
};
use egui_extras::{Size, TableBuilder, TableBody};
use std::{path::PathBuf, io::Read, time::Instant};
use vst::prelude::Plugin;
use winit::{event_loop::EventLoopWindowTarget, window::WindowId};

use crate::{
    plugin_rack::{PluginRack, InputChannelType},
    renderer::{self, Renderer}, msgboxwrapper::messagebox, ui_enums::{ModalWindows, Action, DialogVariant}, image_generators,
};

pub struct State {
    rack: PluginRack,
    modal: ModalWindows,
    save_path: Option<PathBuf>,
    timer: Instant,
}

impl State {
    pub fn new() -> Self {
        Self {
            rack: PluginRack::new(),
            modal: ModalWindows::None,
            save_path: None,
            timer: Instant::now()
        }
    }

    pub fn cleanup_image(&mut self, renderer: &mut Renderer) {
        renderer.texture = None;
    }

    pub fn load_image(&mut self, renderer: &mut Renderer, file: PathBuf) -> anyhow::Result<()> {
        self.cleanup_image(renderer);
        self.rack.load_image(file)?;
        Ok(())
    }

    pub fn open_editor(
        &mut self,
        plugin_index: usize,
        renderer: &mut Renderer,
        event_loop: &EventLoopWindowTarget<renderer::Event>,
    ) {
        let editor_window = self.rack.plugins[plugin_index].editor.show(event_loop);

        match editor_window {
            Ok(editor) => {
                renderer.windows.push(editor);
            }
            Err(error) => {
                messagebox(
                    "Unable to open editor",
                    &error.to_string(),
                );
            }
        }
    }

    pub fn close_editor(&mut self, window_id: WindowId) {
        for plugin in self.rack.plugins.iter_mut() {
            plugin.editor.close(window_id);
        }
    }

    pub fn load_project(&mut self, renderer: &mut Renderer, file: PathBuf) -> anyhow::Result<()> {
        let zip_file = std::fs::File::open(&file)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;
        let mut proj_file = archive.by_name("project.json")?;

        let mut proj_file_string = String::new();
        proj_file.read_to_string(&mut proj_file_string)?;
        let instacnes: Vec<crate::plugin_rack::PluginRackInstance> = serde_json::from_str(&proj_file_string)?;

        self.cleanup_image(renderer);
        renderer.windows.clear();
        self.rack = PluginRack::new();

        self.rack.plugins.extend(instacnes);
        self.rack.load_uninitialzed_plugins()?;

        drop(proj_file);
        
        let mut image_file = archive.by_name("image.png")?;

        let mut buf = Vec::new();
        image_file.read_to_end(&mut buf)?;
        self.rack.load_image_data(&buf)?;
        self.save_path = Some(file);
        Ok(())
    }

    pub fn export_image(&self) {
        let files = rfd::FileDialog::new()
        .set_title("Export image")
        .add_filter(
            "Images",
            &[
                "png", "jpg", "jpeg", "gif", "bmp", "ico", "tiff", "webp", "tga",
            ],
        )
        .save_file();

        if let Some(file) = files {
            self.rack.save_image(file).unwrap_or_else(|op| {
                messagebox(
                    "Image saving error",
                    &format!("Cannot save image: \n{}", op),
                );
            });
        }
    }

    pub fn exit(&mut self, renderer: &mut Renderer) {
        match self.modal {
            ModalWindows::Exit => renderer.close_render(),
            _ => self.modal = ModalWindows::Exit,
        }
    }

    fn plugin_table_draw(&self, mut body: TableBody) -> Option<Action> {
        let mut action: Option<Action> = None;
        for (idx, name) in self.rack.plugins.iter().enumerate() {
            if name.instance.is_none() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("{}", name.get_path().display())).on_hover_text("This plugin is not initialized\nEffect configuration is preserved");
                    });
                    row.col(|ui| {
                        if ui.button("❎").on_hover_text("Remove").clicked() {
                            action = Some(Action::Remove(idx));
                        }
                    });
                });
                continue;
            }
            body.row(20.0, |mut row| {
                let inst = name.instance.as_ref().unwrap();
                let info = inst.get_info();
                row.col(|ui| {
                    ui.label(format!("{} ({})", info.name, info.vendor))
                        .on_hover_text(
                        format!("Right-click for more options\n{} ({})\nCategory: {:?}\nInitial delay: {}\nI/O: {}/{}\n64 bit mixing support: {}", 
                        info.name, info.vendor, info.category, info.initial_delay, info.inputs, info.outputs, info.f64_precision));
                }).context_menu(|ui| {
                    ui.label("Image input channel processing:");
                    ui.separator();
                    let mut radio = name.input_channel;
                                    
                    if ui.selectable_value(&mut radio, InputChannelType::Hue, "H").clicked() {
                        action = Some(Action::ChangeInputChannel(idx, radio))
                    };
                    if ui.selectable_value(&mut radio, InputChannelType::Saturation, "S").clicked() {
                        action = Some(Action::ChangeInputChannel(idx, radio))
                    };
                    if ui.selectable_value(&mut radio, InputChannelType::Value, "V").clicked() {
                        action = Some(Action::ChangeInputChannel(idx, radio))
                    };

                    ui.label("Output audio channel:");
                    ui.separator();
                    let mut output = name.output_channel;
                    let prefix = match output {
                        0 => { "Left " }
                        1 => { "Right " }
                        _ => { "" }
                    };

                    if ui.add(egui::Slider::new(&mut output, 0..=(info.outputs - 1) as usize).prefix(prefix)).changed() {
                        action = Some(Action::ChangeOutputChannel(idx, output));
                    }
                    

                    let mut wet = name.wet * 100.0;
                    ui.label("Wet:");
                    ui.separator();
                    if ui.add(egui::Slider::new(&mut wet, 0.0..=100.0).suffix("%")).changed() {
                        action = Some(Action::ChangeWet(idx, wet / 100.0));
                    }

                    let mut sample_rate = name.sample_rate;
                    ui.label("Sample rate:");
                    ui.separator();
                    if ui.add(egui::Slider::new(&mut sample_rate, 1102.0..=768000.0).suffix("Hz")).changed() {
                        action = Some(Action::ChangeSampleRate(idx, sample_rate));
                    }
                });

                row.col(|ui| {
                        ui.add_enabled_ui(self.rack.is_finished(), |ui| {
                            if ui.button("❎").on_hover_text("Remove").clicked() {
                                action = Some(Action::Remove(idx));
                            }
    
                            let color = if name.bypass {
                                Color32::DARK_RED
                            } else {
                                ui.visuals().widgets.active.bg_fill
                            };
    
                            if ui.add(egui::Button::new("M").fill(color)).on_hover_text("Bypass/Mute processing").clicked() {
                                action = Some(Action::Bypass(idx));
                            } 
                        });

                        if ui.button("🔧").on_hover_text("Open GUI Editor").clicked() {
                            action = Some(Action::OpenEditor(idx));
                        }
                });
            });
        }
        action
    }

    fn init(&mut self, renderer: &mut Renderer) {
        self.cleanup_image(renderer);
        renderer.windows.clear();
        renderer.destroy_texture();
        self.rack = PluginRack::new();
        self.save_path = None;
    }

    fn exit_window(&mut self, context: &Context) -> DialogVariant {
        if self.rack.images.is_empty() && self.rack.plugins.is_empty() { return DialogVariant::No }
        let mut res = DialogVariant::None;
        egui::Window::new("Project management")
        .collapsible(false)
        .auto_sized()
        .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
        .show(context, |ui| {
            if self.save_path.is_some() {
                ui.label(format!("Save {} before exiting?", self.save_path.as_ref().unwrap().display()));
            } else {
                ui.label("Save project before exiting?");
            }

            ui.horizontal(|ui| {
                if ui.button("✅ Yes").clicked() {
                    res = DialogVariant::Yes;
                }
                if ui.button("❎ No").clicked() {
                    res = DialogVariant::No;
                }
                if ui.button("🚫 Cancel").clicked() {
                    self.modal = ModalWindows::None;
                    res = DialogVariant::Cancel;
                }
            });
         });

        res
    }


    fn about_window(&mut self, context: &Context) -> DialogVariant {
        if self.rack.images.is_empty() && self.rack.plugins.is_empty() { return DialogVariant::No }
        let mut res = DialogVariant::None;
        egui::Window::new("About")
        .collapsible(false)
        .auto_sized()
        .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
        .show(context, |ui| {
            let text = RichText::new("PhotoConsequences").heading();
            ui.label(text);
            ui.label(format!("Version: {}", crate::VERSION));
            ui.label("Tool to apply VST™ effects on the images");
            ui.label("VST™ is a trademark of Steinberg Media Technologies GmbH.");

            if ui.button("Ok").clicked() {
                self.modal = ModalWindows::None;
                res = DialogVariant::Cancel;
            }
         });

        res
    }

    fn save_project_as_ui(&mut self) {
        if self.rack.images.is_empty() && self.rack.plugins.is_empty() { return; }

        let files = rfd::FileDialog::new()
        .set_title("Save project")
        .add_filter(
            "PhotoCosnequences project file (*.viproj)",
            &["viproj"],
        )
        .save_file();
        self.save_path = files;
        self.save_project_ui();
    }

    fn save_project_ui(&mut self) {
        if self.rack.images.is_empty() && self.rack.plugins.is_empty() { return; }
        if self.save_path.is_some() {
            let mut save_path = self.save_path.clone().unwrap();
            println!("{}", save_path.display());
            if save_path.extension().is_none() {
                save_path.set_extension("viproj");
            }

            self.rack.save_project(save_path).unwrap_or_else(|error| { 
                messagebox(
                    "Unable to load project",
                    &error.to_string(),
                );
            });
        } else {
            self.save_project_as_ui();
        }
    }

    fn process(&mut self) {
        self.rack.start_process();
    }

    pub fn update(&mut self, renderer: &mut Renderer) {
        self.rack.process_next();

        if !self.rack.is_finished() && self.timer.elapsed().as_millis() > 100 {
            renderer.texture = None;
            self.timer = Instant::now();
        }
    }

    pub fn draw_ui(
        &mut self,
        context: &Context,
        renderer: &mut Renderer,
        event_loop: &EventLoopWindowTarget<renderer::Event>,
    ) {
        match self.modal {
            ModalWindows::Exit => {
                match self.exit_window(context) {
                    DialogVariant::Yes => {
                        self.save_project_ui();
                        self.exit(renderer);
                    }
                    DialogVariant::No => {
                        self.exit(renderer);
                    },
                    _ => {}
                }
            },
            ModalWindows::ExitNew => {
                match self.exit_window(context) {
                    DialogVariant::Yes => {
                        self.save_project_ui();
                        self.init(renderer);
                        self.modal = ModalWindows::None;
                    }
                    DialogVariant::No => {
                        self.init(renderer);
                        self.modal = ModalWindows::None;
                    },
                    _ => {}
                }
            }
            ModalWindows::About => {
                match  self.about_window(context) {
                    _ => {}
                }
            }
            _ => {}
        }
        egui::TopBottomPanel::bottom("statusbar").show(context, |ui| {
            ui.label(format!("Memory used: {} MiB Processed: {}%", self.rack.calculate_memory_size() / 1024 / 1024, self.rack.compute_complete_percentage()));
        });
        egui::SidePanel::left("left_panel").show(context, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("🖹 New").clicked() {
                        self.modal = ModalWindows::ExitNew;
                    }
                    if ui.button("📂 Open project").clicked() {
                        let files = rfd::FileDialog::new()
                        .set_title("Open project")
                        .add_filter(
                            "PhotoCosnequences project file (*.viproj)",
                            &["viproj"],
                        )
                        .pick_file();

                        if let Some(file) = files {
                            self.load_project(renderer, file).unwrap_or_else(|error| { 
                                messagebox(
                                    "Unable to load project",
                                    &error.to_string(),
                                );
                            });
                        }
                    }
                    ui.separator();
                    ui.add_enabled_ui(!self.rack.images.is_empty() && !self.rack.plugins.is_empty() && self.rack.is_finished(), |ui| {
                        if ui.button("💾 Save").clicked() {
                            self.save_project_ui();
                        }

                        if ui.button("💾 Save as").clicked() {
                            self.save_project_as_ui();
                        }
                        ui.separator();
                        if ui.button("🖼 Export image").clicked() {
                            self.export_image();
                        }
                    });
                    if ui.button("❎ Exit").clicked() {
                        self.modal = ModalWindows::Exit;
                    }
                });

                ui.menu_button("Tools", |ui| {
                    if ui.button("⧯ Generate noise image").clicked() {
                        self.cleanup_image(renderer);
                        self.rack.images.clear();
                        self.rack.images.push(image_generators::generate_noise());
                    }
                });

                ui.menu_button("About", |ui| {
                    if ui.button("About").clicked() {
                        self.modal = ModalWindows::About;
                    }
                    if ui.button("GitHub repository page").clicked() {
                        webbrowser::open("http://github.com/LaineZ/PhotoConsequences").unwrap();
                    }
                });
            });

            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::initial(140.0).at_least(120.0))
                .column(Size::remainder().at_least(80.0))
                .resizable(false)
                .body(|body| {
                    if let Some(act) = self.plugin_table_draw(body) {
                        match act {
                            Action::OpenEditor(id) => self.open_editor(id, renderer, event_loop),
                            Action::Remove(id) => {
                                self.rack.remove_plugin(id);
                                renderer.windows.clear();
                            }
                            Action::Bypass(id) => {
                                self.rack.plugins[id].bypass = !self.rack.plugins[id].bypass;
                            }
                            Action::ChangeInputChannel(id, channel) => {
                                self.rack.plugins[id].input_channel = channel;
                            },
                            Action::ChangeWet(id, wet) => {
                                self.rack.plugins[id].wet = wet;
                            }
                            Action::ChangeOutputChannel(id, value) => {
                                self.rack.plugins[id].output_channel = value;
                            },
                            Action::ChangeSampleRate(id, value) => {
                                self.rack.plugins[id].sample_rate = value;
                            },
                        }
                    }
                });

                ui.with_layout(egui::Layout::from_main_dir_and_cross_align(
                    egui::Direction::TopDown,
                    egui::Align::Center), |ui| { 
                    if ui
                        .add_sized([140.0, 1.0], egui::Button::new("➕ Add VST Effect"))
                        .clicked()
                    {
                        let mut extensions = ["so"];
                        
                        if cfg!(target_os = "windows") {
                            extensions = ["dll"];
                        }
                        if cfg!(target_os = "macos") {
                            extensions = ["vst"];
                        }
    
                        let file = rfd::FileDialog::new()
                            .add_filter("VST 2.4 Plugin", &extensions)
                            .pick_file();
    
                        if let Some(file) = file {
                            self.rack.load_plugin(file).unwrap_or_else(|op| {
                                messagebox(
                                    "Plugin loading failed!",
                                    &op.to_string(),
                                );
                            });
                        }
                    }
                });
        });

        egui::CentralPanel::default().show(context, |ui| {
            ui.horizontal(|ui| {
                if ui.button("📂 Open image").clicked() {
                    let files = rfd::FileDialog::new()
                        .add_filter(
                            "Images",
                            &[
                                "png", "jpg", "jpeg", "gif", "bmp", "ico", "tiff", "webp", "avif",
                                "dds", "tga",
                            ],
                        )
                        .pick_file();

                    if let Some(file) = files {
                        self.load_image(renderer, file).unwrap();
                    }
                }

                ui.add_enabled_ui(!self.rack.images.is_empty(), |ui| {
                    if self.rack.is_finished() {
                        if ui.button("✅ Apply FX on image").clicked() {
                            self.process();
                        }
                    } else {
                        if ui.button("☠ Cancel").clicked() {
                            self.cleanup_image(renderer);
                            self.rack.stop_process();
                        }
                    }
                });

                ui.add_enabled_ui(self.rack.images.len() > 1 && self.rack.is_finished(), |ui| {
                    if ui.button("↻ Undo").clicked() {
                        self.cleanup_image(renderer);
                        self.rack.undo();
                    }
                });
            });

            let plot = Plot::new("items_demo")
                .legend(Legend::default().position(Corner::RightBottom))
                .show_x(false)
                .show_y(false)
                .show_background(false)
                .show_axes([false; 2])
                .data_aspect(1.0);
            if let Some(texture) = &renderer.texture {
                let w = self.rack.images.last().unwrap().width() as f32;
                let h = self.rack.images.last().unwrap().height() as f32;
                let image = PlotImage::new(
                    *texture,
                    PlotPoint::new(0.0, 0.0),
                    vec2(1.0 / h, 1.0 / w),
                );

                plot.show(ui, |plot_ui| {
                    plot_ui.image(image);
                });
            } else {
                plot.show(ui, |plot_ui| {
                    let mut text = RichText::new("Processing image, please wait...").heading();
                    if self.rack.is_finished() {
                        text = RichText::new("Welcome to PhotoConsequences!").heading();
                    }

                    plot_ui.text(Text::new(
                        PlotPoint::new(0.0, 0.0),
                        text,
                    ));
                });

                if !self.rack.images.is_empty() {
                    renderer.destroy_texture();
                    renderer.texture = Some(renderer.upload_texture(self.rack.images.last().unwrap()));
                }
            }
        });
    }
}
