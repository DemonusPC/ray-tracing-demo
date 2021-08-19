use crate::aabb::AABB;
use crate::hit::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::{bvh::BvhNode, hit::HitAble};
use std::rc::Rc;

pub struct World {
    objects: Vec<Box<dyn HitAble>>,
    materials: Vec<Rc<dyn Material>>,
    node: BvhNode,
}

impl World {
    pub fn new(mut objects: Vec<Box<dyn HitAble>>, materials: Vec<Rc<dyn Material>>) -> Self {
        let tnode = BvhNode::new_from_list(&mut objects, 0.0, 10.0);

        let node = BvhNode::new_from_list(&mut objects, 0.0, 10.0);
        World {
            objects,
            materials,
            node,
        }
    }

    pub fn get(&self, index: usize) -> (&Box<dyn HitAble>, &Rc<dyn Material>) {
        (&self.objects[index], &self.materials[index])
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::empty();

        // let hit = self.node.hit(r, t_min, t_max, &mut temp_rec, &self);

        // match hit {
        //     true => Some(temp_rec),
        //     false => Option::None,
        // }

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                temp_rec.set_id(object.id())
            }
        }
        match hit_anything {
            true => Some(temp_rec),
            false => Option::None
        }

        // (hit_anything, temp_rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        // In the original file the temp box is outside of the for loop

        let mut first_box = true;

        for object in self.objects.as_slice() {
            let temp_box = object.bounding_box(time0, time1);
            if temp_box.is_none() {
                return false;
            }

            *output_box = if first_box {
                temp_box.unwrap()
            } else {
                AABB::surrounding_box_mut(output_box, temp_box.unwrap())
            };

            first_box = false;
        }

        true
    }
}
