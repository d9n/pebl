use std::cell::{RefCell, UnsafeCell};
use std::fmt;
use std::rc::{Rc, Weak};

pub trait AsProperty<T: PartialEq> {
    fn as_property(&self) -> &Property<T>;
}

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
    value_cell: UnsafeCell<T>,
    borrow_counts: Rc<RefCell<BorrowCounts>>,
}

impl<T: PartialEq> Property<T> {
    pub fn new(value: T) -> Property<T> {
        Property {
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

impl<T: PartialEq> AsProperty<T> for Property<T> {
    fn as_property(&self) -> &Property<T> {
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
        PropertyPtr { value_cell_ptr: &target.value_cell, weak_borrow_counts: Rc::downgrade(&target.borrow_counts) }
    }

    pub fn get<'a>(&'a self) -> Option<PropertyRef<'a, T>> {
        if let Some(borrow_counts) = self.weak_borrow_counts.upgrade() {
            // By sanity checking self.borrow_counts, we know this is safe to deref
            return Some(PropertyRef::new(unsafe { &*self.value_cell_ptr }, borrow_counts));
        }
        None
    }

    pub fn get_mut<'a>(&'a mut self) -> Option<PropertyMutRef<'a, T>> {
        if let Some(borrow_counts) = self.weak_borrow_counts.upgrade() {
            // By sanity checking self.borrow_counts, we know this is safe to deref
            return Some(PropertyMutRef::new(unsafe { &*self.value_cell_ptr }, borrow_counts));
        }
        None
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
        return match self.get() {
            None => write!(f, "*Property {{ null }}"),
            Some(ref p) => write!(f, "*Property {{ {:?} }}", p.get())
        };
    }
}
