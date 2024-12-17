use std::mem::Discriminant;

use crate::point::Point;
use image::Rgba;

const MAX: i32 = 256;

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        let a = (MAX as f32 * self.red)   as u8;
        let b = (MAX as f32 * self.green) as u8;
        let c = (MAX as f32 * self.blue)  as u8;
        return Rgba([a, b, c, MAX as u8]);
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
    // pub spheres: Vec<Sphere>,
}


/*pub struct Intersection {
    pub distance: f64,
    pub object: &Sphere,
}

impl Intersection {
    pub fn new(distance: f64, object: &Sphere) -> Intersection {
        // Elided
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.spheres.min_by();
    }
}*/