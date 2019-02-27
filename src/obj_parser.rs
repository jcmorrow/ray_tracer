use point::{point, Point};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum ObjNodeType {
    Decimal,
    Integer,
    MinusSign,
    Space,
    VectorStart,
}

#[derive(Debug, Clone)]
pub struct ObjNode {
    value: char,
    node_type: ObjNodeType,
}

#[derive(Debug)]
pub struct ObjParser {
    vertices: Vec<Point>,
}

struct ObjLineParser {
    line: Vec<ObjNode>,
}

impl ObjLineParser {
    fn run(&mut self) -> Option<Point> {
        let mut p = point(0., 0., 0.);
        match self.consume(ObjNodeType::VectorStart) {
            None => return None,
            _ => (),
        };
        self.consume_whitespace();
        match self.consume_float() {
            None => return None,
            Some(s) => {
                p.x = s;
            }
        };
        self.consume_whitespace();
        match self.consume_float() {
            None => return None,
            Some(s) => {
                p.y = s;
            }
        };
        self.consume_whitespace();
        match self.consume_float() {
            None => return None,
            Some(s) => {
                p.z = s;
            }
        };

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
        while match self.line.iter().next() {
            Some(_) => true,
            None => false,
        } {
            if self.peek(ObjNodeType::MinusSign) {
                self.consume(ObjNodeType::MinusSign);
                negative = true;
            }
            if self.peek(ObjNodeType::Integer) {
                float.push(self.consume(ObjNodeType::Integer).unwrap().value);
            } else if self.peek(ObjNodeType::Decimal) {
                if decimal == false {
                    decimal = true;
                } else {
                    panic!("Malformed float, two decimal points");
                }
                float.push(self.consume(ObjNodeType::Decimal).unwrap().value);
            } else {
                break;
            }
        }
        if float.len() > 0 {
            return Some(ObjParser::float_from_string(float, negative));
        } else {
            return None;
        }
    }

    fn peek(&self, node_type: ObjNodeType) -> bool {
        match self.line.iter().next() {
            Some(n) => return n.node_type == node_type,
            None => false,
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
        println!("{}", s);
        let mut decimal = false;
        let mut whole: Vec<i32> = Vec::new();
        let mut part: Vec<i32> = Vec::new();
        let mut total = 0.;
        let mut place = 1;
        let mut character_to_integer: HashMap<char, u8> = HashMap::new();
        for i in 0..10 {
            character_to_integer.entry((i + 48) as char).or_insert(i);
        }
        for c in s.chars() {
            if c == '.' {
                decimal = true;
            } else {
                if decimal {
                    println!("c: {}", c);
                    part.push(*character_to_integer.get(&c).unwrap() as i32);
                } else {
                    whole.push(*character_to_integer.get(&c).unwrap() as i32);
                }
            }
        }

        println!("whole {:?}", whole);
        println!("part {:?}", part);

        for n in whole.iter().rev() {
            total = total + *n as f64 * place as f64;
            place = place * 10;
        }
        place = 10;
        for n in part.iter() {
            println!("n {}, place {}", n, place);
            total = total + *n as f64 / place as f64;
            place = place * 10;
        }
        if negative {
            total * -1.
        } else {
            total
        }
    }

    fn parse_line(&mut self, line: Vec<ObjNode>) {
        let output = ObjLineParser { line }.run();
        match output {
            Some(v) => self.vertices.push(v),
            None => (),
        }
    }

    pub fn parse(text: &str) -> ObjParser {
        let mut parsed_lines: Vec<Vec<ObjNode>> = Vec::new();
        let mut obj_parser = ObjParser {
            vertices: Vec::new(),
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
                        node_type: ObjNodeType::VectorStart,
                        value: c,
                    });
                }
                if c == ' ' {
                    parsed.push(ObjNode {
                        node_type: ObjNodeType::Space,
                        value: c,
                    });
                }
                if (c == '0'
                    || c == '1'
                    || c == '2'
                    || c == '5'
                    || c == '6'
                    || c == '7'
                    || c == '8'
                    || c == '9')
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

                maybe_char = chars.next();
            }
            parsed_lines.push(parsed);
        }
        for line in parsed_lines {
            obj_parser.parse_line(line);
        }
        obj_parser
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

    // #[test]
    // fn test_parsing_vertex_data() {
    //     let str = "v -1 1 0
    // v 1 0 0
    // v 1 0 0
    // v 1 1 0
    // v 0 2 0
    // f 1 2 3 4 5";
    //     let parser = ObjParser::parse(&str);

    //     assert_eq!(parser.vertices.len(), 4);
    //     assert_eq!(parser.vertices[0], point(-1., 1., 0.));
    //     assert_eq!(parser.vertices[1], point(-1., 0.5, 0.));
    //     assert_eq!(parser.vertices[2], point(1., 0., 0.));
    //     assert_eq!(parser.vertices[3], point(1., 1., 0.));
    // }
}
