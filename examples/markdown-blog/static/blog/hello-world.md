# A blogpost title

Hi, wow what a great intro!

## A section with code

So did you know about:

```rust
#[tokio::main]
pub async fn main() {}
```

It let's you run async code! Wow!

See [tokio](https://tokio.rs/) for more.

Workin' the stack:

```rust
// It has cooked long enough!
fn birth(mut nodes: Vec<Node>) -> Vec<Node> {
    // Pop the now completed child..
    let child = nodes.pop().expect("Pop child");
    // ..also need a reference to the parent
    let parent = nodes.pop().expect("Pop parent");

    // Add the parent back to the stack with the newly added child.
    stack_add(nodes, parent.kid(child))
}
```

## A section with an image

This is an image:

![sample-image](/static/img/blossom.webp)

> Image: "A Wild Cherry (Prunus avium) in flower"
> Image Author: Benjamin Gimmel
> Photo licensed under the Creative Commons Attribution-Share Alike 3.0 Unported license.

## A section with something animated

This is animated:

![sample-animated](/static/anim/humpback.webp)

> Humpback Whales: [youtube](https://www.youtube.com/watch?v=CnKgRXGt-S8)
