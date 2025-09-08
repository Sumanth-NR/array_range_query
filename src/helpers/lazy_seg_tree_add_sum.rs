use crate::lazy_seg_tree::{LazySegTree, LazySegTreeSpec};
use num_traits::ConstZero;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

pub struct LazySegTreeAddSumSpec<T, U>(PhantomData<T>, PhantomData<U>);

impl<T, U> LazySegTreeSpec for LazySegTreeAddSumSpec<T, U>
where
    T: Clone + Add<Output = T> + Add<U, Output = T> + ConstZero + Ord,
    U: Clone + Add<Output = U> + Mul<usize, Output = U>,
{
    type T = T;
    type U = U;

    const ID: Self::T = <T as ConstZero>::ZERO;

    fn op_on_data(d1: &Self::T, d2: &Self::T) -> Self::T {
        d1.clone() + d2.clone()
    }

    fn op_on_update(u1: &Self::U, u2: &Self::U) -> Self::U {
        u1.clone() + u2.clone()
    }

    fn op_update_on_data(u: &Self::U, d: &Self::T, size: usize) -> Self::T {
        d.clone() + u.clone().mul(size)
    }
}

pub type LazySegTreeAddSum<T, U = T> = LazySegTree<LazySegTreeAddSumSpec<T, U>>;
