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
