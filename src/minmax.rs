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
            /// Alias for [`$name::reduce`]
            pub fn eval(&mut self, item: Item)
            where
                Item: PartialOrd,
            {
                self.reduce(item)
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

/// Container that keeps a tuple with both the min and max values.
#[derive(Debug, Copy, Clone)]
pub struct MinMax<Item> {
    min: Option<Item>,
    max: Option<Item>,
}

impl<Item> MinMax<Item> {
    /// Creates a new `MinMax` with the provided initial values.
    pub fn new(initial: Item) -> Self {
        Self {
            min: Some(initial),
            max: None,
        }
    }
    /// Deconstruct self and return the inner values that were found.
    ///
    /// This function returns `max` as `None` if we are not holding
    /// two values.
    pub fn into_inner(self) -> (Option<Item>, Option<Item>) {
        (self.min, self.max)
    }
    /// Deconstruct self and return the inner values if they were both found.
    ///
    /// This function returns `None` globally if we are not holding
    /// two values.
    pub fn into_inners(self) -> Option<(Item, Item)> {
        match (self.min, self.max) {
            (None, _) => None,
            (_, None) => None,
            (Some(min), Some(max)) => Some((min, max)),
        }
    }
    /// Deconstruct self and return the inner values unwrapped.
    ///
    /// Be aware that this function does an internal `unwrap`, which
    /// panics when we are not holding two values.
    pub fn into_inner_unwrap(self) -> (Item, Item) {
        (self.min.unwrap(), self.max.unwrap())
    }
    /// Returns a reference to the inner values, if they exist.
    ///
    /// If we are not holding two values, this function returns `max`
    /// as `None`.
    pub fn as_ref(&self) -> (Option<&Item>, Option<&Item>) {
        (self.min.as_ref(), self.max.as_ref())
    }
    /// Returns a reference to both inner values, if they both exist.
    ///
    /// This function returns `None` globally if we are not holding
    /// two values.
    pub fn as_refs(&self) -> Option<(&Item, &Item)> {
        match (&self.min, &self.max) {
            (None, _) => None,
            (_, None) => None,
            (Some(min), Some(max)) => Some((min, max)),
        }
    }
    /// Returns a reference to the min inner values, if it exist.
    pub fn min_as_ref(&self) -> Option<&Item> {
        self.min.as_ref()
    }
    /// Returns a reference to the max inner values, if it exist.
    pub fn max_as_ref(&self) -> Option<&Item> {
        self.max.as_ref()
    }
    /// Replaces a current value with the new one if the new one is greater/less.
    ///
    /// When we have a single value, `min` is always filled up first,
    /// and then swapped with `max` if necessary.
    pub fn reduce(&mut self, item: Item)
    where
        Item: PartialOrd,
    {
        let oldmin_opt = std::mem::take(&mut self.min);
        if let Some(oldmin) = oldmin_opt {
            let cmpmin = item.partial_cmp(&oldmin);
            if cmpmin == Some(std::cmp::Ordering::Less) {
                if self.max.is_none() {
                    // We only had min, so we have to put it in max:
                    self.max = Some(oldmin);
                }
                self.min = Some(item);
            } else {
                self.min = Some(oldmin);
                // As we have not moved item into min, we can check it against max:
                let oldmax_opt = std::mem::take(&mut self.max);
                if let Some(oldmax) = oldmax_opt {
                    if item.partial_cmp(&oldmax) == Some(std::cmp::Ordering::Greater) {
                        self.max = Some(item);
                    } else {
                        self.max = Some(oldmax);
                    }
                } else if cmpmin == Some(std::cmp::Ordering::Greater) {
                    // We have a min, and item is greater than it, but we didn't have a max:
                    self.max = Some(item);
                } else {
                    // If it's equal to min, we don't do anything.
                };
            }
        } else {
            // First item always goes to self.min:
            self.min = Some(item);
        };
    }
    /// Alias for [`MinMax::reduce`]
    pub fn eval(&mut self, item: Item)
    where
        Item: PartialOrd,
    {
        self.reduce(item)
    }
}

impl<Item> From<Item> for MinMax<Item> {
    fn from(item: Item) -> Self {
        Self::new(item)
    }
}

impl<Item> Extend<Item> for MinMax<Item>
where
    Item: PartialOrd,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.reduce(i));
    }
}

impl<Item> Default for MinMax<Item> {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
        }
    }
}

impl<Item> std::iter::FromIterator<Item> for MinMax<Item>
where
    Item: PartialOrd,
{
    fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
        let mut autofolder = Self::default();
        autofolder.extend(iter);
        autofolder
    }
}
