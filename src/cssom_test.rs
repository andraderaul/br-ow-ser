#[cfg(test)]
mod tests {
    use crate::cssom::{
        declaration, rule, simple_selector, stylesheet, Selector, SimpleSelector, Unit, Value,
    };

    #[test]
    fn test_simple_selector_parsing() {
        // Test Case 1: Parsing a simple selector with a tag name.
        let selector = simple_selector(Some("div".to_string()), None, vec![]);
        assert_eq!(selector.tag_name, Some("div".to_string()));
        assert_eq!(selector.id, None);
        assert_eq!(selector.class, vec![] as Vec<String>);

        // Test Case 2: Parsing a simple selector with an ID and classes.
        let selector = simple_selector(
            None,
            Some("myId".to_string()),
            vec!["class1".to_string(), "class2".to_string()],
        );
        assert_eq!(selector.tag_name, None);
        assert_eq!(selector.id, Some("myId".to_string()));
        assert_eq!(
            selector.class,
            vec!["class1".to_string(), "class2".to_string()]
        );
    }

    #[test]
    fn test_rule_parsing() {
        // Test Case 1: Parsing a rule with a simple selector and a declaration.
        let rule = rule(
            vec![Selector::Simple(simple_selector(
                Some("div".to_string()),
                None,
                vec![],
            ))],
            vec![declaration(
                "color".to_string(),
                Value::Keyword("red".to_string()),
            )],
        );
        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(
            rule.selectors.first().unwrap(),
            &Selector::Simple(SimpleSelector {
                tag_name: Some("div".to_string()),
                id: None,
                class: vec![]
            })
        );
        assert_eq!(rule.declarations.len(), 1);
        assert_eq!(rule.declarations.first().unwrap().name, "color".to_string());
        assert_eq!(
            rule.declarations.first().unwrap().value,
            Value::Keyword("red".to_string())
        );
    }

    #[test]
    fn test_unit_conversions() {
        let value = Value::Length(20.0, Unit::Px);
        assert_eq!(value.to_px(), 20.0);
        assert_eq!(value.to_rem(), 1.25);
    }

    #[test]
    fn test_stylesheet_creation() {
        // Test Case 1: Creating a stylesheet with two rules.
        let stylesheet = stylesheet(vec![
            rule(
                vec![Selector::Simple(simple_selector(
                    Some("div".to_string()),
                    None,
                    vec![],
                ))],
                vec![declaration(
                    "color".to_string(),
                    Value::Keyword("red".to_string()),
                )],
            ),
            rule(
                vec![Selector::Simple(simple_selector(
                    Some("p".to_string()),
                    None,
                    vec![],
                ))],
                vec![declaration(
                    "font-size".to_string(),
                    Value::Length(16.0, Unit::Px),
                )],
            ),
        ]);

        assert_eq!(stylesheet.rules.len(), 2);
        assert_eq!(
            stylesheet.rules.first().unwrap().selectors.first().unwrap(),
            &Selector::Simple(SimpleSelector {
                tag_name: Some("div".to_string()),
                id: None,
                class: vec![]
            })
        );
        assert_eq!(
            stylesheet.rules.last().unwrap().selectors.first().unwrap(),
            &Selector::Simple(SimpleSelector {
                tag_name: Some("p".to_string()),
                id: None,
                class: vec![]
            })
        );
    }
}
