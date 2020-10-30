use crate::hit::{HitAble, HitRecord};
use crate::ray::Ray;
use crate::utility::random_double;
use crate::vec3::Vec3;
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_prt: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64, mat_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_prt: mat_ptr,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
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
                rec.mat_ptr = self.mat_prt.clone();
                return true;
            }

            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                let outward_normal = (rec.p() - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = self.mat_prt.clone();
                return true;
            }
        }

        false
    }
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal() + Vec3::random_unit_vector();
        *scattered = Ray::new(&rec.p(), &scatter_direction, ray_in.time());
        *attenuation = Vec3::new(self.albedo.x(), self.albedo.y(), self.albedo.z());
        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Vec3, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo: color,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(ray_in.direction()), &rec.normal());
        *scattered = Ray::new(
            &rec.p(),
            &(reflected + Vec3::random_in_unit_sphere() * self.fuzz),
            ray_in.time()
        );
        *attenuation = Vec3::new(self.albedo.x(), self.albedo.y(), self.albedo.z());

        let result = Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0;

        result
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face() {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = Vec3::unit_vector(ray_in.direction());

        let cos_theta = Vec3::dot(&-unit_direction, &rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal());
            *scattered = Ray::new(&rec.p(), &reflected, ray_in.time());
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if random_double() < reflect_prob {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal());
            *scattered = Ray::new(&rec.p(), &reflected, ray_in.time());

            return true;
        }

        let refracted = Vec3::refract(&unit_direction, &rec.normal(), etai_over_etat);

        *scattered = Ray::new(&rec.p(), &refracted, ray_in.time());

        true
    }
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Rc<dyn Material>
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0 + ((time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0));
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
            rec.mat_ptr = self.mat_ptr.clone();
            return true;
        }

        let temp = (-half_b + root) / a;

        if temp < t_max && temp > t_min {
            rec.set_t(temp);
            rec.set_p(r.at(rec.t()));
            let outward_normal = (rec.p() - self.center(r.time())) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            rec.mat_ptr = self.mat_ptr.clone();
            return true;
        }
    }

    false
}
}