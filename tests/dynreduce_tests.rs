// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use autofolder::*;

use anyhow::Result;

/// Test builtin type
#[test]
fn test_builtin_sum_usize() -> Result<()> {
    let mut sum = DynReduce::<usize, _>::new(usize_add_usize);
    sum.reduce(10);
    assert_eq!(sum.as_ref(), Some(&10));
    let sum2 = sum.clone();
    sum.extend((1..=5).rev());
    assert_eq!(sum.into_inner(), Some(25));
    eprintln!("{:?}", sum2);
    assert_eq!(sum2.into_inner(), Some(10));
    Ok(())
}

fn usize_add_usize(a: usize, i: usize) -> usize {
    a + i
}

/// Test newtype wrapper
#[test]
fn test_newtype_with_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, Debug)]
    struct Usize(usize);
    let f = |a: Usize, i: Usize| Usize(a.0 + i.0);
    let mut sum = DynReduce::<Usize, _>::from(f);
    sum.extend((1..=5).map(Usize));
    assert_eq!(sum.as_ref(), Some(&Usize(15)));
    sum.extend((6..=10).rev().map(Usize));
    assert_eq!(sum.into_inner(), Some(Usize(55)));
    Ok(())
}

/// Test newtype wrapper without default
#[test]
fn test_newtype_without_default() -> Result<()> {
    #[derive(Debug)]
    struct Usize(pub usize);
    let f = |a: Usize, i: Usize| Usize(a.0 + i.0);
    let mut sum = DynReduce::<Usize, _>::new(f);
    sum.extend((1..=5).map(Usize).rev());
    assert_eq!(sum.into_inner().unwrap().0, 15);
    Ok(())
}

/// Test Strings, it doesn't impl Copy
#[test]
fn test_string() -> Result<()> {
    let mut autofolder = DynReduce::<String, _>::new(concat);
    let f = |v| format!("{}", v);
    autofolder.extend((1..=5).map(f));
    assert_eq!(autofolder.as_ref().clone(), Some(&format!("1 2 3 4 5")));
    autofolder.extend((6..10).map(f).rev());
    assert_eq!(autofolder.into_inner().unwrap(), "1 2 3 4 5 9 8 7 6");
    Ok(())
}

fn concat(lhs: String, rhs: String) -> String {
    // Not at all efficient, but shows that we don't have to returns a
    // "changed lhs"
    format!("{} {}", lhs, rhs)
}

/// Test vector of Strings, neither impl Copy
#[test]
fn test_empty_vec() -> Result<()> {
    let mut autofolder = DynReduce::<String, _>::new(concat);
    autofolder.extend(vec![].into_iter());
    assert_eq!(autofolder.into_inner(), None);
    Ok(())
}
