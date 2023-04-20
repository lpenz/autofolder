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
    assert_eq!(minmax.as_ref(), Some((&0, &3)));
    minmax.extend(1..=5);
    assert_eq!(minmax.as_ref(), Some((&0, &5)));
    let clone = minmax.clone();
    minmax.extend((6..=10).rev());
    assert_eq!(minmax.to_inner(), Some((0, 10)));
    assert_eq!(clone.to_inner(), Some((0, 5)));
    let collect = (7..=9).rev().into_iter().collect::<MinMax<_>>();
    assert_eq!(collect.to_inner(), Some((7, 9)));
    let vec = (2..=4).rev().collect::<Vec<_>>();
    let collect_ref = vec.iter().collect::<MinMax<usize>>();
    assert_eq!(collect_ref.to_inner(), Some((2, 4)));
    Ok(())
}

/// Test type without clone
#[test]
fn test_type_without_clone() -> Result<()> {
    let mut minmax = MinMax::<Strnum>::default();
    assert_eq!(minmax.min_as_ref(), None);
    assert_eq!(minmax.max_as_ref(), None);
    assert_eq!(minmax.as_ref(), None);
    minmax.eval(Strnum::from(5));
    assert_eq!(minmax.min_as_ref(), Some(&Strnum::from(5)));
    assert_eq!(minmax.max_as_ref(), Some(&Strnum::from(5)));
    assert_eq!(minmax.as_ref(), Some((&Strnum::from(5), &Strnum::from(5))));
    minmax.eval(Strnum::from("NaN"));
    assert_eq!(minmax.min_as_ref(), Some(&Strnum::from(5)));
    assert_eq!(minmax.max_as_ref(), Some(&Strnum::from(5)));
    assert_eq!(minmax.as_ref(), Some((&Strnum::from(5), &Strnum::from(5))));
    let mut minmax = (1..=2).map(Strnum::from).collect::<MinMax<_>>();
    minmax.extend((4..=5).map(Strnum::from));
    assert_eq!(minmax.min_as_ref(), Some(&Strnum::from(1)));
    assert_eq!(minmax.max_as_ref(), Some(&Strnum::from(5)));
    assert_eq!(minmax.as_ref(), Some((&Strnum::from(1), &Strnum::from(5))));
    minmax.extend(
        (6..=10)
            .map(Strnum::from)
            .rev()
            .collect::<Vec<_>>()
            .into_iter(),
    );
    assert_eq!(minmax.as_ref(), Some((&Strnum::from(1), &Strnum::from(10))));
    Ok(())
}

/// Test type with clone
#[test]
fn test_type_with_clone() -> Result<()> {
    let mut minmax = MinMax::<StrnumClone>::default();
    assert_eq!(minmax.clone().to_inner(), None);
    minmax.eval(StrnumClone::from(3));
    assert_eq!(
        minmax.clone().to_inner(),
        Some((StrnumClone::from(3), StrnumClone::from(3)))
    );
    minmax.eval_ref(&StrnumClone::from("NaN"));
    assert_eq!(
        minmax.clone().to_inner(),
        Some((StrnumClone::from(3), StrnumClone::from(3)))
    );
    let vec = (1..=5).map(StrnumClone::from).collect::<Vec<_>>();
    minmax.extend(vec.iter());
    assert_eq!(
        minmax.to_inner(),
        Some((StrnumClone::from(1), StrnumClone::from(5)))
    );
    Ok(())
}
