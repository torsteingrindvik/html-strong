use super::Renderable;

pub struct Original;

impl Renderable for Original {
    fn frontpage(stories: Vec<crate::story::Story>) -> html_strong::document_tree::Node {
        todo!()
    }

    fn comments(story: crate::story::Story) -> html_strong::document_tree::Node {
        todo!()
    }
}
