use crate::hit::{ HitRecord, HitAble};
use crate::vec3::Vec3;
use crate::ray::Ray;

use std::rc::Rc;

pub struct HitAbleList {
    objects: Vec<Rc<HitAble>>
} 

impl HitAbleList {
    pub fn new() -> HitAbleList {
        HitAbleList {
            objects: Vec::new()
        }   
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<HitAble>) {
        self.objects.push(object);
    }
}

impl HitAble for HitAbleList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(Vec3::empty(), Vec3::empty(), 0.0, false);
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();;
                rec.copy_into(&temp_rec);
            }
        }

        hit_anything
    }
}