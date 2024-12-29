use image::{DynamicImage, GenericImageView, ImageReader};
use vector3::Vector3;
use crate::{color::Color, point::Point, rendering::{Intersectable, TextureCoords}, ALBEDO};

/// Object definition
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn color(&self, hit_point: &Point) -> Color {
        match self {
            Object::Plane(plane) => plane.material.color.color(&self.texture_coords(hit_point)),
            Object::Sphere(sphere) => sphere.material.color.color(&self.texture_coords(hit_point)),
        }
    }
    pub fn albedo(&self) -> f32 {
        match self {
            Object::Plane(plane) => plane.material.albedo,
            Object::Sphere(sphere) => sphere.material.albedo,
        }
    }
    pub fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match self {
            Object::Plane(plane) => plane.surface_normal(hit_point),
            Object::Sphere(sphere) => sphere.surface_normal(hit_point),
        }
    }
    pub fn surface_type(&self) -> Surface {
        match self {
            Object::Plane(plane) => plane.material.surface,
            Object::Sphere(sphere) => sphere.material.surface,
        }
    }
}

// Object primitives: vvv
 
/// Object structs: Sphere
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material
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
    pub material: Material,
}

impl Plane {
    fn surface_normal(&self, _: &Point) -> Vector3 {
        Vector3::zero() - self.normal
    }
}

/// Textures and shit
pub struct Material {
    pub color: Coloration,
    pub albedo: f32,
    pub surface: Surface,
}

impl Material {
    pub fn from_color(color: Color, albedo: f32, surface: Surface) -> Material {
        Material { color: Coloration::Color(color), albedo: albedo, surface: surface }
    }

    pub fn get_texture(path: &str, scaling: f32, offset: f32, surface: Surface) -> Material {
        let img = ImageReader::open(path).unwrap().decode();
        Material { color: Coloration::Texture{image: img.unwrap(), scaling, offset}, albedo: ALBEDO, surface: surface }
    }

    pub const CHECKERBOARD: &str = "textures/checkerboard6.png";
    pub const FLOOR: &str = "textures/floor1.jpg";
}

pub enum Coloration {
    Color(Color), 
    Texture { image: DynamicImage, scaling: f32, offset: f32 }
}

impl Coloration {
    fn color(&self, texture_coords: &TextureCoords) -> Color {
        match self {
            Coloration::Color(c) => *c,
            Coloration::Texture { image, scaling , offset} => {
                let tex_x = wrap(texture_coords.x * scaling + offset, image.width());
                let tex_y = wrap(texture_coords.y * scaling + offset, image.height());
                Color::from_rgba(image.get_pixel(tex_x, tex_y))
            },
        }
    }
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

/// Surfaces
#[derive(Clone, Copy)]
pub enum Surface {
    Diffusive,
    Reflective { reflectivity: f32 },
    Refractive { transparency: f32, index: f32 },
}

impl Surface {
    pub fn in_transparent(&self) -> bool {
        match self {
            Self::Refractive { transparency: _, index: _ } => true,
            _ => false,
        }
    }
}