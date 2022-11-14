use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::Add,
};

use safe_arith_traits::ISaturatingOps;

// #[derive(Clo>ne, Copy, Debug, PartialEq, PartialOrd)] implemented conditionally below.
// Note even for `T: Eq`, `(T, true) != (T, true)` since the overflow bit does not track # of wraps.
pub struct SaturatingPolicy<T>(pub T);

impl<T: ISaturatingOps<T>> Add for SaturatingPolicy<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { self.0.saturating_add(rhs.0).into() }
}

impl<T: ISaturatingOps<T>> Add<T> for SaturatingPolicy<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output { self.0.saturating_add(rhs).into() }
}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T: Clone> Clone for SaturatingPolicy<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: Copy> Copy for SaturatingPolicy<T> {}

impl<T: Debug> Debug for SaturatingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SaturatingPolicy").field(&self.0).finish()
    }
}

impl<T: Display> Display for SaturatingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SaturatingPolicy({})", self.0)
    }
}

impl<T: Eq> Eq for SaturatingPolicy<T> {}

impl<T> From<T> for SaturatingPolicy<T> {
    fn from(value: T) -> Self { Self(value) }
}

impl<T: Ord> Ord for SaturatingPolicy<T> {
    fn cmp(&self, rhs: &Self) -> Ordering { self.0.cmp(&rhs.0) }
}

impl<T: PartialEq> PartialEq for SaturatingPolicy<T> {
    fn eq(&self, rhs: &Self) -> bool { self.0 == rhs.0 }
}

impl<T: PartialOrd> PartialOrd for SaturatingPolicy<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> { self.0.partial_cmp(&rhs.0) }
}
