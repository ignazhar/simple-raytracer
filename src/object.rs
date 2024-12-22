use vector3::Vector3;
use crate::{point::Point, color::Color};

/// Object definition
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
    pub fn albedo(&self) -> f32 {
        match self {
            Object::Plane(plane) => plane.albedo,
            Object::Sphere(sphere) => sphere.albedo,
        }
    }
    pub fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match self {
            Object::Sphere(sphere) => sphere.surface_normal(hit_point),
            Object::Plane(plane) => plane.surface_normal(hit_point),
        }
    }
}

// Object primitives: vvv
 
/// Object structs: Sphere
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

impl Sphere {
    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        Vector3::from(*hit_point - self.center).normalize()
    }
}

/// Object structs: Plane
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
}

impl Plane {
    fn surface_normal(&self, _: &Point) -> Vector3 {
        Vector3::zero() - self.normal
    }
}