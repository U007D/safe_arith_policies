use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::Add,
};

use safe_arith_traits::ICheckedOps;

use crate::consts::*;

// #[derive(Clo>ne, Copy, Debug, PartialEq, PartialOrd)] implemented conditionally below.
// Note even for `T: Eq`, `(T, true) != (T, true)` since the overflow bit does not track # of wraps.
pub struct CheckedPolicy<T>(pub Option<T>);

impl<T: ICheckedOps<T>> Add for CheckedPolicy<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.0
            .and_then(|lhs_value| rhs.0.and_then(|rhs_value| lhs_value.checked_add(rhs_value)))
            .into()
    }
}

impl<T: ICheckedOps<T>> Add<T> for CheckedPolicy<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self.0.and_then(|lhs_value| lhs_value.checked_add(rhs)).into()
    }
}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T: Clone> Clone for CheckedPolicy<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: Copy> Copy for CheckedPolicy<T> {}

impl<T: Debug> Debug for CheckedPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CheckedPolicy").field(&self.0).finish()
    }
}

impl<T: Display> Display for CheckedPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref() {
            Some(value) => write!(f, "CheckedPolicy({value})"),
            None => write!(f, "({})", msg::NA),
        }
    }
}

impl<T> From<Option<T>> for CheckedPolicy<T> {
    fn from(value_opt: Option<T>) -> Self { Self(value_opt) }
}

impl<T: PartialEq> PartialEq for CheckedPolicy<T> {
    fn eq(&self, rhs: &Self) -> bool {
        match (&self.0, &rhs.0) {
            (Some(lhs_value), Some(rhs_value)) if lhs_value == rhs_value => true,
            _ => false,
        }
    }
}

impl<T: PartialOrd> PartialOrd for CheckedPolicy<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (&self.0, &rhs.0) {
            (Some(lhs_value), Some(rhs_value)) => lhs_value.partial_cmp(rhs_value),
            _ => None,
        }
    }
}
