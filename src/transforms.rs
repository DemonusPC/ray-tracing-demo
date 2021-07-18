use crate::aabb::AABB;
use crate::ray::Ray;
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
