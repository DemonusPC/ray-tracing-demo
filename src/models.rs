use crate::aabb::AABB;
use crate::hit::{HitAble, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_prt: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64, mat_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_prt: mat_ptr,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn get_sphere_uv(p: &Vec3, u: f64, v: f64) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = -p.z().atan2(p.x()) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v= theta / std::f64::consts::PI;

        (u, v)
    }
}

impl HitAble for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.set_uv(Sphere::get_sphere_uv(&outward_normal, rec.u(), rec.v()));
                rec.mat_ptr = self.mat_prt.clone();
                return true;
            }

            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.set_uv(Sphere::get_sphere_uv(&outward_normal, rec.u(), rec.v()));
                rec.mat_ptr = self.mat_prt.clone();
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let a = self.center() - Vec3::new(self.radius(), self.radius(), self.radius());
        let b = self.center() + Vec3::new(self.radius(), self.radius(), self.radius());
        *output_box = AABB::new(a, b);

        true
    }
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0
            + ((time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0));
    }
}

impl HitAble for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }

            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }

        false
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let a0 = self.center(time0) - Vec3::new(self.radius, self.radius, self.radius);
        let b0 = self.center(time0) + Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB::new(a0, b0);

        let a1 = self.center(time1) - Vec3::new(self.radius, self.radius, self.radius);
        let b1 = self.center(time1) + Vec3::new(self.radius, self.radius, self.radius);

        let box1 = AABB::new(a1, b1);

        *output_box = AABB::surrounding_box(box0, box1);

        true
    }
}
