use std::{collections::HashSet, io};

use crate::{
    global_attributes::{Attribute, Class, Id, Style, Title},
    science_lab::NodeExt,
    tags::{invisible::Invisible, root::Root, Tag},
};

#[derive(Debug, Clone)]
pub struct Node {
    global_attributes: Vec<Box<dyn Attribute>>,
    classes: HashSet<Class>,
    tag: Box<dyn Tag>,
    text: Option<String>,
    children: Vec<Node>,
}

fn node(tag: impl Tag + 'static) -> Node {
    Node::new2(tag)
}

// Shorter version of [node].
pub fn o(tag: impl Tag + 'static) -> Node {
    node(tag)
}

impl Node {
    #[must_use]
    pub fn root() -> Self {
        Self::new(Box::new(Root))
    }

    #[must_use]
    pub fn new(tag: Box<dyn Tag>) -> Self {
        Self {
            tag,
            children: vec![],
            text: None,
            global_attributes: vec![],
            classes: HashSet::new(),
        }
    }

    pub fn new2(tag: impl Tag + 'static) -> Self {
        Self::new(Box::new(tag))
    }

    /// Render HTML to the given [`String`].
    ///
    /// # Errors
    ///
    /// See [`render_writer`].
    pub fn render_string(&self) -> Result<String, io::Error> {
        let mut buf = vec![];

        self.render_writer(&mut buf)?;

        Ok(String::from_utf8(buf).expect("Cannot yield invalid utf-8"))
    }

    /// Render HTML to the given writer.
    ///
    /// Recurses through child nodes and writes as we traverse,
    /// closing opened tags on the way back out.
    ///
    /// # Errors
    ///
    /// All errors are related to io, see [`io::Error`].
    pub fn render_writer<W>(&self, writer: &mut W) -> Result<(), io::Error>
    where
        W: io::Write,
    {
        if self.tag.open_tag() {
            write!(writer, "<{}", self.tag.name())?;

            if !self.classes.is_empty() {
                let all_classes = self
                    .classes
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .as_slice()
                    .join(" ");

                write!(writer, " class=\"{all_classes}\"")?;
            }

            let mut render_attr =
                |attr: &dyn Attribute| write!(writer, " {}=\"{}\"", attr.name(), attr.value());

            for global_attr in &self.global_attributes {
                render_attr(global_attr.as_ref())?;
                // write!(
                //     writer,
                //     " {}=\"{}\"",
                //     global_attr.name(),
                //     global_attr.value()
                // )?;
            }

            if let Some(attrs) = self.tag.attributes() {
                for attr in attrs {
                    render_attr(attr)?;
                }
            }

            write!(writer, ">")?;
        }

        if let Some(text) = self.text() {
            write!(writer, "{}", text)?;
        }

        for child in &self.children {
            child.render_writer(writer)?;
        }

        if self.tag.close_tag() {
            write!(writer, "</{}>", self.tag.name())?;
        }

        Ok(())
    }

    // pub fn render_pretty<W>(&self, writer: W)
    // where
    //     W: io::Write,
    // {
    // }

    /// Create an iterator over this (sub)tree.
    // pub fn iter(&self) -> NodeIter {
    //     NodeIter {
    //         children: &self.children,
    //         parent: None,
    //     }
    // }

    #[must_use]
    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    #[must_use]
    pub fn child2(self, tag: Box<dyn Tag>) -> Self {
        self.child(Self::new(tag))
    }

    #[must_use]
    pub fn child3(self, tag: impl Tag + 'static) -> Self {
        self.child(Self::new(Box::new(tag)))
    }

    // This allows us to pass a Node, but not an impl Tag + 'static.
    // Update: After adding a macro which implements Into, we can!
    #[must_use]
    pub fn child4<C>(self, child: C) -> Self
    where
        C: Into<Self>,
    {
        self.child(child.into())
    }

    // Shorter version of child4.
    // #[must_use]
    // pub fn kid<K>(self, kid: K) -> Self
    // where
    //     K: Into<Self>,
    // {
    //     self.child4(kid)
    // }

    #[must_use]
    pub fn kid<K>(self, kid: K) -> Self
    where
        K: NodeExt,
    {
        self.child4(kid.into_node())
    }

    pub fn push_kid<K>(&mut self, kid: K)
    where
        K: NodeExt,
    {
        self.children.push(kid.into_node());
    }

