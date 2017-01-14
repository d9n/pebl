use std::cell::{RefCell, UnsafeCell};
use std::fmt;
use std::rc::{Rc, Weak};

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
            panic!("value already mutably borrowed");
        }

        self.immutable += 1;
    }

    fn count_borrow_mut(&mut self) {
        if self.mutable {
            panic!("value already mutably borrowed");
        }

        if self.immutable > 0 {
            panic!("value already immutably borrowed");
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

pub struct Observable<T> {
    cell: UnsafeCell<T>,
    borrow_counts: Rc<RefCell<BorrowCounts>>,
    // TODO: Listener
}

impl<T> Observable<T> {
    pub fn new(value: T) -> Self {
        Observable {
            cell: UnsafeCell::new(value),
            borrow_counts: Rc::new(RefCell::new(BorrowCounts::new()))
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.cell.get() }
    }

    pub fn set(&mut self, value: T) {
        unsafe { *self.cell.get() = value; }
    }
}

impl<T: Default> Default for Observable<T> {
    fn default() -> Self {
        Observable::new(Default::default())
    }
}

impl<T: Default> Observable<T> {
    pub fn clear(&mut self) {
        self.set(Default::default());
    }
}

pub struct ObservableRef<'a, T: 'a> {
    value_cell: &'a UnsafeCell<T>,
    borrow_counts: Rc<RefCell<BorrowCounts>>,
}

impl<'a, T: 'a> ObservableRef<'a, T> {
    fn new(value_cell: &'a UnsafeCell<T>, borrow_counts: Rc<RefCell<BorrowCounts>>) -> ObservableRef<'a, T> {
        borrow_counts.borrow_mut().count_borrow(); // Uncounted on Drop
        ObservableRef {
            value_cell: value_cell,
            borrow_counts: borrow_counts
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.value_cell.get() }
    }
}

impl<'a, T: 'a> Drop for ObservableRef<'a, T> {
    fn drop(&mut self) {
        (*self.borrow_counts.borrow_mut()).count_unborrow();
    }
}

pub struct ObservableMutRef<'a, T: 'a> {
    value_cell: &'a UnsafeCell<T>,
    borrow_counts: Rc<RefCell<BorrowCounts>>,
}

impl<'a, T: 'a> ObservableMutRef<'a, T> {
    fn new(value_cell: &'a UnsafeCell<T>, borrow_counts: Rc<RefCell<BorrowCounts>>) -> ObservableMutRef<'a, T> {
        borrow_counts.borrow_mut().count_borrow_mut(); // Uncounted on Drop
        ObservableMutRef {
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

impl<'a, T: 'a> Drop for ObservableMutRef<'a, T> {
    fn drop(&mut self) {
        (*self.borrow_counts.borrow_mut()).count_unborrow_mut();
    }
}


pub struct ObservablePtr<T> {
    cell_ptr: *const UnsafeCell<T>,
    weak_borrow_counts: Weak<RefCell<BorrowCounts>>,
}

impl<T> ObservablePtr<T> {
    pub fn new(target: &Observable<T>) -> ObservablePtr<T> {
        ObservablePtr {
            cell_ptr: &target.cell,
            weak_borrow_counts: Rc::downgrade(&target.borrow_counts),
        }
    }

    pub fn try_deref<'a>(&'a self) -> Option<ObservableRef<'a, T>> {
        if let Some(borrow_counts) = self.weak_borrow_counts.upgrade() {
            // By sanity checking self.borrow_counts, we know this is safe to deref
            return Some(ObservableRef::new(unsafe { &*self.cell_ptr }, borrow_counts));
        }
        None
    }

    pub fn try_deref_mut<'a>(&'a mut self) -> Option<ObservableMutRef<'a, T>> {
        if let Some(borrow_counts) = self.weak_borrow_counts.upgrade() {
            // By sanity checking self.borrow_counts, we know this is safe to deref
            return Some(ObservableMutRef::new(unsafe { &*self.cell_ptr }, borrow_counts));
        }
        None
    }

    pub fn deref<'a>(&'a self) -> ObservableRef<'a, T> {
        self.try_deref().unwrap()
    }

    pub fn deref_mut<'a>(&'a mut self) -> ObservableMutRef<'a, T> {
        self.try_deref_mut().unwrap()
    }
}

impl<T> Clone for ObservablePtr<T> {
    fn clone(&self) -> Self {
        ObservablePtr {
            cell_ptr: self.cell_ptr,
            weak_borrow_counts: self.weak_borrow_counts.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Observable<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Observable {{ {:?} }}", self.get())
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for ObservableRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&Observable {{ {:?} }}", self.get())
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for ObservableMutRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&mut Observable {{ {:?} }}", self.get())
    }
}

impl<T: fmt::Debug> fmt::Debug for ObservablePtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self.try_deref() {
            None => write!(f, "*Observable {{ null }}"),
            Some(ref p) => write!(f, "*Observable {{ {:?} }}", p.get())
        };
    }
}
