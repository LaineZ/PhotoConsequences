use std::path::PathBuf;

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use vst::host::PluginInstance;
use vst::prelude::Plugin;

use crate::editor_wrapper::EditorWrapper;

#[derive(PartialEq, Eq, Copy, Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InputChannelType {
    Hue = 0,
    Saturation = 1,
    Value = 2,
    Red = 3,
    Green = 4,
    Blue = 5,
    Alpha = 6,
}

#[derive(Serialize, Deserialize)]
pub struct PluginRackInstance {
    #[serde(skip)]
    pub instance: Option<PluginInstance>,
    #[serde(skip)]
    pub editor: EditorWrapper,
    #[serde(rename = "ImageProcessingInput")]
    pub input_channel: InputChannelType,
    #[serde(rename = "AudioProcessingOuput")]
    pub output_channel: usize,
    #[serde(rename = "PluginPath")]
    path: PathBuf,
    #[serde(rename = "PluginData")]
    plugin_data: String,
    #[serde(rename = "Wet")]
    pub wet: f32,
    #[serde(rename = "SampleRate")]
    sample_rate: f32,
    #[serde(rename = "Bypass", default)]
    pub bypass: bool,
}

impl PluginRackInstance {
    pub fn new(path: PathBuf, instance: PluginInstance) -> Self {
        Self {
            instance: Some(instance),
            editor: EditorWrapper::default(),
            input_channel: InputChannelType::Hue,
            output_channel: 0,
            path,
            plugin_data: String::new(),
            wet: 1.0,
            sample_rate: 44100.0,
            bypass: false,
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn initialize(&mut self) -> Result<()> {
        if let Some(inst) = self.instance.as_mut() {
            inst.init();
            let info = inst.get_info();
            self.editor = EditorWrapper::new(
                inst.get_editor(),
                format!("Plugin editor for {} ({})", info.name, info.vendor),
            );
            if !self.plugin_data.is_empty() {
                info!("Found a plugin data LOADING NOW!");
                self.load_block()?;
            }
        }
        Ok(())
    }

    pub fn save_block(&mut self) {
        if let Some(inst) = self.instance.as_mut() {
            let bank_data = inst.get_parameter_object().get_bank_data();
            self.plugin_data = base64::encode(bank_data);
        }
    }

    fn load_block(&mut self) -> Result<()> {
        if let Some(inst) = self.instance.as_mut() {
            inst.get_parameter_object()
                .load_bank_data(&base64::decode(&self.plugin_data)?);
        }
        Ok(())
    }

    pub fn set_sample_rate(&mut self, rate: f32) {
        if let Some(inst) = self.instance.as_mut() {
            inst.suspend();
            inst.set_sample_rate(rate);
        }
    }

    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }
}
