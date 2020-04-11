mod vec3;
mod ray;
mod hit;
mod models;
mod utility;
mod hittable_list;
use vec3::Vec3;
use ray::Ray;

use std::rc::Rc;

use std::f64::INFINITY;
use hit::{HitRecord, HitAble};

use hittable_list::HitAbleList;

use models::Sphere;

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

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut world = HitAbleList::new();

    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)) );
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)) );


    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;

            let r = Ray::new(&origin, &(lower_left_corner + (horizontal * u) + (vertical * v)));

            let color = ray_color(&r, &world);
            color.write_color();
        }
    }

    eprint!("done");
}
