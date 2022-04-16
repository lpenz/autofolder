// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem;
use std::ops;

/// Core trait of the accumulator folding collection.
///
/// Implementation should implement this trait for the cooresponding
/// [`Accumulator`] collection.
pub trait Accumulable<Accum, Item> {
    /// Returns an accumulator instance with the provided `item` folded in
    ///
    /// For example: in a summing accumulator, this would return `accum + item`
    fn fold(accum: Accum, item: Item) -> Accum;
}

/// Core `Accumulator` wrapper type that implementa `FromIterator` and `Extend`
///
/// This struct binds an inner accumulator type to an `Accumulable`
/// instance by using a `PhantomData` marker.
#[derive(Debug, Default)]
pub struct Accumulator<Accum, Marker>(Accum, PhantomData<Marker>);

impl<Accum, Marker> ops::Deref for Accumulator<Accum, Marker> {
    type Target = Accum;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Accum, Marker> ops::DerefMut for Accumulator<Accum, Marker> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Accum, Marker> From<Accum> for Accumulator<Accum, Marker> {
    fn from(accum: Accum) -> Self {
        Self(accum, PhantomData)
    }
}

impl<Accum, Marker, Item> FromIterator<Item> for Accumulator<Accum, Marker>
where
    Accum: Default,
    Accumulator<Accum, Marker>: Accumulable<Accum, Item>,
{
    fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
        let accum = iter.into_iter().fold(Accum::default(), |a, i| {
            Accumulator::<Accum, Marker>::fold(a, i)
        });
        Self(accum, PhantomData)
    }
}

impl<Accum, Item, Marker> Extend<Item> for Accumulator<Accum, Marker>
where
    Accumulator<Accum, Marker>: Accumulable<Accum, Item>,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        // SAFETY: we replace self.0 with uninitialized memory but
        // then immediately set it to the result of the iteration.
        let init = mem::replace(&mut self.0, unsafe {
            mem::MaybeUninit::zeroed().assume_init()
        });
        self.0 = iter
            .into_iter()
            .fold(init, |a, i| Accumulator::<Accum, Marker>::fold(a, i));
    }
}

/// Helper macro that implements [`Accumulable`] for a given
/// [`Accumulator`] type using the provided function as `fold`.
#[macro_export]
macro_rules! impl_accumulable {
    ($autofolder: ty, | $accum:ident : $accumtype: ty, $item:ident : $itemtype:ty | { $fn: expr }) => {
        impl Accumulable<$accumtype, $itemtype> for $autofolder {
            fn fold($accum: $accumtype, $item: $itemtype) -> $accumtype {
                $fn
            }
        }
    };
}
