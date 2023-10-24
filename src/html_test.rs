#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::dom;
    use crate::html;

    fn create_attrs() -> dom::AttrMap {
        [("class".to_string(), "container".to_string())]
            .iter()
            .cloned()
            .collect()
    }

    #[test]
    fn test_parse_text_node() {
        let html = "Hello, World!".to_string();
        let parsed_node = html::parse(html);
        assert_eq!(parsed_node, dom::text("Hello, World!".to_string()));
    }

    #[test]
    fn test_parse_element_node() {
        let html = "<div class=\"container\"><p>Hello</p></div>".to_string();
        let parsed_node = html::parse(html);

        let expected_node = dom::elem(
            "div".to_string(),
            create_attrs(),
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
        let parsed_node = html::parse(html);

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
        let parsed_node = html::parse(html);

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