    #[must_use]
    pub fn children(mut self, children: Vec<Self>) -> Self {
        self.children.extend(children);
        self
    }

    #[must_use]
    pub fn add_attr(mut self, attribute: impl Attribute + 'static) -> Self {
        self.global_attributes.push(Box::new(attribute));
        self
    }

    #[must_use]
    pub fn add_style(mut self, style: &str) -> Self {
        self.global_attributes.push(Box::new(Style::new(style)));
        self
    }

    #[must_use]
    pub fn add_class(mut self, class: &str) -> Self {
        for class in class.split_ascii_whitespace() {
            self.classes.insert(class.to_string());
        }
        self
    }

    /// Set the node's id.
    #[must_use]
    pub fn set_id(mut self, id: &str) -> Self {
        self.global_attributes.push(Box::new(Id::new(id)));
        self
    }

    /// Set the node's title.
    #[must_use]
    pub fn set_title(mut self, title: &str) -> Self {
        self.global_attributes.push(Box::new(Title::new(title)));
        self
    }

    /// Get a reference to the node's id.
    // pub fn id(&self) -> Option<&String> {
    //     self.id.as_ref()
    // }

    /// Set the node's text.
    fn set_text(mut self, text: &str) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Get a reference to the node's text.
    #[must_use]
    const fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    #[must_use]
    pub fn add_text(self, text: &str) -> Self {
        let node: Self = Invisible.into_node();
        self.kid(node.set_text(text))
    }
}

#[cfg(test)]
mod tests {
    // http://web.simmons.edu/~grabiner/comm244/weekfour/document-tree.html
    //
    // Correct iteration:
    //
    // * Body Open
    //  * Div Open
    //      * H1 Open
    //      * H1 Close
    //      * P Open
    //      * P Close
    //      * P Open
    //          * Em Open
    //          * Em Close
    //      * P Close
    //      * Hr Open
    //      * (Hr Open)
    //  * Div Close
    //  * Div Open
    //      * Ul Open
    //          * Li Open
    //          * Li Close
    //          * Li Open
    //          * Li Close
    //          * Li Open
    //          * Li Close
    //      * Ul Close
    //  * Div Close

    use super::*;
    use crate::{
        global_attributes::{Id, Lang},
        tags::{body::Body, div::Div, em::Em, h1::H1, hr::Hr, li::Li, p::P, ul::Ul, Html},
    };
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    fn render_to_file(node: &Node, name: &str) {
        let output_file = PathBuf::from(format!(
            "{}/out/{}.html",
            std::env!("CARGO_MANIFEST_DIR"),
            name
        ));
        node.render_writer(&mut std::fs::File::create(output_file).unwrap())
            .unwrap();
    }

    #[test]
    fn build_tree_v1() {
        let tree = Node::root().child(Node::new(Box::new(Body)).children(vec![
            Node::new(Box::new(Div)).children(vec![
                Node::new(Box::new(H1)),
                Node::new(Box::new(P)),
                Node::new(Box::new(P)).child(Node::new(Box::new(Em))),
                Node::new(Box::new(Hr)),
            ]),
            Node::new(Box::new(Div)).child(Node::new(Box::new(Ul)).children(vec![
                Node::new(Box::new(Li)),
                Node::new(Box::new(Li)),
                Node::new(Box::new(Li)),
            ])),
        ]));

        render_to_file(&tree, "build_tree_v1");
    }

    #[test]
    fn build_tree_v2() {
        let tree = Node::root().child(Node::new2(Body).children(vec![
            Node::new2(Div).children(vec![
                Node::new2(H1),
                Node::new2(P),
                Node::new2(P).child2(Box::new(Em)),
                Node::new2(Hr),
            ]),
            Node::new2(Div).child(Node::new2(Ul).children(vec![
                Node::new2(Li),
                Node::new2(Li),
                Node::new2(Li),
            ])),
        ]));

        render_to_file(&tree, "build_tree_v2");
    }

    #[test]
    fn build_tree_v3() {
        let tree = Node::root().child(node(Body).children(vec![
            node(Div).children(vec![node(H1), node(P), node(P).child3(Em), node(Hr)]),
            node(Div).child(node(Ul).children(vec![node(Li), node(Li), node(Li)])),
        ]));

        render_to_file(&tree, "build_tree_v3");
    }

