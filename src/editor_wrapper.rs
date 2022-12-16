use std::io::Cursor;

use crate::renderer;
use log::error;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use vst::editor::Editor;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowId, Icon},
};

use image::io::Reader as ImageReader;

#[derive(Default)]
pub struct EditorWrapper {
    pub editor: Option<Box<dyn Editor>>,
    pub window_id: Option<WindowId>,
    name: String
}

impl EditorWrapper {
    pub fn new(editor: Option<Box<dyn Editor>>, name: String) -> Self {
        Self {
            editor,
            name,
            window_id: None,
        }
    }

    pub fn default() -> Self {
        Self {
            editor: None,
            name: String::new(),
            window_id: None,
        }
    }

    pub fn show(
        &mut self,
        event_loop: &EventLoopWindowTarget<renderer::Event>,
    ) -> anyhow::Result<Window> {
        println!("opening editor");


        let mut bytes = include_bytes!("resources/plugin_icon.png");
        let img = ImageReader::new(Cursor::new(&mut bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
    
        let mut icon = None;
    
        if let Ok(icn) = Icon::from_rgba(img.to_rgba8().to_vec(), img.width(), img.height()) {
            icon = Some(icn);
        }

        if let Some(editor) = &mut self.editor {
            let window = winit::window::WindowBuilder::new()
                .with_title(&self.name)
                .with_resizable(false)
                .with_window_icon(icon)
                .with_inner_size(LogicalSize::new(editor.size().0, editor.size().1))
                .build(event_loop)?;
            self.window_id = Some(window.id());
            let whandle = window.raw_window_handle();
            match whandle {
                RawWindowHandle::AppKit(handle) => editor.open(handle.ns_view as _),
                RawWindowHandle::Xlib(handle) => editor.open(handle.window as _),
                RawWindowHandle::Xcb(handle) => editor.open(handle.window as _),
                RawWindowHandle::Win32(handle) => editor.open(handle.hwnd as _),
                _ => anyhow::bail!("GUI Editor is not available for this platform"),
            };
            Ok(window)
        } else {
            anyhow::bail!("GUI Editor is not available for this plugin")
        }
    }

    pub fn close(&mut self, window_id: WindowId) {
        if let Some(window) = self.window_id {
            if window == window_id {
                self.editor.as_mut().unwrap().close()
            }
        }
    }
}
