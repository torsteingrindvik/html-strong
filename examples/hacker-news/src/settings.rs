use html_strong::{
    document_tree::{o, Node},
    science_lab::NodeExt,
    tags::{form::Method, *},
};

/*

Bootstrap list groups: https://getbootstrap.com/docs/5.1/examples/list-groups/

Example:

<div class="list-group list-group-checkable">

  <input class="list-group-item-check" type="radio" name="listGroupCheckableRadios" id="listGroupCheckableRadios1" value="" checked="">
  <label class="list-group-item py-3" for="listGroupCheckableRadios1">
    First radio
    <span class="d-block small opacity-50">With support text underneath to add more detail</span>
  </label>

  <input class="list-group-item-check" type="radio" name="listGroupCheckableRadios" id="listGroupCheckableRadios2" value="">
  <label class="list-group-item py-3" for="listGroupCheckableRadios2">
    Second radio
    <span class="d-block small opacity-50">Some other text goes here</span>
  </label>

  <input class="list-group-item-check" type="radio" name="listGroupCheckableRadios" id="listGroupCheckableRadios3" value="">
  <label class="list-group-item py-3" for="listGroupCheckableRadios3">
    Third radio
    <span class="d-block small opacity-50">And we end with another snippet of text</span>
  </label>

  <input class="list-group-item-check" type="radio" name="listGroupCheckableRadios" id="listGroupCheckableRadios4" value="" disabled="">
  <label class="list-group-item py-3" for="listGroupCheckableRadios4">
    Fourth disabled radio
    <span class="d-block small opacity-50">This option is disabled</span>
  </label>
</div>

*/

#[derive(Debug)]
pub struct Option {
    title: String,
    description: String,
    checked: bool,
}

impl Option {
    pub fn new(title: &str, description: &str, checked: bool) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            checked,
        }
    }
}

pub struct Settings {
    name: String,
    options: Vec<Option>,
}

impl Settings {
    pub fn new(name: &str) -> Self {
        Self {
            options: vec![],
            name: name.to_string(),
        }
    }

    pub fn new_with_options(name: &str, options: Vec<Option>) -> Self {
        Self {
            options,
            name: name.to_string(),
        }
    }

    pub fn add_option(mut self, title: &str, description: &str, checked: bool) -> Self {
        self.options.push(Option::new(title, description, checked));
        self
    }

    pub fn into_page(self) -> Node {
        let body = Body
            .kid(self.into_node())
            .class("d-flex flex-column min-vh-100 justify-content-center");

        examples_lib::html_doc(
            Some(vec![
                "https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css",
                "/hn/static/list-groups.css",
            ]),
            None,
            None,
            body,
        )
    }
}

impl NodeExt for Settings {
    fn into_node(self) -> Node {
        let mut form =
            o(Form::new(Method::Post, "/hn/settings")).add_class("list-group list-group-checkable");

        for option in self.options {
            let name = option.title.to_lowercase();

            let mut input_radio = Input::radio(&name, &self.name);
            if option.checked {
                input_radio.set_checked();
            }

            let input = o(input_radio)
                .add_class("list-group-item-check")
                .set_id(&name);

            let label = o(Label::new(&name))
                .add_class("list-group-item py-3")
                .add_text(&option.title)
                .kid(
                    o(Span)
                        .add_class("d-block small opacity-50")
                        .add_text(&option.description),
                );

            form.push_kid(input);
            form.push_kid(label);
        }

        form.push_kid(
            o(Button::new())
                .add_class("btn btn-primary")
                .add_text("Save"),
        );

        form
    }
}
