use std::ops::{Add, Mul};

use crate::{point::Point, rendering::Intersectable, Ray};
use image::{Rgb, Rgba};
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

    pub fn abs(&self) -> Color {
        Self {
            red: self.red.abs(),
            green: self.green.abs(),
            blue: self.blue.abs(),
        }
    }

    pub fn clamp(&self) -> Color {
        Self {
            red: self.red.max(0.0).min(1.0),
            green: self.green.max(0.0).min(1.0),
            blue: self.blue.max(0.0).min(0.0),
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

/// Scene
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Object>,
    pub light: Light,
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
            // .filter(|i| i.distance >= 1e-1)
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}

/// Far away parallel lightning struct
pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}
