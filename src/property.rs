use std::cell::{RefCell, UnsafeCell};
use std::fmt;
use std::rc::{Rc, Weak};
use uuid::Uuid;

struct BorrowCounts {
    immutable: usize,
    mutable: bool,
}

impl BorrowCounts {
    fn new() -> BorrowCounts {
        BorrowCounts { immutable: 0, mutable: false }
    }

    fn count_borrow(&mut self) {
        if self.mutable {
            panic!("property already mutably borrowed");
        }

        self.immutable += 1;
    }

    fn count_borrow_mut(&mut self) {
        if self.mutable {
            panic!("property already mutably borrowed");
        }

        if self.immutable > 0 {
            panic!("property already immutably borrowed");
        }

        self.mutable = true;
    }

    fn count_unborrow(&mut self) {
        if self.immutable == 0 {
            panic!("invalid immutable release borrow");
        }

        self.immutable -= 1;
    }

    fn count_unborrow_mut(&mut self) {
        if !self.mutable {
            panic!("invalid mutable release borrow");
        }

        self.mutable = false;
    }
}

pub struct Property<T: PartialEq> {
    id: Uuid,
    value_cell: UnsafeCell<T>,
    borrow_counts: Rc<RefCell<BorrowCounts>>,
}

impl<T: PartialEq> Property<T> {
    pub fn new(value: T) -> Property<T> {
        Property {
            id: Uuid::new_v4(),
            value_cell: UnsafeCell::new(value),
            borrow_counts: Rc::new(RefCell::new(BorrowCounts::new()))
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.value_cell.get() }
    }

    pub fn set(&mut self, value: T) {
        unsafe { *self.value_cell.get() = value; }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl<T: PartialEq + Default> Default for Property<T> {
    fn default() -> Self {
        Property::new(Default::default())
    }
}

impl<T: PartialEq + Default> Property<T> {
    pub fn clear(&mut self) {
        self.set(Default::default());
    }
}

impl<T: PartialEq> AsRef<Property<T>> for Property<T> {
    fn as_ref(&self) -> &Property<T> {
        self
    }
}

pub struct PropertyRef<'a, T: 'a + PartialEq> {
    value_cell: &'a UnsafeCell<T>,
    borrow_counts: Rc<RefCell<BorrowCounts>>,
}

impl<'a, T: 'a + PartialEq> PropertyRef<'a, T> {
    fn new(value_cell: &'a UnsafeCell<T>, borrow_counts: Rc<RefCell<BorrowCounts>>) -> PropertyRef<'a, T> {
        borrow_counts.borrow_mut().count_borrow(); // Uncounted on Drop
        PropertyRef {
            value_cell: value_cell,
            borrow_counts: borrow_counts
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.value_cell.get() }
    }
}

impl<'a, T: 'a + PartialEq> Drop for PropertyRef<'a, T> {
    fn drop(&mut self) {
        (*self.borrow_counts.borrow_mut()).count_unborrow();
    }
}

pub struct PropertyMutRef<'a, T: 'a + PartialEq> {
    value_cell: &'a UnsafeCell<T>,
    borrow_counts: Rc<RefCell<BorrowCounts>>,
}

impl<'a, T: 'a + PartialEq> PropertyMutRef<'a, T> {
    fn new(value_cell: &'a UnsafeCell<T>, borrow_counts: Rc<RefCell<BorrowCounts>>) -> PropertyMutRef<'a, T> {
        borrow_counts.borrow_mut().count_borrow_mut(); // Uncounted on Drop
        PropertyMutRef {
            value_cell: value_cell,
            borrow_counts: borrow_counts
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.value_cell.get() }
    }

    pub fn set(&mut self, value: T) {
        unsafe { *self.value_cell.get() = value; }
    }
}

impl<'a, T: 'a + PartialEq> Drop for PropertyMutRef<'a, T> {
    fn drop(&mut self) {
        (*self.borrow_counts.borrow_mut()).count_unborrow_mut();
    }
}


pub struct PropertyPtr<T: PartialEq> {
    value_cell_ptr: *const UnsafeCell<T>,
    weak_borrow_counts: Weak<RefCell<BorrowCounts>>,
}

impl<T: PartialEq> PropertyPtr<T> {
    pub fn new(target: &Property<T>) -> PropertyPtr<T> {
        PropertyPtr {
            value_cell_ptr: &target.value_cell,
            weak_borrow_counts: Rc::downgrade(&target.borrow_counts),
        }
    }

