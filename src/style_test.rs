#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{cssom, dom, style};

    fn create_attrs() -> dom::AttrMap {
        [("id".to_string(), "my-id".to_string())]
            .iter()
            .cloned()
            .collect()
    }

    #[test]
    fn test_styled_node_value() {
        // Test case: StyledNode with specified values

        let node = dom::elem("div".to_string(), create_attrs(), vec![]);

        let stylesheet = cssom::stylesheet(vec![cssom::rule(
            vec![cssom::Selector::Simple(cssom::simple_selector(
                Some("div".to_string()),
                Some("my-id".to_string()),
                vec![],
            ))],
            vec![cssom::declaration(
                "color".to_string(),
                cssom::Value::Keyword("red".to_string()),
            )],
        )]);

        let styled_node = style::style_tree(&node, &stylesheet);

        assert_eq!(
            styled_node.value("color"),
            Some(cssom::Value::Keyword("red".to_string())),
            "Color is specified, should return Some(Value)"
        );

        assert_eq!(
            styled_node.value("background-color"),
            None,
            "Background-color is not specified, should return None"
        );
    }

    #[test]
    fn test_styled_node_lookup() {
        // Test case: StyledNode with specified values

        let node = dom::elem("div".to_string(), create_attrs(), vec![]);

        let stylesheet = cssom::stylesheet(vec![cssom::rule(
            vec![cssom::Selector::Simple(cssom::simple_selector(
                Some("div".to_string()),
                Some("my-id".to_string()),
                vec![],
            ))],
            vec![cssom::declaration(
                "color".to_string(),
                cssom::Value::Keyword("red".to_string()),
            )],
        )]);

        let styled_node = style::style_tree(&node, &stylesheet);

        assert_eq!(
            styled_node.lookup(
                "color",
                "background-color",
                &cssom::Value::Keyword("default".to_string())
            ),
            cssom::Value::Keyword("red".to_string()),
            "Color is specified, should return specified value"
        );

        assert_eq!(
            styled_node.lookup(
                "font-size",
                "font-size",
                &cssom::Value::Length(12.0, cssom::Unit::Px)
            ),
            cssom::Value::Length(12.0, cssom::Unit::Px),
            "Font size is not specified, should return default value"
        );
    }

    #[test]
    fn test_styled_node_display() {
        // Test case: StyledNode with specified display property
        let node = dom::elem("div".to_string(), create_attrs(), vec![]);

        let stylesheet = cssom::stylesheet(vec![cssom::rule(
            vec![cssom::Selector::Simple(cssom::simple_selector(
                Some("div".to_string()),
                Some("my-id".to_string()),
                vec![],
            ))],
            vec![cssom::declaration(
                "display".to_string(),
                cssom::Value::Keyword("none".to_string()),
            )],
        )]);

        let styled_node = style::style_tree(&node, &stylesheet);

        assert_eq!(
            styled_node.display(),
            style::Display::None,
            "Display property is specified, should return specified value"
        );

        // Test case: StyledNode without specified display property
        let node_without_display = dom::elem("div".to_string(), create_attrs(), vec![]);

        let stylesheet_without_display = cssom::stylesheet(vec![]);

        let styled_node_without_display =
            style::style_tree(&node_without_display, &stylesheet_without_display);

        assert_eq!(
            styled_node_without_display.display(),
            style::Display::Inline,
            "Display property is not specified, should return default value"
        );
    }

    #[test]
    fn style_tree_styles_element() {
        // Create a simple DOM tree with a div element
        let node = dom::elem("div".to_string(), HashMap::new(), vec![]);

        // Create a simple stylesheet with a rule
        let stylesheet = cssom::stylesheet(vec![cssom::rule(
            vec![cssom::Selector::Simple(cssom::simple_selector(
                Some("div".to_string()),
                None,
                vec![],
            ))],
            vec![cssom::declaration(
                "color".to_string(),
                cssom::Value::Keyword("red".to_string()),
            )],
        )]);

        // Apply styles to the DOM tree
        let styled_node = style::style_tree(&node, &stylesheet);

        // Check that the specified color value is present in the styled tree
        assert_eq!(
            styled_node.value("color"),
            Some(cssom::Value::Keyword("red".to_string()))
        );

        // Check that the styled tree has the correct node type and tag name
        assert_eq!(
            styled_node.node.node_type,
            dom::NodeType::Element(dom::ElementData {
                tag_name: "div".to_string(),
                attributes: HashMap::new()
            })
        );
    }

    #[test]
    fn style_tree_styles_text_node() {
        // Create a DOM tree with a text node inside a paragraph
        let node = dom::elem(
            "p".to_string(),
            HashMap::new(),
            vec![dom::text("Hello, world!".to_string())],
        );

        // Create a simple stylesheet with a rule
        let stylesheet = cssom::stylesheet(vec![cssom::rule(
            vec![cssom::Selector::Simple(cssom::simple_selector(
                Some("p".to_string()),
                None,
                vec![],
            ))],
            vec![cssom::declaration(
                "font-size".to_string(),
                cssom::Value::Length(12.0, cssom::Unit::Px),
            )],
        )]);

        // Apply styles to the DOM tree
        let styled_node = style::style_tree(&node, &stylesheet);

        // Check that the specified font-size value is present in the styled text node
        assert_eq!(
            styled_node.value("font-size"),
            Some(cssom::Value::Length(12.0, cssom::Unit::Px))
        );

        // Check that the styled text node has the correct content
        assert_eq!(
            styled_node.children.first().map(|child| {
                if let dom::NodeType::Text(ref text) = child.node.node_type {
                    text.clone()
                } else {
                    String::new()
                }
            }),
            Some("Hello, world!".to_string())
        );
    }

    #[test]
    fn style_tree_styles_comment_node() {
        // Create a DOM tree with a comment node inside a div
        let node = dom::elem(
            "div".to_string(),
            HashMap::new(),
            vec![dom::comment("This is a comment".to_string())],
        );

        let stylesheet = cssom::stylesheet(vec![cssom::rule(
            vec![cssom::Selector::Simple(cssom::simple_selector(
                Some("".to_string()),
                None,
                vec![],
            ))],
            vec![cssom::declaration(
                "color".to_string(),
                cssom::Value::Keyword("blue".to_string()),
            )],
        )]);

        // Apply styles to the DOM tree
        let styled_tree = style::style_tree(&node, &stylesheet);

        // Check that the specified color value is not present in the styled comment node
        assert_eq!(
            styled_tree
                .children
                .first()
                .and_then(|child| child.specified_values.get("color")),
            None
        );

        // Check that the styled comment node has the correct content
        assert_eq!(
            styled_tree.children.first().map(|child| {
                if let dom::NodeType::Comment(ref comment) = child.node.node_type {
                    comment.clone()
                } else {
                    String::new()
                }
            }),
            Some("This is a comment".to_string())
        );
    }
}
