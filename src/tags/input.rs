use crate::global_attributes::Attribute;

use super::Tag;

#[derive(Debug, Clone)]
enum InputType {
    Hidden,
    Submit,
    Text,
    Radio,
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
            InputType::Radio => "radio",
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

#[derive(Debug, Clone)]
struct Checked;

impl Attribute for Checked {
    fn name(&self) -> &'static str {
        "checked"
    }

    fn value(&self) -> String {
        "".into()
    }
}

/// Input.
#[derive(Debug, Clone)]
pub struct Input {
    type_: InputType,
    name: Option<Name>,
    checked: Option<Checked>,
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
            checked: None,
        }
    }

    #[must_use]
    pub fn hidden(name: &str, value: &str) -> Self {
        Self {
            type_: InputType::Hidden,
            name: Some(Name(name.to_string())),
            value: Value(value.to_string()),
            checked: None,
        }
    }

    #[must_use]
    pub fn submit(value: &str) -> Self {
        Self {
            type_: InputType::Submit,
            name: None,
            value: Value(value.to_string()),
            checked: None,
        }
    }

    /// The value of this radio input element,
    /// and the name of the group it belongs to.
    #[must_use]
    pub fn radio(value: &str, name: &str) -> Self {
        Self {
            type_: InputType::Radio,
            name: Some(Name(name.to_string())),
            value: Value(value.to_string()),
            checked: None,
        }
    }

    /// Make this input have the `checked` attribute.
    pub fn set_checked(&mut self) {
        self.checked = Some(Checked);
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

        if let Some(checked) = &self.checked {
            attrs.push(checked);
        }

        attrs.push(&self.value);

        Some(attrs)
    }

    fn close_tag(&self) -> bool {
        false
    }
}
