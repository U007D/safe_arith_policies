mod op;

use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::Add,
};

use safe_arith_traits::IPanickingOps;

use crate::consts::*;
use op::Op;

// #[derive(Clo>ne, Copy, Debug, PartialEq, PartialOrd)] implemented conditionally below.
// Note even for `T: Eq`, `(T, true) != (T, true)` since the overflow bit does not track # of wraps.
pub struct PanickingPolicy<T>(pub T);

impl<T: IPanickingOps<T>> Add for PanickingPolicy<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.0
            .checked_add(rhs.0)
            .unwrap_or_else(|| panic!("{} {} {}.", msg::OP, Op::Add, msg::PANICKED))
            .into()
    }
}

impl<T: IPanickingOps<T>> Add<T> for PanickingPolicy<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self.0
            .checked_add(rhs)
            .unwrap_or_else(|| panic!("{} {} {}.", msg::OP, Op::Add, msg::PANICKED))
            .into()
    }
}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T: Clone> Clone for PanickingPolicy<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: Copy> Copy for PanickingPolicy<T> {}

impl<T: Debug> Debug for PanickingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PanickingPolicy").field(&self.0).finish()
    }
}

impl<T: Display> Display for PanickingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PanickingPolicy({})", self.0)
    }
}

impl<T: Eq> Eq for PanickingPolicy<T> {}

impl<T> From<T> for PanickingPolicy<T> {
    fn from(value: T) -> Self { Self(value) }
}

impl<T: Ord> Ord for PanickingPolicy<T> {
    fn cmp(&self, rhs: &Self) -> Ordering { self.0.cmp(&rhs.0) }
}

impl<T: PartialEq> PartialEq for PanickingPolicy<T> {
    fn eq(&self, rhs: &Self) -> bool { self.0 == rhs.0 }
}

impl<T: PartialOrd> PartialOrd for PanickingPolicy<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> { self.0.partial_cmp(&rhs.0) }
}
