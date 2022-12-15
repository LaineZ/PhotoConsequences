use image::Rgba;
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

pub fn sample_to_rgba(
    sample: f32,
    wet: f32,
    pixel: &mut Rgba<u8>,
    input_channel: InputChannelType,
) {
    let mut srgb = Srgba::new(
        pixel.0[0] as f32 / 255.0,
        pixel.0[1] as f32 / 255.0,
        pixel.0[2] as f32 / 255.0,
        pixel.0[3] as f32 / 255.0,
    );

    match input_channel {
        InputChannelType::Hue => {
            let mut hsv = Hsva::from_color(srgb);
            hsv.hue = palette::RgbHue::from_degrees((sample * 360.0) * wet);
            srgb = Srgba::from_color(hsv);
        }
        InputChannelType::Saturation => {
            let mut hsv = Hsva::from_color(srgb);
            hsv.saturation = sample * wet;
            srgb = Srgba::from_color(hsv);
        }
        InputChannelType::Value => {
            let mut hsv = Hsva::from_color(srgb);
            hsv.value = sample * wet;
            srgb = Srgba::from_color(hsv);
        }
        InputChannelType::Red => {
            srgb.red = sample * wet;
        }
        InputChannelType::Green => {
            srgb.green = sample * wet;
        }
        InputChannelType::Blue => {
            srgb.blue = sample * wet;
        }
        InputChannelType::Alpha => {
            srgb.alpha = sample * wet;
        }
    }

    pixel.0[0] = (srgb.red * 255.0) as u8;
    pixel.0[1] = (srgb.green * 255.0) as u8;
    pixel.0[2] = (srgb.blue * 255.0) as u8;
    pixel.0[3] = (srgb.alpha * 255.0) as u8;
}
