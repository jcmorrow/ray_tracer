use color::Color;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use noise::{NoiseFn, Perlin as PerlinNoise};
use point::point;
use point::Point;
use shape::Shape;
use std::sync::Arc;
use utilities::equal;

#[derive(Debug, Clone)]
pub enum PatternableType {
    Solid,
    Checker,
    Gradient,
}

#[derive(Debug, Clone)]
pub struct Patternable {
    color: Color,
    patternable_type: PatternableType,
    secondary: Color,
    pub transform: Matrix4,
}

impl Patternable {
    pub fn solid(color: Color) -> Patternable {
        Patternable {
            color,
            patternable_type: PatternableType::Solid,
            secondary: Color::white(),
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn gradient(color: Color, secondary: Color) -> Patternable {
        Patternable {
            color,
            patternable_type: PatternableType::Gradient,
            secondary,
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn checker(color: Color, secondary: Color) -> Patternable {
        Patternable {
            color,
            patternable_type: PatternableType::Checker,
            secondary,
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn color_at(&self, point: &Point) -> Color {
        match self.patternable_type {
            PatternableType::Solid => self.color_at_solid(point),
            PatternableType::Gradient => self.color_at_gradient(point),
            PatternableType::Checker => self.color_at_checker(point),
            _ => Color::white(),
        }
    }

    pub fn color_at_object(&self, object: &Shape, point: &Point) -> Color {
        match self.patternable_type {
            PatternableType::Solid => self.color_at_object_solid(object, point),
            PatternableType::Gradient => self.color_at_object_gradient(object, point),
            PatternableType::Checker => self.color_at_object_checker(object, point),
            _ => Color::white(),
        }
    }

    fn color_at_solid(&self, _point: &Point) -> Color {
        self.color
    }

    fn color_at_object_solid(&self, _object: &Shape, _point: &Point) -> Color {
        self.color
    }

    fn color_at_gradient(&self, point: &Point) -> Color {
        let difference = self.secondary.sub(&self.color);
        self.color
            .add(&difference.multiply_scalar(point.x - point.x.floor()))
    }

    fn color_at_object_gradient(&self, object: &Shape, point: &Point) -> Color {
        let local = object.transform.inverse().multiply_point(&point);
        let pattern_local = self.transform.inverse().multiply_point(&local);
        self.color_at(&pattern_local)
    }

    fn color_at_checker(&self, point: &Point) -> Color {
        let sum = point.x.round() + point.y.round() + point.z.round();
        if equal(sum.abs() % 2., 0.) {
            self.color
        } else {
            self.secondary
        }
    }

    fn color_at_object_checker(&self, object: &Shape, point: &Point) -> Color {
        let local = object.transform.inverse().multiply_point(&point);
        let pattern_local = self.transform.inverse().multiply_point(&local);
        self.color_at(&pattern_local)
    }
}

// #[derive(Debug)]
// pub struct Stripe {
//     a: Color,
//     b: Color,
//     pub transform: Matrix4,
// }

// impl Stripe {
//     pub fn new(a: Color, b: Color) -> Stripe {
//         Stripe {
//             a,
//             b,
//             transform: IDENTITY_MATRIX,
//         }
//     }
// }

// impl Patternable for Stripe {
//     fn color_at(&self, point: &Point) -> Color {
//         if equal(point.x.floor().abs() % 2.0, 0.0) {
//             self.a
//         } else {
//             self.b
//         }
//     }

//     fn color_at_object(&self, object: &Shape, point: &Point) -> Color {
//         let local = object.transform.inverse().multiply_point(&point);
//         let pattern_local = self.transform.inverse().multiply_point(&local);
//         self.color_at(&pattern_local)
//     }
// }

// #[derive(Debug)]
// pub struct Ring {
//     a: Color,
//     b: Color,
//     pub transform: Matrix4,
// }

// impl Ring {
//     pub fn new(a: Color, b: Color) -> Ring {
//         Ring {
//             a,
//             b,
//             transform: IDENTITY_MATRIX,
//         }
//     }
// }

// impl Patternable for Ring {
//     fn color_at(&self, point: &Point) -> Color {
//         if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() % 2.0 == 0.0 {
//             self.a
//         } else {
//             self.b
//         }
//     }

//     fn color_at_object(&self, object: &Shape, point: &Point) -> Color {
//         let local = object.transform.inverse().multiply_point(&point);
//         let pattern_local = self.transform.inverse().multiply_point(&local);
//         self.color_at(&pattern_local)
//     }
// }

// #[derive(Debug)]
// pub struct Checker {
//     a: Color,
//     b: Color,
//     pub transform: Matrix4,
// }

// impl Checker {
//     pub fn new(a: Color, b: Color) -> Checker {
//         Checker {
//             a,
//             b,
//             transform: IDENTITY_MATRIX,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Blended {
//     a: Arc<Patternable>,
//     b: Arc<Patternable>,
//     pub transform: Matrix4,
// }

// impl Blended {
//     pub fn new(a: Arc<Patternable>, b: Arc<Patternable>) -> Blended {
//         Blended {
//             a,
//             b,
//             transform: IDENTITY_MATRIX,
//         }
//     }
// }

// impl Patternable for Blended {
//     fn color_at(&self, point: &Point) -> Color {
//         self.a
//             .color_at(&point)
//             .add(&self.b.color_at(&point))
//             .divide(2.0)
//     }

//     fn color_at_object(&self, object: &Shape, point: &Point) -> Color {
//         let local = object.transform.inverse().multiply_point(&point);
//         let pattern_local = self.transform.inverse().multiply_point(&local);
//         self.color_at(&pattern_local)
//     }
// }

// #[derive(Debug)]
// pub struct Perlin {
//     pattern: Arc<Patternable>,
//     perlin: PerlinNoise,
//     pub factor: f64,
//     pub transform: Matrix4,
// }

// impl Perlin {
//     pub fn new(pattern: Arc<Patternable>) -> Perlin {
//         Perlin {
//             pattern,
//             transform: IDENTITY_MATRIX,
//             perlin: PerlinNoise::new(),
//             factor: 1.0,
//         }
//     }
// }

// impl Patternable for Perlin {
//     fn color_at(&self, local_point: &Point) -> Color {
//         let addition = self
//             .perlin
//             .get([local_point.x, local_point.y, local_point.z])
//             * self.factor;
//         self.pattern.color_at(&point(
//             local_point.x + addition,
//             local_point.y + addition,
//             local_point.z + addition,
//         ))
//     }

//     fn color_at_object(&self, object: &Shape, point: &Point) -> Color {
//         let local = object.transform.inverse().multiply_point(&point);
//         let pattern_local = self.transform.inverse().multiply_point(&local);
//         self.color_at(&pattern_local)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use color::Color;
//     use matrix::Matrix4;
//     use pattern::Gradient;
//     use pattern::Patternable;
//     use pattern::Stripe;
//     use point::point;
//     use shape::Shape;

//     #[test]
//     fn test_color_at_stripe() {
//         let p = Stripe::new(Color::white(), Color::black());

//         assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
//         assert_eq!(p.color_at(&point(0.0, 1.0, 0.0)), Color::white());
//         assert_eq!(p.color_at(&point(0.0, 2.0, 0.0)), Color::white());
//         assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
//         assert_eq!(p.color_at(&point(0.0, 0.0, 1.0)), Color::white());
//         assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
//         assert_eq!(p.color_at(&point(1.0, 0.0, 0.0)), Color::black());
//         assert_eq!(p.color_at(&point(2.0, 0.0, 0.0)), Color::white());
//         assert_eq!(p.color_at(&point(-0.1, 0.0, 0.0)), Color::black());
//         assert_eq!(p.color_at(&point(-1.0, 0.0, 0.0)), Color::black());
//         assert_eq!(p.color_at(&point(-2.0, 0.0, 0.0)), Color::white());
//     }

//     #[test]
//     fn test_color_at_object() {
//         let mut sphere = Shape::sphere();
//         sphere.transform = Matrix4::translation(2.0, 2.0, 2.0);
//         let pattern = Stripe::new(Color::black(), Color::white());
//         let color = pattern.color_at_object(&sphere, &point(1.5, 0.0, 0.0));

//         assert_eq!(color, Color::white());
//     }

//     #[test]
//     fn test_color_at_object_with_pattern_transform() {
//         let sphere = Shape::sphere();
//         let mut pattern = Stripe::new(Color::black(), Color::white());
//         pattern.transform = Matrix4::translation(2.0, 2.0, 2.0);
//         let color = pattern.color_at_object(&sphere, &point(1.5, 0.0, 0.0));

//         assert_eq!(color, Color::white());
//     }

//     #[test]
//     fn test_color_at_object_with_both_transforms() {
//         let mut sphere = Shape::sphere();
//         sphere.transform = Matrix4::translation(2.0, 2.0, 2.0);
//         let mut pattern = Stripe::new(Color::black(), Color::white());
//         pattern.transform = Matrix4::translation(0.5, 0.0, 0.0);
//         let color = pattern.color_at_object(&sphere, &point(2.5, 0.0, 0.0));

//         assert_eq!(color, Color::black());
//     }

//     #[test]
//     fn test_color_at_gradient() {
//         let p = Gradient::new(Color::white(), Color::black());

//         assert_eq!(
//             p.color_at(&point(0.25, 0.0, 0.0)),
//             Color::new(0.75, 0.75, 0.75)
//         );
//         assert_eq!(p.color_at(&point(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
//         assert_eq!(
//             p.color_at(&point(0.75, 0.0, 0.0)),
//             Color::new(0.25, 0.25, 0.25)
//         );
//     }
// }
