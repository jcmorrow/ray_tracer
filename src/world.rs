use color::Color;
use material::Material;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use point::point;
use point_light::PointLight;
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
}

#[cfg(test)]
mod tests {
    use color::Color;
    use point::point;
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
}
