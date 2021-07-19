#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
}

impl Rect {
    pub fn center(&self) -> (u16, u16) {
        ((self.left + self.right) / 2, (self.top + self.bottom) / 2)
    }

    pub fn area(&self) -> u16 {
        self.width() * self.height()
    }

    pub fn shape_factor(&self) -> f32 {
        let (w, h) = (self.width() as f32, self.height() as f32);
        w.max(h) / w.min(h)
    }

    pub fn width(&self) -> u16 {
        self.right - self.left
    }

    pub fn height(&self) -> u16 {
        self.bottom - self.top
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}
