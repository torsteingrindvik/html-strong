// use super::Attribute;

pub type Class = String;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Class(String);

// impl Class {
//     #[must_use]
//     pub fn new(value: &str) -> Self {
//         Self(value.into())
//     }
// }

// impl Attribute for Class {
//     fn name(&self) -> &'static str {
//         "class"
//     }

//     fn value(&self) -> String {
//         self.0.clone()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::{document_tree::o, tags::Div};
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn several_classes() {
//         let div = o(Div)
//             .add_text("Some div")
//             .add_class("class-1")
//             .add_class("class-2");

//         let expected = r#"<div class="class-1" class="class-2">Some div</div>"#;
//         let result = div.render_string().unwrap();

//         // TODO: We don't want this.
//         assert_eq!(expected, result);

//         // TODO: We can do this, but this is more of a "set class string" than "add class".
//         let div = o(Div).add_text("Some div").add_class("class-1 class-2");

//         let expected = r#"<div class="class-1 class-2">Some div</div>"#;
//         let result = div.render_string().unwrap();

//         assert_eq!(expected, result);
//     }
// }
