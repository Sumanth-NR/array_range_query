use crate::lazy_seg_tree::{LazySegTree, LazySegTreeSpec};
use num_traits::ConstZero;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

pub struct LazySegTreeAddMaxSpec<T>(PhantomData<T>);

impl<T> LazySegTreeSpec for LazySegTreeAddMaxSpec<T>
where
    T: Clone + Add<Output = T> + Mul<usize, Output = T> + ConstZero + Ord,
{
    type T = T;
    type U = T;

    const ID: Self::T = <T as ConstZero>::ZERO;

    fn op_on_data(d1: &Self::T, d2: &Self::T) -> Self::T {
        d1.clone() + d2.clone()
    }

    fn op_on_update(u1: &Self::U, u2: &Self::U) -> Self::U {
        u1.clone() + u2.clone()
    }

    #[allow(unused_variables)]
    fn op_update_on_data(u: &Self::U, d: &Self::T, size: usize) -> Self::T {
        u.clone().mul(size)
    }
}

pub type LazySegTreeAddMax<T> = LazySegTree<LazySegTreeAddMaxSpec<T>>;
