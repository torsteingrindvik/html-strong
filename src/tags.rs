// use std::io;
// use crate::document_tree::Node;

// /// Hyperlink
// pub mod a;
// /// Body
// pub mod body;
// /// Div
// pub mod div;
// /// Document type.
// pub mod doctype;
// /// Em
// pub mod em;
// /// H1
// pub mod h1;
// /// Header section.
// pub mod head;
// /// HR
// pub mod hr;
// /// Html tag declaration.
// pub mod html;
// /// Li
// pub mod li;
// /// Link tag typically found in header.
// pub mod link;
// /// Meta tag typically found in header.
// pub mod meta;
// /// P
// pub mod p;
// /// Ul
// pub mod ul;
// /// Title
// pub mod title;
// /// Center
// pub mod center;
// /// Table row
// pub mod tr;

////////////////////////////////////////////////////////////////////////////////
/// Re-exports
// pub use a::A;
// pub use body::Body;
// pub use div::Div;
// pub use doctype::Doctype;
// pub use em::Em;
// pub use h1::H1;
// pub use head::Head;
// pub use hr::Hr;
// pub use html::Html;
// pub use li::Li;
// pub use link::Link;
// pub use meta::Meta;
// pub use p::P;
// pub use ul::Ul;
// pub use title::Title;
////////////////////////////////////////////////////////////////////////////////

/// The empty root node.
// pub(crate) mod root;

/// Tagless node.
// pub(crate) mod invisible;
use std::fmt;

use dyn_clonable::clonable;

////////////////////////////////////////////////////////////////////////////////
// We cannot create a blanket impl of Into<Node> for any struct that implements
// the Tag trait, so to avoid repetition we create a macro for doing it
// manually.
//
// Also we `pub use` it.
use crate::{document_tree::Node, global_attributes::Attribute};

macro_rules! decl_mod_impl_into_node_pub_use {
    ( $( $pre:ident::$post:ident ),* ) => {
        $(
            pub mod $pre;

            #[allow(clippy::from_over_into)]
            impl Into<Node> for $pre::$post {
                fn into(self) -> Node {
                    Node::new(Box::new(self))
                }
            }

            pub use $pre::$post;
        )*
    };
}

macro_rules! impl_into_node_and_export_priv {
    ( $( $pre:ident::$post:ident ),* ) => {
        $(
            pub(crate) mod $pre;

            #[allow(clippy::from_over_into)]
            impl Into<Node> for $pre::$post {
                fn into(self) -> Node {
                    Node::new(Box::new(self))
                }
            }
        )*
    };
}

decl_mod_impl_into_node_pub_use![
    a::A,
    b::B,
    br::Br,
    body::Body,
    div::Div,
    em::Em,
    h1::H1,
    hr::Hr,
    li::Li,
    p::P,
    ul::Ul,
    doctype::Doctype,
    head::Head,
    html::Html,
    meta::Meta,
    link::Link,
    title::Title,
    tr::Tr,
    td::Td,
    table::Table,
    img::Img,
    form::Form,
    input::Input,
    script::Script,
    span::Span
];

impl_into_node_and_export_priv![invisible::Invisible, root::Root];

////////////////////////////////////////////////////////////////////////////////

/// An HTML tag.
#[clonable]
pub trait Tag: Send + fmt::Debug + Clone {
    /// The tag's name.
    fn name(&self) -> &'static str;

    /// If false, do not add a <tag> when rendering.
    fn open_tag(&self) -> bool {
        true
    }

    // If given, renders these attributes after opening the tag.
    // fn attributes(&self) -> Option<Vec<Box<dyn Attribute>>> {
    //     None
    // }

    /// If given, renders these attributes after opening the tag.
    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        None
    }

    /// If given, render this towards the end of the open tag part.
    ///
    /// For example, instead of rendering the [`meta::Meta`] open tag as just
    /// <meta>, this function may return `name="author" content="John Doe"`.
    /// The opening tag will then in total be rendered as:
    ///     `<meta name="author" content="John Doe">`
    // fn open_tag_render(&self) -> Option<String> {
    //     None
    // }

    /// If false, do not add a </tag> when rendering.
    fn close_tag(&self) -> bool {
        true
    }

    // fn render_open(&self) -> String {
    //     format!("<{}>", self.name())
    // }

    // fn render_close(&self) -> String {
    //     format!("</{}>", self.name())
    // }

    // fn render_open2<W>(&self, writer: &mut W) -> Result<(), io::Error> {
    //     Ok(())
    // }

    // fn render_close2<W>(&self, writer: &mut W) -> Result<(), io::Error> {
    //     Ok(())
    // }

    // fn to_node(&self) -> Node
    // {
    //     Node::new(Box::new(self))
    // }
}

// impl<T: Tag> Into<Node> for T {
//     fn into(self) -> Node {
//         Node::new(Box::new(self))
//     }
// }
