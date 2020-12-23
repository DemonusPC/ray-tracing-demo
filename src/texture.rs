use crate::vec3::Vec3;


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