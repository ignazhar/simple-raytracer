// foreign crates
use chrono::{Local, Timelike};
use object::Material;
use std::io::{self, BufRead};
use vector3::Vector3;

// declaring domestic crates
pub mod color;
pub mod object;
pub mod point;
pub mod rendering;
pub mod scene;
pub mod tracing;

// domestic crates
use crate::color::Color;
use crate::object::{Object, Plane, Sphere, Surface};
use crate::point::Point;
use crate::scene::{DirectionalLight, Light, Scene, SphericalLight};
use crate::tracing::render;

// consts
const SHADOW_BIAS: f64 = 1e-6;
const ALBEDO: f32 = 0.8;

/// Test render scene
fn test_render_scene() {
    let scene = Scene {
        width: 1920,
        height: 1080,
        fov: 90.0,
        objects: vec![
            Object::Sphere(Sphere {
                center: Point {
                    x: 0.0,
                    y: -1.5,
                    z: -3.0,
                },
                radius: 1.0,
                material: Material::from_color(Color::LIGHT_GREEN, ALBEDO, Surface::Diffusive),
            }),
            Object::Sphere(Sphere {
                center: Point {
                    x: 4.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 2.0,
                material: Material::from_color(Color::MAGENTA, ALBEDO, Surface::Reflective { reflectivity: 0.8 }),
            }),
            Object::Sphere(Sphere {
                center: Point {
                    x: -3.0,
                    y: 0.0,
                    z: -7.0,
                },
                radius: 2.0,
                material: Material::get_texture(Material::CHECKERBOARD, 10.0, 0.0, Surface::Diffusive),
            }),
            Object::Plane(Plane {
                // Floor
                origin: Point {
                    x: 0.0,
                    y: 2.5,
                    z: 0.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }
                .normalize(),
                material: Material::get_texture(Material::FLOOR, 0.2, 0.0, Surface::Reflective { reflectivity: 0.3 }),
            }),
        ],
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: Vector3 {
                    x: 5.0,
                    y: 5.0,
                    z: -5.0,
                }
                .normalize(),
                color: Color::WHITE,
                intensity: 2.5,
            }),
            Light::Directional(DirectionalLight {
                direction: Vector3 {
                    x: -2.0,
                    y: 3.0,
                    z: -5.0,
                }
                .normalize(),
                color: Color::YELLOW,
                intensity: 1.5,
            }),
            Light::Spherical(SphericalLight {
                position: Point {
                    x: 0.0,
                    y: -1.0,
                    z: -3.0,
                },
                color: Color::RED,
                intensity: 200.0,
            }),
        ],
        max_recursion_depth: 5,
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
