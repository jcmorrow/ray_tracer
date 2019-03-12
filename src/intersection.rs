use point::Point;
use ray::Ray;
use shape::Shape;
use std::sync::Arc;
use utilities::EPSILON;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub object: Arc<Shape>,
    pub t: f64,
}

#[derive(Debug, PartialEq)]
pub struct Precompute {
    pub eyev: Point,
    pub inside: bool,
    pub normalv: Point,
    pub object: Arc<Shape>,
    pub n1: f64,
    pub n2: f64,
    pub over_point: Point,
    pub under_point: Point,
    pub point: Point,
    pub reflectv: Point,
    pub t: f64,
}

impl Intersection {
    pub fn intersections(i1: Intersection, i2: Intersection) -> Vec<Intersection> {
        vec![i1, i2]
    }

    pub fn hit(hits: &mut Vec<Intersection>) -> Option<Intersection> {
        hits.retain(|x| x.t > 0.0);
        if !hits.is_empty() {
            let mut hit: Intersection = hits[0].clone();
            for h in hits {
                if h.t > 0.0 && h.t < hit.t {
                    hit = h.clone();
                }
            }
            Some(hit)
        } else {
            None
        }
    }

    pub fn precompute(&self, ray: &Ray, xs: Vec<Intersection>) -> Precompute {
        let point = ray.position(self.t);
        let normalv = self.object.normal_at(&point);
        let mut precompute = Precompute {
            eyev: ray.direction.multiply_scalar(-1.0),
            inside: false,
            n1: 1.,
            n2: 1.,
            normalv,
            object: self.object.clone(),
            over_point: point.add(&normalv.multiply_scalar(EPSILON)),
            under_point: point.sub(&normalv.multiply_scalar(EPSILON)),
            point,
            reflectv: ray.direction.reflect(&normalv),
            t: self.t,
        };

        let mut containers: Vec<Arc<Shape>> = Vec::new();

        for i in xs {
            if i == *self {
                if !containers.is_empty() {
                    precompute.n1 = containers.last().unwrap().material.refractive_index;
                } else {
                    precompute.n1 = 1.;
                }
            }
            let mut included = false;
            for s in &containers {
                if s == &i.object {
                    included = true;
                }
            }
            if included {
                containers = containers
                    .iter()
                    .cloned()
                    .filter(|obj| obj != &i.object)
                    .collect()
            } else {
                containers.push(i.object.clone());
            }
            if i == *self {
                if !containers.is_empty() {
                    precompute.n2 = containers.last().unwrap().material.refractive_index;
                } else {
                    precompute.n2 = 1.;
                }
            }
        }

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
    use std::sync::Arc;
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

        let precompute = i.precompute(&r, Vec::new());

        assert_eq!(
            precompute,
            Precompute {
                eyev: vector(0.0, 0.0, -1.0),
                reflectv: vector(0.0, 0.0, -1.0),
                inside: false,
                n1: 1.,
                n2: 1.,
                normalv: vector(0.0, 0.0, -1.0),
                object: shape,
                over_point: point(0.0, 0.0, -1.00001),
                under_point: point(0.0, 0.0, -0.99999),
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

        let precompute = i.precompute(&r, Vec::new());

        assert_eq!(
            precompute,
            Precompute {
                eyev: vector(0.0, 0.0, -1.0),
                inside: true,
                n1: 1.,
                n2: 1.,
                normalv: vector(0.0, 0.0, -1.0),
                object: shape,
                over_point: point(0.0, 0.0, 1.00001),
                under_point: point(0.0, 0.0, 0.99999),
                point: point(0.0, 0.0, 1.0),
                reflectv: vector(0.0, 0.0, -1.0),
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
        Arc::get_mut(&mut shape).unwrap().transform = Matrix4::translation(0.0, 0.0, 1.0);
        let i = Intersection {
            object: shape,
            t: 5.0,
        };

        let precompute = i.precompute(&r, Vec::new());

        assert!(precompute.over_point.z < -EPSILON / 2.0);
        assert!(precompute.point.z > precompute.over_point.z);
    }

    #[test]
    fn test_precompute_intersection_reflective() {
        let shape = Shape::plane();
        let sqrt_two_over_two = 2.0_f64.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 1.0, -1.0),
            direction: vector(0.0, -sqrt_two_over_two, sqrt_two_over_two),
        };
        let i = Intersection {
            object: shape,
            t: 5.0,
        };

        let precompute = i.precompute(&r, Vec::new());

        assert!(precompute
            .reflectv
            .equal(&vector(0.0, sqrt_two_over_two, sqrt_two_over_two)));
    }
}
