use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug)]
pub struct RcRef<T> {
    value: Rc<RefCell<T>>,
}

#[derive(Debug)]
pub struct WeakRef<T> {
    value: Weak<RefCell<T>>
}

impl<T> RcRef<T> {
    pub fn new(value: T) -> RcRef<T> {
        RcRef { value: Rc::new(RefCell::new(value)) }
    }

    pub fn to_weak(&self) -> WeakRef<T> {
        WeakRef { value: Rc::downgrade(&self.value) }
    }
}

impl<T> WeakRef<T> {
    pub fn upgrade(&self) -> Option<RcRef<T>> {
        if let Some(rc_ref) = self.value.upgrade() {
            return Some(RcRef { value: rc_ref });
        }
        None
    }
}

impl<T> Deref for RcRef<T> {
    type Target = RefCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod private_api_tests {
    extern crate spectral;

    use self::spectral::prelude::*;
    use super::*;

    #[test]
    fn rcref_can_produce_weak_refs() {
        let src = RcRef::new(10);
        let dest1 = src.to_weak();
        let dest2 = src.to_weak();

        assert_that(& *src.borrow()).is_equal_to(&10);

        {
            let dest1_ref = dest1.upgrade().unwrap();
            assert_that(& *dest1_ref.borrow()).is_equal_to(&10);

            let dest2_ref = dest2.upgrade().unwrap();
            assert_that(& *dest2_ref.borrow()).is_equal_to(&10);
        }

        *src.borrow_mut() = 20;

        {
            let dest1_ref = dest1.upgrade().unwrap();
            assert_that(& *dest1_ref.borrow()).is_equal_to(&20);

            let dest2_ref = dest2.upgrade().unwrap();
            assert_that(& *dest2_ref.borrow()).is_equal_to(&20);
        }
    }

    #[test]
    fn weak_refs_become_none_when_source_drops() {
        let src = RcRef::new(10);

        let dest = src.to_weak();
        assert_that(& dest.upgrade()).is_some();

        drop(src);
        assert_that(& dest.upgrade()).is_none();
    }
}