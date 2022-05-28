// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

/// The `DynReduce` type uses a struct field for the folding function.
///
/// This is essentially an [`DynFolder`](crate::DynFolder) that doesn't require an initial
/// value - the first value to be "reduced" is incorporated as-is instead.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create an autofolder that maxs `u16` items into an `usize` output.
/// let mut max = DynReduce::<usize, _>::new(|a, b| a + b as usize);
///
/// // We can "fold-in" individual items:
/// // (note: as this is the first value, we incorporate it
/// //  without calling the trait function)
/// max.reduce(3);
///
/// // We can then peek at the running output:
/// println!("Partial max is {}", max.as_ref().unwrap());
///
/// // And still keep on folding by processing whole iterators:
/// max.extend((1..=5));
///
/// // And finally conmaxe the autofolder to get the final output value:
/// println!("Final max is {}", max.into_inner().unwrap());
/// ```
#[derive(Copy, Clone)]
pub struct DynReduce<Item, Func> {
    item: Option<Item>,
    function: Func,
}

impl<Item, Func> DynReduce<Item, Func> {
    /// Creates a new `DynReduce` with the provided folding function.
    pub fn new(func: Func) -> Self
    where
        Func: Fn(Item, Item) -> Item,
    {
        Self {
            item: None,
            function: func,
        }
    }
    /// Returns the contained value, consuming the self value.
    pub fn into_inner(self) -> Option<Item> {
        self.item
    }
    /// Returns a reference to the inner value, if there is one.
    pub fn as_ref(&self) -> Option<&Item> {
        self.item.as_ref()
    }
    /// Folds an individual value into self.
    pub fn reduce(&mut self, item: Item)
    where
        Func: Fn(Item, Item) -> Item,
    {
        if let Some(current_item) = self.item.take() {
            self.item = Some((self.function)(current_item, item));
        } else {
            self.item = Some(item);
        }
    }
}

impl<Item, Func> std::fmt::Debug for DynReduce<Item, Func>
where
    Item: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DynReduce::<{}, _> {{ item: {:?}, function: {} }}",
            &std::any::type_name::<Item>(),
            self.item,
            &std::any::type_name::<Func>(),
        )
    }
}

impl<Item, Func> From<Func> for DynReduce<Item, Func>
where
    Func: Fn(Item, Item) -> Item,
{
    fn from(func: Func) -> Self {
        Self::new(func)
    }
}

impl<Item, Func> Extend<Item> for DynReduce<Item, Func>
where
    Func: Fn(Item, Item) -> Item,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.reduce(i));
    }
}
