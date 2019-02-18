use color::Color;
use intersection::Precompute;
use material::Material;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use point::point;
use point_light::PointLight;
use ray::Ray;
use sphere::Sphere;

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_source: PointLight,
}

impl World {
    pub fn new() -> World {
        return World {
            objects: vec![
                Sphere {
                    transform: IDENTITY_MATRIX,
                    material: Material {
                        ambient: 0.1,
                        color: Color::new(0.8, 1.0, 0.6),
                        diffuse: 0.7,
                        shininess: 200.0,
                        specular: 0.2,
                    },
                },
                Sphere {
                    material: Material::new(),
                    transform: Matrix4::scaling(0.5, 0.5, 0.5),
                },
            ],
            light_source: PointLight {
                intensity: Color::new(1.0, 1.0, 1.0),
                position: point(-10.0, 10.0, -10.0),
            },
        };
    }

    pub fn shade_hit(&self, precompute: Precompute) -> Color {
        precompute.object.material.lighting(
            &self.light_source,
            &precompute.point,
            &precompute.eyev,
            &precompute.normalv,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let hits = ray.intersect_world(&self);
        if hits.len() > 0 {
            self.shade_hit(hits[0].precompute(&ray))
        } else {
            Color::black()
        }
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use intersection::Intersection;
    use point::point;
    use point::vector;
    use point_light::PointLight;
    use ray::Ray;
    use world::World;

    #[test]
    fn test_default_world() {
        let default_world = World::new();

        assert_eq!(
            default_world.light_source.intensity,
            Color::new(1.0, 1.0, 1.0)
        );
        assert!(default_world
            .light_source
            .position
            .equal(&point(-10.0, 10.0, -10.0)));
        assert_eq!(default_world.objects.len(), 2);
    }

    #[test]
    fn test_shade_color() {
        let default_world = World::new();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let i = Intersection {
            object: default_world.objects[0],
            t: 4.0,
        };
        let comps = i.precompute(&r);
        let c = default_world.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shade_color_2() {
        let mut world = World::new();
        world.light_source = PointLight {
            position: point(0.0, 0.25, 0.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let i = Intersection {
            object: world.objects[1],
            t: 0.5,
        };
        let comps = i.precompute(&r);
        let c = world.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_world_color_at() {
        let mut world = World::new();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 1.0, 0.0),
        };

        assert_eq!(world.color_at(&r), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_world_color_at_2() {
        let mut world = World::new();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };

        assert_eq!(world.color_at(&r), Color::new(0.38066, 0.47583, 0.2855));
    }
}
