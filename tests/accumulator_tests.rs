// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use autofolder::*;

use anyhow::Result;

/* Basic tests */

enum MarkerSum {}

#[derive(Default, PartialEq, Eq, Debug)]
struct UsizeWrapper(usize);
type UsizeSum = Accumulator<UsizeWrapper, MarkerSum>;
impl Accumulable<UsizeWrapper, u16> for UsizeSum {
    fn fold(accum: UsizeWrapper, item: u16) -> UsizeWrapper {
        UsizeWrapper(accum.0 + item as usize)
    }
}

#[test]
fn test_sum() -> Result<()> {
    let mut sum: Accumulator<UsizeWrapper, MarkerSum> = (1_u16..=5_u16).collect();
    assert_eq!(*sum, UsizeWrapper(15));
    sum.extend((6_u16..=10_u16).rev());
    assert_eq!(*sum, UsizeWrapper(55));
    Ok(())
}

/* No-default test: can't use collect */

#[derive(PartialEq, Eq, Debug)]
struct UsizeWrapperNoDefault(usize);
type UsizeSumNoDefault = Accumulator<UsizeWrapperNoDefault, MarkerSum>;
impl_accumulable!(UsizeSumNoDefault, |a: UsizeWrapperNoDefault, i: u16| {
    UsizeWrapperNoDefault(a.0 + i as usize)
});

#[test]
fn test_sum_nodefault() -> Result<()> {
    let mut sum = Accumulator::<UsizeWrapperNoDefault, MarkerSum>::from(UsizeWrapperNoDefault(0));
    assert_eq!(*sum, UsizeWrapperNoDefault(0));
    sum.extend((1_u16..=10_u16).rev());
    assert_eq!(*sum, UsizeWrapperNoDefault(55));
    Ok(())
}
