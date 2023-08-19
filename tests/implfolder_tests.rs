// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use autofolder::*;

use anyhow::Result;

/// Test newtype wrapper
#[test]
fn test_newtype_with_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, Debug, Clone)]
    pub struct Usize(usize);
    pub type Adder = ImplFolder<Usize, u16>;
    impl FolderTrait<Usize, u16> for Adder {
        fn fold(accum: Usize, item: u16) -> Usize {
            Usize(accum.0 + item as usize)
        }
    }
    let mut sum = (1..=5).collect::<Adder>();
    assert_eq!(sum.as_ref(), &Usize(15));
    let sum2 = sum.clone();
    sum.extend((6..=10).rev());
    assert_eq!(sum.into_inner(), Usize(55));
    assert_eq!(sum2.into_inner(), Usize(15));
    Ok(())
}

/// Test newtype wrapper without default
#[test]
fn test_newtype_without_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, Debug)]
    pub struct Usize(usize);
    pub type Adder = ImplFolder<Usize, u16>;
    impl FolderTrait<Usize, u16> for Adder {
        fn fold(accum: Usize, item: u16) -> Usize {
            Usize(accum.0 + item as usize)
        }
    }
    let mut sum = Adder::new(Usize(0));
    sum.extend((1..=5).rev());
    assert_eq!(sum.into_inner().0, 15);
    Ok(())
}

/// Test vector of Strings, neither impl Copy
#[test]
fn test_newtype_vec() -> Result<()> {
    #[derive(PartialEq, Eq, Debug)]
    pub struct VecString(Vec<String>);
    pub type StringJoiner = ImplFolder<VecString, String>;
    autofolder_impl_foldertrait!(|accum: VecString, item: String| {
        accum.0.push(item);
        accum
    });
    let mut autofolder = StringJoiner::from(VecString(vec![]));
    autofolder.extend((6..10).map(|i| format!("{}", i)).rev());
    eprintln!("{:?}", autofolder);
    assert_eq!(autofolder.as_ref().0, vec!["9", "8", "7", "6"]);
    assert_eq!(autofolder.into_inner().0, vec!["9", "8", "7", "6"]);
    Ok(())
}
