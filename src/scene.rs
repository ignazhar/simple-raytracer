use crate::{point::Point, rendering::Intersectable, Ray};
use image::Rgba;
use vector3::Vector3;

const MAX: i32 = 256;

/// Color struct
#[derive(Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        let a = (MAX as f32 * self.red) as u8;
        let b = (MAX as f32 * self.green) as u8;
        let c = (MAX as f32 * self.blue) as u8;
        return Rgba([a, b, c, MAX as u8]);
    }
}

/// Object structs: Sphere
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

/// Object structs: Plane
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
}

/// Scene
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    // pub sphere: Sphere,
    // pub spheres: Vec<Sphere>,
    pub objects: Vec<Object>,
}

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn color(&self) -> Color {
        match self {
            Object::Plane(plane) => plane.color,
            Object::Sphere(sphere) => sphere.color,
        }
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray),
            Object::Plane(plane) => plane.intersect(ray),
        }
    }
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Object,
}

impl Intersection<'_> {
    fn new(distance: f64, object: &Object) -> Intersection {
        Intersection {
            distance: distance,
            object: object,
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}
