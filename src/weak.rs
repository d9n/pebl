use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub struct WeakList<T : ?Sized> {
    // Outer RefCell so that we can clean the vec in notify_invalidated
    items: RefCell<Vec<Weak<T>>>,
}

pub struct WeakListIterator<'a, T: 'a + ?Sized> {
    weak_list: &'a WeakList<T>,
    index: usize,
}

impl<T: ?Sized> WeakList<T> {
    pub fn new() -> Self {
        WeakList::<T>::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        WeakList { items: RefCell::new(Vec::with_capacity(capacity)) }
    }

    pub fn of(items: &[Rc<T>]) -> Self {
        let mut weak_vec: Vec<Weak<T>> = Vec::new();
        for item in items {
            weak_vec.push(Rc::downgrade(item))
        }

        WeakList { items: RefCell::new(weak_vec) }
    }

    pub fn push(&mut self, item: &Rc<T>) {
        self.items.borrow_mut().push(Rc::downgrade(item));
    }

    pub fn iter(&self) -> WeakListIterator<T> {
        return WeakListIterator { weak_list: &self, index: 0 }
    }

    pub fn upgrade(&self) -> Vec<Rc<T>> {
        let mut v = Vec::<Rc<T>>::new();
        for item in self.iter() {
            v.push(item);
        }
        v
    }

    pub fn len(&self) -> usize {
        self.clean();
        self.len_no_clean()
    }

    pub fn capacity(&self) -> usize {
        self.items.borrow().capacity()
    }

    fn len_no_clean(&self) -> usize {
        self.items.borrow().len()
    }

    fn clean(&self) {
        self.items.borrow_mut().retain(|o| { return o.upgrade().is_some() });
    }
}

impl<'a, T: ?Sized> Iterator for WeakListIterator<'a, T> {
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        {
            let items = self.weak_list.items.borrow();

            loop {
                if self.index >= items.len() {
                    break;
                }

                let weak_item = &items[self.index];
                self.index += 1;

                if let Some(i) = weak_item.upgrade() {
                    return Some(i);
                }
            }
        }

        self.weak_list.clean(); // Only do after items is out of scope
        None
    }
}

impl<'a, T: ?Sized> IntoIterator for &'a WeakList<T> {
    type Item = Rc<T>;
    type IntoIter = WeakListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod private_api_tests {
    extern crate spectral;

    use std::rc::Rc;
    use self::spectral::prelude::*;
    use super::*;

    #[test]
    fn weak_list_is_cleaned_after_iterated() {
        let mut list = WeakList::<i32>::new();
        let int1 = Rc::new(10);
        list.push(&int1);
        {
            let int2 = Rc::new(20);
            list.push(&int2);

            assert_that(&list.len_no_clean()).is_equal_to(&2);
        }
        assert_that(&list.len_no_clean()).is_equal_to(&2);

        for _ in list.iter() {} // Once iteration is done, dead weak refs are removed
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
    fn weak_list_ref_can_be_used_in_for_loop() {
        let mut list = WeakList::<&str>::new();
        let item1 = Rc::new("0");
        let item2 = Rc::new("1");
        list.push(&item1);
        list.push(&item2);

        let mut i = 0;
        for item in &list {
            let expected: &str = &i.to_string();
            let actual: &str = *item;
            assert_that(&actual).is_equal_to(&expected);
            i += 1
        }
        assert_that(&i).is_equal_to(2);
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