    pub fn try_deref<'a>(&'a self) -> Option<PropertyRef<'a, T>> {
        if let Some(borrow_counts) = self.weak_borrow_counts.upgrade() {
            // By sanity checking self.borrow_counts, we know this is safe to deref
            return Some(PropertyRef::new(unsafe { &*self.value_cell_ptr }, borrow_counts));
        }
        None
    }

    pub fn try_deref_mut<'a>(&'a mut self) -> Option<PropertyMutRef<'a, T>> {
        if let Some(borrow_counts) = self.weak_borrow_counts.upgrade() {
            // By sanity checking self.borrow_counts, we know this is safe to deref
            return Some(PropertyMutRef::new(unsafe { &*self.value_cell_ptr }, borrow_counts));
        }
        None
    }

    pub fn deref<'a>(&'a self) -> PropertyRef<'a, T> {
        self.try_deref().unwrap()
    }

    pub fn deref_mut<'a>(&'a mut self) -> PropertyMutRef<'a, T> {
        self.try_deref_mut().unwrap()
    }
}

impl<T: PartialEq> Clone for PropertyPtr<T> {
    fn clone(&self) -> Self {
        PropertyPtr {
            value_cell_ptr: self.value_cell_ptr,
            weak_borrow_counts: self.weak_borrow_counts.clone(),
        }
    }
}

impl<T: PartialEq + fmt::Debug> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {{ {:?} }}", self.get())
    }
}

impl<'a, T: PartialEq + fmt::Debug> fmt::Debug for PropertyRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&Property {{ {:?} }}", self.get())
    }
}

impl<'a, T: PartialEq + fmt::Debug> fmt::Debug for PropertyMutRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&mut Property {{ {:?} }}", self.get())
    }
}

impl<T: PartialEq + fmt::Debug> fmt::Debug for PropertyPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self.try_deref() {
            None => write!(f, "*Property {{ null }}"),
            Some(ref p) => write!(f, "*Property {{ {:?} }}", p.get())
        };
    }
}

pub struct PropertyList<T : PartialEq> {
    // Outer RefCell so that we can clean the vec in notify_invalidated
    items: RefCell<Vec<PropertyPtr<T>>>,
}

pub struct PropertyListIterator<'a, T: 'a + PartialEq> {
    property_list: &'a PropertyList<T>,
    index: usize,
}

impl<T: PartialEq> PropertyList<T> {
    pub fn new() -> Self {
        PropertyList::<T> { items: RefCell::new(Vec::new()) }
    }

    pub fn push(&mut self, property: &Property<T>) {
        self.items.borrow_mut().push(PropertyPtr::new(property));
    }

    pub fn iter(&self) -> PropertyListIterator<T> {
        return PropertyListIterator { property_list: &self, index: 0 }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.items.borrow().len()
    }

    fn clean(&self) {
        self.items.borrow_mut().retain(|p_ptr| { return p_ptr.try_deref().is_some() });
    }
}

impl<'a, T: 'a + PartialEq> Iterator for PropertyListIterator<'a, T> {
    type Item = PropertyPtr<T>;

    fn next(&mut self) -> Option<Self::Item> {
        {
            let items = self.property_list.items.borrow();

            loop {
                if self.index >= items.len() {
                    break;
                }

                let p_ptr = &items[self.index];
                self.index += 1;

                if p_ptr.try_deref().is_some() {
                    return Some(p_ptr.clone());
                }
            }
        }

        self.property_list.clean(); // Only do after items is out of scope
        None
    }
}

impl<'a, T: 'a + PartialEq> IntoIterator for &'a PropertyList<T> {
    type Item = PropertyPtr<T>;
    type IntoIter = PropertyListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod private_api_tests {
    extern crate spectral;

    use self::spectral::prelude::*;
    use super::*;

    #[test]
    fn property_list_is_cleaned_after_iterated() {
        let mut list = PropertyList::<i32>::new();
        let p1 = Property::<i32>::new(10);
        list.push(&p1);
        {
            let p2 = Property::<i32>::new(20);
            list.push(&p2);

            assert_that(&list.len()).is_equal_to(&2);
        }
        assert_that(&list.len()).is_equal_to(&2);

        for _ in list.iter() {} // Once iteration is done, dead weak refs are removed
        assert_that(&list.len()).is_equal_to(&1);
    }

    #[test]
    fn weak_list_ref_can_be_used_in_for_loop() {
        let mut list = PropertyList::<i32>::new();
        let p1 = Property::<i32>::new(0);
        let p2 = Property::<i32>::new(1);
        let p3 = Property::<i32>::new(2);
        list.push(&p1);
        list.push(&p2);
        list.push(&p3);

        let mut i = 0;
        for p in &list {
            assert_that(&i).is_equal_to(p.deref().get());
            i += 1;
        }
        assert_that(&i).is_equal_to(3);
    }
}
