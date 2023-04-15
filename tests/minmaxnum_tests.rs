// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(feature = "num")]
mod minmaxnum_tests {
    use autofolder::*;

    use anyhow::Result;

    /// Test extend, collect for MaxNum
    #[test]
    fn test_maxnum() -> Result<()> {
        let mut maxnum = MaxNum::<usize>::default();
        assert_eq!(maxnum.into_inner(), 0);
        maxnum.eval_ref(&3);
        assert_eq!(maxnum.into_inner(), 3);
        maxnum.extend(1..=5);
        assert_eq!(maxnum.as_ref(), &5);
        let clone = maxnum.clone();
        maxnum.extend((6..=10).rev());
        assert_eq!(maxnum.into_inner(), 10);
        assert_eq!(clone.into_inner(), 5);
        let collect = (7..=9).rev().into_iter().collect::<MaxNum<_>>();
        assert_eq!(collect.into_inner(), 9);
        let singleton = (2..=2).rev().into_iter().collect::<MaxNum<usize>>();
        assert_eq!(singleton.into_inner(), 2);
        Ok(())
    }

    /// Test extend, collect for MinNum
    #[test]
    fn test_minnum() -> Result<()> {
        let mut minnum = MinNum::<usize>::default();
        assert_eq!(minnum.into_inner(), usize::MAX);
        minnum.eval_ref(&300);
        assert_eq!(minnum.as_ref(), &300);
        minnum.extend(10..=50);
        assert_eq!(minnum.as_ref(), &10);
        let clone = minnum.clone();
        minnum.extend((6..=10).rev());
        assert_eq!(minnum.into_inner(), 6);
        assert_eq!(clone.into_inner(), 10);
        let collect = (2..=4).rev().into_iter().collect::<MinNum<_>>();
        assert_eq!(collect.into_inner(), 2);
        let singleton = (2..=2).rev().into_iter().collect::<MinNum<usize>>();
        assert_eq!(singleton.into_inner(), 2);
        Ok(())
    }

    /// Test extend, collect for MinMaxNum
    #[test]
    fn test_minmaxnum() -> Result<()> {
        let mut minmaxnum = MinMaxNum::<usize>::default();
        assert_eq!(minmaxnum.into_inner(), (usize::MAX, 0));
        minmaxnum.eval_ref(&300);
        assert_eq!(minmaxnum.as_ref(), (&300, &300));
        minmaxnum.extend(290..=310);
        assert_eq!(minmaxnum.as_ref(), (&290, &310));
        let clone = minmaxnum.clone();
        minmaxnum.extend((280..=320).rev());
        assert_eq!(minmaxnum.into_inner(), (280, 320));
        assert_eq!(clone.into_inner(), (290, 310));
        let collect = (2..=4).rev().into_iter().collect::<MinMaxNum<usize>>();
        assert_eq!(collect.into_inner(), (2, 4));
        let singleton = (2..=2).rev().into_iter().collect::<MinMaxNum<usize>>();
        assert_eq!(singleton.into_inner(), (2, 2));
        Ok(())
    }
}
