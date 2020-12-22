use std::rc::Rc;
use crate::material::Material;
use crate::models::Sphere;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::hit::HitAble;

pub struct World {
    spheres: Vec<Sphere>,
    materials: Vec<Rc<dyn Material>>
}

impl World {
    pub fn empty() -> Self{
        World {
            spheres: vec![],
            materials: vec![]
        }
    }

    pub fn new(spheres: Vec<Sphere>, materials: Vec<Rc<dyn Material>>) -> Self {
        World {
            spheres,
            materials
        }
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
}

