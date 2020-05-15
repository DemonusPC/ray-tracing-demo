use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, HitAble};
use std::rc::Rc;


pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_prt: Rc<dyn Material>
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64, mat_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_prt: mat_ptr
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl HitAble for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
               rec.set_t(temp);
               rec.set_p(r.at(rec.t()));
               let outward_normal = (rec.p() - self.center) / self.radius;
               rec.set_face_normal(r, &outward_normal);
               rec.mat_ptr = self.mat_prt.clone();
               return true;
            }

            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = self.mat_prt.clone();
                return true;
            } 

        }
        
        false
    }
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian {
            albedo: color
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal() + Vec3::random_unit_vector();
        *scattered = Ray::new(&rec.p(), &scatter_direction);
        *attenuation = Vec3::new(self.albedo.x(), self.albedo.y(), self.albedo.z());
        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(color: Vec3, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0{
            fuzz
        }else{
            1.0
        };
        Metal {
            albedo: color,
            fuzz: f
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(ray_in.direction()), &rec.normal());
        *scattered = Ray::new(&rec.p(), &(reflected + Vec3::random_in_unit_sphere() * self.fuzz));
        *attenuation = Vec3::new(self.albedo.x(), self.albedo.y(), self.albedo.z());

        let result = Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0;

        result
    }
}
