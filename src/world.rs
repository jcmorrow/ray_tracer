use color::Color;
use intersectable::Intersectable;
use intersection::Intersection;
use intersection::Precompute;
use material::Material;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use patternable::Patternable;
use point::point;
use point::Point;
use point_light::PointLight;
use ray::Ray;
use shape::Shape;
use std::sync::Arc;

pub struct World {
    pub objects: Vec<Arc<Shape>>,
    pub light_source: PointLight,
}

impl World {
    pub fn new() -> World {
        return World {
            objects: vec![
                Arc::new(Shape {
                    intersectable: Intersectable::sphere(),
                    material: Material {
                        ambient: 0.1,
                        diffuse: 0.7,
                        pattern: Patternable::solid(Color::new(0.8, 1.0, 0.6)),
                        reflective: 0.,
                        refractive_index: 1.,
                        shininess: 200.,
                        specular: 0.2,
                        transparency: 0.,
                    },
                    parent: None,
                    transform: IDENTITY_MATRIX,
                }),
                Arc::new(Shape {
                    intersectable: Intersectable::sphere(),
                    material: Material::new(),
                    parent: None,
                    transform: Matrix4::scaling(0.5, 0.5, 0.5),
                }),
            ],
            light_source: PointLight {
                intensity: Color::new(1.0, 1.0, 1.0),
                position: point(-10.0, 10.0, -10.0),
            },
        };
    }

    pub fn shade_hit(&self, precompute: Precompute, remaining: i32) -> Color {
        let is_shadowed = self.is_shadowed(&precompute.over_point);
        let surface_color = precompute.object.material.lighting(
            &self.light_source,
            &precompute.point,
            &precompute.eyev,
            &precompute.normalv,
            is_shadowed,
            &precompute.object,
        );

        let reflected_color = self.reflected_color(&precompute, remaining);
        let refracted_color = self.refracted_color(&precompute, remaining);

        if precompute.object.material.transparency > 0.
            && precompute.object.material.reflective > 0.
        {
            let reflectance = Intersection::schlick(&precompute);
            surface_color
                .add(&reflected_color.multiply_scalar(reflectance))
                .add(&refracted_color.multiply_scalar(1. - reflectance))
        } else {
            surface_color.add(&reflected_color).add(&refracted_color)
        }
    }

    pub fn color_at(&self, ray: &Ray, remaining: i32) -> Color {
        let hits = ray.intersect_world(&self);
        if hits.is_empty() {
            Color::black()
        } else {
            self.shade_hit(hits[0].precompute(&ray, hits.clone()), remaining)
        }
    }

    pub fn refracted_color(&self, precompute: &Precompute, remaining: i32) -> Color {
        if remaining == 0 {
            return Color::black();
        }
        if precompute.object.material.transparency == 0. {
            return Color::black();
        }

        // total internal reflection
        let n_ratio = precompute.n2 / precompute.n1;
        let cos_i = precompute.normalv.dot(&precompute.eyev);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
        if sin2_t > 1. {
            return Color::black();
        }

        let cos_t = (1. - sin2_t).sqrt();
        let direction = precompute
            .normalv
            .multiply_scalar(n_ratio * cos_i - cos_t)
            .sub(&precompute.eyev.multiply_scalar(n_ratio));

        self.color_at(
            &Ray {
                origin: precompute.under_point,
                direction,
            },
            remaining - 1,
        )
        .multiply_scalar(precompute.object.material.transparency)
    }

    pub fn is_shadowed(&self, point: &Point) -> bool {
        let from_object_to_light_source = self.light_source.position.sub(&point);
        let distance = from_object_to_light_source.magnitude();
        let ray = Ray {
            direction: from_object_to_light_source.normalize(),
            origin: *point,
        };
        match Intersection::hit(&mut ray.intersect_world(self)) {
            Some(hit) => hit.t < distance,
            None => false,
        }
    }

