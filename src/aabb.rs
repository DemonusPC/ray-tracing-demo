use crate::ray::Ray;
use crate::utility::{ffmax, ffmin};
use crate::vec3::Vec3;
#[derive(Copy, Clone)]
pub struct AABB {
    minimum: Vec3,
    maximum: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn min(&self) -> &Vec3 {
        &self.minimum
    }

    pub fn max(&self) -> &Vec3 {
        &self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let invD = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * invD;
            let mut t1 = (self.max()[a] - r.origin()[a]) * invD;

            if invD < 0.0 {
                let s = t1;
                t0 = t1;
                t1 = s;
            }
            let t_min_l = if t0 > t_min { t0 } else { t_min };

            let t_max_l = if t1 < t_max { t1 } else { t_max };

            if t_max_l <= t_min_l {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3::new(
            ffmin(box0.min().x(), box1.min().x()),
            ffmin(box0.min().y(), box1.min().y()),
            ffmin(box0.min().z(), box1.min().z()),
        );

        let big = Vec3::new(
            ffmax(box0.max().x(), box1.max().x()),
            ffmax(box0.max().y(), box1.max().y()),
            ffmax(box0.max().z(), box1.max().z()),
        );

        return AABB::new(small, big);
    }

    pub fn surrounding_box_mut(box0: &mut AABB, box1: AABB) -> AABB {
        let small = Vec3::new(
            ffmin(box0.min().x(), box1.min().x()),
            ffmin(box0.min().y(), box1.min().y()),
            ffmin(box0.min().z(), box1.min().z()),
        );

        let big = Vec3::new(
            ffmax(box0.max().x(), box1.max().x()),
            ffmax(box0.max().y(), box1.max().y()),
            ffmax(box0.max().z(), box1.max().z()),
        );

        return AABB::new(small, big);
    }
}
