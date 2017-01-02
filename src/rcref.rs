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

    pub fn downgrade(&self) -> WeakRef<T> {
        WeakRef { value: Rc::downgrade(&self.value) }
    }

    pub fn peek<F: Fn(&T)>(&self, f: F) {
        f(& *self.value.borrow());
    }
}

impl<T> WeakRef<T> {
    pub fn upgrade(&self) -> Option<RcRef<T>> {
        if let Some(rc_ref) = self.value.upgrade() {
            return Some(RcRef { value: rc_ref });
        }
        None
    }

    pub fn peek<F: Fn(&T)>(&self, f: F) -> bool {
        if let Some(ref rc_ref) = self.value.upgrade() {
            f(& *rc_ref.borrow());
            return true;
        }
        false
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
        let dest1 = src.downgrade();
        let dest2 = src.downgrade();

        assert_that(& *src.borrow()).is_equal_to(&10);
        src.peek(|val| assert_that(val).is_equal_to(&10));

        {
            let result = dest1.peek(|val| assert_that(val).is_equal_to(&10));
            assert_that(&result).is_true();

            let result = dest2.peek(|val| assert_that(val).is_equal_to(&10));
            assert_that(&result).is_true();
        }

        *src.borrow_mut() = 20;

        {
            let result = dest1.peek(|val| assert_that(val).is_equal_to(&20));
            assert_that(&result).is_true();

            let result = dest2.peek(|val| assert_that(val).is_equal_to(&20));
            assert_that(&result).is_true();
        }
    }

    #[test]
    fn weak_refs_become_none_when_source_drops() {
        let src = RcRef::new(10);

        let dest = src.downgrade();
        assert_that(& dest.upgrade()).is_some();

        drop(src);
        assert_that(& dest.upgrade()).is_none();
        assert_that(& dest.peek(|_| {})).is_false();
    }

    fn assert_ten(val: &i32) {
        assert_that(val).is_equal_to(&10);
    }

    #[test]
    fn can_reuse_the_same_method_multiple_times() {
        let src = RcRef::new(10);
        let dest = src.downgrade();

        src.peek(assert_ten);
        let result = dest.peek(assert_ten);
        assert_that(&result).is_true();
    }

}