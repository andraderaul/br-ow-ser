#[cfg(test)]
mod tests {
    use crate::css;
    use crate::cssom;

    #[test]
    fn test_parse_stylesheet() {
        let source = "
            body { color: red; } 
            h1 { font-size: 20px; } 
            h2 { font-size: 10em; }"
            .to_string();
        let stylesheet = css::parse(source);

        assert_eq!(stylesheet.rules.len(), 3);
    }

    #[test]
    fn test_parse_simple_selector() {
        let source = "#my-id { font-size: 10px; }".to_string();
        let stylesheet = css::parse(source);

        assert_eq!(
            stylesheet.rules.first().unwrap().selectors.first().unwrap(),
            &cssom::Selector::Simple(cssom::SimpleSelector {
                tag_name: None,
                id: Some("my-id".to_string()),
                class: vec![]
            })
        );
    }

    #[test]
    fn test_parse_declaration() {
        let source = ".my-class { font-size: 16px; }".to_string();
        let stylesheet = css::parse(source);

        assert_eq!(
            stylesheet
                .rules
                .first()
                .unwrap()
                .declarations
                .first()
                .unwrap(),
            &cssom::Declaration {
                name: "font-size".to_string(),
                value: cssom::Value::Length(16.0, cssom::Unit::Px)
            }
        )
    }

    #[test]
    fn test_parse_px_value() {
        let source = ".my-class { font-size: 12px; }".to_string();
        let stylesheet = css::parse(source);

        assert_eq!(
            stylesheet
                .rules
                .first()
                .unwrap()
                .declarations
                .first()
                .unwrap()
                .value,
            cssom::Value::Length(12.0, cssom::Unit::Px)
        )
    }

    #[test]
    fn test_parse_rem_value() {
        let source = ".my-class { font-size: 12rem; }".to_string();
        let stylesheet = css::parse(source);

        assert_eq!(
            stylesheet
                .rules
                .first()
                .unwrap()
                .declarations
                .first()
                .unwrap()
                .value,
            cssom::Value::Length(12.0, cssom::Unit::Rem)
        );
    }

    #[test]
    fn test_parse_em_value() {
        let source = ".my-class { font-size: 12em; }".to_string();
        let stylesheet = css::parse(source);

        assert_eq!(
            stylesheet
                .rules
                .first()
                .unwrap()
                .declarations
                .first()
                .unwrap()
                .value,
            cssom::Value::Length(12.0, cssom::Unit::Em)
        );
    }

    #[test]
    fn test_parse_color() {
        let source = ".my-class { color: #ff6600; }".to_string();
        let stylesheet = css::parse(source);

        assert_eq!(
            stylesheet
                .rules
                .first()
                .unwrap()
                .declarations
                .first()
                .unwrap()
                .value,
            cssom::Value::ColorValue(cssom::Color {
                r: 255,
                g: 102,
                b: 0,
                a: 255
            })
        );
    }

    #[test]
    fn test_parse_string_value() {
        let source = "div { content: \"Hello, World!\"; }".to_string();
        let stylesheet = css::parse(source);

        assert_eq!(
            stylesheet
                .rules
                .first()
                .unwrap()
                .declarations
                .first()
                .unwrap()
                .value,
            cssom::Value::StringValue("Hello, World!".to_string())
        );
    }
}
