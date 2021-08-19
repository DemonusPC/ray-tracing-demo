use core::f64;

use crate::aabb::AABB;
use crate::ray::Ray;
use crate::utility::{ffmax, ffmin};
use crate::vec3::Vec3;
use crate::HitAble;

pub struct Translate {
    object: Box<dyn HitAble>,
    offset: Vec3
}

impl Translate {
    pub fn new(object: Box<dyn HitAble>, offset: Vec3) -> Self { 
        Self {
            object,
            offset
        }
    }
}


impl HitAble for Translate {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut crate::hit::HitRecord) -> bool {
        let moved_r: Ray = Ray::new(&(r.origin() - self.offset), &r.direction() , r.time());

        if !self.object.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.set_p(rec.p() + self.offset );
        rec.set_face_normal(&moved_r, &rec.normal());

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        match self.object.bounding_box(time0, time1) {
            Some(aabb) =>{
                Some(AABB::new(aabb.min() + &self.offset, aabb.max() + &self.offset))
            },
            None => None
        }
    
    }

    fn id(&self) -> Option<usize> {
        self.object.id()
    }
}


pub struct RotateY {
    object: Box<dyn HitAble>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB
}


impl RotateY {
    pub fn new(object: Box<dyn HitAble>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        // This is dangerous. Kind ahave to assume that this will work 
        let temp_box = object.bounding_box(0.0, 1.0).unwrap();

        let mut min = Vec3::new(f64::INFINITY,f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(-f64::INFINITY,-f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * temp_box.max().x() + (1.0 - i as f64) * temp_box.min().x();
                    let y = j as f64 * temp_box.max().y() + (1.0 - j as f64) * temp_box.min().y();
                    let z = k as f64 * temp_box.max().z() + (1.0 - k as f64) * temp_box.min().z();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = ffmin(min[c], tester[c]);
                        max[c] = ffmax(max[c], tester[c]);
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::new(min, max)
        }

    }
}

impl HitAble for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut crate::hit::HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(&origin, &direction, r.time());

        if !self.object.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p();
        let mut normal = rec.normal();

        p[0] = self.cos_theta * rec.p()[0] + self.sin_theta * rec.p()[2];
        p[2] = -self.sin_theta * rec.p()[0] + self.cos_theta * rec.p()[2];

        normal[0] = self.cos_theta * rec.normal()[0] + self.sin_theta * rec.normal()[2];
        normal[2] = -self.sin_theta * rec.normal()[0] + self.cos_theta * rec.normal()[2];

        rec.set_p(p);
        rec.set_face_normal(&rotated_r, &normal);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.bbox)
    }

    fn id(&self) -> Option<usize> {
        self.object.id()
    }
}
