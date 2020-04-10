fn main() {
    let image_width = 200;
    let image_height = 100;

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev()  {
        eprintln!("Scanlines remaining: {}" , j);
        for i in 0..image_width  {
            let r : f64 = i as f64 / image_width as f64;
            let g : f64 = j as f64 / image_height as f64;
            let b : f64 = 0.2;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }

    eprint!("done");
}
