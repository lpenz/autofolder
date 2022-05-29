// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

/// The `Max` type uses the [`std::cmp::PartialOrd`] trait to contain only the largest iterated
/// value.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create an autofolder that collects the max usize.
/// let mut max = Max::<usize>::default();
///
/// // We can "reduce-in" individual items.
/// // (note: as this is the first value, we incorporate it
/// //  without calling the trait function)
/// max.reduce(3);
///
/// // We can then peek at the running output:
/// println!("Partial max is {:?}", max.as_ref());
///
/// // And still keep on folding by processing whole iterators:
/// max.extend((1..=5));
///
/// // And finally consume the autofolder to get the final output value:
/// let max = max.into_inner().unwrap();
/// println!("Final max is {}", max);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Max<Item> {
    item: Option<Item>,
}

/// Analogous to [`Max`] but for the minimal value.
#[derive(Debug, Copy, Clone)]
pub struct Min<Item> {
    item: Option<Item>,
}

macro_rules! impl_minmax {
    ($name: ident, $cmpval: expr) => {
        impl<Item> $name<Item> {
            /// Creates a new `$name` with the provided initial value.
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
            /// Replaces the current value with the new one if the new one is greater.
            pub fn reduce(&mut self, item: Item)
            where
                Item: PartialOrd,
            {
                if self.item.is_none()
                    || self
                        .item
                        .as_ref()
                        .map(|i| item.partial_cmp(i) == Some($cmpval))
                        == Some(true)
                {
                    self.item = Some(item);
                }
            }
        }

        impl<Item> From<Item> for $name<Item> {
            fn from(item: Item) -> Self {
                Self::new(item)
            }
        }

        impl<Item> Extend<Item> for $name<Item>
        where
            Item: PartialOrd,
        {
            fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
                iter.into_iter().for_each(|i| self.reduce(i));
            }
        }

        impl<Item> Default for $name<Item> {
            fn default() -> Self {
                Self { item: None }
            }
        }

        impl<Item> std::iter::FromIterator<Item> for $name<Item>
        where
            Item: PartialOrd,
        {
            fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
                let mut autofolder = Self::default();
                autofolder.extend(iter);
                autofolder
            }
        }
    };
}

impl_minmax!(Max, std::cmp::Ordering::Greater);
impl_minmax!(Min, std::cmp::Ordering::Less);
