use crate::{SegTree, SegTreeSpec};
use min_max_traits::Min as ConstLowerBound;
use std::marker::PhantomData;

pub struct SegTreeMaxSpec<T>(PhantomData<T>);

impl<T> SegTreeSpec for SegTreeMaxSpec<T>
where
    T: Clone + ConstLowerBound + Ord,
{
    type T = T;
    const ID: Self::T = <T as ConstLowerBound>::MIN;

    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        a.clone().max(b.clone())
    }
}

pub type SegTreeMax<T> = SegTree<SegTreeMaxSpec<T>>;
