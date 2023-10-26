/// Represents a parsed stylesheet with rules.
#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

/// Represents a CSS rule with selectors and declarations.
#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

/// Represents a CSS selector.
#[derive(Debug, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
}

/// Represents a simple CSS selector with tag name, id, and class.
#[derive(Debug, PartialEq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

/// Represents a CSS declaration with a property name and value.
#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

/// Represents a CSS value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    StringValue(String),
}

/// Represents a CSS unit.
#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
    Rem,
    Em,
}

/// Represents a color in CSS.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Copy for Color {}

/// Represents the specificity of a CSS selector.
pub type Specificity = (usize, usize, usize);

impl Selector {
    /// Calculates the specificity of the selector.
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();

        (a, b, c)
    }
}

impl Value {
    // Needs improvement EM and REM
    // because the length is relative to a font-size

    /// Converts a CSS value to pixels.
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            Value::Length(f, Unit::Rem) => f * 16.0,
            Value::Length(f, Unit::Em) => f * 16.0,
            _ => 0.0,
        }
    }

    /// Converts a CSS value to rems.
    pub fn to_rem(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Rem) => f,
            Value::Length(f, Unit::Em) => f,
            Value::Length(f, Unit::Px) => f / 16.0,
            _ => 0.0,
        }
    }

    /// Converts a CSS value to ems.
    pub fn to_em(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Em) => f,
            Value::Length(f, Unit::Rem) => f,
            Value::Length(f, Unit::Px) => f / 16.0,
            _ => 0.0,
        }
    }
}

/// Creates a Stylesheet with the given rules.
pub fn stylesheet(rules: Vec<Rule>) -> Stylesheet {
    Stylesheet { rules }
}

/// Creates a Rule with the given selectors and declarations.
pub fn rule(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
    Rule {
        selectors,
        declarations,
    }
}

/// Creates a SimpleSelector with the specified components.
pub fn simple_selector(
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
) -> SimpleSelector {
    SimpleSelector {
        tag_name,
        id,
        class,
    }
}

/// Creates a Declaration with the specified name and value.
pub fn declaration(name: String, value: Value) -> Declaration {
    Declaration { name, value }
}
