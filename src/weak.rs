//! A module which contains the `WeakList<T>` class. While a useful class in its own right, it is
//! mostly used as an implementation detail for this crate and, as such, is not exposed directly
//! through the `prelude` module.

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

/// A vector of weak references, which will automatically be cleaned up when the values being
/// referenced are dropped.
///
/// # Example
///
/// ```
/// use std::rc::Rc;
/// use pebl::weak::WeakList;
///
/// let mut list = WeakList::<i32>::new();
/// {
///     let item1 = Rc::new(1);
///     let item2 = Rc::new(2);
///     list.push(&item1);
///     list.push(&item2);
///     assert_eq!(2, list.len());
/// }
/// assert_eq!(0, list.len());
/// ```
///
/// To walk over a `WeakList<T>`, upgrade it first:
///
/// ```
/// use std::rc::Rc;
/// use pebl::weak::WeakList;
///
/// let mut list = WeakList::<i32>::new();
/// let item1 = Rc::new(1); list.push(&item1);
/// let item2 = Rc::new(2); list.push(&item2);
/// let mut sum = 0;
/// for val in list.upgrade() {
///     sum += *val;
/// }
/// assert_eq!(3, sum);
/// ```
pub struct WeakList<T: ? Sized> {
    // Outer RefCell so that we can clean the vec in notify_invalidated
    items: RefCell<Vec<Weak<T>>>,
}

impl<T: ? Sized> WeakList<T> {
    /// Construct a new, empty list.
    pub fn new() -> Self {
        WeakList::<T>::with_capacity(0)
    }

    /// Construct a new list with initial capacity, similar to `Vec<T>.with_capacity()`.
    pub fn with_capacity(capacity: usize) -> Self {
        WeakList { items: RefCell::new(Vec::with_capacity(capacity)) }
    }

    /// Construct a new list, populated with initial values.
    ///
    /// # Example
    ///
    /// ```
    ///     use std::rc::Rc;
    ///     use pebl::weak::WeakList;
    ///
    ///     let slice = &[Rc::new(1), Rc::new(2), Rc::new(3)];
    ///     let list = WeakList::of(slice);
    ///     assert_eq!(3, list.len());
    /// ```
    pub fn of(items: &[Rc<T>]) -> Self {
        let mut weak_vec: Vec<Weak<T>> = Vec::with_capacity(items.len());
        for item in items {
            weak_vec.push(Rc::downgrade(item))
        }

        WeakList { items: RefCell::new(weak_vec) }
    }

    /// Add a value to the tail-end of this list.
    ///
    /// # Examples
    ///
    /// ```
    ///     use std::rc::Rc;
    ///     use pebl::weak::WeakList;
    ///
    ///     let mut list = WeakList::<i32>::new();
    ///     let one = Rc::new(1);
    ///     list.push(&one);
    ///     assert_eq!(1, list.len());
    /// ```
    ///
    /// Remember, this list does not keep strong references to its values, so trying to push a
    /// temporary value onto it would be like trying to hold onto smoke:
    ///
    /// ```
    ///     use std::rc::Rc;
    ///     use pebl::weak::WeakList;
    ///
    ///     let mut list = WeakList::<i32>::new();
    ///     list.push(&Rc::new(1));
    ///     assert_eq!(0, list.len());
    /// ```
    pub fn push(&mut self, item: &Rc<T>) {
        self.items.borrow_mut().push(Rc::downgrade(item));
    }

