#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::dom;

    fn create_attrs() -> dom::AttrMap {
        [("class".to_string(), "container".to_string())]
            .iter()
            .cloned()
            .collect()
    }

    fn create_attributes_with_id() -> dom::AttrMap {
        [("id".to_string(), "my-id".to_string())]
            .iter()
            .cloned()
            .collect()
    }

    fn create_attributes_with_class() -> dom::AttrMap {
        [("class".to_string(), "class1 class2".to_string())]
            .iter()
            .cloned()
            .collect()
    }

    #[test]
    fn test_text_node_creation() {
        let text_node = dom::text("Hello, World!".to_string());
        assert!(matches!(text_node.node_type, dom::NodeType::Text(_)));
    }

    #[test]
    fn test_element_node_creation() {
        let attrs = create_attrs();
        let element_node = dom::elem("div".to_string(), attrs, Vec::new());
        assert!(matches!(element_node.node_type, dom::NodeType::Element(_)));
    }

    #[test]
    fn test_element_node_attributes() {
        let attrs = create_attrs();
        let element_node = dom::elem("div".to_string(), attrs, Vec::new());

        if let dom::NodeType::Element(data) = element_node.node_type {
            assert_eq!(data.attributes.get("class"), Some(&"container".to_string()));
        } else {
            panic!("Expected an Element node.");
        }
    }

    #[test]
    fn test_element_data_id() {
        let attributes_with_id = create_attributes_with_id();
        let element_data_with_id = dom::ElementData {
            tag_name: "div".to_string(),
            attributes: attributes_with_id,
        };

        assert_eq!(element_data_with_id.id(), Some(&"my-id".to_string()));

        let element_data_without_id = dom::ElementData {
            tag_name: "div".to_string(),
            attributes: dom::AttrMap::new(),
        };

        assert_eq!(element_data_without_id.id(), None);
    }

    #[test]
    fn test_element_data_classes() {
        let attributes_with_class: dom::AttrMap = create_attributes_with_class();
        let element_data_with_class = dom::ElementData {
            tag_name: "div".to_string(),
            attributes: attributes_with_class,
        };

        let expected_classes: HashSet<&str> = ["class1", "class2"].iter().cloned().collect();
        assert_eq!(element_data_with_class.classes(), expected_classes);

        let element_data_without_class = dom::ElementData {
            tag_name: "div".to_string(),
            attributes: dom::AttrMap::new(),
        };

        assert!(element_data_without_class.classes().is_empty());
    }

    #[test]
    fn test_pretty_print() {
        let tree = dom::elem(
            "html".to_string(),
            dom::AttrMap::new(),
            vec![
                dom::elem(
                    "head".to_string(),
                    dom::AttrMap::new(),
                    vec![dom::elem(
                        "title".to_string(),
                        dom::AttrMap::new(),
                        vec![dom::text("Page Title".to_string())],
                    )],
                ),
                dom::elem(
                    "body".to_string(),
                    dom::AttrMap::new(),
                    vec![
                        dom::elem(
                            "h1".to_string(),
                            dom::AttrMap::new(),
                            vec![dom::text("Hello, World!".to_string())],
                        ),
                        dom::comment("This is a comment".to_string()),
                    ],
                ),
            ],
        );

        dom::pretty_print(&tree, 0);
    }
}
