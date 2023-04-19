// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use autofolder::*;

use anyhow::Result;

mod strnum;
use strnum::*;

/// Test extend, collect
#[test]
fn test_minmax() -> Result<()> {
    let mut minmax = MinMax::<usize>::from(0);
    minmax.eval_ref(&3);
    assert_eq!(minmax.as_refs(), Some((&0, &3)));
    minmax.extend(1..=5);
    assert_eq!(minmax.as_ref(), (Some(&0), Some(&5)));
    let clone = minmax.clone();
    minmax.extend((6..=10).rev());
    assert_eq!(minmax.into_inner(), (Some(0), Some(10)));
    assert_eq!(clone.into_inner(), (Some(0), Some(5)));
    let collect = (7..=9).rev().into_iter().collect::<MinMax<_>>();
    assert_eq!(collect.into_inner(), (Some(7), Some(9)));
    let vec = (2..=4).rev().collect::<Vec<_>>();
    let collect_ref = vec.iter().collect::<MinMax<usize>>();
    assert_eq!(collect_ref.into_inner(), (Some(2), Some(4)));
    Ok(())
}

/// Test type with clone
#[test]
fn test_type_with_clone() -> Result<()> {
    let mut minmax = MinMax::<StrnumClone>::from(StrnumClone::from(0));
    minmax.extend((1..=5).map(StrnumClone::from));
    eprintln!("{:?}", minmax);
    assert_eq!(
        minmax.as_ref(),
        (Some(&StrnumClone::from(0)), Some(&StrnumClone::from(5)))
    );
    let sum2 = minmax.clone();
    minmax.extend(
        (6..=10)
            .map(StrnumClone::from)
            .rev()
            .collect::<Vec<_>>()
            .iter(),
    );
    assert_eq!(
        minmax.into_inner(),
        (Some(StrnumClone::from(0)), Some(StrnumClone::from(10)))
    );
    assert_eq!(
        sum2.into_inners(),
        Some((StrnumClone::from(0), StrnumClone::from(5)))
    );
    Ok(())
}

/// Test type without clone
#[test]
fn test_type_without_clone() -> Result<()> {
    let mut autofolder = MinMax::<Strnum>::default();
    autofolder.extend((6..=30).map(Strnum::from).rev());
    assert_eq!(autofolder.min_as_ref().clone().unwrap(), &Strnum::from("6"));
    assert_eq!(
        autofolder.max_as_ref().clone().unwrap(),
        &Strnum::from("30")
    );
    autofolder.extend((950..=1005).map(Strnum::from));
    assert_eq!(autofolder.into_inner().1.unwrap(), Strnum::from("1005"));
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
