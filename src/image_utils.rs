use crate::models::area::Area;
use image::RgbaImage;

pub const IMAGE_SPLIT_W: u32 = 128;
pub const IMAGE_SPLIT_H: u32 = 128;

#[derive(Clone)]
pub struct SplittedImage {
    pub image: RgbaImage,
    pub splits: Vec<ImageTile>,
}

impl SplittedImage {
    pub fn new(image: RgbaImage) -> Self {
        let mut splits = Vec::new();
        let width = image.width();
        let height = image.height();

        for y in (0..height).step_by(IMAGE_SPLIT_W as usize) {
            for x in (0..width).step_by(IMAGE_SPLIT_H as usize) {
                let split =
                    ImageTile::new(Area::new(x, y, IMAGE_SPLIT_W as u32, IMAGE_SPLIT_H as u32));
                splits.push(split);
            }
        }

        Self {
            image: image,
            splits,
        }
    }

    pub fn request_all_update(&mut self) {
        for blocks in &mut self.splits {
            blocks.needs_update = true;
        }
    }
}

#[derive(Clone, Copy)]
pub struct ImageTile {
    location: Area,
    pub needs_update: bool,
}

impl ImageTile {
    pub fn new(location: Area) -> Self {
        Self {
            location,
            needs_update: true,
        }
    }

    pub fn location(&self) -> Area {
        self.location
    }
}
