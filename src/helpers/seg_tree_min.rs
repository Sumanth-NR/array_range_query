use crate::{SegTree, SegTreeSpec};
use min_max_traits::Max as ConstUpperBound;
use std::marker::PhantomData;

pub struct SegTreeMinSpec<T>(PhantomData<T>);

impl<T> SegTreeSpec for SegTreeMinSpec<T>
where
    T: Clone + ConstUpperBound + Ord,
{
    type T = T;
    const ID: Self::T = <T as ConstUpperBound>::MAX;
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        a.clone().min(b.clone())
    }
}

pub type SegTreeMin<T> = SegTree<SegTreeMinSpec<T>>;