    #[test]
    fn build_tree_v4() {
        let tree = node(Body).children(vec![
            node(Div).children(vec![node(H1), node(P), node(P).child3(Em), node(Hr)]),
            node(Div).child(node(Ul).children(vec![node(Li), node(Li), node(Li)])),
        ]);

        render_to_file(&tree, "build_tree_v4");
    }

    #[test]
    fn build_tree_v5() {
        let tree = node(Body)
            .child(
                node(Div)
                    .child(node(H1))
                    .child(node(P))
                    .child(node(P).child(node(Em)))
                    .child(node(Hr)),
            )
            .child(
                node(Div).child(
                    node(Ul)
                        .child(node(Li))
                        .child(node(Li))
                        .child(node(Li))
                        .child(node(Li)),
                ),
            );

        render_to_file(&tree, "build_tree_v5");
    }

    #[test]
    fn build_tree_v6() {
        // Problem: Can't use same function for adding types which impl Tag
        // and the Node type.

        let tree = node(Body)
            .child(
                node(Div)
                    .child3(H1)
                    .child3(P)
                    .child(node(P).child3(Em))
                    .child3(Hr),
            )
            .child(
                node(Div).child(
                    node(Ul)
                        .child3(Li)
                        .child3(Li)
                        .child3(Li)
                        .child3(Li)
                        .child3(Li),
                ),
            );

        render_to_file(&tree, "build_tree_v6");
    }

    #[test]
    fn build_tree_v8() {
        // Nothing new, just shorted names.
        // Using `o` for `object`. This feels familiar, has some other crate done that?
        let tree = o(Body)
            .kid(o(Div).kid(H1).kid(P).kid(o(P).kid(Em)).kid(Hr))
            .kid(o(Div).kid(o(Ul).kid(Li).kid(Li).kid(Li).kid(Li).kid(Li)));

        render_to_file(&tree, "build_tree_v8");
    }

    #[test]
    fn build_tree_v9() {
        // Try adding some `id`s.
        let tree = o(Body)
            .kid(
                o(Div)
                    .add_attr(Id::new("content"))
                    .kid(H1)
                    .kid(P)
                    .kid(o(P).kid(Em))
                    .kid(Hr),
            )
            .kid(
                o(Div)
                    .add_attr(Id::new("nav"))
                    .kid(o(Ul).kid(Li).kid(Li).kid(Li).kid(Li).kid(Li)),
            );

        render_to_file(&tree, "build_tree_v9");
    }

    #[test]
    fn build_tree_v10() {
        // Try adding some text.
        let tree = o(Body)
            .kid(
                o(Div)
                    .add_attr(Id::new("content"))
                    .kid(o(H1).add_text("Heading here"))
                    .kid(o(P).add_text("Lorem ipsum dolor sit amet."))
                    .kid(
                        o(P).add_text("Lorem ipsum dolor ")
                            .kid(o(Em).add_text("sit"))
                            .add_text(" amet."),
                    )
                    .kid(Hr),
            )
            .kid(
                o(Div).add_attr(Id::new("nav")).kid(
                    o(Ul)
                        .kid(o(Li).add_text("item 1"))
                        .kid(o(Li).add_text("item 2"))
                        .kid(o(Li).add_text("item 3"))
                        .kid(o(Li).add_text("item 4"))
                        .kid(o(Li).add_text("item 5")),
                ),
            );

        render_to_file(&tree, "build_tree_v10");
    }

    #[test]
    fn build_list_v1() {
        let mut list = o(Ul);

        for index in 0..10 {
            let contents = format!("Hi I am {index}- ");

            list.push_kid(o(Li).add_text(&contents).kid(o(Em).add_text("emphasis!")));
        }

        render_to_file(&list, "build_list_v1");
    }

    #[test]
    fn add_attribute_from_tuple() {
        let key = "key";
        let value_owned = "value".to_string();
        let value = "value";

        let _builds = Node::root().add_attr((key, value_owned));
        let _builds = Node::root().add_attr((key, value));
    }

    #[test]
    fn custom_attribute() {
        let expected = r#"<html op="news" lang="en"></html>"#;
        let rendered = o(Html)
            .add_attr(("op", "news"))
            .add_attr(Lang::English)
            .render_string()
            .unwrap();

        assert_eq!(rendered, expected);
    }
}
