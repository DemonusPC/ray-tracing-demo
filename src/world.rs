use crate::aabb::AABB;
use crate::hit::HitAble;
use crate::hit::HitRecord;
use crate::material::Material;
use crate::models::Sphere;
use crate::ray::Ray;
use std::rc::Rc;

pub struct World {
    spheres: Vec<Sphere>,
    materials: Vec<Rc<dyn Material>>,
}

impl World {
    pub fn empty() -> Self {
        World {
            spheres: vec![],
            materials: vec![],
        }
    }

    pub fn new(spheres: Vec<Sphere>, materials: Vec<Rc<dyn Material>>) -> Self {
        World { spheres, materials }
    }

    pub fn get(&self, index: usize) -> (&Sphere, &Rc<dyn Material>) {
        (&self.spheres[index], &self.materials[index])
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut temp_rec = HitRecord::empty();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for (sphere, mat) in self.spheres.iter().zip(self.materials.iter()) {
            if sphere.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                temp_rec.mat_ptr = mat.clone()
            }
        }

        (hit_anything, temp_rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.spheres.is_empty() {
            return false;
        }

        // In the original file the temp box is outside of the for loop

        let mut first_box = true;

        for object in self.spheres.as_slice() {
            let mut temp_box = AABB::empty();
            if !object.bounding_box(time0, time1, &mut temp_box) {
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
