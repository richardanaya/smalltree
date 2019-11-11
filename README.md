# smalltree

<a href="https://docs.rs/smalltree"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

```toml
[dependencies]
smalltree = "0.0"
```

A tree implementation based on [`smallvec`](https://github.com/servo/rust-smallvec) and generational indexes to increase memory cache locality of node members.

- [x] `#![no_std]`
- [x] simple api that is easy to work with borrow checker
- [ ] use generics to pass in a numeric type to be able to tune sizing

# Example

```rust
struct Foo;

fn main(){
  let g = smallgraph::SmallTree::new(Foo);
  let n1 = g.attach(g.root);
  let n2 = g.attach(n1);
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `smallgraph` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
