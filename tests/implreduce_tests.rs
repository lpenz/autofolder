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
    pub type Adder = ImplReduce<Usize>;
    impl ReduceTrait<Usize> for Adder {
        fn reduce(lhs: Usize, rhs: Usize) -> Usize {
            Usize(lhs.0 + rhs.0)
        }
    }
    let mut sum = Adder::from(Usize::default());
    sum.extend((1..=5).map(Usize));
    eprintln!("{:?}", sum);
    assert_eq!(sum.as_ref(), Some(&Usize(15)));
    let sum2 = sum.clone();
    sum.extend((6..=10).map(Usize).rev());
    assert_eq!(sum.into_inner(), Some(Usize(55)));
    assert_eq!(sum2.into_inner(), Some(Usize(15)));
    Ok(())
}

/// Test newtype wrapper without default
#[test]
fn test_newtype_without_default() -> Result<()> {
    #[derive(Default, PartialEq, Eq, Debug)]
    pub struct Usize(usize);
    pub type Adder = ImplReduce<Usize>;
    impl ReduceTrait<Usize> for Adder {
        fn reduce(lhs: Usize, rhs: Usize) -> Usize {
            Usize(lhs.0 + rhs.0)
        }
    }
    let sum = (1..=5).map(Usize).rev().collect::<Adder>();
    assert_eq!(sum.into_inner().unwrap().0, 15);
    Ok(())
}

/// Test vector of Strings, neither impl Copy
#[test]
fn test_newtype_vec() -> Result<()> {
    #[derive(PartialEq, Eq, Debug)]
    pub struct MyString(String);
    pub type StringMax = ImplReduce<MyString>;
    autofolder_impl_reducetrait!(|lhs, rhs| -> MyString {
        if lhs.0.len() > rhs.0.len() {
            lhs
        } else {
            rhs
        }
    });
    let mut autofolder = StringMax::default();
    autofolder.extend((6..=10).map(|i| MyString(format!("{}", i))).rev());
    assert_eq!(autofolder.as_ref().clone().unwrap().0, "10");
    assert_eq!(autofolder.into_inner().unwrap().0, "10");
    Ok(())
}

/// Test empty vector
#[test]
fn test_empty() -> Result<()> {
    #[derive(Default, PartialEq, Eq, Debug)]
    pub struct Usize(usize);
    pub type Adder = ImplReduce<Usize>;
    impl ReduceTrait<Usize> for Adder {
        fn reduce(lhs: Usize, rhs: Usize) -> Usize {
            Usize(lhs.0 + rhs.0)
        }
    }
    let sum = vec![].into_iter().collect::<Adder>();
    assert_eq!(sum.into_inner(), None);
    Ok(())
}
