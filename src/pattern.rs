use color::Color;
use point::Point;
use std::fmt::Debug;

pub trait Patternable: Debug + PatternableClone {
    fn stripe_at(&self, point: &Point) -> Color;
}

pub trait PatternableClone {
    fn clone_box(&self) -> (Box<Patternable>);
}

impl Clone for Box<Patternable> {
    fn clone(&self) -> Box<Patternable> {
        self.clone_box()
    }
}

impl<T> PatternableClone for T
where
    T: 'static + Patternable + Clone,
{
    fn clone_box(&self) -> Box<Patternable> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> StripePattern {
        StripePattern { a, b }
    }
}

impl Patternable for StripePattern {
    fn stripe_at(&self, point: &Point) -> Color {
        if point.x.floor().abs() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Clone, Debug)]
pub struct SolidPattern {
    color: Color,
}

impl SolidPattern {
    pub fn new(color: Color) -> SolidPattern {
        SolidPattern { color }
    }
}

impl Patternable for SolidPattern {
    fn stripe_at(&self, _point: &Point) -> Color {
        self.color
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use pattern::Patternable;
    use pattern::StripePattern;
    use point::point;

    #[test]
    fn test_stripe_at() {
        let p = StripePattern::new(Color::white(), Color::black());

        assert_eq!(p.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(&point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(&point(0.0, 2.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(&point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(p.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(&point(2.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(&point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(&point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(&point(-2.0, 0.0, 0.0)), Color::white());
    }
}
