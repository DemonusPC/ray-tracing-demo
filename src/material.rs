use std::rc::Rc;

use crate::{hit::HitRecord, texture::CheckerTexture};
use crate::random_double;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::texture::{SolidColor, Texture};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian { albedo: Rc::new(SolidColor::new(color)) }
    }

    pub fn from_checker(texture: CheckerTexture) -> Lambertian {
        Lambertian { albedo: Rc::new(texture) }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal() + Vec3::random_unit_vector();
        *scattered = Ray::new(&rec.p(), &scatter_direction, ray_in.time());
        *attenuation = self.albedo.value(rec.u(), rec.v(), &rec.p());
        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Vec3, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo: color,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(ray_in.direction()), &rec.normal());
        *scattered = Ray::new(
            &rec.p(),
            &(reflected + Vec3::random_in_unit_sphere() * self.fuzz),
            ray_in.time(),
        );
        *attenuation = Vec3::new(self.albedo.x(), self.albedo.y(), self.albedo.z());

        let result = Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0;

        result
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face() {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = Vec3::unit_vector(ray_in.direction());

        let cos_theta = Vec3::dot(&-unit_direction, &rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal());
            *scattered = Ray::new(&rec.p(), &reflected, ray_in.time());
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if random_double() < reflect_prob {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal());
            *scattered = Ray::new(&rec.p(), &reflected, ray_in.time());

            return true;
        }

        let refracted = Vec3::refract(&unit_direction, &rec.normal(), etai_over_etat);

        *scattered = Ray::new(&rec.p(), &refracted, ray_in.time());

        true
    }
}
