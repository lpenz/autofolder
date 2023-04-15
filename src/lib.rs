// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! *autofolder* provides a single-element "folding" container that
//! can be used to accumulate/select/etc. values in an ad-hoc fashion.
//!
//! # TL;DR: [`DynFolder`] example
//!
//! ```rust
//! use autofolder::*;
//!
//! // Create an autofolder that retains the max u32 value:
//! let mut max = DynFolder::new(0_u32, std::cmp::max);
//!
//! // We can "fold-in" individual items:
//! max.fold(3);
//!
//! // We can then peek at the running output:
//! println!("Partial max is {}", max.as_ref());
//!
//! // And still keep on folding by processing whole iterators:
//! max.extend((1..=5));
//!
//! // And finally consume the autofolder to get the final output value:
//! println!("Max value is {}", max.into_inner());
//! ```
//!
//! See also [`Min`], [`Max`] and [`MinMax`] for useful specific reducers.
//!
//! # Rationale
//!
//! *Folding* in Rust is accomplished via the [`Iterator::fold`] method, like so:
//! ```rust
//! # let iterator = vec![0_i32].into_iter();
//! # let initial = 0_i32;
//! # let function = |a: i32, b:i32| a+b;
//! iterator.fold(initial, function);
//! // (and this is all you can do)
//! ```
//!
//! That works well when all the data we need is provided by a single iterator. If we have a
//! more complex logic, `fold` can't be used.
//!
//! *autofolder* flips this structure by being built with the initial value and the folding
//! function, and accepting values from various types of different sources during its lifetime.
//!
//! ```rust
//! # use autofolder::*;
//! # type Autofolder<F> = DynFolder::<i32, i32, F>;
//! # let initial = 0_i32;
//! # let function = |a: i32, b:i32| a + b;
//! # let iterator = vec![0_i32].into_iter();
//! # let value = 0_i32;
//! let mut autofolder = Autofolder::new(initial, function);
//! // Fold in a whole iterator, can be repeated:
//! autofolder.extend(iterator);
//! // Fold in an individual value:
//! autofolder.fold(value);
//! ```
//!
//! # Types of folders
//!
//! ## By binding strategy
//!
//! This crate provides two types of autofolders with different function binding strategies:
//! - [`DynFolder`]: the folding function is provided as a closure
//!   that is kept in a struct field. Characteristics:
//!   - Folding function can use any type, builtin or otherwise.
//!   - Each instance can use a different folding function, provided as a constructor argument.
//!     On the flip side, we can't use `DynFolder` with [`.collect()`](Iterator::collect).
//!   - Slightly less efficient than `ImplFolder` due to the use of dynamic dispatch - we are
//!     effectively using a function pointer instead of a function call, after all.
//! - [`ImplFolder`]: the folding function is implemented via a trait.
//!   - Folding function can only use types defined in the user crate, which is a limitation of
//!     using traits.
//!   - Each parameterized `ImplFolder`, defined by the pair of types, can only have one folding
//!     function. Because of that, we can use `ImplFolder` with
//!     [`.collect()`](Iterator::collect) if the `output` type implements [`Default`]
//!   - Slighly more efficient than `DynFolder` due to monomorphization, which turns `.fold`
//!     calls into direct function calls.
//!
//! ## By aggregation strategy
//!
//! *Reduce* in rust is a special kind of folding where the aggregator and the item types of
//! the folding function are the same (`Fn(Item, Item) -> Item`). That allows us to set the
//! internal autofolder state with the first yielded value, without calling the corresponding
//! function.
//!
//! This crate provides the following "autoreducer" types:
//! - [`DynReduce`]: the reduce function is implemented via a trait.
//!   - Similar to [`DynFolder`].
//!   - [`.into_inner()`](DynReduce::into_inner) returns an [`Option`].
//!   - Constructor takes the `reduce` function.
//! - [`ImplReduce`]: the reduce function is implemented via a trait.
//!   - Similar to [`ImplFolder`].
//!   - [`.into_inner()`](ImplReduce::into_inner) returns an [`Option`].
//!   - Implements [`.collect()`](Iterator::collect) even when the type parameters don't
//!     implement [`Default`].
//!
//! ## Specific autofolders
//!
//! This create also provides some built-in autofolders for specific functions:
//! - [`Min`]: container that keeps only the minimal value iterated, as given by [`std::cmp::PartialOrd`].
//! - [`Max`]: analogous to `Max`, but for the max value.
//! - [`MinMax`]: container that keeps a tuple with both the min and max values.
//!

mod dynfolder;
pub use self::dynfolder::*;

mod dynreduce;
pub use self::dynreduce::*;

mod implfolder;
pub use self::implfolder::*;

mod implreduce;
pub use self::implreduce::*;

mod minmax;
pub use self::minmax::*;

#[cfg(feature = "num")]
mod minmaxnum;

#[cfg(feature = "num")]
pub use self::minmaxnum::*;
