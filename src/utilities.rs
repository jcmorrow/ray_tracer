pub const EPSILON: f64 = 0.00001;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn clamp(number: f64, min: f64, max: f64) -> f64 {
    if number > max {
        max
    } else if number < min {
        min
    } else {
        number
    }
}

pub fn min(xs: &[f64]) -> f64 {
    xs.iter().cloned().fold(std::f64::NAN, f64::min)
}

pub fn max(xs: &[f64]) -> f64 {
    xs.iter().cloned().fold(std::f64::NAN, f64::max)
}

#[cfg(test)]
mod tests {
    use utilities::max;
    use utilities::min;

    #[test]
    fn test_min() {
        assert_eq!(min(&vec!(0., 1.)), 0.);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(&vec!(1., 2.)), 2.);
    }
}
