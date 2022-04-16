use cached::proc_macro::cached;
use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*, template};

use crate::story::Story;

use super::Renderable;

pub struct Candy;

/*

<!--

GRADIENT BANNER DESIGN BY SIMON LURWER ON DRIBBBLE:
https://dribbble.com/shots/14101951-Banners

-->
<div class="main-container">
  <div class="heading">
    <h1 class="heading__title">Gradient Banner Cards</h1>
    <p class="heading__credits"><a class="heading__link" target="_blank" href="https://dribbble.com/sl">Design by Simon Lurwer on Dribbble</a></p>
  </div>
  <div class="cards">
    <div class="card card-1">
      <div class="card__icon"><i class="fas fa-bolt"></i></div>
      <p class="card__exit"><i class="fas fa-times"></i></p>
      <h2 class="card__title">Lorem ipsum dolor sit amet, consectetur adipiscing elit.</h2>
      <p class="card__apply">
        <a class="card__link" href="#">Apply Now <i class="fas fa-arrow-right"></i></a>
      </p>
    </div>
    <div class="card card-2">
      <div class="card__icon"><i class="fas fa-bolt"></i></div>
      <p class="card__exit"><i class="fas fa-times"></i></p>
      <h2 class="card__title">Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</h2>
      <p class="card__apply">
        <a class="card__link" href="#">Apply Now <i class="fas fa-arrow-right"></i></a>
      </p>
    </div>
  </div>
</div>

*/

#[cached]
fn candy_story(story: Story, class: String) -> Node {
    let outer = Div.class(format!("card {class}"));
    let icon = Div.class("card__icon").kid(I.class("fas fa-bolt"));
    let exit = P.class("card__exit").kid(I.class("fas fa-times"));
    let title = H2.class("card__title").text(story.title);

    let apply = P.class("card__apply").kid(
        A::href("#")
            .class("card__link")
            .text("Idk go to the comments?")
            .kid(I.class("fas fa-arrow-right")),
    );

    outer.kid(icon).kid(exit).kid(title).kid(apply)
}

#[cached]
fn candy_story2(story: Story, class: String) -> Node {
    Div.class(format!("card {class}"))
        .kid(Div.class("card__icon").kid(I.class("fas fa-bolt")))
        .kid(P.class("card__exit").kid(I.class("fas fa-times")))
        .kid(H2.class("card__title").text(story.title))
        .kid(
            P.class("card__apply").kid(
                A::href("#")
                    .class("card__link")
                    .text("Idk go to the comments?")
                    .kid(I.class("fas fa-arrow-right")),
            ),
        )
}

impl Renderable for Candy {
    fn frontpage(&self, stories: Vec<Story>) -> Node {
        let mut cards = Div.class("cards");

        for (index, story) in stories.into_iter().enumerate() {
            // The CSS defines card-0 through card-4, which decides colors.
            cards.push_kid(candy_story(story, format!("card-{}", index % 5)));
        }

        let body = Body.kid(Div.class("main-container").kid(cards));
        let head = template::head()
            .kid(Link::stylesheet(mime::TEXT_CSS, "/static/candy.css"))
            .kid(Link::stylesheet(
                mime::TEXT_CSS,
                "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.1.1/css/all.min.css",
            ));

        template::HtmlDocumentBuilder::new()
            .with_head(head)
            .with_body(body)
            .build()
    }

    fn comments(&self, _story: Story) -> Node {
        todo!()
    }
}
