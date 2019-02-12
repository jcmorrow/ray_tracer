use sphere::Sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection {
    pub object: Sphere,
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
}

#[cfg(test)]
mod tests {
    use intersection::Intersection;
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
}
