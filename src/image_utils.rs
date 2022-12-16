use image::{imageops::crop, GenericImage, RgbaImage};
use log::debug;

use crate::models::area::Area;

pub const IMAGE_SPLIT_W: u32 = 128;
pub const IMAGE_SPLIT_H: u32 = 128;

#[derive(Clone)]
pub struct SplittedImage {
    origianl_dimensions: Area,
    pub splits: Vec<ImageTile>,
}

impl SplittedImage {
    pub fn new(image: &mut RgbaImage) -> Self {
        let width = image.width();
        let height = image.height();

        let mut splits = Vec::new();

        for y in (0..height).step_by(IMAGE_SPLIT_W as usize) {
            for x in (0..width).step_by(IMAGE_SPLIT_H as usize) {
                let split = ImageTile::new(
                    Area::new(x, y, IMAGE_SPLIT_W as u32, IMAGE_SPLIT_H as u32),
                    image,
                );
                splits.push(split);
            }
        }
        Self {
            splits,
            origianl_dimensions: Area::new(0, 0, image.width(), image.height()),
        }
    }

    pub fn origianl_dimensions(&self) -> Area {
        self.origianl_dimensions
    }

    pub fn join_image(&self) -> RgbaImage {
        let mut img = RgbaImage::new(
            self.origianl_dimensions.width,
            self.origianl_dimensions.height,
        );
    
        for split in &self.splits {
            img.copy_from(&split.data, split.location.x, split.location.y)
                .unwrap();
        }
        img
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: image::Rgba<u8>) {
        let x_f = x % IMAGE_SPLIT_W;
        let y_f = y % IMAGE_SPLIT_H;

        for tile in self.splits.iter_mut() {
            let loc = Area::new(x, y, 1, 1);
            if loc.check_position(tile.location()) {
                //debug!("{}x{}  x{} y{}", x_f, y_f, x, y);
                tile.data.put_pixel(x_f, y_f, pixel);
                tile.needs_update = true;
                break;
            }
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> Option<&image::Rgba<u8>> {
        let x_f = x % IMAGE_SPLIT_W;
        let y_f = y % IMAGE_SPLIT_H;

        for tile in self.splits.iter() {
            let loc = Area::new(x, y, 1, 1);
            if loc.check_position(tile.location()) {
                //debug!("{}x{}  x{} y{}", x_f, y_f, x, y);
                return Some(tile.data.get_pixel(x_f, y_f))
            }
        }

        None
    }

    pub fn request_all_update(&mut self) {
        for blocks in &mut self.splits {
            blocks.needs_update = true;
        }
    }
}

#[derive(Clone)]
pub struct ImageTile {
    location: Area,
    pub data: image::RgbaImage,
    pub needs_update: bool,
}

impl ImageTile {
    pub fn new(location: Area, data: &mut image::RgbaImage) -> Self {
        let crop = crop(
            data,
            location.x,
            location.y,
            location.width,
            location.height,
        );

        let img = crop.to_image();

        let width = img.width();
        let height = img.height();

        debug!(
            "Created image: x: {} y: {} w: {} h: {} actual: {}x{}",
            location.x, location.y, location.width, location.height, width, height
        );

        Self {
            location,
            data: img,
            needs_update: true,
        }
    }

    pub fn location(&self) -> Area {
        self.location
    }
}
