use color::Color;
use pattern::Patternable;
use pattern::Solid;
use point::Point;
use point_light::PointLight;
use shape::Shape;
use std::sync::Arc;
use utilities::equal;

#[derive(Debug)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub pattern: Solid,
    pub reflective: f64,
    pub shininess: f64,
    pub specular: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            ambient: 0.1,
            diffuse: 0.9,
            shininess: 200.0,
            specular: 0.9,
            pattern: Solid::new(Color::white()),
            reflective: 0.0,
        }
    }

    pub fn equal(&self, other: &Material) -> bool {
        equal(self.ambient, other.ambient)
            && equal(self.diffuse, other.diffuse)
            && equal(self.shininess, other.shininess)
            && equal(self.specular, other.specular)
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        position: &Point,
        eye: &Point,
        normal: &Point,
        in_shadow: bool,
        object: &Shape,
    ) -> Color {
        let mut diffuse: Color = Color::black();
        let mut specular: Color = Color::black();

        let color = self.pattern.color_at_object(&object, &position);
        let effective_color = color.hadamard_product(&light.intensity);
        let ambient = effective_color.multiply_scalar(self.ambient);
        let lightv = light.position.sub(position).normalize();
        let light_dot_normal = lightv.dot(normal);
        if light_dot_normal >= 0.0 {
            diffuse = effective_color
                .multiply_scalar(self.diffuse)
                .multiply_scalar(light_dot_normal);
            let reflectv = lightv.multiply_scalar(-1.0).reflect(normal);
            let reflect_dot_eye = reflectv.dot(eye);
            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light
                    .intensity
                    .multiply_scalar(self.specular)
                    .multiply_scalar(factor);
            }
        }
        if in_shadow {
            ambient
        } else {
            ambient.add(&diffuse).add(&specular)
        }
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use material::Material;
    use pattern::Stripe;
    use point::point;
    use point::vector;
    use point_light::PointLight;
    use shape::Shape;
    use utilities::equal;

    #[test]
    fn test_default_material() {
        let m = Material::new();

        assert!(equal(m.ambient, 0.1));
        assert!(equal(m.diffuse, 0.9));
        assert!(equal(m.specular, 0.9));
        assert!(equal(m.shininess, 200.0));
        assert!(equal(m.reflective, 0.0));
    }

    #[test]
    fn test_lighting() {
        let object = Shape::sphere();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: point(0.0, 0.0, -10.0),
        };
        let position = point(0.0, 0.0, 0.0);

        let result = Material::new().lighting(&light, &position, &eyev, &normalv, false, &object);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_2() {
        let object = Shape::sphere();
        let sqrt_2_over_2 = 2.0_f64.sqrt() / 2.0;
        let eyev = vector(0.0, sqrt_2_over_2, sqrt_2_over_2);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: point(0.0, 0.0, -10.0),
        };
        let position = point(0.0, 0.0, 0.0);

        let result = Material::new().lighting(&light, &position, &eyev, &normalv, false, &object);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_lighting_3() {
        let object = Shape::sphere();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: point(0.0, 10.0, -10.0),
        };
        let position = point(0.0, 0.0, 0.0);

        let result = Material::new().lighting(&light, &position, &eyev, &normalv, false, &object);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn test_lighting_4() {
        let object = Shape::sphere();
        let sqrt_2_over_2 = 2.0_f64.sqrt() / 2.0;
        let eyev = vector(0.0, -sqrt_2_over_2, -sqrt_2_over_2);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: point(0.0, 10.0, -10.0),
        };
        let position = point(0.0, 0.0, 0.0);

        let result = Material::new().lighting(&light, &position, &eyev, &normalv, false, &object);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn test_lighting_5() {
        let object = Shape::sphere();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: point(0.0, 0.0, 10.0),
        };
        let position = point(0.0, 0.0, 0.0);

        let result = Material::new().lighting(&light, &position, &eyev, &normalv, false, &object);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_lighting_with_the_surface_in_shadow() {
        let object = Shape::sphere();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: point(0.0, 0.0, -10.0),
        };
        let position = point(0.0, 0.0, 0.0);

        let result = Material::new().lighting(&light, &position, &eyev, &normalv, true, &object);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_lighting_with_pattern() {
        let object = Shape::sphere();
        let mut m = Material::new();
        m.pattern = Arc::new(Stripe::new(Color::black(), Color::white()));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: point(0.0, 0.0, -10.0),
        };
        let c1 = m.lighting(
            &light,
            &point(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
            &object,
        );
        let c2 = m.lighting(
            &light,
            &point(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
            &object,
        );

        assert_eq!(c1, Color::black());
        assert_eq!(c2, Color::white());
    }
}
