use image::{RgbaImage, imageops::crop};

use crate::models::area::Area;

pub struct SplittedImage {
    location: Area,
    pub data: image::RgbaImage,
    pub needs_update: bool
}

impl SplittedImage {
    pub fn new(location: Area, data: &mut image::RgbaImage) -> Self {
        let crop = crop(data, location.x, location.y, location.width, location.height);

        Self { location, data: crop.to_image(), needs_update: true }
    }

    pub fn location(&self) -> Area {
        self.location
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
                width
            } else {
                width_last_column
            };

            let h = if i == (m - 1) {
                height
            } else {
                height_last_row
            };

            let split = SplittedImage::new(Area::new(i, j, w, h), image);

            result.push(split);
        }
    }

    result
}
