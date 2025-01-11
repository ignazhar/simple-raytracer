use image::{DynamicImage, GenericImage};
use vector3::Vector3;

use crate::color::Color;
use crate::object::Surface;
use crate::point::Point;
use crate::rendering::Ray;
use crate::scene::{Intersection, Light, Scene};
use crate::SHADOW_BIAS;

fn shade_diffuse_color(
    scene: &Scene,
    intersection: &Intersection,
    hit_point: &Point,
    surface_normal: &Vector3,
) -> Color {
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
        color += intersection.object.color(&hit_point)
            * light_source.color()
            * light_power
            * light_reflected;
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
            let color = diffuse_color * (1.0 - reflectivity)
                + cast_ray(scene, &ray.reflect(hit_point, surface_normal), depth + 1)
                    * reflectivity;
            color
        }
        Surface::Refractive {
            transparency,
            index,
        } => {
            let reflection_ray = ray.reflect(hit_point, surface_normal);
            let option_refraction_ray = ray.refract(hit_point, surface_normal, index);

            let reflection_color = cast_ray(scene, &reflection_ray, depth + 1);
            let refraction_color = if let Some(refraction_ray) = option_refraction_ray {
                cast_ray(scene, &refraction_ray, depth + 1)
            } else {
                Color::BLACK
            };

            // Shlick's works better for some reason.
            #[allow(non_snake_case)]
            let R_eff = ray.fresnel(surface_normal, index) as f32;
            // let R_eff = ray.schlicks(surface_normal, index) as f32;
            // let R_eff = 1.0;

            // Debug
            if R_eff < 0.0 {
                println!("too small");
            } else if R_eff > 1.0 {
                println!("too big");
            }

            let transmission_color = reflection_color * R_eff + refraction_color * (1.0 - R_eff);

            let color = diffuse_color * (1.0 - transparency) + transmission_color * transparency;
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
pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
        }
    }

    image
}
