[![CI](https://github.com/lpenz/autofolder/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/autofolder/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/autofolder/badge.svg?branch=main)](https://coveralls.io/github/lpenz/autofolder?branch=main)
[![crates.io](https://img.shields.io/crates/v/autofolder)](https://crates.io/crates/autofolder)
[![doc.rs](https://docs.rs/autofolder/badge.svg)](https://docs.rs/autofolder)


# autofolder

*autofolder* provides a single-element "folding" container that
can be used to accumulate/select/etc. values in an ad-hoc fashion.

## TL;DR

```rust
use autofolder::*;

// Create an autofolder that retains the max u32 value:
let mut max = DynFolder::new(0_u32, std::cmp::max);

// We can "fold-in" individual items:
max.fold(3);

// We can then peek at the running output:
println!("Partial max is {}", max.as_ref());

// And still keep on folding by processing whole iterators:
max.extend((1..=5));

// And finally consume the autofolder to get the final output value:
println!("Max value is {}", max.into_inner());
```

## Rationale

*Folding* in Rust is accomplished via the [`Iterator::fold`]
method, like so:
```rust
iterator.fold(initial, function);
```

That works well when all the data we need is provided by a single iterator. If we have a
more complex logic, `fold` can't be used.

*autofolder* flips this structure by being built with the initial value and the folding
function, and accepting values from various types of different sources during its lifetime.

```rust
let mut autofolder = Autofolder::new(initial, function);
// Fold in a whole iterator, can be repeated:
autofolder.extend(iterator);
// Fold in an individual value:
autofolder.fold(value);
```

[`Iterator::fold`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
[`DynFolder`]:https://docs.rs/autofolder/latest/autofolder/struct.DynFolder.html
