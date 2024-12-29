use image::Rgba;
use std::ops::{Add, AddAssign, Mul};

const MAX: f32 = 255.0;

/// Color struct
#[derive(Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

// Implementations for Color
impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        return Rgba([
            (MAX * self.red) as u8,
            (MAX * self.green) as u8,
            (MAX * self.blue) as u8,
            MAX as u8,
        ]);
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: rgba.0[0] as f32 / MAX,
            green: rgba.0[1] as f32 / MAX,
            blue: rgba.0[2] as f32 / MAX,
        }
    }

    pub fn clamp(&self) -> Color {
        Self {
            red: self.red.max(0.0).min(1.0),
            green: self.green.max(0.0).min(1.0),
            blue: self.blue.max(0.0).min(1.0),
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Color constants
impl Color {
    pub const WHITE: Color = Color { red: 1.0, green: 1.0, blue: 1.0, };
    pub const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0, };
    pub const YELLOW: Color = Color { red: 1.0, green: 1.0, blue: 0.0, };
    pub const DARK_ORANGE: Color = Color { red: 1.0, green: 0.6, blue: 0.0, };
    pub const RED: Color = Color { red: 1.0, green: 0.0, blue: 0.0, };
    pub const LIGHT_GREEN: Color = Color { red: 0.4, green: 1.0, blue: 0.4, };
    pub const MAGENTA: Color = Color { red: 0.8, green: 0.1, blue: 0.8, };
    pub const DARK_BLUE: Color = Color { red: 0.4, green: 0.4, blue: 0.8, };
    pub const LIGHT_BLUE: Color = Color { red: 0.6, green: 0.9, blue: 1.0, };
}
