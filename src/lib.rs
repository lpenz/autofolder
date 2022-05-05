// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! *autofolder* is a single-element folding container that can be
//! used to accumulate and/or select values (for example) in an ad-hoc
//! fashion.
//!
//!
//! # Rationale
//!
//! *Folding* in Rust is accomplished via the [`Iterator::fold`]
//! method, like so:
//! ```
//! # let iterator = vec![0_i32].into_iter();
//! # let initial = 0_i32;
//! # let function = |a: i32, b:i32| a+b;
//! iterator.fold(initial, function);
//! ```
//!
//! That works well when all the data we need is provided by a single
//! iterator. If we have a more complex logic, `fold` can't be used.
//! *autofolder* flips this structure by being built with the initial
//! value and the folding function, and accepting values from various
//! types of different sources during its lifetime.
//!
//!
//! # Example
//!
//! A more concrete example, using a [`DynFolder`]:
//!
//! ```rust
//! use autofolder::*;
//!
//! // Create an autofolder that sums `u16` items into an `usize` output.
//! let mut sum = DynFolder::<usize, u16, _>::new(7, |a, b| a + b as usize);
//!
//! // We can "fold-in" individual items:
//! sum.fold(3);
//!
//! // We can then peek at the running output:
//! println!("Partial sum is {}", sum.as_ref());
//!
//! // And still keep on folding by processing whole iterators:
//! sum.extend((1..=5));
//!
//! // And finally consume the autofolder to get the final output value:
//! let total = sum.into_inner();
//! println!("Total sum is {}", total);
//! ```
mod dynfolder;
pub use self::dynfolder::*;
