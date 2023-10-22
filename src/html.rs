//! A simple parser for a tiny subset of HTML.
//!
//! Can parse basic opening and closing tags, and text nodes.
//!
/// https://html.spec.whatwg.org/multipage/introduction.html#a-quick-introduction-to-html
///
use std::collections::HashMap;

use crate::dom;

/// A simple HTML parser that converts HTML strings into a DOM tree.
/// https://html.spec.whatwg.org/multipage/parsing.html#parsing
struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn new(input: String) -> Self {
        Parser { pos: 0, input }
    }

    /// Get the next character in the input string.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Check if the input string starts with a given substring.
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// Check if the parser has reached the end of the input.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Consume the current character and advance the position to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        cur_char
    }

    /// Consume characters while the given predicate function returns true.
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

    /// Consume whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Parse the tag name of an HTML element.
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    /// Parse the text content of an HTML node.
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    /// Parse the value of an HTML attribute.
    // Parse a quoted value.
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        value
    }

    /// Parse an HTML attribute.
    // Parse a single name="value" pair.
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    /// Parse a map of HTML attributes.
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    /// Parse an HTML node.
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => {
                if self.starts_with("<!--") {
                    self.parse_comment()
                } else {
                    self.parse_element()
                }
            }
            _ => self.parse_text(),
        }
    }

    /// Parse a vector of HTML nodes.
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// Parse a single HTML element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> dom::Node {
        // Opening tag.
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents.
        let children = self.parse_nodes();

        // Closing tag.
        if self.starts_with("</") {
            assert!(self.consume_char() == '<');
            assert!(self.consume_char() == '/');
            let closing_tag_name = self.parse_tag_name();
            assert!(self.consume_char() == '>');

            // Check if the closing tag matches the opening tag.
            if tag_name != closing_tag_name {
                eprintln!(
                    "Warning: Closing tag '{}' does not match opening tag '{}', at position {}",
                    closing_tag_name, tag_name, self.pos
                );
            }
        } else {
            eprintln!(
                "Warning: Missing closing tag for '{}',  at position {}",
                tag_name, self.pos
            );
        }

        dom::elem(tag_name, attrs, children)
    }

    fn parse_comment(&mut self) -> dom::Node {
        // Opening comment.
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '!');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');

        let mut comment = String::new();
        let mut consecutive_dashes = 0;

        // Data.

        loop {
            let current_char = self.consume_char();

            match current_char {
                '-' => consecutive_dashes += 1,
                '>' if consecutive_dashes >= 2 => break, // Closing tag.
                _ => {
                    // Append consecutive dashes if any.
                    comment.push_str(&"-".repeat(consecutive_dashes));
                    consecutive_dashes = 0;

                    comment.push(current_char);
                }
            }
        }

        dom::comment(comment)
    }
}

/// Parse the entire HTML document and return the root node of the DOM tree.
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser::new(source).parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text_node() {
        let html = "Hello, World!".to_string();
        let parsed_node = parse(html);
        assert_eq!(parsed_node, dom::text("Hello, World!".to_string()));
    }

    #[test]
    fn test_parse_element_node() {
        let html = "<div class=\"container\"><p>Hello</p></div>".to_string();
        let parsed_node = parse(html);

        let expected_node = dom::elem(
            "div".to_string(),
            {
                let mut attrs = HashMap::new();
                attrs.insert("class".to_string(), "container".to_string());
                attrs
            },
            vec![dom::elem(
                "p".to_string(),
                HashMap::new(),
                vec![dom::text("Hello".to_string())],
            )],
        );

        assert_eq!(parsed_node, expected_node);
    }

    #[test]
    fn test_parse_comment() {
        let html = "<body><h1>Hello, World!</h1> <!--This is a comment.--> <p>This is a paragraph.</p></body>".to_string();
        let parsed_node = parse(html);

        let expected_node = dom::elem(
            "body".to_string(),
            HashMap::new(),
            vec![
                dom::elem(
                    "h1".to_string(),
                    HashMap::new(),
                    vec![dom::text("Hello, World!".to_string())],
                ),
                dom::comment("This is a comment.".to_string()),
                dom::elem(
                    "p".to_string(),
                    HashMap::new(),
                    vec![dom::text("This is a paragraph.".to_string())],
                ),
            ],
        );

        assert_eq!(parsed_node, expected_node);
    }

    #[test]
    fn test_parse_invalid_node() {
        let html = "<div><p>Some text</p><p>Unclosed paragraph<p>should work</p></div>".to_string();
        let parsed_node = parse(html);

        let expected_node = dom::elem(
            "div".to_string(),
            HashMap::new(),
            vec![
                dom::elem(
                    "p".to_string(),
                    HashMap::new(),
                    vec![dom::text("Some text".to_string())],
                ),
                dom::elem(
                    "p".to_string(),
                    HashMap::new(),
                    vec![
                        dom::text("Unclosed paragraph".to_string()),
                        // in a real implementation they should not be a siblings,
                        dom::elem(
                            "p".to_string(),
                            HashMap::new(),
                            vec![dom::text("should work".to_string())],
                        ),
                    ],
                ),
                // should stay here the node <p>should work</p>
            ],
        );

        assert_eq!(parsed_node, expected_node);
    }
}
