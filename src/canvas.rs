use color::Color;
use std::vec::Vec;

struct Canvas {
    height: i64,
    width: i64,
    pixels: Vec<Color>,
}

impl Canvas {
    fn empty(width: i64, height: i64) -> Canvas {
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for _i in 0..(width * height) {
            pixels.push(Color {
                blue: 0.0,
                green: 0.0,
                red: 0.0,
            });
        }
        return Canvas {
            width,
            height,
            pixels,
        };
    }

    fn write_pixel(&mut self, column: usize, row: usize, color: &Color) {
        println!("{}", row * self.width as usize + column);
        self.pixels[row * self.width as usize + column] = *color;
    }

    fn render_ppm(&self) -> String {
        return format!(
            "P3
{} {}
255
{}",
            self.width,
            self.height,
            self.pixels_to_ppm()
        );
    }

    fn pixels_to_ppm(&self) -> String {
        let mut rows: Vec<String> = Vec::new();
        for i in 0..self.height {
            let mut pixels: Vec<String> = Vec::new();
            for j in 0..self.width {
                pixels.push(self.pixels[(i * self.width + j) as usize].ppm());
            }
            rows.push(pixels.join(" "));
        }

        let mut string = rows.join("\n");
        string.push_str("\n");
        return string;
    }
}

#[cfg(test)]
mod tests {
    use canvas::Canvas;
    use color::Color;

    #[test]
    fn test_canvas() {
        let canvas = Canvas::empty(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 200);
    }

    #[test]
    fn test_write_pixel() {
        let mut canvas = Canvas::empty(10, 20);
        let red = Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        };

        canvas.write_pixel(2, 3, &red);

        assert!(canvas.pixels[32].equal(&red));
    }

    #[test]
    fn test_render_to_ppm() {
        let mut canvas = Canvas::empty(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, &c1);
        canvas.write_pixel(2, 1, &c2);
        canvas.write_pixel(4, 2, &c3);

        assert_eq!(
            canvas.render_ppm(),
            "P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"
        );
    }
}
