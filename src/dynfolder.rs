// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::marker;
use std::mem;

/// The `DynFolder` type uses a struct field for the folding function, making use of dynamically
/// dispatch.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create an autofolder that sums `u16` items into an `usize` output.
/// let mut sum = DynFolder::<usize, u16, _>::new(7, |a, b| a + b as usize);
///
/// // We can "fold-in" individual items:
/// sum.fold(3);
///
/// // We can then peek at the running output:
/// println!("Partial sum is {}", sum.as_ref());
///
/// // And still keep on folding by processing whole iterators:
/// sum.extend((1..=5));
///
/// // And finally consume the autofolder to get the final output value:
/// let total = sum.into_inner();
/// println!("Total sum is {}", total);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct DynFolder<Output, Item, Func> {
    output: Output,
    function: Func,
    item: marker::PhantomData<Item>,
}

impl<Output, Item, Func> DynFolder<Output, Item, Func> {
    /// Creates a new `DynFolder` with the provided initial value and folding function.
    pub fn new(initial: Output, func: Func) -> Self
    where
        Func: Fn(Output, Item) -> Output,
    {
        Self {
            output: initial,
            function: func,
            item: marker::PhantomData,
        }
    }
    /// Returns the contained value, consuming the self value.
    pub fn into_inner(self) -> Output {
        self.output
    }
    /// Folds an individual value into self.
    pub fn fold(&mut self, item: Item)
    where
        Func: Fn(Output, Item) -> Output,
    {
        // SAFETY: we move out current output to the folding function;
        // to do that, we replace it with an uninitialized value.
        // This is safe because we immediately put back the new value
        // returned by the folding function.
        #[allow(clippy::uninit_assumed_init)]
        let uninit = unsafe { mem::MaybeUninit::<Output>::uninit().assume_init() };
        let current_output = mem::replace(&mut self.output, uninit);
        // self.0 now has the uninitalized value
        // Give ownership of current_output to self.function, and get new_output:
        let new_output = (self.function)(current_output, item);
        // Put new_output in self.0 and get the old uninit back:
        let uninit = mem::replace(&mut self.output, new_output);
        // We need to mem::forget it to avoid running destructors on
        // the uninitialized value:
        mem::forget(uninit);
    }
}

impl<Output, Item, Func> AsRef<Output> for DynFolder<Output, Item, Func> {
    fn as_ref(&self) -> &Output {
        &self.output
    }
}

impl<Output, Item, Func> Extend<Item> for DynFolder<Output, Item, Func>
where
    Func: Fn(Output, Item) -> Output,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.fold(i));
    }
}
