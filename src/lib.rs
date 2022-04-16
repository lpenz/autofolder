// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! *autofolder* is a single-element folding container that can be used
//! to accumulate and select values (for example) in an ad-hoc fashion.

mod accumulator;
pub use self::accumulator::*;
