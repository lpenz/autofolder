// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::iter;
use std::marker;
use std::ops;

/// `Adder` type that folds values by using `+=`
///
/// This is a wrapper type that implements `FromIterator` and
/// `Extend`, and folds incoming items by using `+=`, which is
/// implemented by the [`std::ops::AddAssign`] trait.
///
/// Note: to use `FromIterator` (via `collect`) the `Inner` type must
/// implement `Default`.
#[derive(Debug, Default)]
pub struct Adder<Inner, Item>(pub Inner, marker::PhantomData<Item>);

impl<Inner, Item> Adder<Inner, Item> {
    /// Deconstruct self and return the inner value.
    pub fn into_inner(self) -> Inner {
        self.0
    }
    /// Fold value into self.
    pub fn fold(&mut self, item: Item)
    where
        Inner: ops::AddAssign<Item>,
    {
        self.0 += item;
    }
}

impl<Inner, Item> From<Inner> for Adder<Inner, Item> {
    fn from(inner: Inner) -> Self {
        Self(inner, marker::PhantomData)
    }
}

impl<Inner, Item> ops::Deref for Adder<Inner, Item> {
    type Target = Inner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Inner, Item> ops::AddAssign<Item> for Adder<Inner, Item>
where
    Inner: ops::AddAssign<Item>,
{
    fn add_assign(&mut self, other: Item) {
        self.0 += other;
    }
}

impl<Inner, Item> iter::FromIterator<Item> for Adder<Inner, Item>
where
    Inner: Default,
    Inner: ops::AddAssign<Item>,
{
    fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
        let mut autofolder = Adder::<Inner, Item>::from(Inner::default());
        iter.into_iter().for_each(|i| autofolder.fold(i));
        autofolder
    }
}

impl<Inner, Item> Extend<Item> for Adder<Inner, Item>
where
    Inner: ops::AddAssign<Item>,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.fold(i));
    }
}
