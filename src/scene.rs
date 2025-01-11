use vector3::Vector3;
use crate::{color::Color, object::Object, point::Point, rendering::{Intersectable, Ray}};

/// Scene definition
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub max_recursion_depth: u32,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .filter(|i| !i.distance.is_nan())
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }

    // pub fn trace_for_light(&self, ray: &Ray) -> Option<Intersection> {
    //     self.objects
    //         .iter()
    //         .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
    //         .filter(|i| !i.distance.is_nan())
    //         .filter(|i| !i.object.surface_type().in_transparent())
    //         .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    // }
}

/// Intersection struct
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

/// Light objects
pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

/// Far away parallel lightning struct
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

/// Spherical light sources
pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    pub fn color(&self) -> Color {
        match self {
            Light::Directional(light) => light.color,
            Light::Spherical(light) => light.color,
        }
    }
}