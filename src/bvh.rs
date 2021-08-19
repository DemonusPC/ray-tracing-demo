use crate::aabb::AABB;
use crate::HitAble;
use crate::{hit::HitRecord, utility::random_int_from_values, world::World};
use std::cmp::Ordering;

#[derive(Clone)]
pub struct BvhNode {
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>,
    pub node_box: AABB,
    id: Option<usize>,
}

fn box_compare(a: &AABB, b: &AABB, axis: usize) -> Ordering {
    if a.min()[axis] == b.min()[axis] {
        return Ordering::Equal;
    }

    match a.min()[axis] < b.min()[axis] {
        true => Ordering::Less,
        false => Ordering::Greater,
    }
}

impl BvhNode {
    pub fn new(
        left: Option<Box<BvhNode>>,
        right: Option<Box<BvhNode>>,
        node_box: AABB,
        id: Option<usize>,
    ) -> BvhNode {
        BvhNode {
            left,
            right,
            node_box,
            id,
        }
    }

    pub fn new_from_list(objects: &mut [Box<dyn HitAble>], time0: f64, time1: f64) -> BvhNode {
        // eprintln!("{}", objects.len());
        let axis = random_int_from_values(0, 2) as usize;

        let object_span = objects.len();

        if object_span == 1 {
            // eprintln!("Leaf");
            // At 1 node we just return a leaf
            // This unwrap is dangerous. Need to fix this later
            let aabb_box = objects[0].bounding_box(time0, time1).unwrap();

            return BvhNode::new(None, None, aabb_box, objects[0].id());
        } else if object_span == 2 {
            // eprintln!("Split Leaf");
            match box_compare(
                &objects[0].bounding_box(time0, time1).unwrap(),
                &objects[1].bounding_box(time0, time1).unwrap(),
                axis,
            ) {
                Ordering::Less => {
                    let left_node = BvhNode::new(
                        None,
                        None,
                        objects[0].bounding_box(time0, time1).unwrap(),
                        objects[0].id(),
                    );
                    let right_node = BvhNode::new(
                        None,
                        None,
                        objects[1].bounding_box(time0, time1).unwrap(),
                        objects[1].id(),
                    );

                    // Because im using ids. I can just build the last 2 levels of the node.
                    let aabb_box = AABB::surrounding_box(
                        objects[0].bounding_box(time0, time1).unwrap(),
                        objects[1].bounding_box(time0, time1).unwrap(),
                    );

                    return BvhNode::new(
                        Option::from(Box::new(left_node)),
                        Option::from(Box::new(right_node)),
                        aabb_box,
                        None,
                    );
                }
                _ => {
                    let left_node = BvhNode::new(
                        None,
                        None,
                        objects[1].bounding_box(time0, time1).unwrap(),
                        objects[1].id(),
                    );
                    let right_node = BvhNode::new(
                        None,
                        None,
                        objects[0].bounding_box(time0, time1).unwrap(),
                        objects[0].id(),
                    );

                    // Because im using ids. I can just build the last 2 levels of the node.
                    let aabb_box = AABB::surrounding_box(
                        objects[0].bounding_box(time0, time1).unwrap(),
                        objects[1].bounding_box(time0, time1).unwrap(),
                    );

                    return BvhNode::new(
                        Option::from(Box::new(left_node)),
                        Option::from(Box::new(right_node)),
                        aabb_box,
                        None,
                    );
                }
            }
        } else {
            // eprintln!("Digging further");
            objects.sort_by(|a, b| {
                return box_compare(
                    &a.bounding_box(time0, time1).unwrap(),
                    &b.bounding_box(time0, time1).unwrap(),
                    axis,
                );
            });

            let mid = 0 + object_span / 2;
            let end = objects.len();

            let left_node = BvhNode::new_from_list(&mut objects[0..mid], time0, time1);
            let right_node = BvhNode::new_from_list(&mut objects[mid..end], time0, time1);

            let aabb_box = AABB::surrounding_box(
                left_node.bounding_box(time0, time1).unwrap(),
                right_node.bounding_box(time0, time1).unwrap(),
            );

            return BvhNode::new(
                Option::from(Box::new(left_node)),
                Option::from(Box::new(right_node)),
                aabb_box,
                None,
            );
        }
    }

    pub fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hit::HitRecord,
        world: &World,
    ) -> bool {
        // old
        if !self.node_box.hit(r, t_min, t_max) {
            return false;
        }

        if self.left.is_none() && self.right.is_none() {

            let (object, material) = world.get(self.id.unwrap());

            if object.hit(r, t_min, t_max, rec) {
                rec.set_id(self.id);
                return true;
            }
            return false;            
        }

        let hit_left = match &self.left {
            Some(l) => l.hit(r, t_min, t_max, rec, world),
            None => false,
        };

        let param = if hit_left { rec.t() } else { t_max };

        let hit_right = match &self.right {
            Some(ri) => ri.hit(r, t_min, param, rec, world),
            None => false,
        };

        // if hit_left || hit_right {
        //     eprintln!("BVH hit");
        // }

        hit_left || hit_right
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.node_box)
    }
}

// impl HitAble for BvhNode {
//     fn hit(
//         &self,
//         r: &crate::ray::Ray,
//         t_min: f64,
//         t_max: f64,
//         rec: &mut crate::hit::HitRecord,
//     ) -> bool {
//         if self.node_box.hit(r, t_min, t_max) {
//             let left_rec = HitRecord::empty();
//             let right_rec = HitRecord::empty();

//             let hit_left = self.left

//         }

//         // old
//         if !self.node_box.hit(r, t_min, t_max) {
//             return false;
//         }

//         if self.left.is_none() && self.right.is_none() {
//             rec.set_id(self.id);
//             return true;
//         }

//         let hit_left = match &self.left {
//             Some(l) => l.hit(r, t_min, t_max, rec),
//             None => false
//         };

//         let param = if hit_left { rec.t() } else { t_max };

//         let hit_right = match &self.right {
//             Some(ri) => ri.hit(r, t_min, param, rec),
//             None => false
//         };

//         // if hit_left || hit_right {
//         //     eprintln!("BVH hit");
//         // }

//         hit_left || hit_right
//     }

//     fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
//         Some(self.node_box)
//     }

//     fn id(&self) -> Option<usize> {
//         self.id
//     }
// }
