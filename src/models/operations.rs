use image::imageops::crop;

use crate::ui::State;

use super::area::Area;

pub trait Operation {
    fn name(&self) -> &str {
        "Operation"
    }

    fn apply(&mut self, _state: &mut State) {}

    fn undo(&mut self, _state: &mut State) {}
}

pub struct BrushDrawOperation {
    area: Area,
    wet: f32,
}

impl BrushDrawOperation {
    pub fn new(area: Area, wet: f32) -> Self {
        Self { area, wet }
    }
}

impl Operation for BrushDrawOperation {
    fn apply(&mut self, state: &mut State) {
        state.rack.process_area(self.area, self.wet);
    }

    fn undo(&mut self, state: &mut State) {
        let first_image = &mut state.rack.layers[0];
        let crop = crop(
            &mut first_image.image,
            self.area.x,
            self.area.y,
            self.area.width,
            self.area.height,
        );
        let mut crop_img = crop.to_image();
    }
}
