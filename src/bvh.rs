use crate::utility::random_int_from_values;
use crate::HitAble;
use crate::{aabb::AABB, world::World};
use std::rc::Rc;

pub struct BvhNode {
    left: Box<BvhNode>,
    right: Box<BvhNode>,
    node_box: AABB,
    id: Option<usize>
}

impl BvhNode {}

impl HitAble for BvhNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hit::HitRecord,
    ) -> bool {
        if !self.node_box.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let param = if hit_left { rec.t() } else { t_max };
        let hit_right = self.right.hit(r, t_min, param, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.node_box)
    }

    fn id(&self) -> Option<usize> {
        self.id
    }
}
