use crate::{
    utility::{random_double, random_int_from_values},
    vec3::Vec3,
};

const POINT_COUNT: i32 = 256;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        SolidColor { color }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        SolidColor {
            color: Vec3::new(r, g, b),
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
    even: SolidColor,
}

impl CheckerTexture {
    pub fn new(c1: Vec3, c2: Vec3) -> Self {
        Self {
            odd: SolidColor::new(c1),
            even: SolidColor::new(c2),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = ((10.0 * p.x()).sin()) * ((10.0 * p.y()).sin()) * ((10.0 * p.z()).sin());
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
    perm_z: Vec<i32>,
}

impl PerlinTexture {
    pub fn new() -> Self {
        let mut ran_temp: Vec<f64> = vec![];

        for _ in 0..POINT_COUNT {
            ran_temp.push(random_double());
        }

        Self {
            ranfloat: ran_temp,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    fn noise(&self, p: &Vec3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u * u * (3.0-2.0*u);
        v = v * v * (3.0-2.0*v);
        w = w * w * (3.0-2.0*w);



        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0f64; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        trilinear_interp(&mut c, u, v, w)
    }
}

impl Texture for PerlinTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        return Vec3::new(1.0, 1.0, 1.0) * self.noise(p);
    }
}

fn perlin_generate_perm() -> Vec<i32> {
    let mut p: Vec<i32> = vec![];

    for i in 0..POINT_COUNT {
        p.push(i);
    }

    permute(&mut p, POINT_COUNT);

    return p;
}

fn permute(p: &mut [i32], n: i32) {
    for i in (0..n).rev() {
        let target = random_int_from_values(0, i);
        let tmp = p[i as usize];
        p[i as usize] = p[target as usize];
        p[target as usize] = tmp
    }
}

fn trilinear_interp(c: &mut [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                    * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                    * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }

    return accum;
}
