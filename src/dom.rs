//! Basic DOM data structures.
// https://dom.spec.whatwg.org/

use std::collections::{HashMap, HashSet};

/// A mapping of attribute names to their values.
pub type AttrMap = HashMap<String, String>;

/// Represents a node in the DOM (Document Object Model).
#[derive(Debug, PartialEq)]
pub struct Node {
    /// Children nodes of this node.
    pub children: Vec<Node>,
    /// Type of the node, either text or element.
    pub node_type: NodeType,
}

/// Enum representing the type of a Node.
/// https://dom.spec.whatwg.org/#dom-node-nodetype
#[derive(Debug, PartialEq)]
pub enum NodeType {
    /// https://dom.spec.whatwg.org/#interface-text
    Text(String),

    /// https://dom.spec.whatwg.org/#interface-element
    Element(ElementData),

    /// https://dom.spec.whatwg.org/#interface-processinginstruction
    ProcessingInstruction(ProcessingInstructionData),

    /// https://dom.spec.whatwg.org/#interface-comment
    Comment(String),
}

/// Struct representing the data of an Element node.
#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

/// Struct representing the data of a Processing Instruction node.
#[derive(Debug, PartialEq)]
pub struct ProcessingInstructionData {
    pub target: String,
    pub data: String,
}

/// Creates a text node with the given data.
pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

/// Creates an element node with the given tag name, attributes, and children nodes.
pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

/// Creates a processing instruction node with the given target and data
pub fn processing_instruction(target: String, data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::ProcessingInstruction(ProcessingInstructionData {
            target: target,
            data: data,
        }),
    }
}

pub fn comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data),
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

// Pretty-print a tree of DOM nodes
pub fn pretty_print(node: &Node, indent: usize) {
    match &node.node_type {
        NodeType::Text(data) => {
            println!("{}Text: {}", " ".repeat(indent), data);
        }

        NodeType::Element(element_data) => {
            println!(
                "{}Element: <{}>",
                "  ".repeat(indent),
                element_data.tag_name
            );
            for (attr, value) in &element_data.attributes {
                println!("{}  {}=\"{}\"", "  ".repeat(indent), attr, value);
            }
        }

        NodeType::Comment(data) => {
            println!("{}Comment: <!-- {} -->", "  ".repeat(indent), data);
        }

        NodeType::ProcessingInstruction(processing_instruction) => {
            println!(
                "{}Data: {}",
                " ".repeat(indent),
                processing_instruction.data,
            );
            println!(
                "{}Target: {}",
                " ".repeat(indent),
                processing_instruction.target,
            );
        }
    }

    for child in &node.children {
        pretty_print(child, indent + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_node_creation() {
        let text_node = text("Hello, World!".to_string());
        assert!(matches!(text_node.node_type, NodeType::Text(_)));
    }

    #[test]
    fn test_element_node_creation() {
        let attrs = [("class".to_string(), "container".to_string())]
            .iter()
            .cloned()
            .collect();
        let element_node = elem("div".to_string(), attrs, Vec::new());
        assert!(matches!(element_node.node_type, NodeType::Element(_)));
    }

    #[test]
    fn test_element_node_attributes() {
        let attrs = [("class".to_string(), "container".to_string())]
            .iter()
            .cloned()
            .collect();
        let element_node = elem("div".to_string(), attrs, Vec::new());

        if let NodeType::Element(data) = element_node.node_type {
            assert_eq!(data.attributes.get("class"), Some(&"container".to_string()));
        } else {
            panic!("Expected an Element node.");
        }
    }

    #[test]
    fn test_element_data_id() {
        let attributes_with_id: AttrMap = [("id".to_string(), "my-id".to_string())]
            .iter()
            .cloned()
            .collect();
        let element_data_with_id = ElementData {
            tag_name: "div".to_string(),
            attributes: attributes_with_id,
        };

        assert_eq!(element_data_with_id.id(), Some(&"my-id".to_string()));

        let element_data_without_id = ElementData {
            tag_name: "div".to_string(),
            attributes: AttrMap::new(),
        };

        assert_eq!(element_data_without_id.id(), None);
    }

    #[test]
    fn test_element_data_classes() {
        let attributes_with_class: AttrMap = [("class".to_string(), "class1 class2".to_string())]
            .iter()
            .cloned()
            .collect();
        let element_data_with_class = ElementData {
            tag_name: "div".to_string(),
            attributes: attributes_with_class,
        };

        let expected_classes: HashSet<&str> = ["class1", "class2"].iter().cloned().collect();
        assert_eq!(element_data_with_class.classes(), expected_classes);

        let element_data_without_class = ElementData {
            tag_name: "div".to_string(),
            attributes: AttrMap::new(),
        };

        assert!(element_data_without_class.classes().is_empty());
    }

    #[test]
    fn test_pretty_print() {
        let tree = elem(
            "html".to_string(),
            AttrMap::new(),
            vec![
                elem(
                    "head".to_string(),
                    AttrMap::new(),
                    vec![elem(
                        "title".to_string(),
                        AttrMap::new(),
                        vec![text("Page Title".to_string())],
                    )],
                ),
                elem(
                    "body".to_string(),
                    AttrMap::new(),
                    vec![
                        elem(
                            "h1".to_string(),
                            AttrMap::new(),
                            vec![text("Hello, World!".to_string())],
                        ),
                        comment("This is a comment".to_string()),
                    ],
                ),
            ],
        );

        pretty_print(&tree, 0);
    }
}
