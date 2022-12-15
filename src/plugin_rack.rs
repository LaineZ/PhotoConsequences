use std::{
    io::{Cursor, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use image::{imageops::{replace, crop_imm}, io::Reader as ImageReader, RgbaImage};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    editor_wrapper::EditorWrapper,
    image_utils::{self, SplittedImage, IMAGE_SPLIT_H, IMAGE_SPLIT_W},
    models::area::Area,
    processing::{rgba_to_sample, sample_to_rgba},
};
use anyhow::Result;

use vst::{
    host::{Host, HostBuffer, PluginInstance, PluginLoader},
    prelude::Plugin,
};

pub struct PluginHost;

pub struct PluginRack {
    pub host: Arc<Mutex<PluginHost>>,
    pub plugins: Vec<PluginRackInstance>,
    pub images: Vec<Vec<SplittedImage>>,
    /// Current tile processing position
    position: usize,
    /// Total processing tiles
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
    sample_rate: f32,
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
                info!("Found a plugin data LOADING NOW!");
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

impl Host for PluginHost {
    fn automate(&self, index: i32, value: f32) {
        debug!("Parameter {} had its value changed to {}", index, value);
    }

    fn process_events(&self, events: &vst::api::Events) {
        debug!("Plugin called the {:?} event", events.events);
    }
}

impl PluginRack {
    pub fn new() -> Self {
        let host = Arc::new(Mutex::new(PluginHost));
        Self {
            host,
            plugins: Vec::new(),
            images: Vec::new(),
            position: 0,
            total: 0,
            finished: true,
        }
    }

    fn request_all_update(&mut self) {
        for blocks in self.images.last_mut().unwrap() {
            blocks.needs_update = true;
        }
    }

    pub fn undo(&mut self) {
        if self.images.len() > 1 {
            self.images.remove(self.images.len() - 1);
            self.request_all_update();
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

    pub fn load_plugin(&mut self, file: PathBuf) -> anyhow::Result<()> {
        let mut loader = PluginLoader::load(&file, Arc::clone(&self.host))?;
        let instance = loader.instance()?;
        self.insert_plugin(file, instance);
        Ok(())
    }

    pub fn compute_complete_percentage(&self) -> usize {
        (self.position as f32 / self.total as f32 * 100.0) as usize
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

        let split = image_utils::split_image(&mut img.into_rgba8(), IMAGE_SPLIT_W, IMAGE_SPLIT_H);
        self.images.push(split);
        Ok(())
    }

    pub fn load_image_data(&mut self, file: &[u8]) -> anyhow::Result<()> {
        self.images.clear();
        let img = ImageReader::new(Cursor::new(file))
            .with_guessed_format()?
            .decode()?;

        self.images
            .push(image_utils::split_image(&mut img.into_rgba8(), 256, 256));
        Ok(())
    }

    pub fn load_image_rgba(&mut self, image: &mut RgbaImage) {
        self.images.clear();
        self.images.push(image_utils::split_image(image, 256, 256));
    }

    pub fn save_image<P: AsRef<std::path::Path>>(&self, file: P) -> Result<(), image::ImageError> {
        let img = image_utils::join_image(self.images.last().unwrap());
        img.save(file)
    }

    pub fn change_pixel(&mut self, x: u32, y: u32, pixel: image::Rgba<u8>) {
        let x_f = x % 256;
        let y_f = y % 256;

        for tile in self.images.last_mut().unwrap() {
            let loc = Area::new(x, y, 1, 1);
            if loc.check_position(tile.location()) {
                //debug!("{}x{}  x{} y{}", x_f, y_f, x, y);
                tile.data.put_pixel(x_f, y_f, pixel);
                tile.needs_update = true;
                break;
            }
        }
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

        let img = image_utils::join_image(self.images.last().unwrap());
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
        zip.write_all(&bytes)?;

        zip.finish()?;
        Ok(())
    }

    fn insert_plugin(&mut self, file: PathBuf, instance: PluginInstance) {
        self.plugins.push(PluginRackInstance::new(file, instance));
        self.plugins.last_mut().unwrap().initialize().unwrap();
    }

    pub fn remove_plugin(&mut self, id: usize) {
        debug!("Removing: {}", id);
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
            self.images.remove(1);
        }

        self.images.push(img.clone());
        self.finished = false;
        self.position = 0;
        self.total = self.images.last().unwrap().len() - 1;

        for plugin in &mut self.plugins {
            let instance = plugin.instance.as_mut();

            if instance.is_none() {
                continue;
            }

            let instance = instance.unwrap();

            instance.suspend();
            instance.set_block_size(256 * 256);
            instance.resume();
        }
    }

    pub fn stop_process(&mut self) {
        self.images.remove(self.images.len() - 1);
        self.finished = true;
        self.position = 0;
        self.total = 0;
        self.request_all_update();
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

        let last_image = self.images.last_mut().unwrap();

        for plugin in &mut self.plugins {
            let instance = plugin.instance.as_mut();

            if instance.is_none() {
                continue;
            }

            let instance = instance.unwrap();
            let start = std::time::Instant::now();
            let input_count = instance.get_info().inputs as usize;
            let output_count = instance.get_info().outputs as usize;

            if plugin.bypass || input_count == 0 {
                continue;
            }
            debug!("i: {} o: {}", input_count, output_count);
            // zeroing buffers
            let mut buf: HostBuffer<f32> = HostBuffer::new(input_count, output_count);
            let mut inputs: Vec<Vec<f32>> = vec![vec![0.0]; input_count];
            let mut outputs = vec![vec![0.0]; output_count];

            for sample in last_image[self.position].data.pixels() {
                for i in 0..input_count {
                    inputs[i].push(rgba_to_sample(plugin.input_channel, sample))
                }

                for i in 0..output_count {
                    outputs[i].push(0.0);
                }
            }

            let mut audio_buffer = buf.bind(&inputs, &mut outputs);

            debug!("Mapping took: {} ms", start.elapsed().as_millis());

            let start = std::time::Instant::now();
            debug!("processing");
            instance.start_process();
            instance.process(&mut audio_buffer);

            debug!("VST Processing took: {} ms", start.elapsed().as_millis());
            let start = std::time::Instant::now();
            for (pixel, sample) in last_image[self.position]
                .data
                .pixels_mut()
                .zip(&outputs[plugin.output_channel])
            {
                sample_to_rgba(*sample, plugin.wet, pixel, plugin.input_channel);
            }
            debug!("Image return took: {} ms", start.elapsed().as_millis());
        }

        last_image[self.position].needs_update = true;
        if self.total <= self.position {
            self.finished = true;
            for plugin in &mut self.plugins {
                if let Some(instance) = plugin.instance.as_mut() {
                    instance.stop_process();
                    instance.suspend();
                }
            }
            debug!("finished")
        } else {
            self.position += 1;
        }

        debug!("{}/{}", self.position, self.total);
    }

    pub fn process_area(&mut self, area: Area) {
        for plugin in &mut self.plugins {
            let instance = plugin.instance.as_mut();

            if instance.is_none() || plugin.bypass {
                continue;
            }
            let instance = instance.unwrap();
            let input_count = instance.get_info().inputs as usize;
            let output_count = instance.get_info().outputs as usize;

            if input_count == 0 {
                continue;
            }

            let mut buf: HostBuffer<f32> = HostBuffer::new(input_count, output_count);
            let mut inputs: Vec<Vec<f32>> = vec![vec![0.0]; input_count];
            let mut outputs = vec![vec![0.0]; output_count];

            let mut crop_img = None;

            let x_f = area.x % 256;
            let y_f = area.y % 256;
            let mut idx = 0;
            let last_image = self.images.last_mut().unwrap();

            for tile in last_image.iter() {
                if area.check_position(tile.location()) {
                    let crop = crop_imm(&tile.data, x_f, y_f, area.width, area.height);
                    crop_img = Some(crop.to_image());
                    break;
                }
                idx += 1;
            }

            for sample in crop_img.as_mut().unwrap().pixels() {
                for i in 0..input_count {
                    inputs[i].push(rgba_to_sample(plugin.input_channel, sample))
                }

                for i in 0..output_count {
                    outputs[i].push(0.0);
                }
            }

            let mut audio_buffer = buf.bind(&inputs, &mut outputs);

            instance.suspend();
            instance.set_block_size(area.area() as i64);
            instance.resume();
            instance.start_process();
            instance.process(&mut audio_buffer);
            instance.stop_process();
            instance.suspend();

            for (pixel, sample) in crop_img.as_mut().unwrap()
                .pixels_mut()
                .zip(&outputs[plugin.output_channel])
            {
                sample_to_rgba(*sample, plugin.wet, pixel, plugin.input_channel);
            }

            replace(&mut last_image[idx].data, &crop_img.unwrap(), x_f as i64, y_f as i64);

            last_image[idx].needs_update = true;
        }
    }
}
