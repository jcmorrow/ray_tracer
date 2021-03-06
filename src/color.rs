use utilities::clamp;
use utilities::equal;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub blue: f64,
    pub green: f64,
    pub red: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        return Color { red, green, blue };
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn add(&self, other: &Color) -> Color {
        Color {
            blue: self.blue + other.blue,
            green: self.green + other.green,
            red: self.red + other.red,
        }
    }

    pub fn sub(&self, other: &Color) -> Color {
        Color {
            blue: self.blue - other.blue,
            green: self.green - other.green,
            red: self.red - other.red,
        }
    }

    pub fn hadamard_product(&self, other: &Color) -> Color {
        return Color {
            blue: self.blue * other.blue,
            green: self.green * other.green,
            red: self.red * other.red,
        };
    }

    pub fn multiply_scalar(&self, factor: f64) -> Color {
        Color {
            blue: self.blue * factor,
            green: self.green * factor,
            red: self.red * factor,
        }
    }

    pub fn divide(&self, factor: f64) -> Color {
        Color {
            blue: self.blue / factor,
            green: self.green / factor,
            red: self.red / factor,
        }
    }

    pub fn ppm(&self) -> String {
        return format!(
            "{} {} {}",
            (clamp(self.red, 0.0, 1.0) * 255.0).round(),
            (clamp(self.green, 0.0, 1.0) * 255.0).round(),
            (clamp(self.blue, 0.0, 1.0) * 255.0).round()
        );
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        return equal(self.red, other.red)
            && equal(self.green, other.green)
            && equal(self.blue, other.blue);
    }
}

#[cfg(test)]
mod tests {
    use color::Color;

    #[test]
    fn test_color() {
        let a = Color {
            red: -0.5,
            green: 0.4,
            blue: 1.7,
        };

        assert_eq!(a.red, -0.5);
        assert_eq!(a.green, 0.4);
        assert_eq!(a.blue, 1.7);
    }

    #[test]
    fn test_add_color() {
        let a = Color {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };
        let b = Color {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };

        assert_eq!(
            a.add(&b),
            Color {
                red: 1.6,
                green: 0.7,
                blue: 1.0
            }
        );
    }

    #[test]
    fn test_multiply_color() {
        let a = Color {
            red: 1.0,
            green: 0.2,
            blue: 0.4,
        };
        let b = Color {
            red: 0.9,
            green: 1.0,
            blue: 0.1,
        };

        assert_eq!(
            a.hadamard_product(&b),
            Color {
                red: 0.9,
                green: 0.2,
                blue: 0.04,
            }
        )
    }
}
