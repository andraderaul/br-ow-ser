//! A simple parser for a tiny subset of CSS.

use crate::cssom;
use crate::cssom::{Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Unit, Value};

/// Parses a CSS source string into a stylesheet.
pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser::new(source);

    cssom::stylesheet(parser.parse_rules())
}

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    // Create a new parser struct
    fn new(input: String) -> Self {
        Parser { pos: 0, input }
    }

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
        cssom::rule(self.parse_selectors(), self.parse_declarations())
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
        let mut selector = cssom::simple_selector(None, None, Vec::new());

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

        cssom::declaration(property_name, value)
    }

    /// Parses a CSS value.
    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            '"' => self.parse_string(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    /// Parses a CSS string value.
    fn parse_string(&mut self) -> Value {
        // Example: "example string"
        assert_eq!(self.consume_char(), '"');
        let value = self.consume_while(|c| c != '"');
        assert_eq!(self.consume_char(), '"');
        Value::StringValue(value)
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
            "rem" => Unit::Rem,
            "em" => Unit::Em,
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
