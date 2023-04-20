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
/// let max = max.into_inner().unwrap();
/// println!("Final max is {}", max);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Max<Item> {
    item: Option<Item>,
}

/// The `Min` type uses the [`std::cmp::PartialOrd`] trait to contain only the smallest iterated
/// value.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create an autofolder that collects the min usize.
/// let mut min = Min::<usize>::default();
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
/// let min = min.into_inner().unwrap();
/// println!("Final min is {}", min);
/// ```
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
            /// Replaces the current value with the new one if the new one is greater/smaller.
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
            /// Replaces the current value with the one behing the ref if it is greater/smaller.
            ///
            /// This function requires the `Clone` trait, but uses it only if necessary.
            pub fn reduce_ref(&mut self, item: &Item)
            where
                Item: PartialOrd + Clone,
            {
                if self.item.is_none()
                    || self
                        .item
                        .as_ref()
                        .map(|i| item.partial_cmp(i) == Some($cmpval))
                        == Some(true)
                {
                    self.item = Some(item.clone());
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

        impl<'a, Item> Extend<&'a Item> for $name<Item>
        where
            Item: PartialOrd + Clone,
        {
            fn extend<It: IntoIterator<Item = &'a Item>>(&mut self, iter: It) {
                iter.into_iter().for_each(|i| self.reduce_ref(i));
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

        impl<'a, Item> std::iter::FromIterator<&'a Item> for $name<Item>
        where
            Item: PartialOrd + Clone,
        {
            fn from_iter<It: IntoIterator<Item = &'a Item>>(iter: It) -> Self {
                let mut autofolder = Self::default();
                autofolder.extend(iter);
                autofolder
            }
        }
    };
}

impl_minmax!(Max, std::cmp::Ordering::Greater);
impl_minmax!(Min, std::cmp::Ordering::Less);

/// The `MinMax` type uses the [`std::cmp::PartialOrd`] trait to
/// contain both the smallest and largest iterated values.
///
/// Example:
/// ```
/// use autofolder::*;
///
/// // Create an autofolder that collects the min+max usize.
/// let mut minmax = MinMax::<usize>::default();
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
/// let (min, max) = minmax.to_inner().unwrap();
/// println!("Final min is {}, max is {}", min, max);
/// ```
#[derive(Debug, Copy, Clone, Default)]
pub enum MinMax<Item> {
    /// Empty; no item evaluated.
    #[default]
    None,
    /// Single item evaluated.
    Single(Item),
    /// Two or more items evaluated - min and max values.
    Both(Item, Item),
}

impl<Item> MinMax<Item> {
    /// Creates a new `MinMax` with the provided initial values.
    pub fn new(initial: Item) -> Self {
        Self::Single(initial)
    }
    /// Deconstruct self and return the inner values that were found.
    ///
    /// This function returns `max` as `None` if we are not holding
    /// two values.
    pub fn to_inner(self) -> Option<(Item, Item)>
    where
        Item: Clone,
    {
        match self {
            Self::None => None,
            Self::Single(item) => Some((item.clone(), item)),
            Self::Both(min, max) => Some((min, max)),
        }
    }
    /// Returns a reference to the inner values, if they exist.
    ///
    /// If we are not holding two values, this function returns `max`
    /// as `None`.
    pub fn as_ref(&self) -> Option<(&Item, &Item)> {
        match self {
            Self::None => None,
            Self::Single(item) => Some((&item, &item)),
            Self::Both(min, max) => Some((&min, &max)),
        }
    }
    /// Returns a reference to the min inner values, if it exist.
    pub fn min_as_ref(&self) -> Option<&Item> {
        match self {
            Self::None => None,
            Self::Single(item) => Some(&item),
            Self::Both(min, _) => Some(&min),
        }
    }
    /// Returns a reference to the max inner values, if it exist.
    pub fn max_as_ref(&self) -> Option<&Item> {
        match self {
            Self::None => None,
            Self::Single(item) => Some(&item),
            Self::Both(_, max) => Some(&max),
        }
    }
    /// Replaces a current value with the new one if the new one is greater/smaller.
    ///
    /// When we have a single value, `min` is always filled up first,
    /// and then swapped with `max` if necessary.
    pub fn reduce(&mut self, item: Item)
    where
        Item: PartialOrd,
    {
        let old = std::mem::take(self);
        *self = match old {
            Self::None => Self::Single(item),
            Self::Single(olditem) => {
                if item.partial_cmp(&olditem) == Some(std::cmp::Ordering::Less) {
                    Self::Both(item, olditem)
                } else if item.partial_cmp(&olditem) == Some(std::cmp::Ordering::Greater) {
                    Self::Both(olditem, item)
                } else {
                    Self::Single(olditem)
                }
            }
            Self::Both(oldmin, oldmax) => {
                if item.partial_cmp(&oldmin) == Some(std::cmp::Ordering::Less) {
                    Self::Both(item, oldmax)
                } else if item.partial_cmp(&oldmax) == Some(std::cmp::Ordering::Greater) {
                    Self::Both(oldmin, item)
                } else {
                    Self::Both(oldmin, oldmax)
                }
            }
        };
    }
    /// Replaces a current value with the one behind the ref if it is greater/smaller.
    ///
    /// When we have a single value, `min` is always filled up first,
    /// and then swapped with `max` if necessary.
    ///
    /// This function requires the `Clone` trait, but uses it only if necessary.
    pub fn reduce_ref(&mut self, item: &Item)
    where
        Item: PartialOrd + Clone,
    {
        let old = std::mem::take(self);
        *self = match old {
            Self::None => Self::Single(item.clone()),
            Self::Single(olditem) => {
                if item.partial_cmp(&olditem) == Some(std::cmp::Ordering::Less) {
                    Self::Both(item.clone(), olditem)
                } else if item.partial_cmp(&olditem) == Some(std::cmp::Ordering::Greater) {
                    Self::Both(olditem, item.clone())
                } else {
                    Self::Single(olditem)
                }
            }
            Self::Both(oldmin, oldmax) => {
                if item.partial_cmp(&oldmin) == Some(std::cmp::Ordering::Less) {
                    Self::Both(item.clone(), oldmax)
                } else if item.partial_cmp(&oldmax) == Some(std::cmp::Ordering::Greater) {
                    Self::Both(oldmin, item.clone())
                } else {
                    Self::Both(oldmin, oldmax)
                }
            }
        };
    }
    /// Alias for [`MinMax::reduce`]
    pub fn eval(&mut self, item: Item)
    where
        Item: PartialOrd,
    {
        self.reduce(item)
    }
    /// Alias for [`MinMax::reduce_ref`]
    pub fn eval_ref(&mut self, item: &Item)
    where
        Item: PartialOrd + Clone,
    {
        self.reduce_ref(item)
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

impl<'a, Item> Extend<&'a Item> for MinMax<Item>
where
    Item: PartialOrd + Clone,
{
    fn extend<It: IntoIterator<Item = &'a Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.reduce_ref(i));
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

impl<'a, Item> std::iter::FromIterator<&'a Item> for MinMax<Item>
where
    Item: PartialOrd + Clone,
{
    fn from_iter<It: IntoIterator<Item = &'a Item>>(iter: It) -> Self {
        let mut autofolder = Self::default();
        autofolder.extend(iter);
        autofolder
    }
}
