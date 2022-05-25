// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use autofolder::*;

use anyhow::Result;

/// Test builtin type
#[test]
fn test_builtin_sum_usize() -> Result<()> {
    let mut sum = DynFolder::<usize, u16, _>::new(0_usize, usize_add_u16);
    sum.fold(10);
    assert_eq!(*sum.as_ref(), 10);
    let sum2 = sum.clone();
    sum.extend((1..=5).rev());
    assert_eq!(sum.into_inner(), 25);
    eprintln!("{:?}", sum2);
    assert_eq!(sum2.into_inner(), 10);
    Ok(())
}

fn usize_add_u16(a: usize, i: u16) -> usize {
    a + i as usize
}

/// Test newtype wrapper
#[test]
fn test_newtype_with_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, Debug)]
    struct Usize(usize);
    let f = |a: Usize, i| Usize(a.0 + i);
    let mut sum = DynFolder::<Usize, usize, _>::new(Usize::default(), f);
    sum.extend(1..=5);
    assert_eq!(sum.as_ref(), &Usize(15));
    sum.extend((6..=10).rev());
    assert_eq!(sum.into_inner(), Usize(55));
    Ok(())
}

/// Test newtype wrapper without default
#[test]
fn test_newtype_without_default() -> Result<()> {
    #[derive(Debug)]
    struct Usize(pub usize);
    let f = |a: Usize, i| Usize(a.0 + i);
    let mut sum = DynFolder::<Usize, usize, _>::new(Usize(0), f);
    sum.extend((1..=5).rev());
    assert_eq!(sum.into_inner().0, 15);
    Ok(())
}

/// Test vector of Strings, neither impl Copy
#[test]
fn test_newtype_vec() -> Result<()> {
    let mut autofolder = DynFolder::<Vec<String>, String, _>::new(vec![], folder);
    let f = |v| format!("{}", v);
    autofolder.extend((1..=5).map(f));
    assert_eq!(autofolder.as_ref().clone(), vec!["1", "2", "3", "4", "5"]);
    autofolder.extend((6..10).map(f).rev());
    assert_eq!(
        autofolder.into_inner(),
        vec!["1", "2", "3", "4", "5", "9", "8", "7", "6"]
    );
    Ok(())
}

fn folder(mut inner: Vec<String>, item: String) -> Vec<String> {
    inner.push(item);
    inner
}