    /// Create a copy of this weak list that holds only its strong references.
    ///
    /// It is useful to call this method anytime you want to iterate over this list.
    ///
    /// # Example
    /// ```
    ///     use std::rc::Rc;
    ///     use pebl::weak::WeakList;
    ///
    ///     let slice = &[Rc::new(1), Rc::new(2), Rc::new(3)];
    ///     let weak_list = WeakList::of(slice);
    ///     let mut sum = 0;
    ///     for value in weak_list.upgrade() {
    ///         sum += *value;
    ///     }
    ///     assert_eq!(6, sum);
    /// ```
    pub fn upgrade(&self) -> Vec<Rc<T>> {
        self.clean();
        let items = self.items.borrow();
        let mut v = Vec::<Rc<T>>::with_capacity(items.len());

        for weak_item in &(*items) {
            if let Some(i) = weak_item.upgrade() {
                v.push(i);
            }
        }
        v
    }

    /// Consume this weak list, converting into a strong list.
    ///
    /// # Example
    /// ```
    ///     use std::rc::Rc;
    ///     use pebl::weak::WeakList;
    ///
    ///     let slice = &[Rc::new(1), Rc::new(2), Rc::new(3)];
    ///     let weak_list = WeakList::of(slice);
    ///     let strong_list = weak_list.upgrade_owned();
    ///     // weak_list.push(&Rc::new(4)); // Cannot uncomment: weak_list was consumed
    ///     let mut sum = 0;
    ///     for value in strong_list {
    ///         sum += *value;
    ///     }
    ///     assert_eq!(6, sum);
    /// ```
    pub fn upgrade_owned(self) -> Vec<Rc<T>> {
        self.upgrade()
    }

    /// Return the number of *strong* references in this list. That is, if there were originally
    /// 5 items added but 3 have since been deallocated, `len` will return 2.
    pub fn len(&self) -> usize {
        self.clean();
        self.len_no_clean()
    }

    /// Return the capacity of this list, similar to [`Vec<T>`][`capacity`]
    pub fn capacity(&self) -> usize {
        self.items.borrow().capacity()
    }

    /// Return the current length of this list, before calling `clean` on it.
    fn len_no_clean(&self) -> usize {
        self.items.borrow().len()
    }

    /// Remove dead references from this list
    fn clean(&self) {
        self.items.borrow_mut().retain(|o| { return o.upgrade().is_some() });
    }
}

#[cfg(test)]
mod private_api_tests {
    extern crate spectral;

    use std::rc::Rc;
    use self::spectral::prelude::*;
    use super::*;

    #[test]
    fn weak_list_is_cleaned_after_upgrade() {
        let mut list = WeakList::<i32>::new();
        let int1 = Rc::new(10);
        list.push(&int1);
        {
            let int2 = Rc::new(20);
            list.push(&int2);

            assert_that(&list.len_no_clean()).is_equal_to(&2);
        }
        assert_that(&list.len_no_clean()).is_equal_to(&2);

        list.upgrade();
        assert_that(&list.len_no_clean()).is_equal_to(&1);
    }

    #[test]
    fn weak_list_is_cleaned_after_len_is_called() {
        let mut list = WeakList::<i32>::new();
        let int1 = Rc::new(10);
        list.push(&int1);
        {
            let int2 = Rc::new(20);
            list.push(&int2);
        }
        assert_that(&list.len_no_clean()).is_equal_to(&2);
        assert_that(&list.len()).is_equal_to(&1);
    }

    #[test]
    #[allow(dead_code)]
    fn weak_list_works_with_unsized_types() {
        trait DynamicallySized {}
        struct NoInt;
        struct OneInt { a: i32 }
        struct TwoInts { a: i32, b: i32 }
        impl DynamicallySized for NoInt {}
        impl DynamicallySized for OneInt {}
        impl DynamicallySized for TwoInts {}

        let mut list = WeakList::<DynamicallySized>::new();
        let item1: Rc<DynamicallySized> = Rc::new(NoInt {});
        let item2: Rc<DynamicallySized> = Rc::new(OneInt { a: 1 });
        let item3: Rc<DynamicallySized> = Rc::new(TwoInts { a: 2, b: 3 });
        list.push(&item1);
        list.push(&item2);
        list.push(&item3);

        assert_that(&list.len_no_clean()).is_equal_to(&3);
    }
}
