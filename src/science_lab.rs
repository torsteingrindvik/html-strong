use crate::document_tree::Node;
use crate::tags::Tag;

// struct Foo;

// trait Thing {
//     fn nice(&self) -> String;
// }

// struct Element;

// trait MakeElement {
//     fn do_it(&self) -> Element;
// }

// We cannot do this, because:
//
//	> implementing a foreign trait is only possible if at least one of the types
//	> for which it is implemented is local, and no uncovered type parameters appear
//	> before that first local type
//
// `Into<_>` is a foreign trait
//
// "At least one of the types for which it is implemented is local" ??
// "type parameter `T` must be covered by another type..." ??
// impl<T> Into<Element> for T {
//     fn into(self) -> Element {
//         todo!()
//     }
// }

// We can do this, because `MakeElement` is local
// impl<T> MakeElement for T
// where
//     T: Thing,
// {
//     fn do_it(&self) -> Element {
//         todo!()
//     }
// }

// Isn't this what we want?

// pub trait IntoNode {
//     fn into_node(self) -> Node;
// }

// impl<T> IntoNode for T
// where
//     T: Tag + 'static,
// {
//     fn into_node(self) -> Node {
//         Node::new(Box::new(self))
//     }
// }

// impl<T> Into<Node> for T where T: IntoNode {}
// impl<S> IntoNode for S where S: Into<Node> + 'static {}

// impl IntoNode for Node {
//     fn into_node(self) -> Node {
//         self
//     }
// }

// extension trait

pub trait NodeExt {
    fn into_node(self) -> Node;

    fn id<S>(&self, id: S) -> Node
    where
        S: AsRef<str>,
        Self: Clone,
    {
        self.clone().into_node().set_id(id.as_ref())
    }

    fn class<S>(&self, class: S) -> Node
    where
        S: AsRef<str>,
        Self: Clone,
    {
        self.clone().into_node().add_class(class.as_ref())
    }

    fn kid<K>(&self, kid: K) -> Node
    where
        K: NodeExt,
        Self: Clone,
    {
        self.clone().into_node().kid(kid)
    }

    fn text<T>(&self, text: T) -> Node
    where
        T: AsRef<str>,
        Self: Clone,
    {
        self.clone().into_node().add_text(text.as_ref())
    }

    fn style<S>(&self, style: S) -> Node
    where
        S: AsRef<str>,
        Self: Clone,
    {
        self.clone().into_node().add_style(style.as_ref())
    }
}

impl<T> NodeExt for T
where
    T: Tag + 'static,
{
    fn into_node(self) -> Node {
        Node::new(Box::new(self))
    }
}

impl NodeExt for Node {
    fn into_node(self) -> Node {
        self
    }
}
