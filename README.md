# html-strong

Opinionated, strongly typed HTML suitable for server side rendering and templating.

Early WIP.

## Why?

I wanted to learn more about web development and write Rust at the same time.

### What does it look like

TODO.

See the [Axum](https://github.com/tokio-rs/axum) [hello world example](./examples/axum-hello-world.rs).

See the [Hacker News](https://news.ycombinator.com/) [clone example (WIP)](./examples/hacker-news.rs).

## Features

### Strongly typed

Gives us IDE support and compile time guarantees.

### No user-facing macros 

Most templating libraries I've seen do this.
I wanted to avoid this because some macros break `rust-analyzer`.

Also, domain specific languages carry an extra cognitive load that I'm not always interested in having.

### Templating is Rust code

Create HTML dynamically using normal Rust control loops.

## Future efforts

### Ergonomics

Try finding improvements.

Example:

```rust
o(Td::default())
	.add_class("subtext")
	.kid(score_span)
	.add_text(" by ")
	.kid(user_href)
	.add_text(ONE_SPACE)
	.kid(unv_span)
	.add_text(PIPE_DELIMITER)
	.kid(hide_a)
	.add_text(PIPE_DELIMITER)
```

Would look better as:

```rust
o(Td
	.class("subtext")
	.kid(score_span)
	.text(" by ")
	.kid(user_href)
	.text(ONE_SPACE)
	.kid(unv_span)
	.text(PIPE_DELIMITER)
	.kid(hide_a)
	.text(PIPE_DELIMITER)
```

All tag struct implement the `Tag` trait.

If we use an extension trait, we should be able to make e.g. `class` available for any struct which implements `Tag`.

### Missing stuff

Lots of tags and attributes are missing.
Add them as we go.

### Optimization pass

I ❤️ "Don't let the perfect be the enemy of the good".
Allocate and box stuff liberally, then benchmark later and see where/if optimization are needed.

### Caching

Try using the [cached](https://docs.rs/cached/latest/cached/) library (in examples) to see if we can get a nice pattern going for
not having to re-render templates all the time.

### Figure out CSS

What do we do about CSS?

* Ignore it? We can already set an id and add classes to tags, the user can then handle CSS.
* Find an API for adding style to tags?
* Attempt a similar API (or library) for writing CSS?
