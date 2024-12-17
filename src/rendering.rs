use crate::scene::{Scene, Sphere};
use vector3::Vector3;
use crate::point::Point;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        // TODO: fov adjustment
        let fov_adjustment = 1.0f64;
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = ((2.0 * (x as f64 + 0.5) / (scene.width as f64)  - 1.0) * fov_adjustment) * aspect_ratio;
        let sensor_y =  (2.0 * (y as f64 + 0.5) / (scene.height as f64) - 1.0) * fov_adjustment;
        // TODO:
        // sensor_y *= -1
        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }.normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
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

        Some(adj - inside)
    }
}