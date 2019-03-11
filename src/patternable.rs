use color::Color;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use noise::{NoiseFn, Perlin as PerlinNoise};
use point::point;
use point::Point;
use shape::Shape;
use utilities::equal;

#[derive(Debug, Clone)]
pub enum PatternableType {
    Blended,
    Checker,
    Gradient,
    Perlin,
    Ring,
    Solid,
    Stripe,
}

#[derive(Debug, Clone)]
pub struct Patternable {
    color: Color,
    patternable_type: PatternableType,
    secondary: Color,
    pub transform: Matrix4,
    primary_pattern: Option<Box<Patternable>>,
    secondary_pattern: Option<Box<Patternable>>,
    perlin: PerlinNoise,
    pub perlin_factor: f64,
}

impl Patternable {
    pub fn solid(color: Color) -> Patternable {
        Patternable {
            color,
            patternable_type: PatternableType::Solid,
            secondary: Color::white(),
            transform: IDENTITY_MATRIX,
            primary_pattern: None,
            secondary_pattern: None,
            perlin: PerlinNoise::new(),
            perlin_factor: 0.,
        }
    }

    pub fn gradient(color: Color, secondary: Color) -> Patternable {
        Patternable {
            color,
            patternable_type: PatternableType::Gradient,
            secondary,
            transform: IDENTITY_MATRIX,
            primary_pattern: None,
            secondary_pattern: None,
            perlin: PerlinNoise::new(),
            perlin_factor: 0.,
        }
    }

    pub fn checker(color: Color, secondary: Color) -> Patternable {
        Patternable {
            color,
            patternable_type: PatternableType::Checker,
            secondary,
            transform: IDENTITY_MATRIX,
            primary_pattern: None,
            secondary_pattern: None,
            perlin: PerlinNoise::new(),
            perlin_factor: 0.,
        }
    }

    pub fn stripe(color: Color, secondary: Color) -> Patternable {
        Patternable {
            color,
            patternable_type: PatternableType::Stripe,
            secondary,
            transform: IDENTITY_MATRIX,
            primary_pattern: None,
            secondary_pattern: None,
            perlin: PerlinNoise::new(),
            perlin_factor: 0.,
        }
    }

    pub fn ring(color: Color, secondary: Color) -> Patternable {
        Patternable {
            color,
            patternable_type: PatternableType::Ring,
            secondary,
            transform: IDENTITY_MATRIX,
            primary_pattern: None,
            secondary_pattern: None,
            perlin: PerlinNoise::new(),
            perlin_factor: 0.,
        }
    }

    pub fn blended(primary_pattern: Patternable, secondary_pattern: Patternable) -> Patternable {
        Patternable {
            color: Color::white(),
            patternable_type: PatternableType::Blended,
            secondary: Color::white(),
            transform: IDENTITY_MATRIX,
            primary_pattern: Some(Box::new(primary_pattern)),
            secondary_pattern: Some(Box::new(secondary_pattern)),
            perlin: PerlinNoise::new(),
            perlin_factor: 0.,
        }
    }

    pub fn perlin(primary_pattern: Patternable) -> Patternable {
        Patternable {
            color: Color::white(),
            patternable_type: PatternableType::Perlin,
            secondary: Color::white(),
            transform: IDENTITY_MATRIX,
            primary_pattern: Some(Box::new(primary_pattern)),
            secondary_pattern: None,
            perlin: PerlinNoise::new(),
            perlin_factor: 0.25,
        }
    }

    pub fn color_at(&self, point: &Point) -> Color {
        match self.patternable_type {
            PatternableType::Blended => self.color_at_blended(point),
            PatternableType::Checker => self.color_at_checker(point),
            PatternableType::Gradient => self.color_at_gradient(point),
            PatternableType::Perlin => self.color_at_perlin(point),
            PatternableType::Solid => self.color_at_solid(point),
            PatternableType::Stripe => self.color_at_stripe(point),
            _ => Color::white(),
        }
    }

