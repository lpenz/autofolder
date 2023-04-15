// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use num;

/// The `MaxNum` type uses the [`num`] crate functionality to keep
/// only the largest iterated value.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create an autofolder that collects the max usize.
/// let mut max = MaxNum::<usize>::default();
///
/// // We can "reduce-in" individual items.
/// // (note: as this is the first value, we incorporate it
/// //  without calling the trait function)
/// max.reduce(2);
///
/// // `eval` does the same as `reduce`:
/// max.eval(3);
///
/// // We can then peek at the running output:
/// println!("Partial max is {:?}", max.as_ref());
///
/// // And still keep on folding by processing whole iterators:
/// max.extend((1..=5));
///
/// // And finally consume the autofolder to get the final output value:
/// let max = max.into_inner();
/// println!("Final max is {}", max);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MaxNum<Item> {
    item: Item,
}

/// The `MinNum` type uses the [`num`] crate functionality to keep
/// only the smallest iterated value.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create an autofolder that collects the min usize.
/// let mut min = MinNum::<usize>::default();
///
/// // We can "reduce-in" individual items.
/// // (note: as this is the first value, we incorporate it
/// //  without calling the trait function)
/// min.reduce(3);
///
/// // `eval` does the same as `reduce`:
/// min.eval(2);
///
/// // We can then peek at the running output:
/// println!("Partial min is {:?}", min.as_ref());
///
/// // And still keep on folding by processing whole iterators:
/// min.extend((1..=5));
///
/// // And finally consume the autofolder to get the final output value:
/// let min = min.into_inner();
/// println!("Final min is {}", min);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MinNum<Item> {
    item: Item,
}

macro_rules! impl_minmax {
    ($name: ident, $initial: expr, $cmpval: expr) => {
        impl<Item> $name<Item> {
            /// Creates a new `$name` with the provided initial value.
            pub fn new() -> Self
            where
                Item: num::Bounded,
            {
                Self { item: $initial }
            }
            /// Deconstruct self and return the inner value.
            pub fn into_inner(self) -> Item {
                self.item
            }
            /// Returns a reference to the inner value, if there is one.
            pub fn as_ref(&self) -> &Item {
                &self.item
            }
            /// Replaces the current value with the new one if the new one is greater/smaller.
            pub fn reduce(&mut self, item: Item)
            where
                Item: PartialOrd,
            {
                if item.partial_cmp(&self.item) == Some($cmpval) {
                    self.item = item;
                }
            }
            /// Replaces the current value with the one behing the ref if it is greater/smaller.
            ///
            /// This function requires the `Clone` trait, but uses it only if necessary.
            pub fn reduce_ref(&mut self, item: &Item)
            where
                Item: PartialOrd + Clone,
            {
                if item.partial_cmp(&self.item) == Some($cmpval) {
                    self.item = item.clone();
                }
            }
            /// Alias for [`$name::reduce`]
            pub fn eval(&mut self, item: Item)
            where
                Item: PartialOrd,
            {
                self.reduce(item)
            }
            /// Alias for [`$name::reduce_ref`]
            pub fn eval_ref(&mut self, item: &Item)
            where
                Item: PartialOrd + Clone,
            {
                self.reduce_ref(item)
            }
        }

        impl<Item> Default for $name<Item>
        where
            Item: num::Bounded,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<Item> From<Item> for $name<Item>
        where
            Item: PartialOrd + num::Bounded,
        {
            fn from(item: Item) -> Self {
                let mut i = Self::new();
                i.eval(item);
                i
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

        impl<'a, Item> Extend<&'a Item> for $name<Item>
        where
            Item: PartialOrd + Clone,
        {
            fn extend<It: IntoIterator<Item = &'a Item>>(&mut self, iter: It) {
                iter.into_iter().for_each(|i| self.reduce_ref(i));
            }
        }

        impl<Item> std::iter::FromIterator<Item> for $name<Item>
        where
            Item: PartialOrd + num::Bounded,
        {
            fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
                let mut autofolder = Self::default();
                autofolder.extend(iter);
                autofolder
            }
        }

        impl<'a, Item> std::iter::FromIterator<&'a Item> for $name<Item>
        where
            Item: PartialOrd + Clone + num::Bounded,
        {
            fn from_iter<It: IntoIterator<Item = &'a Item>>(iter: It) -> Self {
                let mut autofolder = Self::default();
                autofolder.extend(iter);
                autofolder
            }
        }
    };
}

impl_minmax!(MaxNum, Item::min_value(), std::cmp::Ordering::Greater);
impl_minmax!(MinNum, Item::max_value(), std::cmp::Ordering::Less);

