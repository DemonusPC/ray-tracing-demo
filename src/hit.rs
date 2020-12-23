use crate::aabb::AABB;
use crate::material::Lambertian;
use crate::material::Material;
use crate::material::Metal;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    t: f64,
    u: f64,
    v: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        normal: Vec3,
        mat_ptr: Rc<dyn Material>,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            p: p,
            normal: normal,
            mat_ptr: mat_ptr,
            t: t,
            u,
            v,
            front_face: front_face,
        }
    }

    pub fn empty() -> HitRecord {
        HitRecord {
            p: Vec3::empty(),
            normal: Vec3::empty(),
            mat_ptr: Rc::new(Lambertian::new(Vec3::empty())),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: true,
        }
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn set_p(&mut self, v: Vec3) {
        self.p = v
    }

    pub fn set_normal(&mut self, v: Vec3) {
        self.normal = v;
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal.clone(),
            false => -outward_normal.clone(),
        }
    }

    pub fn set_t(&mut self, new_t: f64) {
        self.t = new_t;
    }

    pub fn copy_into(&mut self, other: &HitRecord) {
        self.p = other.p();
        self.normal = other.normal();
        self.t = other.t();
        self.front_face = other.front_face();
        self.mat_ptr = other.mat_ptr.clone();
    }

    pub fn set_uv(&mut self, uv: (f64, f64)) {
        self.set_u(uv.0);
        self.set_v(uv.1);
    } 

    pub fn set_u(&mut self, new_u: f64) {
        self.u = new_u;
    }

    pub fn set_v(&mut self, new_v: f64) {
        self.v = new_v;
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }

    
}

pub trait HitAble {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn equality(result: &Vec3, x: f64, y: f64, z: f64) {
        assert_eq!(result.x(), x);
        assert_eq!(result.y(), y);
        assert_eq!(result.z(), z);
    }

    #[test]
    fn test_new() {
        let result = HitRecord::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 0.0, 0.0),
            Rc::new(Lambertian::new(Vec3::empty())),
            0.0,
            true,
        );
        equality(&result.p(), 1.0, 2.0, 3.0);
        equality(&result.normal(), 2.00, 0.0, 0.0);
    }

    #[test]
    fn test_set_p() {
        let mut result = HitRecord::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 0.0, 0.0),
            Rc::new(Lambertian::new(Vec3::empty())),
            0.0,
            true,
        );
        equality(&result.p(), 1.0, 2.0, 3.0);
        result.set_p(Vec3::new(50.0, 10.0, 20.0));
        equality(&result.p(), 50.0, 10.0, 20.0);
    }

    #[test]
    fn test_set_normal() {
        let mut result = HitRecord::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 0.0, 0.0),
            Rc::new(Lambertian::new(Vec3::empty())),
            0.0,
            true,
        );
        equality(&result.normal(), 2.00, 0.0, 0.0);
        result.set_normal(Vec3::new(50.0, 10.0, 20.0));
        equality(&result.normal(), 50.0, 10.0, 20.0);
    }
}
