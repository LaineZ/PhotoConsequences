use std::{
    io::{Cursor, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use image::io::Reader as ImageReader;
use palette::{FromColor, Hsva, RgbHue, Srgba};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::editor_wrapper::EditorWrapper;
use anyhow::Result;

use vst::{
    host::{Host, HostBuffer, PluginInstance, PluginLoader},
    prelude::Plugin,
};

pub struct PluginHost;

pub struct PluginRack {
    pub host: Arc<Mutex<PluginHost>>,
    pub plugins: Vec<PluginRackInstance>,
    pub block_size: i64,
    pub images: Vec<image::RgbaImage>,
    position: usize,
    total: usize,
    finished: bool,
}

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
    pub sample_rate: f32,
    #[serde(rename = "Bypass", default)]
    pub bypass: bool,
}

impl PluginRackInstance {
    fn new(path: PathBuf, instance: PluginInstance) -> Self {
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

    fn initialize(&mut self) -> Result<()> {
        if let Some(inst) = self.instance.as_mut() {
            inst.init();
            self.editor = EditorWrapper::new(inst.get_editor());
            if !self.plugin_data.is_empty() {
                println!("found a plugin data LOADING NOW!");
                self.load_block()?;
            }
        }
        Ok(())
    }

    fn save_block(&mut self) {
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
}

impl Host for PluginHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }

    fn process_events(&self, events: &vst::api::Events) {
        println!("Plugin called the {:?} event", events.events);
    }
}

impl PluginRack {
    pub fn new() -> Self {
        let host = Arc::new(Mutex::new(PluginHost));
        Self {
            host,
            plugins: Vec::new(),
            images: Vec::new(),
            block_size: 8192,
            position: 0,
            total: 0,
            finished: true,
        }
    }

    pub fn undo(&mut self) {
        if self.images.len() > 1 {
            self.images.remove(self.images.len() - 1);
        }
    }

    pub fn calculate_memory_size(&self) -> usize {
        let mut size = 0;
        for image in &self.images {
            size += std::mem::size_of_val(image) * image.len();
        }
        size
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn revert(&mut self) {
        if self.images.len() > 1 {
            self.images.drain(1..);
        }
    }

    pub fn load_plugin(&mut self, file: PathBuf) -> anyhow::Result<()> {
        let mut loader = PluginLoader::load(&file, Arc::clone(&self.host))?;
        let instance = loader.instance()?;
        self.insert_plugin(file, instance);
        Ok(())
    }

    pub fn compute_complete_percentage(&self) -> usize {
        self.position.checked_div(self.total).unwrap_or(0) * 100
    }

    pub fn get_processed_position(&self) -> usize {
        self.position
    }

    pub fn get_processing_size(&self) -> usize {
        self.total
    }

    pub fn load_uninitialzed_plugins(&mut self) -> anyhow::Result<()> {
        for plugin in &mut self.plugins {
            if let Ok(mut loader) = PluginLoader::load(&plugin.path, Arc::clone(&self.host)) {
                if let Ok(instance) = loader.instance() {
                    plugin.instance = Some(instance);
                    plugin.initialize()?;
                }
            }
        }

        Ok(())
    }

    pub fn load_image<P: AsRef<std::path::Path>>(&mut self, file: P) -> anyhow::Result<()> {
        self.images.clear();
        let img = ImageReader::open(file)?.decode()?;
        self.images.push(img.to_rgba8());
        Ok(())
    }

    pub fn load_image_data(&mut self, file: &[u8]) -> anyhow::Result<()> {
        self.images.clear();
        let img = ImageReader::new(Cursor::new(file))
            .with_guessed_format()?
            .decode()?;
        self.images.push(img.to_rgba8());
        Ok(())
    }

    pub fn save_image<P: AsRef<std::path::Path>>(&self, file: P) -> Result<(), image::ImageError> {
        self.images.last().unwrap().save(file)
    }

    pub fn save_project(&mut self, file: std::path::PathBuf) -> anyhow::Result<()> {
        for plugin in &mut self.plugins {
            plugin.save_block();
        }

        let file = std::fs::File::create(&file).unwrap();

        let mut zip = zip::ZipWriter::new(file);

        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Zstd);

        zip.start_file("project.json", options)?;
        let j = serde_json::to_string(&self.plugins)?;
        zip.write_all(j.as_bytes())?;

        zip.start_file("image.png", options)?;
        let mut bytes: Vec<u8> = Vec::new();
        self.images
            .last()
            .unwrap()
            .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
        zip.write_all(&bytes)?;

        zip.finish()?;
        Ok(())
    }

    fn insert_plugin(&mut self, file: PathBuf, instance: PluginInstance) {
        self.plugins.push(PluginRackInstance::new(file, instance));
        self.plugins.last_mut().unwrap().initialize().unwrap();
    }

    pub fn remove_plugin(&mut self, id: usize) {
        println!("removing: {}", id);
        if let Some(instance) = self.plugins[id].instance.as_mut() {
            instance.suspend();
        }
        self.plugins.remove(id);
    }

    pub fn start_process(&mut self) {
        if self.plugins.is_empty() || self.images.is_empty() {
            return;
        }

        let img = self.images.last().unwrap().clone();

        if self.images.len() >= 2 {
            //println!("{}", img.len());
            self.images.remove(1);
        }

        self.images.push(img);
        self.finished = false;
        self.position = 0;
        self.total = 0;
    }

    pub fn stop_process(&mut self) {
        self.images.remove(self.images.len() - 1);
        self.finished = true;
        self.position = 0;
        self.total = 0;
    }

    pub fn can_update_ui(&self) -> bool {
        self.position % self.block_size as usize * 2 == 0
    }

    /// Lazy iterative processing of VST effects (should called in a loop)
    pub fn process_next(&mut self) {
        if self.plugins.is_empty() {
            self.finished = true;
            return;
        }

        if self.finished || self.plugins.is_empty() {
            return;
        }

        //let full_process_time = std::time::Instant::now();

        for plugin in &mut self.plugins {
            let instance = plugin.instance.as_mut();

            if instance.is_none() {
                continue;
            }

            let instance = instance.unwrap();
            //let start = std::time::Instant::now();
            let input_count = instance.get_info().inputs as usize;
            let output_count = instance.get_info().outputs as usize;

            if plugin.bypass || input_count == 0 {
                continue;
            }
            //println!("i: {} o: {}", input_count, output_count);
            // zeroing buffers
            let mut buf: HostBuffer<f32> = HostBuffer::new(input_count, output_count);
            let mut inputs: Vec<Vec<f32>> = vec![vec![0.0]; input_count];
            let mut outputs = vec![vec![0.0]; output_count];

            for sample in self
                .images
                .last()
                .unwrap()
                .pixels()
                .skip(self.position)
                .take(self.block_size as usize)
            {
                let srgb = Srgba::new(
                    sample.0[0] as f32 / 255.0,
                    sample.0[1] as f32 / 255.0,
                    sample.0[2] as f32 / 255.0,
                    sample.0[3] as f32 / 255.0,
                );
                match plugin.input_channel {
                    InputChannelType::Hue => {
                        let hsv = Hsva::from_color(srgb);
                        for i in 0..input_count {
                            inputs[i].push(hsv.hue.to_positive_degrees() / 360.0);
                        }
                    }
                    InputChannelType::Saturation => {
                        let hsv = Hsva::from_color(srgb);
                        for i in 0..input_count {
                            inputs[i].push(hsv.saturation);
                        }
                    }
                    InputChannelType::Value => {
                        let hsv = Hsva::from_color(srgb);
                        for i in 0..input_count {
                            inputs[i].push(hsv.value);
                        }
                    }
                    InputChannelType::Red => {
                        for i in 0..input_count {
                            inputs[i].push(srgb.red);
                        }
                    }
                    InputChannelType::Green => {
                        for i in 0..input_count {
                            inputs[i].push(srgb.green);
                        }
                    }
                    InputChannelType::Blue => {
                        for i in 0..input_count {
                            inputs[i].push(srgb.blue);
                        }
                    }
                    InputChannelType::Alpha => {
                        for i in 0..input_count {
                            inputs[i].push(srgb.alpha);
                        }
                    }
                }

                for i in 0..output_count {
                    outputs[i].push(0.0);
                }
            }

            let mut audio_buffer = buf.bind(&inputs, &mut outputs);

            //println!("Mapping took: {} ms", start.elapsed().as_millis());

            //let start = std::time::Instant::now();
            //println!("processing");
            instance.suspend();
            instance.set_sample_rate(plugin.sample_rate);
            instance.set_block_size(inputs[0].len() as i64);
            instance.resume();
            instance.start_process();
            instance.process(&mut audio_buffer);
            instance.stop_process();
            instance.suspend();

            //println!("VST Processing took: {} ms", start.elapsed().as_millis());
            //let start = std::time::Instant::now();
            for (pixel, sample) in self
                .images
                .last_mut()
                .unwrap()
                .pixels_mut()
                .skip(self.position)
                .take(self.block_size as usize)
                .zip(&outputs[plugin.output_channel])
            {
                let mut srgb = Srgba::new(
                    pixel.0[0] as f32 / 255.0,
                    pixel.0[1] as f32 / 255.0,
                    pixel.0[2] as f32 / 255.0,
                    pixel.0[3] as f32 / 255.0,
                );

                match plugin.input_channel {
                    InputChannelType::Hue => {
                        let mut hsv = Hsva::from_color(srgb);
                        hsv.hue = RgbHue::from_degrees((*sample * 360.0) * plugin.wet);
                        srgb = Srgba::from_color(hsv);
                    }
                    InputChannelType::Saturation => {
                        let mut hsv = Hsva::from_color(srgb);
                        hsv.saturation = *sample * plugin.wet;
                        srgb = Srgba::from_color(hsv);
                    }
                    InputChannelType::Value => {
                        let mut hsv = Hsva::from_color(srgb);
                        hsv.value = *sample * plugin.wet;
                        srgb = Srgba::from_color(hsv);
                    }
                    InputChannelType::Red => {
                        srgb.red = *sample * plugin.wet;
                    }
                    InputChannelType::Green => {
                        srgb.green = *sample * plugin.wet;
                    }
                    InputChannelType::Blue => {
                        srgb.blue = *sample * plugin.wet;
                    }
                    InputChannelType::Alpha => {
                        srgb.alpha = *sample * plugin.wet;
                    }
                }

                pixel.0[0] = (srgb.red * 255.0) as u8;
                pixel.0[1] = (srgb.green * 255.0) as u8;
                pixel.0[2] = (srgb.blue * 255.0) as u8;
                pixel.0[3] = (srgb.alpha * 255.0) as u8;
            }
            //println!("Image return took: {} ms", start.elapsed().as_millis());
        }

        if self.total == 0 {
            self.total = self.images.last().unwrap().pixels().len();
        }

        if ((self.total as f32 * 1.2) as usize) < self.position {
            self.finished = true;
        } else {
            self.position += self.block_size as usize;
            //println!("processing: {} {} {}", len, self.position, self.block_size);
        }
    }
}
