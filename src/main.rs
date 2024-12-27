// foreign crates
use chrono::{Local, Timelike};
use image::{DynamicImage, GenericImage};
use object::Material;
use std::io::{self, BufRead};
use vector3::Vector3;

// declaring domestic crates
pub mod color;
pub mod object;
pub mod point;
pub mod rendering;
pub mod scene;

// domestic crates
use crate::color::Color;
use crate::object::{Object, Plane, Sphere, Surface};
use crate::point::Point;
use crate::rendering::Ray;
use crate::scene::{DirectionalLight, Intersection, Light, Scene, SphericalLight};

// consts
const SHADOW_BIAS: f64 = 1e-6;
const ALBEDO: f32 = 0.8;

fn shade_diffuse_color(scene: &Scene, intersection: &Intersection, hit_point: &Point, surface_normal: &Vector3) -> Color {
    let mut color = Color::BLACK;

    for light_source in &scene.lights {
        let direction_to_light = match light_source {
            Light::Directional(light) => Vector3::zero() - light.direction,
            Light::Spherical(light) => Vector3::from(light.position - *hit_point).normalize(),
        };

        let shadow_ray = Ray {
            origin: *hit_point + (*surface_normal * SHADOW_BIAS).into(),
            direction: direction_to_light,
        };

        let light_intensity = match light_source {
            Light::Directional(light) => {
                if let Some(_intersection) = scene.trace(&shadow_ray) {
                    0.0
                } else {
                    light.intensity
                }
            }
            Light::Spherical(light) => {
                let d_sq = (*hit_point - light.position).magnitude_sq();
                let d = d_sq.sqrt();
                if let Some(intersection) = scene.trace(&shadow_ray) {
                    if intersection.distance < d {
                        light.intensity / (4.0 * std::f32::consts::PI * (d_sq as f32))
                    } else {
                        0.0
                    }
                } else {
                    light.intensity / (4.0 * std::f32::consts::PI * (d_sq as f32))
                }
            }
        };

        // Lambert's cosine law
        let light_power = (surface_normal.dot(&direction_to_light) as f32) * light_intensity;
        // TODO: figure out the derivation
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/diffuse-lambertian-shading.html
        // ^^^
        let light_reflected = intersection.object.albedo() / std::f32::consts::PI;
        color += intersection.object.color(&hit_point) * light_source.color() * light_power * light_reflected;
    }

    color.clamp()
}

fn get_color(scene: &Scene, intersection: &Intersection, ray: &Ray, depth: u32) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance).into();
    let surface_normal = intersection.object.surface_normal(&hit_point).normalize();

    let diffuse_color = shade_diffuse_color(scene, intersection, &hit_point, &surface_normal);

    match intersection.object.surface_type() {
        Surface::Diffusive => diffuse_color,
        Surface::Reflective { reflectivity } => {
            let color = diffuse_color * (1.0 - reflectivity) + cast_ray(scene, &ray.reflect(hit_point, surface_normal), depth + 1) * reflectivity;
            color
        }
    }
}

fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth == scene.max_recursion_depth {
        return Color::BLACK;
    }

    let intersection = scene.trace(ray);
    if let Some(blabla) = intersection {
        get_color(scene, &blabla, ray, depth)
    } else {
        Color::BLACK
    }
}

/// Actual rendering process: shooting rays
fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    // let black = Rgba([0, 0, 0, 0]);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            // if let Some(intersection) = scene.trace(&ray) {
            //     image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
            // } else {
            //     image.put_pixel(x, y, black);
            // }
            image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
        }
    }

    image
}

/// Test render scene
fn test_render_scene() {
    // Creating example scene
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
            // Object::Plane(Plane {
            //     // Background
            //     origin: Point {
            //         x: 0.0,
            //         y: 0.0,
            //         z: -30.0,
            //     },
            //     normal: Vector3 {
            //         x: 0.0,
            //         y: 0.0,
            //         z: -1.0,
            //     }
            //     .normalize(),
            //     material: Material::from_color(Color::WHITE * 0.8, ALBEDO, Surface::Diffusive),
            // }),
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
