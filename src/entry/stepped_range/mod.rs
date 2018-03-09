extern crate num;

use std::fmt::Debug;
use std::ops::Add;

use self::num::{Integer, Zero};

pub struct SteppedRange<T>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: Clone,
    T: Debug,
    T: Integer,
{
    pub start: T,
    pub end: T,
    // TODO: Use NonZero trait when stable
    pub step: T,
}

impl<T> SteppedRange<T>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: Clone,
    T: Debug,
    T: Integer,
{
    fn new(start: T, end: T, step: T) -> SteppedRange<T> {
        assert_ne!(step, T::zero());
        SteppedRange { start, end, step }
    }
}

impl<T> Iterator for SteppedRange<T>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: Clone,
    T: Debug,
    T: Integer,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start < self.end {
            let current = self.start.clone();
            self.start = &self.start + &self.step;
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SteppedRange;

    #[test]
    fn should_return_none_for_empty_range() {
        let mut range = SteppedRange::new(0, 0, 1);
        assert_eq!(range.next(), None);
    }

    #[test]
    fn should_return_one_item_for_trivial_range() {
        let mut range = SteppedRange::new(0, 1, 1);
        assert_eq!(range.next(), Some(0));
        assert_eq!(range.next(), None);
    }

    #[test]
    #[should_panic]
    fn should_fail_with_step_of_zero() {
        let mut range = SteppedRange::new(0, 0, 0);
        range.next();
    }
}
