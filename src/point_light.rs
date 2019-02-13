use color::Color;
use point::Point;

pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}

impl PointLight {}

#[cfg(test)]
mod tests {
    use color::Color;
    use point::point;
    use point_light::PointLight;

    #[test]
    fn test_point_light() {
        let l = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: point(0.0, 0.0, 0.0),
        };

        assert_eq!(l.intensity, Color::new(1.0, 1.0, 1.0));
        assert!(l.position.equal(&point(0.0, 0.0, 0.0)));
    }
}
