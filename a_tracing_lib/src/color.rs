/// Struct holding a color in rgb format using f32's
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        let r = (value.r.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (value.g.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (value.b.clamp(0.0, 1.0) * 255.0) as u8;
        [r, g, b, 255]
    }
}
