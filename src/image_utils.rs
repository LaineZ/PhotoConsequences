use image::{imageops::crop, GenericImage, RgbaImage};
use log::debug;

use crate::models::area::Area;

pub const IMAGE_SPLIT_W: usize = 128;
pub const IMAGE_SPLIT_H: usize = 128;

#[derive(Clone)]
pub struct SplittedImage {
    location: Area,
    origianl_dimensions: Area,
    pub data: image::RgbaImage,
    pub needs_update: bool,
}

impl SplittedImage {
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
            origianl_dimensions: Area::new(0, 0, data.width(), data.height()),
            needs_update: true,
        }
    }

    pub fn location(&self) -> Area {
        self.location
    }

    pub fn origianl_dimensions(&self) -> Area {
        self.origianl_dimensions
    }
}

pub fn split_image(
    image: &mut RgbaImage,
    tile_width: usize,
    tile_height: usize,
) -> Vec<SplittedImage> {
    let width = image.width();
    let height = image.height();

    let mut result = Vec::new();

    for x in (0..width).step_by(tile_width) {
        for y in (0..height).step_by(tile_height) {
            let split = SplittedImage::new(
                Area::new(x, y, tile_width as u32, tile_height as u32),
                image,
            );
            result.push(split);
        }
    }
    result
}

pub fn join_image(splits: &Vec<SplittedImage>) -> RgbaImage {
    let mut img = RgbaImage::new(
        splits[0].origianl_dimensions.width,
        splits[0].origianl_dimensions.height,
    );

    for split in splits {
        img.copy_from(&split.data, split.location.x, split.location.y)
            .unwrap();
    }

    img
}
