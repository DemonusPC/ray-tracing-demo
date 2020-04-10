mod vec3;
use vec3::Vec3;

fn main() {
    let image_width = 200;
    let image_height = 100;

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let color: Vec3 = Vec3::new(
                i as f64 / image_width as f64,
                j as f64 / image_height as f64,
                0.2,
            );
            color.write_color();
        }
    }

    eprint!("done");
}
