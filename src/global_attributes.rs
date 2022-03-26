macro_rules! decl_mod_pub_use {
    ( $( $pre:ident::$post:ident ),* ) => {
        $(
            pub mod $pre;
            pub use $pre::$post;
        )*
    };
}

decl_mod_pub_use![id::Id, class::Class, lang::Lang, style::Style, title::Title];

pub trait Attribute {
    /// The attribute's name.
    fn name(&self) -> &'static str;

    /// The attribute's value.
    fn value(&self) -> String;
}

impl Attribute for (&'static str, String) {
    fn name(&self) -> &'static str {
        self.0
    }

    fn value(&self) -> String {
        self.1.clone()
    }
}

impl Attribute for (&'static str, &str) {
    fn name(&self) -> &'static str {
        self.0
    }

    fn value(&self) -> String {
        self.1.to_string()
    }
}
