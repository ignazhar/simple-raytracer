use std::io::{self, BufRead};

use crate::point::Point;
use crate::scene::{Color, Scene, Sphere};
use chrono::{Local, Timelike};
use image::{DynamicImage, GenericImage, Rgba};
use rendering::{Intersectable, Ray};

pub mod point;
pub mod rendering;
pub mod scene;

/// Actual rendering process: shooting rays
fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba([0, 0, 0, 0]);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if let Some(_) = scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, scene.sphere.color.to_rgba());
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }

    image
}

/// Creating the name for the picture to save it using date
fn get_name(tag: String) -> String {
    let time = Local::now();
    format!(
        "{}={}={}",
        tag,
        time.date_naive(),
        format!("{:02}-{:02}-{:02}", time.hour(), time.minute(), time.second())
    )
}

/// Test render scene
fn test_render_scene() {
    // Creating example scene
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
        },
    };

    // Getting image
    let img = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    // Saving the image
    let save = io::stdin().lock().lines().next().unwrap().unwrap();

    if !save.is_empty() {
        let path = "img/".to_owned() + &get_name("pic".to_string()) + ".jpg";

        println!("image saved to: {}", path);
        img.save(path).unwrap();
    }
}

fn main() {
    test_render_scene();
}
