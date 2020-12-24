use crate::{utility::{random_double, random_int_from_values}, vec3::Vec3};

const POINT_COUNT: i32 = 256;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColor {
    color: Vec3
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        SolidColor {
            color
        }
    }

    pub fn from_rgb(r: f64, g: f64, b:f64) -> Self {
        SolidColor{
            color: Vec3::new(r, g, b)
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color.clone()
    }
}

pub struct CheckerTexture {
    odd: SolidColor,
    even: SolidColor
}

impl CheckerTexture {
    pub fn new(c1: Vec3, c2: Vec3) -> Self {
        Self {
            odd: SolidColor::new(c1),
            even: SolidColor::new(c2)
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = ((10.0*p.x()).sin()) * ((10.0*p.y()).sin()) * ((10.0*p.z()).sin());
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        }
        self.even.value(u, v, p)
    }
}

pub struct PerlinTexture {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>
}

impl PerlinTexture {
    pub fn new() -> Self {
        let mut ran_temp : Vec<f64> = vec![];

        for _ in 0..POINT_COUNT {
            ran_temp.push(random_double());
        }

        Self {
            ranfloat: ran_temp,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm()
        }
    }

    fn noise(&self, p: &Vec3) -> f64 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;

        return self.ranfloat[(self.perm_x[i as usize] as usize) ^ self.perm_y[j as usize] as usize ^ self.perm_z[k as usize] as usize]
    }
}

impl Texture for PerlinTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        return Vec3::new(1.0, 1.0, 1.0) * self.noise(p);
    }
}

fn perlin_generate_perm() -> Vec<i32> {     
    let mut  p: Vec<i32> = vec![];
    
    for i in 0..POINT_COUNT {
        p.push(i);
    }

    permute(&mut p, POINT_COUNT);

    return p;
}

fn permute(p: &mut [i32], n:i32) {
    for i in (0..n).rev() {
        let target = random_int_from_values(0, i);
        let tmp = p[i as usize];
        p[i as usize] = p[target as usize];
        p[target as usize] = tmp
    }
}