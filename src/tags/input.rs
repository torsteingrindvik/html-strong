use crate::global_attributes::Attribute;

use super::Tag;

#[derive(Debug, Clone)]
enum InputType {
    Hidden,
    Submit,
    Text,
}

impl Attribute for InputType {
    fn name(&self) -> &'static str {
        "type"
    }

    fn value(&self) -> String {
        match self {
            InputType::Hidden => "hidden",
            InputType::Submit => "submit",
            InputType::Text => "text",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
struct Name(String);

impl Attribute for Name {
    fn name(&self) -> &'static str {
        "name"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
struct Value(String);

impl Attribute for Value {
    fn name(&self) -> &'static str {
        "value"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Input.
#[derive(Debug, Clone)]
pub struct Input {
    type_: InputType,
    name: Option<Name>,
    value: Value,
    // TODO: Missing stuff:
    //  * size
    //  * autocorrect
    //  * spellcheck
    //  * autocapitalize
    //  * autcomplete
}

impl Input {
    #[must_use]
    pub fn text(name: &str, value: &str) -> Self {
        Self {
            type_: InputType::Text,
            name: Some(Name(name.to_string())),
            value: Value(value.to_string()),
        }
    }

    #[must_use]
    pub fn hidden(name: &str, value: &str) -> Self {
        Self {
            type_: InputType::Hidden,
            name: Some(Name(name.to_string())),
            value: Value(value.to_string()),
        }
    }

    #[must_use]
    pub fn submit(value: &str) -> Self {
        Self {
            type_: InputType::Submit,
            name: None,
            value: Value(value.to_string()),
        }
    }
}

impl Tag for Input {
    fn name(&self) -> &'static str {
        "input"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        let mut attrs: Vec<&dyn Attribute> = vec![&self.type_];

        if let Some(name) = &self.name {
            attrs.push(name);
        }

        attrs.push(&self.value);

        Some(attrs)
    }

    fn close_tag(&self) -> bool {
        false
    }
}
