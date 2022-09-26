use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, self},
};

use image::RgbaImage;

use crate::{plugin_rack::PluginRack, ui_enums};

pub enum SendCommand {
    RemovePlugin(usize),
    SaveProject(PathBuf),
    SaveImage(PathBuf),
    LoadPlugin(PathBuf),
    LoadUnitializedPlugins,
    Revert,
    Undo,
    ProcessArray,
}

pub enum RecieveCommand {
    Progress(usize, usize)
}

struct RackThread {
    cmd_rx: Receiver<SendCommand>,
    tx: Sender<Option<RecieveCommand>>,
    pub rack: PluginRack,
}

impl RackThread {
    fn new(cmd_rx: Receiver<SendCommand>, tx: Sender<Option<RecieveCommand>>) -> Self {
        Self {
            cmd_rx,
            tx,
            rack: PluginRack::new(),
        }
    }

    fn run(mut self) -> anyhow::Result<()> {
        loop {
            let cmd = self.cmd_rx.recv()?;

            match cmd {
                SendCommand::RemovePlugin(id) => {
                    self.rack.remove_plugin(id);
                },
                SendCommand::SaveProject(file) => {
                    self.rack.save_project(file)?;
                },
                SendCommand::SaveImage(file) => {
                    self.rack.save_image(file)?;
                },
                SendCommand::LoadPlugin(file) => {
                    self.rack.load_plugin(file)?;
                },
                SendCommand::Revert => {
                    self.rack.revert();
                },
                SendCommand::Undo => {
                    self.rack.undo();
                },
                SendCommand::ProcessArray => {
                    self.rack.process_array();
                },
                SendCommand::LoadUnitializedPlugins => {
                    self.rack.load_uninitialzed_plugins();
                },
            }
        }
    }
}

struct Rack {
    cmd_tx: Sender<SendCommand>,
    rx: Receiver<Option<RecieveCommand>>,
}

impl Rack {
    pub fn new() -> Rack {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (tx, rx) = mpsc::channel();

        std::thread::Builder::new()
            .name("player".into())
            .spawn(move || {
                let rack = RackThread::new(cmd_rx, tx);
            })
            .expect("Can't spawn player thread");

        Rack {
            cmd_tx,
            rx,
        }
    }

}