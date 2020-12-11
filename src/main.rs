mod aabb;
mod bvh;
mod camera;
mod hit;
mod hittable_list;
mod material;
mod models;
mod ray;
mod utility;
mod vec3;

use ray::Ray;
use vec3::Vec3;

use std::rc::Rc;

use hit::{HitAble, HitRecord};
use std::f64::INFINITY;

use hittable_list::HitAbleList;

use camera::Camera;
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use models::MovingSphere;
use models::Sphere;
use utility::{random_double, random_double_from_values};

fn ray_color(r: &Ray, world: &dyn HitAble, depth: i32) -> Vec3 {
    let mut rec = HitRecord::empty();

    if depth <= 0 {
        return Vec3::empty();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        // let target = rec.p() + rec.normal() + Vec3::random_unit_vector();

        let mut scattered = Ray::empty();
        let mut attenuation: Vec3 = Vec3::empty();
        let scatter_result =
            rec.mat_ptr
                .as_ref()
                .scatter(r, &rec, &mut attenuation, &mut scattered);
        if scatter_result {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        // let target = rec.p() + Vec3::random_in_hemisphere(&rec.normal());

        // return ray_color(&Ray::new( &rec.p() , &(target - rec.p())), world, depth-1) * 0.5;

        return Vec3::empty();
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t)
}

fn random_scene() -> HitAbleList {
    let mut world = HitAbleList::new();

    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

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
                    let center1 = center + Vec3::new(0.0, random_double_from_values(0.0, 0.5), 0.0);
                    world.add(Rc::new(MovingSphere::new(
                        center,
                        center1,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_from_values(0.5, 1.0);
                    let fuzz = random_double_from_values(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass

                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let world = random_scene();

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
                color += ray_color(&r, &world, max_depth);
            }
            color.write_color(samples_per_pixel);
        }
    }

    eprint!("done");
}
