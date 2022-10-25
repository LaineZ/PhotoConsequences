use egui_wgpu_backend::ScreenDescriptor;
use msgboxwrapper::messagebox;
use renderer::{Renderer, Event};
use ui::State;
use winit::dpi::PhysicalSize;
use winit::event::Event::*;
use egui_winit_platform::{Platform, PlatformDescriptor};
use std::iter;
use std::time::Instant;
use ::egui::FontDefinitions;

pub mod plugin_rack;
pub mod ui;
pub mod renderer;
pub mod msgboxwrapper;
pub mod ui_enums;
pub mod editor_wrapper;
pub mod image_generators;



pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::with_user_event().build();
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title(format!("PhotoConsequences {}", VERSION))
        .build(&event_loop)
        .unwrap();


    let mut renderer = Renderer::new(&window).unwrap_or_else(|op| {
        messagebox("Failed to initialize rendering engine", &format!("PhotoConsequences was unable to initialize graphics engine due to error:\n{}", op));
        std::process::exit(1);
    });

    let size = window.inner_size();
    // We use the egui_winit_platform crate as the platform.
    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let mut state = State::new();
    let mut resized = false;

    // We use the egui_wgpu_backend crate as the render backend

    let start_time = Instant::now();
    event_loop.run(move |event, event_loop, _control_flow| {
        // Pass the winit events to the platform integration.
        platform.handle_event(&event);

        match event {
            RedrawRequested(window_id) => {
                if window_id != window.id() {
                    return;
                }
                platform.update_time(start_time.elapsed().as_secs_f64());

                let output_frame = match renderer.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(wgpu::SurfaceError::Outdated) | Err(wgpu::SurfaceError::Timeout) => {
                        return;
                    },
                    Err(e) => {
                        eprintln!("Dropped frame with error: {}", e);
                        return;
                    }
                };
                let output_view = output_frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Begin to draw the UI frame.
                platform.begin_frame();
                state.draw_ui(&platform.context(), &mut renderer, event_loop);
                state.update(&mut renderer);

                // End the UI frame. We could now handle the output and draw the UI with the backend.
                let full_output = platform.end_frame(Some(&window));
                let paint_jobs = platform.context().tessellate(full_output.shapes);

                let mut encoder =
                    renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("encoder"),
                    });

                // Upload all resources for the GPU.
                let screen_descriptor = ScreenDescriptor {
                    physical_width: renderer.surface_config.width,
                    physical_height: renderer.surface_config.height,
                    scale_factor: window.scale_factor() as f32,
                };
                let tdelta: egui::TexturesDelta = full_output.textures_delta;
                renderer.render_pass
                    .add_textures(&renderer.device, &renderer.queue, &tdelta)
                    .expect("add texture ok");
                renderer.render_pass.update_buffers(&renderer.device, &renderer.queue, &paint_jobs, &screen_descriptor);

                // Record all render passes.
                renderer.render_pass
                    .execute(
                        &mut encoder,
                        &output_view,
                        &paint_jobs,
                        &screen_descriptor,
                        Some(wgpu::Color::BLACK),
                    )
                    .unwrap();
                // Submit the commands.
                renderer.queue.submit(iter::once(encoder.finish()));

                // Redraw egui
                output_frame.present();

                renderer.render_pass
                    .remove_textures(tdelta)
                    .expect("remove texture ok");

                // weird workaround for main window rendering resolution bug
                if !renderer.windows.is_empty() {
                    let size = window.inner_size();
                    if !resized {
                        window.set_inner_size(PhysicalSize::new(size.width + 1, size.height + 1));
                        window.set_inner_size(PhysicalSize::new(size.width - 1, size.height - 1));
                        resized = true;
                    }
                } else {
                    resized = false;
                }
            }
            MainEventsCleared | UserEvent(Event::RequestRedraw) => {
                window.request_redraw();
            }
            
            WindowEvent { event, window_id } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                    // See: https://github.com/rust-windowing/winit/issues/208
                    // This solves an issue where the app would panic when minimizing on Windows.
                    if window_id == window.id() && size.width > 0 && size.height > 0 {
                        renderer.surface_config.width = size.width;
                        renderer.surface_config.height = size.height;
                        renderer.surface.configure(&renderer.device, &renderer.surface_config);
                    }
                }
                winit::event::WindowEvent::CloseRequested => {
                    println!("id: {:?}", window_id);
                    for _ in &renderer.windows {
                        resized = false;
                        state.close_editor(window_id);
                    }
                    renderer.windows.retain(|w| window_id != w.id());
                    if window_id == window.id() {
                        state.exit(&mut renderer);
                    }
                }
                _ => {}
            },
            _ => (),
        }
    });
}
