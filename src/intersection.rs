use point::Point;
use ray::Ray;
use shape::Shape;
use utilities::EPSILON;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub object: Shape,
    pub t: f64,
}

#[derive(Debug, PartialEq)]
pub struct Precompute {
    pub over_point: Point,
    pub eyev: Point,
    pub inside: bool,
    pub normalv: Point,
    pub object: Shape,
    pub point: Point,
    pub t: f64,
}

impl Intersection {
    pub fn intersections(i1: Intersection, i2: Intersection) -> Vec<Intersection> {
        vec![i1, i2]
    }

    pub fn hit(hits: &mut Vec<Intersection>) -> Option<Intersection> {
        hits.retain(|x| x.t > 0.0);
        if hits.len() > 0 {
            let mut hit: Intersection = hits[0].clone();
            for h in hits {
                if h.t > 0.0 && h.t < hit.t {
                    hit = h.clone();
                }
            }
            return Some(hit);
        }
        return None;
    }

    pub fn precompute(&self, ray: &Ray) -> Precompute {
        let point = ray.position(self.t);
        let normalv = self.object.normal_at(&point);
        let mut precompute = Precompute {
            eyev: ray.direction.multiply_scalar(-1.0),
            inside: false,
            normalv: normalv,
            object: self.object.clone(),
            over_point: point.add(&normalv.multiply_scalar(EPSILON)),
            point: point,
            t: self.t,
        };
        if precompute.normalv.dot(&precompute.eyev) < 0.0 {
            precompute.inside = true;
            precompute.normalv = precompute.normalv.multiply_scalar(-1.0);
        }
        precompute
    }
}

#[cfg(test)]
mod tests {
    use intersection::Intersection;
    use intersection::Precompute;
    use matrix::Matrix4;
    use point::point;
    use point::vector;
    use ray::Ray;
    use shape::Shape;
    use utilities::equal;
    use utilities::EPSILON;

    #[test]
    fn test_intersection() {
        let s = Shape::sphere();
        let i = Intersection {
            object: s.clone(),
            t: 3.5,
        };

        assert_eq!(i.object, s);
        assert!(equal(i.t, 3.5));
    }

    #[test]
    fn test_intersections() {
        let s = Shape::sphere();
        let i1 = Intersection {
            object: s.clone(),
            t: 1.0,
        };
        let i2 = Intersection {
            object: s.clone(),
            t: 2.0,
        };
        let xs = Intersection::intersections(i1, i2);

        assert_eq!(xs.len(), 2);
        assert!(equal(xs[0].t, 1.0));
        assert!(equal(xs[1].t, 2.0));
    }

    #[test]
    fn test_hits_when_all_positive() {
        let s = Shape::sphere();
        let i1 = Intersection {
            t: 1.0,
            object: s.clone(),
        };
        let i2 = Intersection {
            t: 2.0,
            object: s.clone(),
        };

        let hit = Intersection::hit(&mut vec![i1.clone(), i2.clone()]);

        assert_eq!(hit.unwrap(), i1);
    }

    #[test]
    fn test_hits_when_some_negative() {
        let s = Shape::sphere();
        let i1 = Intersection {
            t: -1.0,
            object: s.clone(),
        };
        let i2 = Intersection {
            t: 2.0,
            object: s.clone(),
        };

        let hit = Intersection::hit(&mut vec![i1.clone(), i2.clone()]);

        assert_eq!(hit.unwrap(), i2);
    }

    #[test]
    fn test_hits_when_all_negative() {
        let s = Shape::sphere();
        let i1 = Intersection {
            t: -1.0,
            object: s.clone(),
        };
        let i2 = Intersection {
            t: -2.0,
            object: s.clone(),
        };

        let hit = Intersection::hit(&mut vec![i1.clone(), i2.clone()]);

        assert_eq!(hit, None);
    }

    #[test]
    fn test_precompute_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Shape::sphere();
        let i = Intersection {
            object: shape.clone(),
            t: 4.0,
        };

        let precompute = i.precompute(&r);

        assert_eq!(
            precompute,
            Precompute {
                eyev: vector(0.0, 0.0, -1.0),
                inside: false,
                normalv: vector(0.0, 0.0, -1.0),
                object: shape,
                over_point: point(0.0, 0.0, -1.00001),
                point: point(0.0, 0.0, -1.0),
                t: i.t,
            }
        );
    }

    #[test]
    fn test_precompute_intersection_inside() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Shape::sphere();
        let i = Intersection {
            object: shape.clone(),
            t: 1.0,
        };

        let precompute = i.precompute(&r);

        assert_eq!(
            precompute,
            Precompute {
                eyev: vector(0.0, 0.0, -1.0),
                inside: true,
                normalv: vector(0.0, 0.0, -1.0),
                object: shape,
                over_point: point(0.0, 0.0, 1.00001),
                point: point(0.0, 0.0, 1.0),
                t: i.t,
            }
        );
    }

    #[test]
    fn test_precompute_intersection_slightly_above() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut shape = Shape::sphere();
        shape.transform = Matrix4::translation(0.0, 0.0, 1.0);
        let i = Intersection {
            object: shape,
            t: 5.0,
        };

        let precompute = i.precompute(&r);

        assert!(precompute.over_point.z < -EPSILON / 2.0);
        assert!(precompute.point.z > precompute.over_point.z);
    }
}
