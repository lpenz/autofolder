// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::marker;
use std::mem;

/// The `ImplFolder` type uses the [`FolderTrait`] for the folding function.
///
/// - Pros:
///   - Slighly more efficient than [`DynFolder`](crate::DynFolder) due to monomorphization,
///     which turns `.fold` calls into direct function calls.
///   - Can be used with `.collect()` if the `output` type implements [`Default`].
/// - Cons:
///   - Folding function can only use types defined in the user crate, which is a limitation of
///     using traits.
///   - Each parameterized `ImplFolder`, defined by the pair of types, can only have one folding
///     function.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create a type wrapper for usize:
/// pub struct Usize(usize);
///
/// // Create an autofolder that sums `u16` items into an `Usize` output.
/// let mut sum = ImplFolder::<Usize, u16>::new(Usize(7));
///
/// // Implement FolderTrait for the desired ImplFolder type.
/// autofolder_impl_foldertrait!(|a: Usize, b: u16| {
///     Usize(a.0 + b as usize)
/// });
///
/// // We can "fold-in" individual items:
/// sum.fold(3);
///
/// // We can then peek at the running output:
/// println!("Partial sum is {}", sum.as_ref().0);
///
/// // And still keep on folding by processing whole iterators:
/// sum.extend((1..=5));
///
/// // And finally consume the autofolder to get the final output value:
/// let total = sum.into_inner();
/// println!("Total sum is {}", total.0);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct ImplFolder<Output, Item> {
    output: Output,
    item: marker::PhantomData<Item>,
}

/// Trait that provides the `fold` implementation for [`ImplFolder`]
pub trait FolderTrait<Output, Item> {
    /// User-defined folding function.
    /// The user should return a new `output` with `item` folded in.
    fn fold(output: Output, item: Item) -> Output;
}

impl<Output, Item> ImplFolder<Output, Item> {
    /// Creates a new `ImplFolder` with the provided initial value.
    pub fn new(initial: Output) -> Self {
        Self {
            output: initial,
            item: marker::PhantomData,
        }
    }
    /// Deconstruct self and return the inner value.
    pub fn into_inner(self) -> Output {
        self.output
    }
    /// Folds an individual value into self.
    pub fn fold(&mut self, item: Item)
    where
        Self: FolderTrait<Output, Item>,
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
        let new_output = <Self as FolderTrait<Output, Item>>::fold(current_output, item);
        // Put new_output in self.0 and get the old uninit back:
        let uninit = mem::replace(&mut self.output, new_output);
        // We need to mem::forget it to avoid running destructors on
        // the uninitialized value:
        mem::forget(uninit);
    }
}

impl<Output, Item> From<Output> for ImplFolder<Output, Item> {
    fn from(output: Output) -> Self {
        Self::new(output)
    }
}

impl<Output, Item> AsRef<Output> for ImplFolder<Output, Item> {
    fn as_ref(&self) -> &Output {
        &self.output
    }
}

impl<Output, Item> Extend<Item> for ImplFolder<Output, Item>
where
    ImplFolder<Output, Item>: FolderTrait<Output, Item>,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.fold(i));
    }
}

/* We can implement Default and FromIterator (.collect) if Output implements Default: */

impl<Output, Item> Default for ImplFolder<Output, Item>
where
    Output: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<Output, Item> std::iter::FromIterator<Item> for ImplFolder<Output, Item>
where
    Output: Default,
    ImplFolder<Output, Item>: FolderTrait<Output, Item>,
{
    fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
        let mut autofolder = ImplFolder::<Output, Item>::default();
        autofolder.extend(iter);
        autofolder
    }
}

/// Macro that implements [`FolderTrait`] with the provide closure.
///
/// It extracts the types used in the parameters of the closure to fill in FolderTrait's
/// arguments, reducing the amount of repetition.
#[macro_export]
macro_rules! autofolder_impl_foldertrait{
    (|$a:ident : $output_type: ty, $i:ident : $item_type: ty| $body: block) => {
        impl FolderTrait<$output_type, $item_type> for ImplFolder<$output_type, $item_type> {
            fn fold(mut $a: $output_type, $i: $item_type) -> $output_type $body
        }
    }
}
