// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use autofolder::*;

use anyhow::Result;
use std::cmp::Ordering;

/// Test extend, collect
#[test]
fn test_minmax() -> Result<()> {
    let mut minmax = MinMax::<usize>::from(0);
    minmax.eval_ref(&3);
    assert_eq!(minmax.as_refs(), Some((&0, &3)));
    minmax.extend(1..=5);
    assert_eq!(minmax.as_ref(), (Some(&0), Some(&5)));
    let minmax2 = minmax.clone();
    minmax.extend((6..=10).rev());
    assert_eq!(minmax.into_inner(), (Some(0), Some(10)));
    assert_eq!(minmax2.into_inner(), (Some(0), Some(5)));
    let minmax = (7..=9).rev().into_iter().collect::<MinMax<_>>();
    assert_eq!(minmax.into_inner(), (Some(7), Some(9)));
    Ok(())
}

/// Test newtype wrapper
#[test]
fn test_newtype_with_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, PartialOrd, Debug, Clone)]
    pub struct Usize(usize);
    let mut minmax = MinMax::<Usize>::from(Usize::default());
    minmax.extend((1..=5).map(Usize));
    eprintln!("{:?}", minmax);
    assert_eq!(minmax.as_ref(), (Some(&Usize(0)), Some(&Usize(5))));
    let sum2 = minmax.clone();
    minmax.extend((6..=10).map(Usize).rev().collect::<Vec<_>>().iter());
    assert_eq!(minmax.into_inner(), (Some(Usize(0)), Some(Usize(10))));
    assert_eq!(sum2.into_inners(), Some((Usize(0), Usize(5))));
    Ok(())
}

/// Test newtype wrapper without default
#[test]
fn test_newtype_without_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, PartialOrd, Debug)]
    pub struct Usize(usize);
    let minmax = (1..=5).map(Usize).rev().collect::<MinMax<Usize>>();
    assert_eq!(minmax.into_inner_unwrap(), (Usize(1), Usize(5)));
    Ok(())
}

/// Test vector of Strings, neither impl Copy
#[test]
fn test_newtype_vec() -> Result<()> {
    #[derive(PartialEq, Eq, Debug)]
    pub struct MyString(String);
    impl PartialOrd for MyString {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.len().partial_cmp(&other.0.len())
        }
    }
    let mut autofolder = MinMax::<MyString>::default();
    autofolder.extend((6..=30).map(|i| MyString(format!("{}", i))).rev());
    assert_eq!(autofolder.min_as_ref().clone().unwrap().0, "9");
    assert_eq!(autofolder.max_as_ref().clone().unwrap().0, "30");
    autofolder.extend((950..=1005).map(|i| MyString(format!("{}", i))));
    assert_eq!(autofolder.into_inner().1.unwrap().0, "1000");
    Ok(())
}

/// Test empty vector
#[test]
fn test_empty() -> Result<()> {
    let minmax = vec![].iter().collect::<MinMax<usize>>();
    assert_eq!(minmax.min_as_ref(), None);
    assert_eq!(minmax.max_as_ref(), None);
    assert_eq!(minmax.clone().into_inner(), (None, None));
    assert_eq!(minmax.into_inners(), None);
    Ok(())
}
