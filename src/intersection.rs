use point::Point;
use ray::Ray;
use sphere::Sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection {
    pub object: Sphere,
    pub t: f64,
}

#[derive(Debug, PartialEq)]
pub struct Precompute {
    pub eyev: Point,
    pub inside: bool,
    pub normalv: Point,
    pub object: Sphere,
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
            let mut hit: Intersection = hits[0];
            for h in hits {
                if h.t > 0.0 && h.t < hit.t {
                    hit = *h;
                }
            }
            return Some(hit);
        }
        return None;
    }

    pub fn precompute(&self, ray: &Ray) -> Precompute {
        let point = ray.position(self.t);
        let mut precompute = Precompute {
            eyev: ray.direction.multiply_scalar(-1.0),
            inside: false,
            normalv: self.object.normal_at(&point),
            object: self.object,
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
    use point::point;
    use point::vector;
    use ray::Ray;
    use sphere::Sphere;
    use utilities::equal;

    #[test]
    fn test_intersection() {
        let s = Sphere::new();
        let i = Intersection { object: s, t: 3.5 };

        assert_eq!(i.object, s);
        assert!(equal(i.t, 3.5));
    }

    #[test]
    fn test_intersections() {
        let s = Sphere::new();
        let i1 = Intersection { object: s, t: 1.0 };
        let i2 = Intersection { object: s, t: 2.0 };
        let xs = Intersection::intersections(i1, i2);

        assert_eq!(xs.len(), 2);
        assert!(equal(xs[0].t, 1.0));
        assert!(equal(xs[1].t, 2.0));
    }

    #[test]
    fn test_hits_when_all_positive() {
        let s = Sphere::new();
        let i1 = Intersection { t: 1.0, object: s };
        let i2 = Intersection { t: 2.0, object: s };

        let hit = Intersection::hit(&mut vec![i1, i2]);

        assert_eq!(hit.unwrap(), i1);
    }

    #[test]
    fn test_hits_when_some_negative() {
        let s = Sphere::new();
        let i1 = Intersection { t: -1.0, object: s };
        let i2 = Intersection { t: 2.0, object: s };

        let hit = Intersection::hit(&mut vec![i1, i2]);

        assert_eq!(hit.unwrap(), i2);
    }

    #[test]
    fn test_hits_when_all_negative() {
        let s = Sphere::new();
        let i1 = Intersection { t: -1.0, object: s };
        let i2 = Intersection { t: -2.0, object: s };

        let hit = Intersection::hit(&mut vec![i1, i2]);

        assert_eq!(hit, None);
    }

    #[test]
    fn test_precompute_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let i = Intersection {
            object: sphere,
            t: 4.0,
        };

        let precompute = i.precompute(&r);

        assert_eq!(
            precompute,
            Precompute {
                eyev: vector(0.0, 0.0, -1.0),
                inside: false,
                normalv: vector(0.0, 0.0, -1.0),
                object: sphere,
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
        let sphere = Sphere::new();
        let i = Intersection {
            object: sphere,
            t: 1.0,
        };

        let precompute = i.precompute(&r);

        assert_eq!(
            precompute,
            Precompute {
                eyev: vector(0.0, 0.0, -1.0),
                inside: true,
                normalv: vector(0.0, 0.0, -1.0),
                object: sphere,
                point: point(0.0, 0.0, 1.0),
                t: i.t,
            }
        );
    }
}
