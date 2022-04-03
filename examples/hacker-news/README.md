# Hacker News clone

`Hacker News.html` was pulled from the official website.

In general, the clone is very close to this.

For HTML which uses deprecated attributes, a file with [extra css](static/news-extra.css) is used instead.

## How it works

The HTML structure is generated when a GET request lands on the correct route (`/`) in the Axum server.

The static parts of the page is only rendered once (the top nav, the footer, the general HTML skeleton).

The dynamic part is the stories themselves. These are continously kept fresh at some interal in a background worker on the server ([here](src/state_worker.rs)).
The stories are stored in a shared state ([defined as such](src/state.rs)), a simple read-write lock.

The server replies to clients by combining the cached static parts and the dynamic part, where the latter is generated via reading the shared state.


## TODO

* Comments page
* Use the cached crate on the shared state with a timeout.
* Decide on what we want to do with style inlined into the HTML.
  * Move it as well to CSS?

## Probably won't do

* Login
* Search
