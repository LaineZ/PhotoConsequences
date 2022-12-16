/// VST Brush area

#[derive(Clone, Copy)]
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Area {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Compares two area positions. returns `true` if area have same position
    pub fn check_position(&self, area: Area) -> bool {
        self.x < area.x + area.width
            && area.x < self.x + self.width
            && self.y < area.y + area.height
            && area.y < self.y + self.height
    }

    /// Computes area of Area
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
