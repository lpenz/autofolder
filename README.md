[![CI](https://github.com/lpenz/autofolder/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/autofolder/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/autofolder/badge.svg?branch=main)](https://coveralls.io/github/lpenz/autofolder?branch=main)
[![crates.io](https://img.shields.io/crates/v/autofolder)](https://crates.io/crates/autofolder)
[![doc.rs](https://docs.rs/autofolder/badge.svg)](https://docs.rs/autofolder)


# autofolder

*autofolder* provides a single-element "folding" container that
can be used to accumulate/select/etc. values in an ad-hoc fashion.

## TL;DR: [`DynFolder`] example

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

*Folding* in Rust is accomplished via the [`Iterator::fold`] method, like so:
```rust
iterator.fold(initial, function);
// (and this is all you can do)
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

## Types of folders

This crate provides two types of autofolders with different strategies:
- [`DynFolder`]: the folding function is provided as a closure
  that is kept in a struct field. Characteristics:
  - Folding function can use any type, builtin or otherwise.
  - Each instance can use a different folding function, provided as a constructor argument.
    On the flip side, we can't use `DynFolder` with [`.collect()`](Iterator::collect).
  - Slightly less efficient than `ImplFolder` due to the use of dynamic dispatch - we are
    effectively using a function pointer instead of a function call, after all.
- [`ImplFolder`]: the folding function is implemented via a trait.
  - Folding function can only use types defined in the user crate, which is a limitation of
    using traits.
  - Each parameterized `ImplFolder`, defined by the pair of types, can only have one folding
    function. Because of that, we can use `ImplFolder` with
    [`.collect()`](Iterator::collect) if the `output` type implements [`Default`]
  - Slighly more efficient than `DynFolder` due to monomorphization, which turns `.fold`
    calls into direct function calls.

[`Iterator::fold`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
[`DynFolder`]: https://docs.rs/autofolder/latest/autofolder/struct.DynFolder.html
[`ImplFolder`]: https://docs.rs/autofolder/latest/autofolder/struct.ImplFolder.html
[`Default`]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html
[Iterator::collect]: https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect
