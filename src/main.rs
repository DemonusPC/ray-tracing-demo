mod vec3;
mod ray;
use vec3::Vec3;
use ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
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

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;

            let r = Ray::new(&origin, &(lower_left_corner + (horizontal * u) + (vertical * v)));

            let color = ray_color(&r);
            color.write_color();
        }
    }

    eprint!("done");
}
