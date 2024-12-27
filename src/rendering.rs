use core::f32;
use vector3::Vector3;

use crate::object::{Object, Plane, Sphere};
use crate::point::Point;
use crate::scene::Scene;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        // TODO: fov adjustment
        let fov_adjustment = 1.0f64;
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((2.0 * (x as f64 + 0.5) / (scene.width as f64) - 1.0) * fov_adjustment) * aspect_ratio;
        let sensor_y = (2.0 * (y as f64 + 0.5) / (scene.height as f64) - 1.0) * fov_adjustment;
        // TODO:
        // sensor_y *= -1
        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }

    pub fn reflect(&self, hit_point: Point, surface_normal: Vector3) -> Ray {
        Ray {
            origin: hit_point + (surface_normal * crate::SHADOW_BIAS).into(),
            direction: self.direction - (surface_normal * 2.0 * self.direction.dot(&surface_normal)),
        }
    }
}

pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray),
            Object::Plane(plane) => plane.intersect(ray),
        }
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        match self {
            Self::Plane(plane) => plane.texture_coords(hit_point),
            Self::Sphere(sphere) => sphere.texture_coords(hit_point),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector3 = Point::from(self.center.clone() - ray.origin.clone()).into();
        let adj = l.dot(&ray.direction);
        let d_sq = l.dot(&l) - adj * adj;

        let radius_sq = self.radius * self.radius;

        if d_sq > radius_sq {
            return None;
        }

        let inside = (radius_sq - d_sq).sqrt();

        // TODO: consider the case that camera is inside the sphere.

        let t0 = adj - inside;
        let t1 = adj + inside;

        if t0 < 0.0 && t1 < 0.0 {
            None
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }

        // Some(adj - inside)
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let hit_vec = *hit_point - self.center;
        // TODO: figure out the formulas:
        // https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/spherical-coordinates-and-trigonometric-functions.html
        // ^^^
        TextureCoords {
            x: (1.0 + (hit_vec.z.atan2(hit_vec.x) as f32) * f32::consts::FRAC_1_PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() as f32 * f32::consts::FRAC_1_PI,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // TODO: figure this out
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html
        // ^^^

        let normal = self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v: Vector3 = (self.origin.clone() - ray.origin.clone()).into();
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.magnitude() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);

        let hit_vec = Vector3::from(*hit_point - self.origin);

        TextureCoords {
            x: hit_vec.dot(&x_axis) as f32,
            y: hit_vec.dot(&y_axis) as f32,
        }
    }
}
