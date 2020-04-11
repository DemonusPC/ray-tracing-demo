use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, HitAble};

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r
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
               return true;
            }

            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            } 

        }
        
        false
    }
}