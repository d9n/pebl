use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub struct WeakList<T> {
    // Outer RefCell so that we can clean the vec in notify_invalidated
    items: RefCell<Vec<Weak<T>>>,
}

pub struct WeakListIterator<'a, T: 'a> {
    weak_list: &'a WeakList<T>,
    index: usize,
}

impl<T> WeakList<T> {
    pub fn new() -> Self {
        WeakList::<T> { items: RefCell::new(Vec::new()) }
    }

    pub fn push(&mut self, observer: &Rc<T>) {
        self.items.borrow_mut().push(Rc::downgrade(observer));
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

    #[cfg(test)]
    fn len(&self) -> usize {
        self.items.borrow().len()
    }

    fn clean(&self) {
        self.items.borrow_mut().retain(|o| { return o.upgrade().is_some() });
    }
}

impl<'a, T> Iterator for WeakListIterator<'a, T> {
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

            assert_that(&list.len()).is_equal_to(&2);
        }
        assert_that(&list.len()).is_equal_to(&2);

        for _ in list.iter() {} // Once iteration is done, dead weak refs are removed
        assert_that(&list.len()).is_equal_to(&1);
    }
}