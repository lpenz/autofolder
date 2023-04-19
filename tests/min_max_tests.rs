// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use autofolder::*;

use anyhow::Result;

mod strnum;
use strnum::*;

/// Test extend, collect for min
#[test]
fn test_min() -> Result<()> {
    let mut min = Min::<usize>::from(500);
    min.eval_ref(&300);
    assert_eq!(min.as_ref(), Some(&300));
    min.extend((250..=255).by_ref());
    assert_eq!(min.as_ref(), Some(&250));
    let clone = min.clone();
    min.extend((6..=10).rev());
    assert_eq!(min.into_inner(), Some(6));
    assert_eq!(clone.into_inner(), Some(250));
    let collect = (2..=4).rev().into_iter().collect::<Min<_>>();
    assert_eq!(collect.into_inner(), Some(2));
    let vec = (2..=4).rev().collect::<Vec<_>>();
    let collect_ref = vec.iter().collect::<Min<usize>>();
    assert_eq!(collect_ref.into_inner(), Some(2));
    Ok(())
}

/// Test extend, collect for max
#[test]
fn test_max() -> Result<()> {
    let mut max = Max::<usize>::from(0);
    max.eval_ref(&3);
    assert_eq!(max.as_ref(), Some(&3));
    max.extend((1..=5).by_ref());
    assert_eq!(max.as_ref(), Some(&5));
    let clone = max.clone();
    max.extend((6..=10).rev());
    assert_eq!(max.into_inner(), Some(10));
    assert_eq!(clone.into_inner(), Some(5));
    let collect = (7..=9).rev().into_iter().collect::<Max<_>>();
    assert_eq!(collect.into_inner(), Some(9));
    let vec = (2..=4).rev().collect::<Vec<_>>();
    let collect_ref = vec.iter().collect::<Max<usize>>();
    assert_eq!(collect_ref.into_inner(), Some(4));
    Ok(())
}

/// Test wtype with clone
#[test]
fn test_type_with_clone() -> Result<()> {
    let mut max = Max::<StrnumClone>::default();
    max.extend((1..=5).map(StrnumClone::from));
    assert_eq!(max.as_ref(), Some(&StrnumClone::from(5)));
    let sum2 = max.clone();
    max.extend((6..=10).map(StrnumClone::from).rev());
    assert_eq!(max.into_inner(), Some(StrnumClone::from(10)));
    assert_eq!(sum2.into_inner(), Some(StrnumClone::from(5)));
    Ok(())
}

/// Test type without clone
#[test]
fn test_type_without_clone() -> Result<()> {
    let mut autofolder = Max::<Strnum>::default();
    autofolder.extend((6..=30).map(Strnum::from).rev());
    assert_eq!(autofolder.as_ref().clone().unwrap(), &Strnum::from(30));
    autofolder.extend((950..=1005).map(Strnum::from));
    assert_eq!(autofolder.into_inner().unwrap(), Strnum::from("1005"));
    Ok(())
}

/// Test empty vector
#[test]
fn test_empty() -> Result<()> {
    let sum = vec![].into_iter().collect::<Max<Strnum>>();
    assert_eq!(sum.into_inner(), None);
    Ok(())
}
