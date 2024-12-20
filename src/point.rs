use std::ops::Sub;

use vector3::Vector3;

/// Point struct to ...
#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Implementations for point
impl Point {
    pub fn zero() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl From<Point> for Vector3 {
    fn from(value: Point) -> Self {
        Vector3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}