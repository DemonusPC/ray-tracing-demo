mod vec3;
mod ray;
mod hit;
mod models;
mod utility;
mod hittable_list;
mod camera;

use vec3::Vec3;
use ray::Ray;

use std::rc::Rc;

use std::f64::INFINITY;
use hit::{HitRecord, HitAble};

use hittable_list::HitAbleList;

use models::Sphere;
use camera::Camera;
use utility::random_double;

fn ray_color(r: &Ray, world: &dyn HitAble) -> Vec3 {
    let mut rec = HitRecord::empty();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal() + Vec3::new(1.0, 1.0, 1.0)) * 0.5 ;
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (Vec3::new(1.0, 1.0, 1.0)*(1.0-t)) + (Vec3::new(0.5, 0.7, 1.0)*t)
}


fn main() {
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    

    let mut world = HitAbleList::new();

    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)) );
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)) );

    let cam = Camera::new();

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Vec3::empty();
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / image_width as f64;
                let v = (j as f64 + random_double()) / image_height as f64;

                let r = cam.get_ray(u, v);
                color += ray_color(&r, &world);
        }
        color.write_color(samples_per_pixel);
        }
    }

    eprint!("done");
}
