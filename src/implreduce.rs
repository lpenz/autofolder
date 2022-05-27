// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

/// The `ImplReduce` type uses the [`ReduceTrait`] for the reduce function.
///
/// This is essentially an [`ImplFolder`](crate::ImplFolder) that doesn't require an initial
/// value - the first value to be "reduced" is incorporated as-is instead.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create a type wrapper for usize:
/// #[derive(Debug)]
/// pub struct Usize(usize);
///
/// // Create an autofolder that collects the max Usize.
/// let mut max = ImplReduce::<Usize>::default();
///
/// // Implement ReduceTrait for the desired ImplReduce type.
/// autofolder_impl_reducetrait!(|a, b| -> Usize {
///     Usize(std::cmp::max(a.0, b.0))
/// });
///
/// // We can "reduce-in" individual items.
/// // (note: as this is the first value, we incorporate it
/// //  without calling the trait function)
/// max.reduce(Usize(3));
///
/// // We can then peek at the running output:
/// println!("Partial max is {:?}", max.as_ref());
///
/// // And still keep on folding by processing whole iterators:
/// max.extend((1..=5).map(Usize));
///
/// // And finally consume the autofolder to get the final output value:
/// let max = max.into_inner().unwrap();
/// println!("Final max is {}", max.0);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct ImplReduce<Item> {
    item: Option<Item>,
}

/// Trait that provides the `reduce` implementation for [`ImplReduce`]
pub trait ReduceTrait<Item> {
    /// User-defined reduce function.
    /// The user should return a new item that "reduces" `lhs` and `rhs`.
    fn reduce(lhs: Item, rhs: Item) -> Item;
}

impl<Item> ImplReduce<Item> {
    /// Creates a new `ImplReduce` with the provided initial value.
    pub fn new(initial: Item) -> Self {
        Self {
            item: Some(initial),
        }
    }
    /// Deconstruct self and return the inner value.
    pub fn into_inner(self) -> Option<Item> {
        self.item
    }
    /// Returns a reference to the inner value, if there is one.
    pub fn as_ref(&self) -> Option<&Item> {
        self.item.as_ref()
    }
    /// Reduce the given item into the current self item.
    pub fn reduce(&mut self, item: Item)
    where
        Self: ReduceTrait<Item>,
    {
        if let Some(current_item) = self.item.take() {
            self.item = Some(<Self as ReduceTrait<Item>>::reduce(current_item, item));
        } else {
            self.item = Some(item);
        }
    }
}

impl<Item> From<Item> for ImplReduce<Item> {
    fn from(item: Item) -> Self {
        Self::new(item)
    }
}

impl<Item> Extend<Item> for ImplReduce<Item>
where
    ImplReduce<Item>: ReduceTrait<Item>,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.reduce(i));
    }
}

impl<Item> Default for ImplReduce<Item> {
    fn default() -> Self {
        Self { item: None }
    }
}

impl<Item> std::iter::FromIterator<Item> for ImplReduce<Item>
where
    ImplReduce<Item>: ReduceTrait<Item>,
{
    fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
        let mut autofolder = Self::default();
        autofolder.extend(iter);
        autofolder
    }
}

/// Macro that implements [`ReduceTrait`] with the provide closure.
///
/// It extracts the types used in the parameters of the closure to fill in ReduceTrait's
/// arguments, reducing the amount of repetition.
#[macro_export]
macro_rules! autofolder_impl_reducetrait{
    (|$a:ident , $i:ident| -> $item_type: ty $body: block) => {
        impl ReduceTrait<$item_type> for ImplReduce<$item_type> {
            fn reduce(mut $a: $item_type, $i: $item_type) -> $item_type $body
        }
    }
}
