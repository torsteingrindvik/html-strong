// #![deny(missing_docs)]
//! No macros, type-safe HTML tags.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

/// See [reference](https://www.w3schools.com/TAgs/default.asp).
pub mod tags;

/// See [reference](https://www.w3schools.com/TAgs/ref_standardattributes.asp).
pub mod global_attributes;

/// Attributes.
pub mod attributes;

/// The tree.
pub mod document_tree;

/// HTML document templates.
pub mod template;
