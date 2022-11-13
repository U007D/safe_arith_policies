use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::Add,
};

use arith_traits::IOverflowingOps;

// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)] implemented conditionally below.
// Note even for `T: Eq`, `(T, true) != (T, true)` since the overflow bit does not track # of wraps.
pub struct OverflowingPolicy<T>(pub T, pub bool);

impl<T: IOverflowingOps<T>> Add for OverflowingPolicy<T> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
        let (value, mut overflow) = self.0.overflowing_add(rhs.0);
        overflow |= self.1 | rhs.1;
        (value, overflow).into()
    }
}

impl<T: IOverflowingOps<T>> Add<T> for OverflowingPolicy<T> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: T) -> Self::Output {
        let (value, mut overflow) = self.0.overflowing_add(rhs);
        overflow |= self.1;
        (value, overflow).into()
    }
}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T: Clone> Clone for OverflowingPolicy<T> {
    fn clone(&self) -> Self { Self(self.0.clone(), self.1) }
}

impl<T: Copy> Copy for OverflowingPolicy<T> {}

impl<T: Debug> Debug for OverflowingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Overflowing").field(&self.0).field(&self.1).finish()
    }
}

impl<T: Display> Display for OverflowingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T> From<(T, bool)> for OverflowingPolicy<T> {
    fn from((value, overflow): (T, bool)) -> Self { Self(value, overflow) }
}

impl<T: PartialEq> PartialEq for OverflowingPolicy<T> {
    fn eq(&self, rhs: &Self) -> bool { self.0.eq(&rhs.0) && self.1 == rhs.1 }
}

impl<T: PartialOrd> PartialOrd for OverflowingPolicy<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (self.1, rhs.1, self.0.partial_cmp(&rhs.0)) {
            (false, false, value_ord) => value_ord,
            (true, false, Some(_)) => Some(Ordering::Greater),
            (false, true, Some(_)) => Some(Ordering::Less),
            _ => None,
        }
    }
}
