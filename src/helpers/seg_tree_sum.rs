use crate::seg_tree::{SegTree, SegTreeSpec};
use num_traits::ConstZero;
use std::marker::PhantomData;
use std::ops::Add;

pub struct SegTreeSumSpec<T>(PhantomData<T>);

impl<T> SegTreeSpec for SegTreeSumSpec<T>
where
    T: Clone + ConstZero + Add<Output = T>,
{
    type T = T;
    const ID: Self::T = <T as ConstZero>::ZERO;

    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        a.clone() + b.clone()
    }
}

/// Convenience alias: a `SegTree` specialized to perform sums over `T`.
pub type SegTreeSum<T: Clone + ConstZero + Add<Output = T>> = SegTree<SegTreeSumSpec<T>>;
