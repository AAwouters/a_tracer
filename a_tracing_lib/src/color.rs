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
pub const SKYBLUE: Color = Color::new(0.5, 0.7, 1.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        self + ((rhs - self) * t)
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

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl std::ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(rhs.r * self, rhs.g * self, rhs.b * self)
    }
}

impl std::ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl std::ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}