    pub fn color_at_object(&self, object: &Shape, point: &Point) -> Color {
        let local = object.transform.inverse().multiply_point(&point);
        let pattern_local = self.transform.inverse().multiply_point(&local);
        self.color_at(&pattern_local)
    }

    fn color_at_solid(&self, _point: &Point) -> Color {
        self.color
    }

    fn color_at_gradient(&self, point: &Point) -> Color {
        let difference = self.secondary.sub(&self.color);
        self.color
            .add(&difference.multiply_scalar(point.x - point.x.floor()))
    }

    fn color_at_checker(&self, point: &Point) -> Color {
        let sum = point.x.round() + point.y.round() + point.z.round();
        if equal(sum.abs() % 2., 0.) {
            self.color
        } else {
            self.secondary
        }
    }

    fn color_at_stripe(&self, point: &Point) -> Color {
        if equal(point.x.floor().abs() % 2.0, 0.0) {
            self.color
        } else {
            self.secondary
        }
    }

    fn color_at_ring(&self, point: &Point) -> Color {
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() % 2.0 == 0.0 {
            self.color
        } else {
            self.secondary
        }
    }

    fn color_at_perlin(&self, local_point: &Point) -> Color {
        let addition = self
            .perlin
            .get([local_point.x, local_point.y, local_point.z])
            * self.perlin_factor;
        if let Some(ref p) = self.primary_pattern {
            p.color_at(&point(
                local_point.x + addition,
                local_point.y + addition,
                local_point.z + addition,
            ))
        } else {
            Color::white()
        }
    }

    fn color_at_blended(&self, point: &Point) -> Color {
        if let Some(ref a) = self.primary_pattern {
            if let Some(ref b) = self.secondary_pattern {
                a.color_at(&point).add(&b.color_at(&point)).divide(2.0)
            } else {
                Color::white()
            }
        } else {
            Color::white()
        }
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use matrix::Matrix4;
    use patternable::Patternable;
    use point::point;
    use shape::Shape;
    use std::sync::Arc;

    #[test]
    fn test_color_at_stripe() {
        let p = Patternable::stripe(Color::white(), Color::black());

        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 2.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.color_at(&point(2.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(p.color_at(&point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.color_at(&point(-2.0, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn test_color_at_object() {
        let mut sphere = Shape::sphere();
        Arc::get_mut(&mut sphere).unwrap().transform = Matrix4::translation(2.0, 2.0, 2.0);
        let pattern = Patternable::stripe(Color::black(), Color::white());
        let color = pattern.color_at_object(&sphere, &point(1.5, 0.0, 0.0));

        assert_eq!(color, Color::white());
    }

    #[test]
    fn test_color_at_object_with_pattern_transform() {
        let sphere = Shape::sphere();
        let mut pattern = Patternable::stripe(Color::black(), Color::white());
        pattern.transform = Matrix4::translation(2.0, 2.0, 2.0);
        let color = pattern.color_at_object(&sphere, &point(1.5, 0.0, 0.0));

        assert_eq!(color, Color::white());
    }

    #[test]
    fn test_color_at_object_with_both_transforms() {
        let mut sphere = Shape::sphere();
        Arc::get_mut(&mut sphere).unwrap().transform = Matrix4::translation(2.0, 2.0, 2.0);
        let mut pattern = Patternable::stripe(Color::black(), Color::white());
        pattern.transform = Matrix4::translation(0.5, 0.0, 0.0);
        let color = pattern.color_at_object(&sphere, &point(2.5, 0.0, 0.0));

        assert_eq!(color, Color::black());
    }

    #[test]
    fn test_color_at_gradient() {
        let p = Patternable::gradient(Color::white(), Color::black());

        assert_eq!(
            p.color_at(&point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(p.color_at(&point(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(
            p.color_at(&point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
