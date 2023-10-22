use crate::css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

/// Represents a map of CSS properties.
pub type PropertyMap = HashMap<String, Value>;

/// Represents a styled node in the DOM tree.
#[derive(PartialEq, Debug)]
pub struct StyledNode<'a> {
    /// The original node in the DOM tree.
    pub node: &'a Node,
    /// The specified CSS values for the node.
    pub specified_values: PropertyMap,
    /// Styled children nodes.
    pub children: Vec<StyledNode<'a>>,
}

/// Represents the display property of a styled node.
#[derive(PartialEq, Debug)]
pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    /// Gets the specified value for a given CSS property if it exists, otherwise `None`.
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    /// Looks up a CSS property value, falling back to a default if not present.
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }

    /// Gets the display property of the styled node. (defaults to inline).
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

/// Styles the entire DOM tree rooted at the given node based on the provided stylesheet.
/// This finds only the specified values at the moment. Eventually it should be extended to find the
/// computed values too, including inherited values.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new(),
            //TODO: finish this later
            _ => todo!(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}

/// Computes the specified CSS values for an element based on the given stylesheet.
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

/// Represents a matched rule with specificity.
type MatchedRule<'a> = (Specificity, &'a Rule);

/// Finds matching rules for an element in the stylesheet.
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

/// Matches an element against a rule in the stylesheet.
/// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

/// Checks if an element matches a simple selector.
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

/// Checks if an element matches a simple selector.
fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID selector
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check class selectors
    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }

    // We didn't find any non-matching selector components.
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::{Declaration, Unit};

    #[test]
    fn test_styled_node_value() {
        // Test case: StyledNode with specified values
        let node = Node {
            children: vec![],
            node_type: NodeType::Element(ElementData {
                tag_name: "div".to_string(),
                attributes: [("id".to_string(), "my-id".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            }),
        };

        let stylesheet = Stylesheet {
            rules: vec![Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("div".to_string()),
                    id: Some("my-id".to_string()),
                    class: vec![],
                })],
                declarations: vec![Declaration {
                    name: "color".to_string(),
                    value: Value::Keyword("red".to_string()),
                }],
            }],
        };

        let styled_node = style_tree(&node, &stylesheet);

        assert_eq!(
            styled_node.value("color"),
            Some(Value::Keyword("red".to_string())),
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
        let node = Node {
            children: vec![],
            node_type: NodeType::Element(ElementData {
                tag_name: "div".to_string(),
                attributes: [("id".to_string(), "my-id".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            }),
        };

        let stylesheet = Stylesheet {
            rules: vec![Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("div".to_string()),
                    id: Some("my-id".to_string()),
                    class: vec![],
                })],
                declarations: vec![Declaration {
                    name: "color".to_string(),
                    value: Value::Keyword("red".to_string()),
                }],
            }],
        };

        let styled_node = style_tree(&node, &stylesheet);

        assert_eq!(
            styled_node.lookup(
                "color",
                "background-color",
                &Value::Keyword("default".to_string())
            ),
            Value::Keyword("red".to_string()),
            "Color is specified, should return specified value"
        );

        assert_eq!(
            styled_node.lookup("font-size", "font-size", &Value::Length(12.0, Unit::Px)),
            Value::Length(12.0, Unit::Px),
            "Font size is not specified, should return default value"
        );
    }

    #[test]
    fn test_styled_node_display() {
        // Test case: StyledNode with specified display property
        let node = Node {
            children: vec![],
            node_type: NodeType::Element(ElementData {
                tag_name: "div".to_string(),
                attributes: [("id".to_string(), "my-id".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            }),
        };

        let stylesheet = Stylesheet {
            rules: vec![Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("div".to_string()),
                    id: Some("my-id".to_string()),
                    class: vec![],
                })],
                declarations: vec![Declaration {
                    name: "display".to_string(),
                    value: Value::Keyword("none".to_string()),
                }],
            }],
        };

        let styled_node = style_tree(&node, &stylesheet);

        assert_eq!(
            styled_node.display(),
            Display::None,
            "Display property is specified, should return specified value"
        );

        // Test case: StyledNode without specified display property
        let node_without_display = Node {
            children: vec![],
            node_type: NodeType::Element(ElementData {
                tag_name: "div".to_string(),
                attributes: [("id".to_string(), "my-id".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            }),
        };

        let stylesheet_without_display = Stylesheet { rules: vec![] };

        let styled_node_without_display =
            style_tree(&node_without_display, &stylesheet_without_display);

        assert_eq!(
            styled_node_without_display.display(),
            Display::Inline,
            "Display property is not specified, should return default value"
        );
    }
}
