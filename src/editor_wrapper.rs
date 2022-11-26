use crate::renderer;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use vst::editor::Editor;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct EditorWrapper {
    pub editor: Option<Box<dyn Editor>>,
    pub window_id: Option<WindowId>,
}

impl EditorWrapper {
    pub fn new(editor: Option<Box<dyn Editor>>) -> Self {
        Self {
            editor,
            window_id: None,
        }
    }

    pub fn default() -> Self {
        Self {
            editor: None,
            window_id: None,
        }
    }

    pub fn show(
        &mut self,
        event_loop: &EventLoopWindowTarget<renderer::Event>,
    ) -> anyhow::Result<Window> {
        println!("opening editor");

        if let Some(editor) = &mut self.editor {
            let window = winit::window::WindowBuilder::new()
                .with_resizable(false)
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
