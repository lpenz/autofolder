// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! *autofolder* provides a single-element "folding" container that
//! can be used to accumulate/select/etc. values in an ad-hoc fashion.
//!
//! # TL;DR
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
//! # Rationale
//!
//! *Folding* in Rust is accomplished via the [`Iterator::fold`]
//! method, like so:
//! ```rust
//! # let iterator = vec![0_i32].into_iter();
//! # let initial = 0_i32;
//! # let function = |a: i32, b:i32| a+b;
//! iterator.fold(initial, function);
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

mod dynfolder;
pub use self::dynfolder::*;
