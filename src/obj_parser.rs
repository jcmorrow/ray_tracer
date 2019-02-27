use point::{point, Point};
use shape::Shape;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum ObjNodeType {
    Decimal,
    Integer,
    MinusSign,
    ShapeStart,
    Slash,
    Space,
    VertexStart,
}

#[derive(Debug, Clone)]
pub struct ObjNode {
    value: char,
    node_type: ObjNodeType,
}

#[derive(Debug)]
pub struct ObjParser {
    pub group: Rc<RefCell<Shape>>,
    vertices: Vec<Point>,
}

struct ObjLineParser {
    line: Vec<ObjNode>,
}

impl ObjLineParser {
    fn run(&mut self, vertices: &[Point]) -> (Vec<Point>, Vec<Shape>) {
        let mut points: Vec<Point> = Vec::new();
        let mut shapes: Vec<Shape> = Vec::new();
        if self.peek(ObjNodeType::VertexStart) {
            if let Some(vector) = self.parse_point() {
                points.push(vector);
            }
        } else if self.peek(ObjNodeType::ShapeStart) {
            if let Some(shape) = self.parse_shape(vertices) {
                shapes.push(shape);
            }
        }
        (points, shapes)
    }

    fn parse_shape(&mut self, vertices: &[Point]) -> Option<Shape> {
        self.consume(ObjNodeType::ShapeStart)?;
        self.consume_whitespace();
        let ai = self.consume_integer()? as usize - 1;
        let a = vertices[ai];
        self.consume_whitespace();
        let bi = self.consume_integer()? as usize - 1;
        self.consume_whitespace();
        let b = vertices[bi];
        let ci = self.consume_integer()? as usize - 1;
        self.consume_whitespace();
        let c = vertices[ci];
        Some(Shape::triangle(a, b, c))
    }

    fn parse_point(&mut self) -> Option<Point> {
        let mut p = point(0., 0., 0.);
        self.consume(ObjNodeType::VertexStart)?;
        self.consume_whitespace();
        p.x = self.consume_float()?;
        self.consume_whitespace();
        p.y = self.consume_float()?;
        self.consume_whitespace();
        p.z = self.consume_float()?;

        Some(p)
    }

    fn consume_whitespace(&mut self) {
        while self.peek(ObjNodeType::Space) {
            self.consume(ObjNodeType::Space);
        }
    }

    fn consume_float(&mut self) -> Option<f64> {
        let mut negative = false;
        let mut decimal = false;
        let mut float = String::new();
        while let Some(_) = self.line.iter().next() {
            if self.peek(ObjNodeType::MinusSign) {
                self.consume(ObjNodeType::MinusSign);
                negative = true;
            }
            if self.peek(ObjNodeType::Integer) {
                float.push(self.consume(ObjNodeType::Integer).unwrap().value);
            } else if self.peek(ObjNodeType::Decimal) {
                if decimal {
                    panic!("Malformed float, two decimal points");
                } else {
                    decimal = true;
                }
                float.push(self.consume(ObjNodeType::Decimal).unwrap().value);
            } else {
                break;
            }
        }
        if !float.is_empty() {
            Some(ObjParser::float_from_string(float, negative))
        } else {
            None
        }
    }

    fn consume_integer(&mut self) -> Option<i32> {
        let mut int = String::new();
        while let Some(_) = self.line.iter().next() {
            if self.peek(ObjNodeType::Integer) {
                int.push(self.consume(ObjNodeType::Integer)?.value);
            } else if self.peek(ObjNodeType::Slash) {
                self.consume(ObjNodeType::Slash);
                self.consume_integer();
            } else {
                break;
            }
        }
        if !int.is_empty() {
            Some(ObjParser::integer_from_string(int))
        } else {
            None
        }
    }

    fn peek(&self, node_type: ObjNodeType) -> bool {
        if let Some(node) = self.line.iter().next() {
            node.node_type == node_type
        } else {
            false
        }
    }

    fn consume(&mut self, node_type: ObjNodeType) -> Option<ObjNode> {
        if self.line.len() == 0 {
            return None;
        }
        let cloned_line = self.line.clone();
        let (head, tail) = cloned_line.split_at(1);
        if head[0].node_type == node_type {
            self.line = tail.to_vec();
            Some(head[0].clone())
        } else {
            None
        }
    }
}