    pub fn reflected_color(&self, precompute: &Precompute, remaining: i32) -> Color {
        if precompute.object.material.reflective == 0.0 || remaining == 0 {
            Color::black()
        } else {
            let ray = Ray {
                origin: precompute.over_point,
                direction: precompute.reflectv,
            };
            let color = self.color_at(&ray, remaining - 1);
            color.multiply_scalar(precompute.object.material.reflective)
        }
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use intersectable::Intersectable;
    use intersection::Intersection;
    use material::Material;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use patternable::Patternable;
    use point::point;
    use point::vector;
    use point_light::PointLight;
    use ray::Ray;
    use shape::Shape;
    use std::sync::Arc;
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
            object: default_world.objects[0].clone(),
            t: 4.0,
        };
        let comps = i.precompute(&r, Vec::new());
        let c = default_world.shade_hit(comps, 10);

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
            object: world.objects[1].clone(),
            t: 0.5,
        };
        let comps = i.precompute(&r, Vec::new());
        let c = world.shade_hit(comps, 10);

        assert_eq!(c, Color::new(0.904984472, 0.904984472, 0.904984472));
    }

    #[test]
    fn test_world_color_at() {
        let world = World::new();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 1.0, 0.0),
        };

        assert_eq!(world.color_at(&r, 10), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_world_color_at_2() {
        let world = World::new();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };

        assert_eq!(world.color_at(&r, 10), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_world_in_shadow() {
        let world = World::new();
        let point = point(0.0, 10.0, 0.0);

        assert!(!world.is_shadowed(&point));
    }

    #[test]
    fn test_world_in_shadow_2() {
        let world = World::new();
        let point = point(10.0, -10.0, 10.0);

        assert!(world.is_shadowed(&point));
    }

    #[test]
    fn test_world_in_shadow_3() {
        let world = World::new();
        let point = point(-20.0, 20.0, -20.0);

        assert!(!world.is_shadowed(&point));
    }

    #[test]
    fn test_world_in_shadow_4() {
        let world = World::new();
        let point = point(-2.0, 2.0, -2.0);

        assert!(!world.is_shadowed(&point));
    }

    #[test]
    fn test_world_reflected_color_for_non_reflective_material() {
        let mut world = World::new();
        let ray = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        Arc::get_mut(&mut world.objects[1])
            .unwrap()
            .material
            .ambient = 1.0;
        let intersection = ray.intersect_world(&world)[0].clone();
        let comps = intersection.precompute(&ray, Vec::new());
        let color = world.reflected_color(&comps, 10);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn test_world_reflected_color_for_reflective_material() {
        let mut plane = Shape::plane();
        Arc::get_mut(&mut plane).unwrap().transform = Matrix4::translation(0.0, -1.0, 0.0);
        Arc::get_mut(&mut plane).unwrap().material.reflective = 0.5;
        let mut world = World::new();
        let sqrt_two_over_two = 2.0_f64.sqrt() / 2.0;
        world.objects.push(plane.clone());
        let ray = Ray {
            origin: point(0.0, 0.0, -3.0),
            direction: vector(0.0, -sqrt_two_over_two, sqrt_two_over_two),
        };
        let intersection = Intersection {
            object: plane,
            t: 2.0_f64.sqrt(),
        };
        let comps = intersection.precompute(&ray, Vec::new());
        let color = world.reflected_color(&comps, 10);
        assert_eq!(
            color,
            Color::new(
                0.19033232037953468,
                0.23791540047441834,
                0.14274924028465102
            )
        );
    }

    #[test]
    fn test_world_reflected_color_infinite_recursion() {
        let mut world = World::new();
        world.light_source = PointLight {
            position: point(0.0, 0.0, 0.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let mut lower = Shape::plane();
        Arc::get_mut(&mut lower).unwrap().material.reflective = 1.0;
        Arc::get_mut(&mut lower).unwrap().transform = Matrix4::translation(0.0, -1.0, 0.0);
        let mut upper = Shape::plane();
        Arc::get_mut(&mut upper).unwrap().material.reflective = 1.0;
        Arc::get_mut(&mut upper).unwrap().transform = Matrix4::translation(0.0, 1.0, 0.0);
        world.objects.push(lower);
        world.objects.push(upper);
        let ray = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 1.0, 0.0),
        };

        assert_eq!(world.color_at(&ray, 10), Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_refracted_color_of_opaque_object() {
        let w = World::new();
        let shape = w.objects[0].clone();
        let ray = Ray {
            origin: point(0., 0., -5.),
            direction: vector(0., 0., 1.),
        };
        let xs: Vec<Intersection> = vec![
            Intersection {
                t: 4.,
                object: shape.clone(),
            },
            Intersection {
                t: 6.,
                object: shape.clone(),
            },
        ];
        let comps = Intersection::precompute(&xs[0].clone(), &ray, xs);
        let color = w.refracted_color(&comps, 5);

        assert_eq!(color, Color::black());
    }

    #[test]
    fn test_refracted_color_of_maximum_depth() {
        let mut w = World::new();
        w.objects = vec![Arc::new(Shape {
            intersectable: Intersectable::sphere(),
            material: Material {
                ambient: 0.1,
                diffuse: 0.7,
                pattern: Patternable::solid(Color::new(0.8, 1.0, 0.6)),
                reflective: 0.,
                refractive_index: 1.5,
                shininess: 200.,
                specular: 0.2,
                transparency: 1.,
            },
            parent: None,
            transform: IDENTITY_MATRIX,
        })];
        let shape = w.objects[0].clone();
        let ray = Ray {
            origin: point(0., 0., -5.),
            direction: vector(0., 0., 1.),
        };
        let xs: Vec<Intersection> = vec![
            Intersection {
                t: 4.,
                object: shape.clone(),
            },
            Intersection {
                t: 6.,
                object: shape.clone(),
            },
        ];
        let comps = Intersection::precompute(&xs[0].clone(), &ray, xs);
        let color = w.refracted_color(&comps, 0);

        assert_eq!(color, Color::black());
    }

    #[test]
    fn test_refracted_color_total_internal_reflection() {
        let mut w = World::new();
        w.objects = vec![Arc::new(Shape {
            intersectable: Intersectable::sphere(),
            material: Material {
                ambient: 0.1,
                diffuse: 0.7,
                pattern: Patternable::solid(Color::new(0.8, 1.0, 0.6)),
                reflective: 0.,
                refractive_index: 1.5,
                shininess: 200.,
                specular: 0.2,
                transparency: 1.,
            },
            parent: None,
            transform: IDENTITY_MATRIX,
        })];
        let shape = w.objects[0].clone();
        let ray = Ray {
            origin: point(0., 0., (2.0_f64).sqrt() / 2.),
            direction: vector(0., 1., 0.),
        };
        let xs: Vec<Intersection> = vec![
            Intersection {
                t: -((2.0_f64).sqrt()) / 2.,
                object: shape.clone(),
            },
            Intersection {
                t: (2.0_f64).sqrt() / 2.,
                object: shape.clone(),
            },
        ];
        let comps = Intersection::precompute(&xs[1].clone(), &ray, xs);
        let color = w.refracted_color(&comps, 0);

        assert_eq!(color, Color::black());
    }

    // #[test]
    // fn test_refracted_color_with_refracted_ray() {
    //     let mut w = World::new();
    //     let a = Arc::new(Shape {
    //         intersectable: Intersectable::sphere(),
    //         material: Material {
    //             ambient: 1.0,
    //             diffuse: 0.7,
    //             pattern: Patternable::solid(Color::new(0.8, 1.0, 0.6)),
    //             reflective: 0.,
    //             refractive_index: 1.,
    //             shininess: 200.,
    //             specular: 0.2,
    //             transparency: 0.,
    //         },
    //         parent: None,
    //         transform: IDENTITY_MATRIX,
    //     });
    //     let b = Arc::new(Shape {
    //         intersectable: Intersectable::sphere(),
    //         material: Material {
    //             ambient: 0.1,
    //             diffuse: 0.7,
    //             pattern: Patternable::solid(Color::new(0.8, 1.0, 0.6)),
    //             reflective: 0.,
    //             refractive_index: 1.5,
    //             shininess: 200.,
    //             specular: 0.2,
    //             transparency: 1.,
    //         },
    //         parent: None,
    //         transform: Matrix4::scaling(0.5, 0.5, 0.5),
    //     });
    //     w.objects = vec![a, b];

    //     let r = Ray {
    //         direction: vector(0., 1., 0.),
    //         origin: point(0., 0., 0.1),
    //     };

    //     let xs: Vec<Intersection> = vec![Intersection {}];
    // }
}
