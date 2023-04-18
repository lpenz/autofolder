// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::cmp::Ordering;
use std::convert::TryFrom;

macro_rules! impl_strnum {
    ($name: ident) => {
        impl From<String> for $name {
            fn from(s: String) -> Self {
                $name(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                $name::from(String::from(s))
            }
        }

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                $name::from(format!("{}", value))
            }
        }

        impl TryFrom<$name> for usize {
            type Error = std::num::ParseIntError;
            fn try_from(value: $name) -> Result<Self, Self::Error> {
                value.0.parse::<usize>()
            }
        }

        impl TryFrom<&$name> for usize {
            type Error = std::num::ParseIntError;
            fn try_from(value: &$name) -> Result<Self, Self::Error> {
                value.0.parse::<usize>()
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                let selfnum = usize::try_from(self).ok();
                let othernum = usize::try_from(other).ok();
                Some(selfnum.cmp(&othernum))
            }
        }
    };
}

#[derive(PartialEq, Eq, Debug)]
pub struct Strnum(String);
impl_strnum!(Strnum);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StrnumClone(String);
impl_strnum!(StrnumClone);
