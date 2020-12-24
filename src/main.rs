mod aabb;
mod bvh;
mod camera;
mod hit;
mod material;
mod models;
mod ray;
mod texture;
mod utility;
mod vec3;
mod world;

use crate::material::Material;
use ray::Ray;
use texture::{CheckerTexture, PerlinTexture};
use vec3::Vec3;

use std::rc::Rc;

use hit::HitAble;
use std::f64::INFINITY;

use camera::Camera;
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use models::Sphere;
use utility::{random_double, random_double_from_values};
use world::World;

fn ray_color(r: &Ray, world: &World, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::empty();
    }

    let result = world.hit(r, 0.001, INFINITY);

    if result.0 {
        let mut scattered = Ray::empty();
        let mut attenuation: Vec3 = Vec3::empty();

        let scatter_result =
            result
                .1
                .mat_ptr
                .as_ref()
                .scatter(r, &result.1, &mut attenuation, &mut scattered);

        if scatter_result {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Vec3::empty();
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t)
}

fn random_scene_new() -> World {
    let mut spheres: Vec<Sphere> = vec![];
    let mut materials: Vec<Rc<dyn Material>> = vec![];

    // let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));

    let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));

    let ground_material = Rc::new(Lambertian::from_checker(checker));
    spheres.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    ));
    materials.push(ground_material);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();

            let center = Vec3::new(
                (a as f64) + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    spheres.push(Sphere::new(center, 0.2, sphere_material.clone()));
                    materials.push(sphere_material);
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_from_values(0.5, 1.0);
                    let fuzz = random_double_from_values(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    spheres.push(Sphere::new(center, 0.2, sphere_material.clone()));
                    materials.push(sphere_material);
                } else {
                    // glass

                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    spheres.push(Sphere::new(center, 0.2, sphere_material.clone()));
                    materials.push(sphere_material);
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    spheres.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    ));
    materials.push(material1);

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    spheres.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    ));
    materials.push(material2);

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    spheres.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    ));
    materials.push(material3);

    World::new(spheres, materials)
}

fn two_spheres_scene() -> World {
    let mut spheres: Vec<Sphere> = vec![];
    let mut materials: Vec<Rc<dyn Material>> = vec![];

    let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let ground_material = Rc::new(Lambertian::from_checker(checker));
    spheres.push(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        ground_material.clone(),
    ));
    materials.push(ground_material);

    let checker2 = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let ground_material2 = Rc::new(Lambertian::from_checker(checker2));
    spheres.push(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        ground_material2.clone(),
    ));
    materials.push(ground_material2);

    World::new(spheres, materials)
}

fn two_perlin_spheres() -> World {
    let mut spheres: Vec<Sphere> = vec![];
    let mut materials: Vec<Rc<dyn Material>> = vec![];

    let pertext1 = PerlinTexture::new();

    let ground_material = Rc::new(Lambertian::from_perlin(pertext1));
    spheres.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    ));
    materials.push(ground_material);

    let pertext2 = PerlinTexture::new();

    let ground_material2 = Rc::new(Lambertian::from_perlin(pertext2));
    spheres.push(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        ground_material2.clone(),
    ));
    materials.push(ground_material2);

    World::new(spheres, materials)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    // let world = random_scene();
    // let world = random_scene_new();
    // let world = two_spheres_scene();
    let world = two_perlin_spheres();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        image_width as f64 / image_height as f64,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Vec3::empty();
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / image_width as f64;
                let v = (j as f64 + random_double()) / image_height as f64;

                let r = cam.get_ray(u, v);
                // color += ray_color(&r, &world, max_depth);
                color += ray_color(&r, &world, max_depth);
            }
            color.write_color(samples_per_pixel);
        }
    }

    eprint!("done");
}
