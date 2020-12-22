use rand::prelude::*;

pub fn ffmin(a: f64, b: f64) -> f64 {
    if a <= b {
        a
    } else {
        b
    }
}

pub fn ffmax(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    let result: f64 = rng.gen_range(0.0, 1.0);
    result
}

pub fn random_double_from_values(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    x
}

pub fn random_int_from_values(min: i32, max: i32) -> i32 {
    return random_double_from_values(min as f64, max as f64) as i32
}
