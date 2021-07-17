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

use std::time::{Duration, SystemTime};

use crate::material::Material;
use ray::Ray;
use texture::{CheckerTexture, ImageTexture, PerlinTexture};
use vec3::Vec3;

use std::rc::Rc;

use hit::HitAble;
use std::f64::INFINITY;

use camera::Camera;
use material::Lambertian;
use material::Metal;
use material::{Dielectric, DiffuseLight};
use models::{Sphere, XYRect, XZRect, YZRect, Box3D};
use utility::{random_double, random_double_from_values};
use world::World;

fn ray_color(r: &Ray, background: Vec3, world: &World, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::empty();
    }

    let hit_res = match world.hit(r, 0.001, INFINITY) {
        Some(v) => v,
        None => {
            return background;
        }
    };

    let mut scattered = Ray::empty();
    let mut attenuation: Vec3 = Vec3::empty();

    let hit_index = hit_res.id().unwrap();

    let hit_pair = world.get(hit_index);

    let emitted = hit_pair.1.emitted(hit_res.u(), hit_res.v(), &hit_res.p());

    let scatter = hit_pair
        .1
        .scatter(r, &hit_res, &mut attenuation, &mut scattered);


    if !scatter {
        return emitted;
    }

    return emitted + attenuation * ray_color(&scattered, background, world, depth - 1);
}

fn random_scene_new() -> World {
    let mut objects: Vec<Box<dyn HitAble>> = vec![];
    let mut materials: Vec<Rc<dyn Material>> = vec![];

    let mut id = 0;

    // let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));

    let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));

    let ground_material = Rc::new(Lambertian::from_checker(checker));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        id,
    )));
    materials.push(ground_material);

    id += 1;

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
                    objects.push(Box::new(Sphere::new(center, 0.2, id)));
                    materials.push(sphere_material);
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_from_values(0.5, 1.0);
                    let fuzz = random_double_from_values(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    objects.push(Box::new(Sphere::new(center, 0.2, id)));
                    materials.push(sphere_material);
                } else {
                    // glass

                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    objects.push(Box::new(Sphere::new(center, 0.2, id)));
                    materials.push(sphere_material);
                }
                id += 1;
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, id)));
    materials.push(material1);

    id += 1;

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    objects.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, id)));
    materials.push(material2);

    id += 1;

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    objects.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, id)));
    materials.push(material3);

    id += 1;

    World::new(objects, materials)
}

// fn two_spheres_scene() -> World {
//     let mut spheres: Vec<Sphere> = vec![];
//     let mut materials: Vec<Rc<dyn Material>> = vec![];

//     let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
//     let ground_material = Rc::new(Lambertian::from_checker(checker));
//     spheres.push(Sphere::new(
//         Vec3::new(0.0, -10.0, 0.0),
//         10.0,
//         ground_material.clone(),
//     ));
//     materials.push(ground_material);

//     let checker2 = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
//     let ground_material2 = Rc::new(Lambertian::from_checker(checker2));
//     spheres.push(Sphere::new(
//         Vec3::new(0.0, 10.0, 0.0),
//         10.0,
//         ground_material2.clone(),
//     ));
//     materials.push(ground_material2);

//     World::new(spheres, materials, vec![], vec![])
// }

// fn two_perlin_spheres() -> World {
//     let mut spheres: Vec<Sphere> = vec![];
//     let mut materials: Vec<Rc<dyn Material>> = vec![];

//     let pertext1 = PerlinTexture::new(4.0);

//     let ground_material = Rc::new(Lambertian::from_perlin(pertext1));
//     spheres.push(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         ground_material.clone(),
//     ));
//     materials.push(ground_material);

//     let pertext2 = PerlinTexture::new(4.0);

//     let ground_material2 = Rc::new(Lambertian::from_perlin(pertext2));
//     spheres.push(Sphere::new(
//         Vec3::new(0.0, 2.0, 0.0),
//         2.0,
//         ground_material2.clone(),
//     ));
//     materials.push(ground_material2);

//     World::new(spheres, materials, vec![], vec![])
// }

fn earth() -> World {
    let mut objects: Vec<Box<dyn HitAble>> = vec![];
    let mut materials: Vec<Rc<dyn Material>> = vec![];

    let mut id = 0;

    let texture = ImageTexture::new("");

    let ground_material = Rc::new(Lambertian::from_image(texture));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, id)));
    materials.push(ground_material);

    id += 1;

    let diff_light = Rc::new(DiffuseLight::new(Vec3::new(20.0, 20.0, 20.0)));
    objects.push(Box::new(Sphere::new(Vec3::new(3.0, 0.0, -3.0), 1.0, id)));
    materials.push(diff_light);

    World::new(objects, materials)
}

fn simple_light() -> World {
    let mut objects: Vec<Box<dyn HitAble>> = vec![];
    let mut materials: Vec<Rc<dyn Material>> = vec![];

    let mut id = 0;

    let pertext1 = PerlinTexture::new(4.0);

    let ground_material = Rc::new(Lambertian::from_perlin(pertext1));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        id as usize,
    )));
    materials.push(ground_material);

    id += 1;

    let pertext2 = PerlinTexture::new(4.0);

    // let ground_material2 = Rc::new(Lambertian::from_perlin(pertext2));
    let ground_material2 = Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        id as usize,
    )));
    materials.push(ground_material2);

    id += 1;

    let diff_light = Rc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0)));
    objects.push(Box::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, id as usize)));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(4.0, 4.0, 5.0),
    //     5.0,
    //     id as usize
    // )));
    materials.push(diff_light);

    World::new(objects, materials)
}

fn cornell_box() -> World {
    let mut objects: Vec<Box<dyn HitAble>> = vec![];
    let mut materials: Vec<Rc<dyn Material>> = vec![];

    let mut id = 0;

    let red = Rc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0)));
    


    objects.push(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, id as usize)));
    materials.push(green.clone());
    id += 1;

    objects.push(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, id as usize)));
    materials.push(red.clone());
    id += 1;

    objects.push(Box::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, id as usize)));
    materials.push(light);
    id += 1;

    objects.push(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, id as usize)));
    materials.push(white.clone());
    id += 1;

    objects.push(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, id as usize)));
    materials.push(white.clone());
    id += 1;

    objects.push(Box::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, id as usize)));
    materials.push(white.clone());
    id += 1;

    objects.push(Box::new(Box3D::new(Vec3::new(130.0, 0.0, 65.0), Vec3::new(295.0, 165.0, 230.0), id as usize)));
    materials.push(white.clone());
    id += 1;

    objects.push(Box::new(Box3D::new(Vec3::new(265.0, 0.0, 295.0), Vec3::new(430.0, 330.0, 460.0), id as usize)));
    materials.push(white.clone());
    id += 1;
    

    World::new(objects, materials)

}

fn main() {
    let now = SystemTime::now();

    let aspect_ratio = 1.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 200;
    let max_depth = 50;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    // let world = simple_light();
    let world = cornell_box();


    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);



    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let background = Vec3::empty();

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
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
                color += ray_color(&r, background, &world, max_depth);
            }
            color.write_color(samples_per_pixel);
        }
    }

    eprint!("done");
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            eprintln!("Time taken: {} seconds", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            eprintln!("Error: {:?}", e);
        }
    }
}
