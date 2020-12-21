use crate::aabb::AABB;
use crate::hit::{HitAble, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitAbleList {
    objects: Vec<Box<dyn HitAble>>,
}

impl HitAbleList {
    pub fn new() -> HitAbleList {
        HitAbleList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn HitAble>) {
        self.objects.push(object);
    }
}

impl HitAble for HitAbleList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(
            Vec3::empty(),
            Vec3::empty(),
            rec.mat_ptr.clone(),
            0.0,
            false,
        );
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                rec.copy_into(&temp_rec);
            }
        }

        hit_anything
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        // In the original file the temp box is outside of the for loop

        let mut first_box = true;

        for object in self.objects.as_slice() {
            let mut temp_box = AABB::new(Vec3::empty(), Vec3::empty());
            if !object.as_ref().bounding_box(time0, time1, &mut temp_box) {
                return false;
            }

            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box_mut(output_box, temp_box)
            };

            first_box = false;
        }

        true
    }
}
