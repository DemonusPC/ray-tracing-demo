mod camera;
mod hit;
mod hittable_list;
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
use models::Dielectric;
use models::Lambertian;
use models::Metal;
use models::Sphere;
use utility::random_double;

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

fn main() {
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let mut world = HitAbleList::new();

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    )));

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));

    world.add(Rc::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)),
    )));

    world.add(Rc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Rc::new(Dielectric::new(1.5)),
    )));

    let r = (std::f64::consts::PI / 4.0).cos();
    let cam = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        image_width as f64 / image_height as f64,
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
