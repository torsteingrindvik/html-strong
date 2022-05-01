use crate::global_attributes::Attribute;
use dyn_clonable::clonable;
use std::fmt;

macro_rules! pub_mod_and_export {
    ( $( $pre:ident::$post:ident ),* ) => {
        $(
            pub mod $pre;

            pub use $pre::$post;
        )*
    };
}

macro_rules! crate_mod {
    ( $( $pre:ident::$post:ident ),* ) => {
        $(
            pub(crate) mod $pre;
        )*
    };
}

pub_mod_and_export![
    a::A,
    b::B,
    br::Br,
    body::Body,
    div::Div,
    em::Em,
    h1::H1,
    h2::H2,
    hr::Hr,
    li::Li,
    p::P,
    u::U,
    ul::Ul,
    doctype::Doctype,
    head::Head,
    html::Html,
    meta::Meta,
    link::Link,
    title::Title,
    tr::Tr,
    th::Th,
    td::Td,
    table::Table,
    img::Img,
    form::Form,
    input::Input,
    script::Script,
    span::Span,
    textarea::Textarea,
    label::Label,
    button::Button,
    i::I,
    blockquote::Blockquote,
    pre::Pre,
    code::Code,
    nav::Nav
];

crate_mod![invisible::Invisible, root::Root];

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
