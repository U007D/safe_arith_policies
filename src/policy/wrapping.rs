// TODO: Implement traits below in `core::num::Wrapping` someday
#[cfg(feature = "arithmetic_overflow_wrapping_policy")]
pub use core::num::Wrapping;

#[cfg(not(feature = "arithmetic_overflow_wrapping_policy"))]
use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::Add,
};

use arith_traits::IWrappingOps;

// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)] implemented conditionally below.
pub struct WrappingPolicy<T>(pub T);

impl<T: IWrappingOps<T>> Add for WrappingPolicy<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { self.0.wrapping_add(rhs.0).into() }
}

impl<T: IWrappingOps<T>> Add<T> for WrappingPolicy<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output { self.0.wrapping_add(rhs).into() }
}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T: Clone> Clone for WrappingPolicy<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: Copy> Copy for WrappingPolicy<T> {}

impl<T: Debug> Debug for WrappingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Wrapping").field(&self.0).finish()
    }
}

impl<T: Display> Display for WrappingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({})", self.0) }
}

impl<T> From<T> for WrappingPolicy<T> {
    fn from(value: T) -> Self { Self(value) }
}

impl<T: PartialEq> PartialEq for WrappingPolicy<T> {
    fn eq(&self, rhs: &Self) -> bool { self.0.eq(&rhs.0) }
}

impl<T: PartialOrd> PartialOrd for WrappingPolicy<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> { self.0.partial_cmp(&rhs.0) }
}
