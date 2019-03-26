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
    Blended(Box<Patternable>, Box<Patternable>),
    Checker(Box<Patternable>, Box<Patternable>),
    Gradient(Box<Patternable>, Box<Patternable>),
    Perlin(PerlinNoise, Box<Patternable>, f64),
    Ring(Box<Patternable>, Box<Patternable>),
    Solid(Color),
    Stripe(Box<Patternable>, Box<Patternable>),
}

#[derive(Debug, Clone)]
pub struct Patternable {
    patternable_type: PatternableType,
    pub transform: Matrix4,
}

impl Patternable {
    pub fn solid(color: Color) -> Patternable {
        Patternable {
            patternable_type: PatternableType::Solid(color),
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn gradient(color: Color, secondary: Color) -> Patternable {
        Patternable {
            patternable_type: PatternableType::Gradient(
                Box::new(Patternable::solid(color)),
                Box::new(Patternable::solid(secondary)),
            ),
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn checker(color: Color, secondary: Color) -> Patternable {
        Patternable {
            patternable_type: PatternableType::Checker(
                Box::new(Patternable::solid(color)),
                Box::new(Patternable::solid(secondary)),
            ),
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn stripe(color: Color, secondary: Color) -> Patternable {
        Patternable {
            patternable_type: PatternableType::Stripe(
                Box::new(Patternable::solid(color)),
                Box::new(Patternable::solid(secondary)),
            ),
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn ring(color: Color, secondary: Color) -> Patternable {
        Patternable {
            patternable_type: PatternableType::Ring(
                Box::new(Patternable::solid(color)),
                Box::new(Patternable::solid(secondary)),
            ),
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn blended(primary: Patternable, secondary: Patternable) -> Patternable {
        Patternable {
            patternable_type: PatternableType::Blended(Box::new(primary), Box::new(secondary)),
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn perlin(pattern: Patternable) -> Patternable {
        Patternable {
            patternable_type: PatternableType::Perlin(PerlinNoise::new(), Box::new(pattern), 0.25),
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn color_at(&self, point: &Point) -> Color {
        match self.patternable_type {
            PatternableType::Blended(ref a, ref b) => self.color_at_blended(point, a, b),
            PatternableType::Checker(ref a, ref b) => {
                self.color_at_checker(point, a.color_at(point), b.color_at(point))
            }
            PatternableType::Gradient(ref a, ref b) => {
                self.color_at_gradient(point, a.color_at(point), b.color_at(point))
            }
            PatternableType::Perlin(perlin, ref pattern, factor) => {
                self.color_at_perlin(point, pattern, perlin, factor)
            }
            PatternableType::Ring(ref a, ref b) => {
                self.color_at_ring(point, a.color_at(point), b.color_at(point))
            }
            PatternableType::Solid(c) => c,
            PatternableType::Stripe(ref a, ref b) => {
                self.color_at_stripe(point, a.color_at(point), b.color_at(point))
            }
        }
    }

    pub fn color_at_object(&self, object: &Shape, point: &Point) -> Color {
        let local = object.transform.inverse().multiply_point(&point);
        let pattern_local = self.transform.inverse().multiply_point(&local);
        self.color_at(&pattern_local)
    }

    fn color_at_gradient(&self, point: &Point, a: Color, b: Color) -> Color {
        let difference = a.sub(&b);
        a.add(&difference.multiply_scalar(point.x - point.x.floor()))
    }

    fn color_at_checker(&self, point: &Point, primary: Color, secondary: Color) -> Color {
        let sum = point.x.round() + point.y.round() + point.z.round();
        if equal(sum.abs() % 2., 0.) {
            primary
        } else {
            secondary
        }
    }

    fn color_at_stripe(&self, point: &Point, a: Color, b: Color) -> Color {
        if equal(point.x.floor().abs() % 2.0, 0.0) {
            a
        } else {
            b
        }
    }

    fn color_at_ring(&self, point: &Point, a: Color, b: Color) -> Color {
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() % 2.0 == 0.0 {
            a
        } else {
            b
        }
    }

    fn color_at_perlin(
        &self,
        local_point: &Point,
        pattern: &Patternable,
        perlin: PerlinNoise,
        factor: f64,
    ) -> Color {
        let addition = perlin.get([local_point.x, local_point.y, local_point.z]) * factor;
        pattern.color_at(&point(
            local_point.x + addition,
            local_point.y + addition,
            local_point.z + addition,
        ))
    }

    fn color_at_blended(&self, point: &Point, a: &Patternable, b: &Patternable) -> Color {
        a.color_at(&point).add(&b.color_at(&point)).divide(2.0)
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
