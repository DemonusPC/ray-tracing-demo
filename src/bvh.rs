use crate::aabb::AABB;
use crate::HitAble;
use std::rc::Rc;

pub struct BvhNode {
    left: Rc<HitAble>,
    right: Rc<HitAble>,
    node_box: AABB,
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

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::AABB) -> bool {
        *output_box = self.node_box;
        true
    }
}
