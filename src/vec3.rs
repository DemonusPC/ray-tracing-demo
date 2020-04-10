use std::fmt;
use std::ops;

pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn empty() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn write_color(&self) {
        let ir = (255.999 * self.e[0]) as i32;
        let ig = (255.999 * self.e[1]) as i32;
        let ib = (255.999 * self.e[2]) as i32;
        print!(
            "{} {} {}\n",
            ir,
            ig,
            ib
        );
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs[0];
        self.e[1] += rhs[1];
        self.e[2] += rhs[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] *= 1.0 / rhs;
        self.e[1] *= 1.0 / rhs;
        self.e[2] *= 1.0 / rhs;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] + other[0],
            self.e[1] + other[1],
            self.e[2] + other[2],
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other[0],
            self.e[1] - other[1],
            self.e[2] - other[2],
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Vec3 {
        Vec3::new(
            self.e[0] * other[0],
            self.e[1] * other[1],
            self.e[2] * other[2],
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
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
        let result = Vec3::new(22.12, 40.0, 0.01);
        equality(&result, 22.12, 40.0, 0.01);
    }

    #[test]
    fn test_empty() {
        let result = Vec3::empty();
        equality(&result, 0.0, 0.0, 0.0);
    }

    #[test]
    fn test_invert() {
        let result = -Vec3::new(1.0, 2.0, 3.0);

        equality(&result, -1.0, -2.0, -3.0);
    }

    #[test]
    fn test_indexing() {
        let result = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(result[0], 1.0);
    }

    #[test]
    fn test_add_assign() {
        let mut result = Vec3::new(1.0, 2.0, 3.0);
        let to_add = Vec3::new(3.0, 2.0, 1.0);

        result += to_add;
        equality(&result, 4.0, 4.0, 4.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut result = Vec3::new(1.0, 2.0, 3.0);

        result *= 3.0;
        equality(&result, 3.0, 6.0, 9.0);
    }

    #[test]
    fn test_div_assign() {
        let mut result = Vec3::new(3.0, 3.0, 3.0);

        result /= 3.0;
        equality(&result, 1.0, 1.0, 1.0);
    }

    #[test]
    fn test_length_squared() {
        let result = Vec3::new(1.0, 2.0, 3.0).length_squared();

        assert_eq!(result, 14.0)
    }

    #[test]
    fn test_length() {
        let result = Vec3::new(1.0, 2.0, 3.0).length();

        assert_eq!(result, 3.7416573867739413)
    }

    #[test]
    fn test_add() {
        let one = Vec3::new(1.0, 2.0, 3.0);
        let two = Vec3::new(3.0, 2.0, 1.0);
        let result = one + two;
        equality(&result, 4.0, 4.0, 4.0);
    }

    #[test]
    fn test_sub() {
        let one = Vec3::new(1.0, 2.0, 3.0);
        let two = Vec3::new(3.0, 2.0, 1.0);
        let result = one - two;
        equality(&result, -2.0, 0.0, 2.0);
    }

    #[test]
    fn test_mul() {
        let one = Vec3::new(1.0, 2.0, 3.0);
        let two = Vec3::new(3.0, 2.0, 1.0);
        let result = one * two;
        equality(&result, 3.0, 4.0, 3.0);
    }

    #[test]
    fn test_mul_scalar() {
        let one = Vec3::new(1.0, 2.0, 3.0);
        let result = one * 10.0;
        equality(&result, 10.0, 20.0, 30.0);
    }

    #[test]
    fn test_div_scalar() {
        let one = Vec3::new(3.0, 3.0, 3.0);
        let result = one / 3.0;
        equality(&result, 1.0, 1.0, 1.0);
    }

    #[test]
    fn test_dot_product() {
        let one = Vec3::new(1.0, 2.0, 3.0);
        let two = Vec3::new(1.0, 5.0, 7.0);
        let result = Vec3::dot(&one, &two);
        assert_eq!(result, 32.0)
    }

    #[test]
    fn test_cross_product() {
        let one = Vec3::new(1.0, 2.0, 3.0);
        let two = Vec3::new(1.0, 5.0, 7.0);
        let result = Vec3::cross(&one, &two);
        equality(&result, -1.0, -4.0, 3.0)
    }
}