impl ObjParser {
    fn float_from_string(s: String, negative: bool) -> f64 {
        let num = s.parse::<f64>().ok().unwrap();
        if negative {
            num * -1.
        } else {
            num
        }
    }

    fn integer_from_string(s: String) -> i32 {
        s.parse::<i32>().ok().unwrap()
    }

    fn parse_line(&mut self, line: Vec<ObjNode>) {
        let output = ObjLineParser { line }.run(&self.vertices);
        self.vertices.extend(output.0);
        for shape in output.1 {
            Shape::add_shape(self.group.clone(), shape);
        }
    }

    pub fn parse(text: &str) -> Shape {
        let mut parsed_lines: Vec<Vec<ObjNode>> = Vec::new();
        let mut obj_parser = ObjParser {
            vertices: Vec::new(),
            group: Shape::group(),
        };
        for line in text.lines() {
            let mut parsed: Vec<ObjNode> = Vec::new();
            let mut chars = line.chars();
            let mut maybe_char = chars.next();
            while match maybe_char {
                Some(_) => true,
                None => false,
            } {
                let c = maybe_char.unwrap();
                if c == 'v' {
                    parsed.push(ObjNode {
                        node_type: ObjNodeType::VertexStart,
                        value: c,
                    });
                }
                if c == 'f' {
                    parsed.push(ObjNode {
                        node_type: ObjNodeType::ShapeStart,
                        value: c,
                    });
                }
                if c == ' ' {
                    parsed.push(ObjNode {
                        node_type: ObjNodeType::Space,
                        value: c,
                    });
                }
                if c == '0'
                    || c == '1'
                    || c == '2'
                    || c == '3'
                    || c == '4'
                    || c == '5'
                    || c == '6'
                    || c == '7'
                    || c == '8'
                    || c == '9'
                {
                    parsed.push(ObjNode {
                        node_type: ObjNodeType::Integer,
                        value: c,
                    });
                }
                if c == '.' {
                    parsed.push(ObjNode {
                        node_type: ObjNodeType::Decimal,
                        value: c,
                    });
                }
                if c == '-' {
                    parsed.push(ObjNode {
                        node_type: ObjNodeType::MinusSign,
                        value: c,
                    });
                }
                if c == '/' {
                    parsed.push(ObjNode {
                        node_type: ObjNodeType::Slash,
                        value: c,
                    });
                }

                maybe_char = chars.next();
            }
            parsed_lines.push(parsed);
        }
        for line in parsed_lines {
            obj_parser.parse_line(line);
        }
        let group = obj_parser.group.replace(Shape::cube());
        group
    }
}

#[cfg(test)]
mod tests {
    use obj_parser::*;

    #[test]
    fn test_ignoring_unrecognized_lines() {
        let str = "There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.
";
        assert_eq!(ObjParser::parse(&str).vertices.len(), 0);
    }

    #[test]
    fn test_parsing_vertex_data() {
        let str = "v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0
";
        let parser = ObjParser::parse(&str);

        assert_eq!(parser.vertices.len(), 4);
        assert_eq!(parser.vertices[0], point(-1., 1., 0.));
        assert_eq!(parser.vertices[1], point(-1., 0.5, 0.));
        assert_eq!(parser.vertices[2], point(1., 0., 0.));
        assert_eq!(parser.vertices[3], point(1., 1., 0.));
    }

    #[test]
    fn test_parsing_incomplete_data() {
        let str = "v -1 1";
        let parser = ObjParser::parse(&str);

        assert_eq!(parser.vertices.len(), 0);
    }

    #[test]
    fn test_parsing_float_from_string() {
        assert_eq!(ObjParser::float_from_string(String::from("1"), false), 1.);
        assert_eq!(ObjParser::float_from_string(String::from("2.0"), false), 2.);
        assert_eq!(
            ObjParser::float_from_string(String::from("25.0"), true),
            -25.
        );
        assert_eq!(
            ObjParser::float_from_string(String::from("0.5"), false),
            0.5
        );
    }
}
