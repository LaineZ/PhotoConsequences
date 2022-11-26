use image::RgbaImage;
use rand::Rng;

pub fn generate_noise() -> RgbaImage {
    let mut rng = rand::thread_rng();
    let mut image = RgbaImage::new(512, 512);

    for pixel in image.pixels_mut() {
        pixel.0[0] = rng.gen_range(0..255);
        pixel.0[1] = rng.gen_range(0..255);
        pixel.0[2] = rng.gen_range(0..255);
        pixel.0[3] = 255;
    }

    image
}
