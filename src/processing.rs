use palette::{FromColor, Hsva, Srgba};

use crate::plugin_rack::InputChannelType;

pub fn rgba_to_sample(input: InputChannelType, sample: &image::Rgba<u8>) -> f32 {
    let srgb = Srgba::new(
        sample.0[0] as f32 / 255.0,
        sample.0[1] as f32 / 255.0,
        sample.0[2] as f32 / 255.0,
        sample.0[3] as f32 / 255.0,
    );

    match input {
        InputChannelType::Hue => {
            let hsv = Hsva::from_color(srgb);
            hsv.hue.to_positive_degrees() / 360.0
        }
        InputChannelType::Saturation => {
            let hsv = Hsva::from_color(srgb);
            hsv.saturation
        }
        InputChannelType::Value => {
            let hsv = Hsva::from_color(srgb);
            hsv.value
        }
        InputChannelType::Red => srgb.red,
        InputChannelType::Green => srgb.green,
        InputChannelType::Blue => srgb.blue,
        InputChannelType::Alpha => srgb.alpha,
    }
}
