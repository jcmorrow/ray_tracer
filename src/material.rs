use color::Color;
use point::Point;
use point_light::PointLight;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Material {
    pub ambient: f64,
    pub color: Color,
    pub diffuse: f64,
    pub shininess: f64,
    pub specular: f64,
}

impl Material {
    pub fn new() -> Material {
        return Material {
            ambient: 0.1,
            color: Color::new(1.0, 1.0, 1.0),
            diffuse: 0.9,
            shininess: 200.0,
            specular: 0.9,
        };
    }

    pub fn lighting(&self, light: PointLight, position: Point, eye: Point, normal: Point) -> Color {
        return Color::new(1.0, 1.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use material::Material;
    use utilities::equal;

    #[test]
    fn test_default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert!(equal(m.ambient, 0.1));
        assert!(equal(m.diffuse, 0.9));
        assert!(equal(m.specular, 0.9));
        assert!(equal(m.shininess, 200.0));
    }
}
