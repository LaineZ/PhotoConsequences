use egui::{
    menu,
    plot::{Corner, Legend, Plot, PlotImage, PlotPoint, Text},
    vec2, Align2, Button, Color32, Context, Response, RichText,
};
use egui_extras::{Size, TableBody, TableBuilder};
use log::{debug, trace};
use std::{io::Read, ops::Neg, path::PathBuf, time::Instant};
use vst::prelude::Plugin;
use winit::{dpi::PhysicalSize, event_loop::EventLoopWindowTarget, window::WindowId};

use crate::{
    image_generators,
    models::{
        area::Area,
        ui_enums::{Action, DialogVariant, ModalWindows},
    },
    msgboxwrapper::messagebox,
    rack::{
        instance::{InputChannelType, PluginRackInstance},
        PluginRack,
    },
    renderer::{self, Renderer},
};

pub struct State {
    rack: PluginRack,
    modal: ModalWindows,
    save_path: Option<PathBuf>,
    timer: Instant,
    grid_enabled: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            rack: PluginRack::new(),
            modal: ModalWindows::None,
            save_path: None,
            timer: Instant::now(),
            grid_enabled: false,
        }
    }

    pub fn load_image(&mut self, renderer: &mut Renderer, file: PathBuf) -> anyhow::Result<()> {
        renderer.clear_render();
        self.rack.load_image(file)?;
        Ok(())
    }

    pub fn resize_editors(&mut self, renderer: &mut Renderer) {
        for window in renderer.windows.iter_mut() {
            for plugin in self.rack.plugins.iter_mut() {
                if let Some(editor) = &mut plugin.editor.editor {
                    if editor.is_open() && plugin.editor.window_id == Some(window.id()) {
                        window.set_inner_size(PhysicalSize::new(editor.size().0, editor.size().1));
                    }
                }
            }
        }
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
                messagebox("Unable to open editor", &error.to_string());
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
        let instacnes: Vec<PluginRackInstance> = serde_json::from_str(&proj_file_string)?;

        renderer.clear_render();
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
            .add_filter("JPEG Image", &["jpg", "jpeg"])
            .add_filter("PNG Image", &["png"])
            .add_filter("GIF Image", &["gif"])
            .add_filter("BMP Image", &["bmp"])
            .add_filter("ICO Image", &["ico"])
            .add_filter("TIFF Image", &["tiff"])
            .add_filter("WebP Image", &["webp"])
            .add_filter("TGA Image", &["tga"])
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
                        ui.label(format!("{}", name.get_path().display()))
                            .on_hover_text(
                                "This plugin is not initialized\nEffect configuration is preserved",
                            );
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
                    ui.label(&info.name)
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
                    if ui.selectable_value(&mut radio, InputChannelType::Red, "Red").clicked() {
                        action = Some(Action::ChangeInputChannel(idx, radio))
                    };
                    if ui.selectable_value(&mut radio, InputChannelType::Green, "Green").clicked() {
                        action = Some(Action::ChangeInputChannel(idx, radio))
                    };
                    if ui.selectable_value(&mut radio, InputChannelType::Blue, "Blue").clicked() {
                        action = Some(Action::ChangeInputChannel(idx, radio))
                    };
                    if ui.selectable_value(&mut radio, InputChannelType::Alpha, "Alpha").clicked() {
                        action = Some(Action::ChangeInputChannel(idx, radio))
                    };

                    ui.label("Output audio channel:");
                    ui.separator();
                    let mut output = name.output_channel;
                    let prefix = match output {
                        0 => { "Left/Mono " }
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

                    let mut sample_rate = name.get_sample_rate();
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
        renderer.clear_render();
        renderer.windows.clear();
        self.rack = PluginRack::new();
        self.save_path = None;
    }

    fn exit_window(&mut self, context: &Context) -> DialogVariant {
        if self.rack.images.is_empty() && self.rack.plugins.is_empty() {
            return DialogVariant::No;
        }
        let mut res = DialogVariant::None;
        egui::Window::new("Project management")
            .collapsible(false)
            .auto_sized()
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(context, |ui| {
                if self.save_path.is_some() {
                    ui.label(format!(
                        "Save {} before exiting?",
                        self.save_path.as_ref().unwrap().display()
                    ));
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

    fn about_window(&mut self, context: &Context) {
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
                }
            });
    }

    fn save_project_as_ui(&mut self) {
        if self.rack.images.is_empty() && self.rack.plugins.is_empty() {
            return;
        }

        let files = rfd::FileDialog::new()
            .set_title("Save project")
            .add_filter("PhotoCosnequences project file (*.viproj)", &["viproj"])
            .save_file();
        self.save_path = files;
        self.save_project_ui();
    }

    fn save_project_ui(&mut self) {
        if self.rack.images.is_empty() && self.rack.plugins.is_empty() {
            return;
        }
        if self.save_path.is_some() {
            let mut save_path = self.save_path.clone().unwrap();
            debug!("{}", save_path.display());
            if save_path.extension().is_none() {
                save_path.set_extension("viproj");
            }

            self.rack.save_project(save_path).unwrap_or_else(|error| {
                messagebox("Unable to load project", &error.to_string());
            });
        } else {
            self.save_project_as_ui();
        }
    }

    fn process(&mut self) {
        self.rack.start_process();
    }

    fn mouse_movement(
        &mut self,
        position: PlotPoint,
        response: Response,
        _renderer: &mut Renderer,
    ) {
        if response.dragged() {
            //debug!("x: {} y: {}", (position.x as f64), (position.y as f64));

            let xpos = position.x as i32;
            let ypos = position.y as i32;

            let w = 32;
            let h = 32;

            self.rack.process_area(Area::new(
                (xpos as u32).saturating_sub(w / 2),
                (ypos as u32).saturating_sub(h / 2),
                w,
                h,
            ));
        }
    }

    pub fn update(&mut self, renderer: &mut Renderer) {
        self.rack.process_next();
        self.resize_editors(renderer);
        //println!("{:#?}", renderer.windows);

        if self.timer.elapsed().as_millis() > 33 && !self.rack.images.is_empty() {
            let img = self.rack.images.last_mut().unwrap();
            for (i, img) in img.splits.iter_mut().enumerate() {
                if img.needs_update {
                    if let Some(idx) = renderer.textures.get_mut(i) {
                        idx.cleanup_image();
                    }
                    renderer.upload_texture(&img.data, i);
                    renderer.textures[i].location.x = img.location().x;
                    renderer.textures[i].location.y = img.location().y;
                    img.needs_update = false;
                    trace!("{}: updated", i);
                }
            }
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
            ModalWindows::Exit => match self.exit_window(context) {
                DialogVariant::Yes => {
                    self.save_project_ui();
                    self.exit(renderer);
                }
                DialogVariant::No => {
                    self.exit(renderer);
                }
                _ => {}
            },
            ModalWindows::ExitNew => match self.exit_window(context) {
                DialogVariant::Yes => {
                    self.save_project_ui();
                    self.init(renderer);
                    self.modal = ModalWindows::None;
                }
                DialogVariant::No => {
                    self.init(renderer);
                    self.modal = ModalWindows::None;
                }
                _ => {}
            },
            ModalWindows::About => {
                self.about_window(context);
            }
            _ => {}
        }
        egui::TopBottomPanel::bottom("statusbar").show(context, |ui| {
            ui.label(format!(
                "Processed: {}%",
                self.rack.compute_complete_percentage()
            ));
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
                            .add_filter("PhotoCosnequences project file (*.viproj)", &["viproj"])
                            .pick_file();

                        if let Some(file) = files {
                            self.load_project(renderer, file).unwrap_or_else(|error| {
                                messagebox("Unable to load project", &error.to_string());
                            });
                        }
                    }
                    ui.separator();
                    ui.add_enabled_ui(
                        !self.rack.images.is_empty()
                            && !self.rack.plugins.is_empty()
                            && self.rack.is_finished(),
                        |ui| {
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
                        },
                    );
                    if ui.button("❎ Exit").clicked() {
                        self.modal = ModalWindows::Exit;
                    }
                });

                ui.menu_button("Tools", |ui| {
                    if ui.button("⧯ Generate noise image").clicked() {
                        self.rack
                            .load_image_rgba(&mut image_generators::generate_noise());
                    }
                });

                ui.menu_button("About", |ui| {
                    if ui.button("ℹ About").clicked() {
                        self.modal = ModalWindows::About;
                    }

                    if ui.button("⬌ GitHub repository page").clicked() {
                        webbrowser::open("http://github.com/LaineZ/PhotoConsequences").unwrap();
                    }
                });
            });

            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::initial(100.0).at_least(80.0))
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
                            }
                            Action::ChangeWet(id, wet) => {
                                self.rack.plugins[id].wet = wet;
                            }
                            Action::ChangeOutputChannel(id, value) => {
                                self.rack.plugins[id].output_channel = value;
                            }
                            Action::ChangeSampleRate(id, value) => {
                                self.rack.plugins[id].set_sample_rate(value);
                            }
                        }
                    }
                });

            ui.with_layout(
                egui::Layout::from_main_dir_and_cross_align(
                    egui::Direction::TopDown,
                    egui::Align::Center,
                ),
                |ui| {
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
                                messagebox("Plugin loading failed!", &op.to_string());
                            });
                        }
                    }
                },
            );
        });

        egui::CentralPanel::default().show(context, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(self.rack.is_finished(), Button::new("📂 Open image"))
                    .clicked()
                {
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
                    ui.checkbox(&mut self.grid_enabled, "Grid");
                    if self.rack.is_finished() {
                        if ui.button("✅ Apply FX on image").clicked() {
                            self.process();
                        }
                    } else {
                        if ui.button("☠ Cancel").clicked() {
                            self.rack.stop_process();
                        }
                    }
                });

                ui.add_enabled_ui(
                    self.rack.images.len() > 1 && self.rack.is_finished(),
                    |ui| {
                        if ui.button("↻ Undo").clicked() {
                            self.rack.undo();
                        }
                    },
                );
            });

            let plot = Plot::new("photo_preview")
                .legend(Legend::default().position(Corner::RightBottom))
                .show_x(self.grid_enabled)
                .show_y(self.grid_enabled)
                .show_background(false)
                .show_axes([self.grid_enabled; 2])
                .allow_drag(false)
                .data_aspect(1.0);

            let mut mouse_position = None;

            if !renderer.textures.is_empty() {
                let response = plot.show(ui, |plot_ui| {
                    for txt in renderer.textures.iter_mut() {
                        if txt.native.is_none() {
                            continue;
                        }
                        if let Some(id) = txt.id() {
                            let image = PlotImage::new(
                                id,
                                PlotPoint::new(
                                    (txt.location.width as f32 / 2.0) + txt.location.x as f32,
                                    txt.location.height as f32 / -2.0 - txt.location.y as f32,
                                ),
                                vec2(txt.location.width as f32, txt.location.height as f32),
                            );
                            plot_ui.image(image);
                        }
                    }

                    mouse_position = plot_ui.pointer_coordinate();
                });

                if let Some(mut position) = mouse_position {
                    position.y = position.y.neg();

                    if position.x.is_sign_positive() && position.y.is_sign_positive() {
                        self.mouse_movement(position, response.response, renderer);
                    }
                }
                // check for destroyed textures
                for txt in renderer.textures.iter_mut() {
                    txt.destroy_texture();
                }
            } else {
                ui.add_enabled_ui(false, |ui| {
                    plot.show(ui, |plot_ui| {
                        let mut text = RichText::new("Processing image, please wait...").heading();
                        if self.rack.is_finished() {
                            text = RichText::new("Welcome to PhotoConsequences!").heading();
                        }

                        plot_ui.text(Text::new(PlotPoint::new(0.0, 0.0), text));
                    });
                });
            }
        });
    }
}
