use image::{imageops::crop, GenericImage, RgbaImage};

use crate::models::area::Area;

pub const IMAGE_SPLIT_W: u32 = 8;
pub const IMAGE_SPLIT_H: u32 = 8;

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

        println!(
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

pub fn split_image(image: &mut RgbaImage, m: u32, n: u32) -> Vec<SplittedImage> {
    let width = image.width() / m;
    let height = image.height() / n;

    let width_last_column = width + (image.width() % width);
    let height_last_row = height + (image.height() % height);

    let mut result = Vec::new();

    for i in 0..n {
        for j in 0..m {
            let w = if j == (m - 1) {
                width_last_column
            } else {
                width
            };

            let h = if i == (m - 1) {
                height_last_row
            } else {
                height
            };

            let split = SplittedImage::new(Area::new(i * w, j * h, w, h), image);
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
