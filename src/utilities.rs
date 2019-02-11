const EPSILON: f64 = 0.00001;

pub fn equal(a: f64, b: f64) -> bool {
    return (a - b).abs() < EPSILON;
}

pub fn clamp(number: f64, min: f64, max: f64) -> f64 {
    if number > max {
        return max;
    } else if number < min {
        return min;
    } else {
        return number;
    }
}