/// The `MinMaxNum` type uses the [`num`] crate functionality to keep
/// both the smallest and largest iterated values.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create an autofolder that collects the min+max usize.
/// let mut minmax = MinMaxNum::<usize>::default();
///
/// // We can "reduce-in" individual items.
/// // (note: as this is the first value, we incorporate it
/// //  without calling the trait function)
/// minmax.reduce(3);
///
/// // Minmax fills up `min` first, so at this moment we can see that
/// // min is all we have:
/// println!("Partial minmax is {:?}", minmax.as_ref());
///
/// // `eval` does the same as `reduce`, and in this case sets
/// // min=2 and max=3:
/// minmax.eval(2);
///
/// // We can now peek at the running output as a single `Option`:
/// println!("Partial minmax is {:?}", minmax.as_ref());
///
/// // And still keep on folding by processing whole iterators:
/// minmax.extend((1..=5));
///
/// // And finally consume the autofolder to get the final output value:
/// let (min, max) = minmax.into_inner();
/// println!("Final min is {}, max is {}", min, max);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MinMaxNum<Item> {
    min: Item,
    max: Item,
}

impl<Item> MinMaxNum<Item> {
    /// Creates a new `MinMaxNum` with the provided initial values.
    pub fn new() -> Self
    where
        Item: num::Bounded,
    {
        Self {
            min: Item::max_value(),
            max: Item::min_value(),
        }
    }
    /// Deconstruct self and return the inner values.
    pub fn into_inner(self) -> (Item, Item) {
        (self.min, self.max)
    }
    /// Returns a reference to the inner values.
    pub fn as_ref(&self) -> (&Item, &Item) {
        (&self.min, &self.max)
    }
    /// Returns a reference to the min inner values, if it exist.
    pub fn min_as_ref(&self) -> &Item {
        &self.min
    }
    /// Returns a reference to the max inner values, if it exist.
    pub fn max_as_ref(&self) -> &Item {
        &self.max
    }
    /// Replaces a current value with the new one if the new one is greater/smaller.
    pub fn reduce(&mut self, item: Item)
    where
        Item: PartialOrd + Clone,
    {
        if item.partial_cmp(&self.min) == Some(std::cmp::Ordering::Less) {
            self.min = item.clone();
        }
        if item.partial_cmp(&self.max) == Some(std::cmp::Ordering::Greater) {
            self.max = item;
        }
    }
    /// Replaces a current value with the one behind the ref if it is greater/smaller.
    pub fn reduce_ref(&mut self, item: &Item)
    where
        Item: PartialOrd + Clone,
    {
        if item.partial_cmp(&self.min) == Some(std::cmp::Ordering::Less) {
            self.min = item.clone();
        }
        if item.partial_cmp(&self.max) == Some(std::cmp::Ordering::Greater) {
            self.max = item.clone();
        }
    }
    /// Alias for [`MinMaxNum::reduce`]
    pub fn eval(&mut self, item: Item)
    where
        Item: PartialOrd + Clone,
    {
        self.reduce(item)
    }
    /// Alias for [`MinMaxNum::reduce_ref`]
    pub fn eval_ref(&mut self, item: &Item)
    where
        Item: PartialOrd + Clone,
    {
        self.reduce_ref(item)
    }
}

impl<Item> Default for MinMaxNum<Item>
where
    Item: num::Bounded,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Item> From<Item> for MinMaxNum<Item>
where
    Item: PartialOrd + num::Bounded + Clone,
{
    fn from(item: Item) -> Self {
        let mut i = Self::new();
        i.eval(item);
        i
    }
}

impl<Item> Extend<Item> for MinMaxNum<Item>
where
    Item: PartialOrd + Clone,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.reduce(i));
    }
}

impl<'a, Item> Extend<&'a Item> for MinMaxNum<Item>
where
    Item: PartialOrd + Clone,
{
    fn extend<It: IntoIterator<Item = &'a Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.reduce_ref(i));
    }
}

impl<Item> std::iter::FromIterator<Item> for MinMaxNum<Item>
where
    Item: PartialOrd + num::Bounded + Clone,
{
    fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
        let mut autofolder = Self::default();
        autofolder.extend(iter);
        autofolder
    }
}

impl<'a, Item> std::iter::FromIterator<&'a Item> for MinMaxNum<Item>
where
    Item: PartialOrd + Clone + num::Bounded,
{
    fn from_iter<It: IntoIterator<Item = &'a Item>>(iter: It) -> Self {
        let mut autofolder = Self::default();
        autofolder.extend(iter);
        autofolder
    }
}
