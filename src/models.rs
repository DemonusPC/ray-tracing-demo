use crate::{aabb::{self, AABB}, world::World};
use crate::hit::{HitAble, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    id: usize,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64, id: usize) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            id,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn get_sphere_uv(p: &Vec3, u: f64, v: f64) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = -p.z().atan2(p.x()) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
    }
}

impl HitAble for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.set_uv(Sphere::get_sphere_uv(&outward_normal, rec.u(), rec.v()));
                rec.set_id(Some(self.id));
                return true;
            }

            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.set_uv(Sphere::get_sphere_uv(&outward_normal, rec.u(), rec.v()));
                rec.set_id(Some(self.id));
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let a = self.center() - Vec3::new(self.radius(), self.radius(), self.radius());
        let b = self.center() + Vec3::new(self.radius(), self.radius(), self.radius());
        Some(AABB::new(a, b))
    }

    fn id(&self) -> Option<usize> {
        Some(self.id)
    }
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    id: usize,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        id: usize,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            id,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0
            + ((time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0));
    }
}

impl HitAble for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.set_id(Some(self.id));
                return true;
            }

            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.set_id(Some(self.id));
                return true;
            }
        }

        false
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let a0 = self.center(time0) - Vec3::new(self.radius, self.radius, self.radius);
        let b0 = self.center(time0) + Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB::new(a0, b0);

        let a1 = self.center(time1) - Vec3::new(self.radius, self.radius, self.radius);
        let b1 = self.center(time1) + Vec3::new(self.radius, self.radius, self.radius);

        let box1 = AABB::new(a1, b1);

        Some(AABB::surrounding_box(box0, box1))
    }

    fn id(&self) -> Option<usize> {
        Some(self.id)
    }
}

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    id: usize,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, id: usize) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            id,
        }
    }
}

impl HitAble for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();

        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.set_u((x - self.x0) / (self.x1 - self.x0));
        rec.set_u((y - self.y0) / (self.y1 - self.y0));

        rec.set_t(t);

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);

        rec.set_face_normal(r, &outward_normal);
        rec.set_p(r.at(t));
        rec.set_id(Some(self.id));

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(aabb::AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn id(&self) -> Option<usize> {
        Some(self.id)
    }
}

pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    id: usize,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, id: usize) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            id,
        }
    }
}

impl HitAble for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();

        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.set_u((x - self.x0) / (self.x1 - self.x0));
        rec.set_u((z - self.z0) / (self.z1 - self.z0));

        rec.set_t(t);

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);

        rec.set_face_normal(r, &outward_normal);
        rec.set_p(r.at(t));
        rec.set_id(Some(self.id));

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(aabb::AABB::new(
            Vec3::new(self.x0, self.z0, self.k - 0.0001),
            Vec3::new(self.x1, self.z1, self.k + 0.0001),
        ))
    }

    fn id(&self) -> Option<usize> {
        Some(self.id)
    }
}


pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    id: usize,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, id: usize) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            id,
        }
    }
}

impl HitAble for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();

        if t < t_min || t > t_max {
            return false;
        }

        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.set_u((y - self.y0) / (self.y1 - self.y0));
        rec.set_u((z - self.z0) / (self.z1 - self.z0));

        rec.set_t(t);

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);

        rec.set_face_normal(r, &outward_normal);
        rec.set_p(r.at(t));
        rec.set_id(Some(self.id));

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(aabb::AABB::new(
            Vec3::new(self.y0, self.z0, self.k - 0.0001),
            Vec3::new(self.y1, self.z1, self.k + 0.0001),
        ))
    }

    fn id(&self) -> Option<usize> {
        Some(self.id)
    }
}

pub struct Box3D {
    id: usize,
    box_min: Vec3,
    box_max: Vec3,
    sides: Vec<Box<dyn HitAble>>,
}

impl Box3D {
    pub fn new(p0: Vec3, p1: Vec3, id: usize) -> Self{
        let mut sides: Vec<Box<dyn HitAble>>  = vec![];

        sides.push(Box::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), id)));
        sides.push(Box::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), id)));

        sides.push(Box::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), id)));
        sides.push(Box::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), id)));

        sides.push(Box::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), id)));
        sides.push(Box::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), id)));

        Box3D {
            id,
            box_min: p0,
            box_max: p1,
            sides
        }

    }
}

impl HitAble for Box3D {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        for object in self.sides.iter() {
            if object.hit(r, t_min, t_max, rec) {
                rec.set_id(Some(self.id));
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }

    fn id(&self) -> Option<usize> {
        Some(self.id)
    }
}