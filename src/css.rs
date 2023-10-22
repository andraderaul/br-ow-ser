use serde::Serialize;

/// Represents a parsed stylesheet with rules.
#[derive(Debug, Serialize)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

/// Represents a CSS rule with selectors and declarations.
#[derive(Debug, Serialize)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

/// Represents a CSS selector.
#[derive(Debug, Serialize)]
pub enum Selector {
    Simple(SimpleSelector),
}

/// Represents a simple CSS selector with tag name, id, and class.
#[derive(Debug, Serialize)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

/// Represents a CSS declaration with a property name and value.
#[derive(Debug, Serialize)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

/// Represents a CSS value.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

/// Represents a CSS unit.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Unit {
    Px,
}

/// Represents a color in CSS.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Copy for Color {}

/// Represents the specificity of a CSS selector.
pub type Specificity = (usize, usize, usize);

impl Selector {
    /// Calculates the specificity of the selector.
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();

        (a, b, c)
    }
}

impl Value {
    /// Converts a CSS value to pixels.
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

/// Parses a CSS source string into a stylesheet.
pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {
        pos: 0,
        input: source,
    };

    Stylesheet {
        rules: parser.parse_rules(),
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    /// Parses a list of CSS rules.
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }

        rules
    }

    /// Parses a single CSS rule.
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    /// Parses a list of CSS selectors.
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }

        // Return selectors with highest specificity first, for use in matching.
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    /// Parses a simple CSS selector.
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }

        selector
    }

    /// Parses a list of CSS declarations.
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }

        declarations
    }

    /// Parses a single CSS declaration.
    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ';');

        Declaration {
            name: property_name,
            value: value,
        }
    }

    /// Parses a CSS value.
    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    /// Parses a CSS length value.
    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    /// Parses a floating-point number.
    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });
        s.parse().unwrap()
    }

    /// Parses a CSS unit.
    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit"),
        }
    }

    /// Parses a color in CSS.
    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255,
        })
    }

    /// Parses a pair of hexadecimal digits.
    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    /// Parses a CSS identifier.
    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    /// Consumes whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Consumes characters while a condition is met.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    /// Consumes a single character and advances the position.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        cur_char
    }

    /// Returns the next character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Checks if the end of the input is reached.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

/// Checks if a character is a valid identifier character in CSS.
fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stylesheet() {
        let source = "body { color: red; } h1 { font-size: 20px; }".to_string();
        let stylesheet = parse(source);

        assert_eq!(stylesheet.rules.len(), 2);
    }

    #[test]
    fn test_parse_simple_selector() {
        let source = "#my-id".to_string();
        let mut parser = Parser {
            pos: 0,
            input: source,
        };
        let selector = parser.parse_simple_selector();

        assert_eq!(selector.id, Some("my-id".to_string()));
    }

    #[test]
    fn test_parse_declaration() {
        let source = "font-size: 16px;".to_string();
        let mut parser = Parser {
            pos: 0,
            input: source,
        };
        let declaration = parser.parse_declaration();

        assert_eq!(declaration.name, "font-size".to_string());
        assert_eq!(declaration.value, Value::Length(16.0, Unit::Px));
    }

    #[test]
    fn test_parse_value() {
        let source = "12px".to_string();
        let mut parser = Parser {
            pos: 0,
            input: source,
        };
        let value = parser.parse_value();

        assert_eq!(value, Value::Length(12.0, Unit::Px));
    }

    #[test]
    fn test_parse_color() {
        let source = "#ff6600".to_string();
        let mut parser = Parser {
            pos: 0,
            input: source,
        };
        let value = parser.parse_color();

        assert_eq!(
            value,
            Value::ColorValue(Color {
                r: 255,
                g: 102,
                b: 0,
                a: 255
            })
        );
    }
}
