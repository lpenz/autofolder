// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use autofolder::*;

use anyhow::Result;
use std::cmp::Ordering;

/// Test extend, collect
#[test]
fn test_max() -> Result<()> {
    let mut max = Max::<usize>::from(0);
    max.eval_ref(&3);
    assert_eq!(max.as_ref(), Some(&3));
    max.extend(1..=5);
    assert_eq!(max.as_ref(), Some(&5));
    let max2 = max.clone();
    max.extend((6..=10).rev());
    assert_eq!(max.into_inner(), Some(10));
    assert_eq!(max2.into_inner(), Some(5));
    let max = (7..=9).rev().into_iter().collect::<Max<_>>();
    assert_eq!(max.into_inner(), Some(9));
    Ok(())
}

/// Test newtype wrapper
#[test]
fn test_newtype_with_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, PartialOrd, Debug, Clone)]
    pub struct Usize(usize);
    let mut max = Max::<Usize>::from(Usize::default());
    max.extend((1..=5).map(Usize));
    eprintln!("{:?}", max);
    assert_eq!(max.as_ref(), Some(&Usize(5)));
    let sum2 = max.clone();
    max.extend((6..=10).map(Usize).rev());
    assert_eq!(max.into_inner(), Some(Usize(10)));
    assert_eq!(sum2.into_inner(), Some(Usize(5)));
    Ok(())
}

/// Test newtype wrapper without default
#[test]
fn test_newtype_without_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, PartialOrd, Debug)]
    pub struct Usize(usize);
    let max = (1..=5).map(Usize).rev().collect::<Max<Usize>>();
    assert_eq!(max.into_inner().unwrap().0, 5);
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
    let mut autofolder = Max::<MyString>::default();
    autofolder.extend((6..=30).map(|i| MyString(format!("{}", i))).rev());
    assert_eq!(autofolder.as_ref().clone().unwrap().0, "30");
    autofolder.extend((950..=1005).map(|i| MyString(format!("{}", i))));
    assert_eq!(autofolder.into_inner().unwrap().0, "1000");
    Ok(())
}

/// Test empty vector
#[test]
fn test_empty() -> Result<()> {
    #[derive(Default, PartialEq, Eq, PartialOrd, Debug)]
    pub struct Usize(usize);
    let sum = vec![].into_iter().collect::<Max<Usize>>();
    assert_eq!(sum.into_inner(), None);
    Ok(())
}
