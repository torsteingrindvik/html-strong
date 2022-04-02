use crate::global_attributes::Attribute;

use super::Tag;

/// Method used when submitting form.
#[derive(Debug, Clone)]
pub enum Method {
    Get,
    Post,
    // Dialog, // Todo
}

impl Attribute for Method {
    fn name(&self) -> &'static str {
        "method"
    }

    fn value(&self) -> String {
        match self {
            Method::Get => "get",
            Method::Post => "post",
        }
        .into()
    }
}

/// Url which processes the form.
#[derive(Debug, Clone)]
struct Action(String);

impl Attribute for Action {
    fn name(&self) -> &'static str {
        "action"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Data form.
#[derive(Debug, Clone)]
pub struct Form {
    method: Method,
    action: Action,
}

impl Form {
    #[must_use]
    pub fn new(method: Method, action_url: &str) -> Self {
        Self {
            method,
            action: Action(action_url.to_string()),
        }
    }
}

impl Tag for Form {
    fn name(&self) -> &'static str {
        "form"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        Some(vec![&self.method, &self.action])
    }
}
