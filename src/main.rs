use std::io::{self, BufRead};

use crate::point::Point;
use crate::scene::{Color, Scene, Sphere};
use chrono::{Local, Timelike};
use image::{DynamicImage, GenericImage, Rgba};
use rendering::Ray;
use scene::{Intersection, Light, Object, Plane};
use vector3::Vector3;

pub mod point;
pub mod rendering;
pub mod scene;

const SHADOW_BIAS: f64 = 1e-6;

fn get_color(scene: &Scene, intersection: &Intersection, ray: &Ray) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance).into();
    let surface_normal = intersection.object.surface_normal(&hit_point).normalize();
    let direction_to_light = Vector3::zero() - scene.light.direction;

    let shadow_ray = Ray {
        origin: hit_point + (surface_normal * SHADOW_BIAS).into(),
        direction: direction_to_light,
    };
    let light_intensity = if let Some(intersection) = scene.trace(&shadow_ray) {
        0.0
    } else {
        scene.light.intensity
    };
    // Lambert's cosine law
    let light_power = (surface_normal.dot(&direction_to_light) as f32) * light_intensity;
    // TODO: figure out the derivation
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/diffuse-lambertian-shading.html
    // ^^^
    let light_reflected = intersection.object.albedo() / std::f32::consts::PI;
    let color = intersection.object.color() * scene.light.color * light_power * light_reflected;
    color.abs().clamp()
}

/// Actual rendering process: shooting rays
fn render(scene: &Scene) -> DynamicImage {
    normalize(&scene);

    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba([0, 0, 0, 0]);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if let Some(intersection) = scene.trace(&ray) {
                image.put_pixel(x, y, get_color(scene, &intersection, &ray).to_rgba());
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }

    image
}

fn normalize(scene: &Scene) {
    // scene.light.direction = scene.light.direction.normalize();
    for object in scene.objects.iter() {
        // match object {
        //     // Object::Plane(plane) => plane.
        // }
    }
}

const ALBEDO: f32 = 1.0;

/// Test render scene
fn test_render_scene() {
    // Creating example scene
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        objects: vec![
            Object::Sphere(Sphere {
                center: Point {
                    x: 0.0,
                    y: -1.5,
                    z: -3.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
                albedo: ALBEDO,
            }),
            Object::Sphere(Sphere {
                center: Point {
                    x: 4.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 2.0,
                color: Color {
                    red: 0.8,
                    green: 0.1,
                    blue: 0.8,
                },
                albedo: ALBEDO,
            }),
            Object::Plane(Plane {
                origin: Point {
                    x: 0.0,
                    y: 2.5,
                    z: 0.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }.normalize(),
                color: Color {
                    red: 0.4,
                    green: 0.4,
                    blue: 0.8,
                },
                albedo: ALBEDO,
            }),
            Object::Plane(Plane {
                origin: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -30.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                }.normalize(),
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                albedo: ALBEDO,
            }),
        ],
        light: Light {
            direction: Vector3 {
                x: 5.0,
                y: 5.0,
                z: -5.0,
            }
            .normalize(),
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 5.0,
        },
    };

    // Getting image
    let img = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    // Saving the image
    let save = io::stdin().lock().lines().next().unwrap().unwrap();

    if !save.is_empty() {
        let path = "img/".to_owned() + &get_name("pic".to_string()) + "=" + save.as_str() + ".jpg";

        println!("image saved to: {}", path);
        img.save(path).unwrap();
    }
}

/// Creating the name for the picture to save it using date
fn get_name(tag: String) -> String {
    let time = Local::now();
    format!(
        "{}={}={}",
        tag,
        time.date_naive(),
        format!(
            "{:02}-{:02}-{:02}",
            time.hour(),
            time.minute(),
            time.second()
        )
    )
}

fn main() {
    test_render_scene();
}
