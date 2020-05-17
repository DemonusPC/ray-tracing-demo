use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn empty() -> Ray {
        Ray::new(&Vec3::empty(), &Vec3::empty())
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn equality(result: &Vec3, x: f64, y: f64, z: f64) {
        assert_eq!(result.x(), x);
        assert_eq!(result.y(), y);
        assert_eq!(result.z(), z);
    }

    #[test]
    fn test_new() {
        let o = Vec3::empty();
        let dir = Vec3::new(1.0, 1.0, 1.0);
        let result = Ray::new(&o, &dir);
        equality(&result.origin(), 0.0, 0.0, 0.0);
        equality(&result.direction(), 1.0, 1.0, 1.0);
    }

    #[test]
    fn test_at() {
        let o = Vec3::empty();
        let dir = Vec3::new(1.0, 1.0, 1.0);
        let result = Ray::new(&o, &dir);
        equality(&result.at(2.0), 2.0, 2.0, 2.0);
    }
}
