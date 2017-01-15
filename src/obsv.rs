use std::cell::UnsafeCell;
use std::fmt;
use std::rc::{Rc, Weak};
use weak::*;

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

pub struct InvalidationHandler {
    callback: Rc<Fn()>,
}

impl InvalidationHandler {
    pub fn new<F: 'static + Fn()>(callback: F) -> Self {
        InvalidationHandler { callback: Rc::new(callback) }
    }
}

struct ObservableData<T: PartialEq> {
    value: T,
    handle: Rc<()>,
    borrow_counts: BorrowCounts,
    on_invalidated: WeakList<Fn()>,
}

impl<T: PartialEq> ObservableData<T> {
    fn get(&self) -> &T {
        &self.value
    }

    fn set(&mut self, value: T) {
        if self.value != value {
            self.value = value;
            for callback in &self.on_invalidated {
                callback();
            }
        }
    }
}

pub struct Observable<T: PartialEq> {
    cell: UnsafeCell<ObservableData<T>>,
}

impl<T: PartialEq> Observable<T> {
    pub fn new(value: T) -> Self {
        let data = ObservableData {
            value: value,
            handle: Rc::new(()),
            borrow_counts: BorrowCounts::new(),
            on_invalidated: WeakList::with_capacity(1),
        };
        Observable { cell: UnsafeCell::new(data) }
    }

    pub fn get(&self) -> &T {
        &self.get_data().get()
    }

    pub fn set(&mut self, value: T) {
        self.get_data().set(value);
    }

    pub fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        self.get_data().on_invalidated.push(&handler.callback);
    }

    fn get_data(&self) -> &mut ObservableData<T> {
        unsafe { &mut (*self.cell.get()) }
    }
}

impl<T: Default + PartialEq> Default for Observable<T> {
    fn default() -> Self {
        Observable::new(Default::default())
    }
}

impl<T: Default + PartialEq> Observable<T> {
    pub fn clear(&mut self) {
        self.set(Default::default());
    }
}

pub struct ObservableRef<'a, T: 'a + PartialEq> {
    obsv_ptr: &'a ObservablePtr<T>,
}

impl<'a, T: 'a + PartialEq> ObservableRef<'a, T> {
    // Only call this if you know that `data_ptr` can safely be dereferenced
    fn new(obsv_ptr: &'a ObservablePtr<T>) -> Self {
        obsv_ptr.deref_data().borrow_counts.count_borrow();
        ObservableRef { obsv_ptr: obsv_ptr }
    }

    pub fn get(&self) -> &T {
        self.obsv_ptr.deref_data().get()
    }
}

impl<'a, T: 'a + PartialEq> Drop for ObservableRef<'a, T> {
    fn drop(&mut self) {
        self.obsv_ptr.deref_data().borrow_counts.count_unborrow();
    }
}

pub struct ObservableMutRef<'a, T: 'a + PartialEq> {
    obsv_ptr: &'a ObservablePtr<T>,
}

impl<'a, T: 'a + PartialEq> ObservableMutRef<'a, T> {
    // Only call this if you know that `data_ptr` can safely be dereferenced
    fn new(obsv_ptr: &'a ObservablePtr<T>) -> Self {
        obsv_ptr.deref_data().borrow_counts.count_borrow_mut(); // Uncounted on Drop
        ObservableMutRef { obsv_ptr: obsv_ptr }
    }

    pub fn get(&self) -> &T {
        &self.obsv_ptr.deref_data().get()
    }

    pub fn set(&mut self, value: T) {
        self.obsv_ptr.deref_data().set(value);
    }
}

impl<'a, T: 'a + PartialEq> Drop for ObservableMutRef<'a, T> {
    fn drop(&mut self) {
        self.obsv_ptr.deref_data().borrow_counts.count_unborrow_mut();
    }
}


pub struct ObservablePtr<T: PartialEq> {
    cell_ptr: *const UnsafeCell<ObservableData<T>>,
    valid_handle: Weak<()>,
}

impl<T: PartialEq> ObservablePtr<T> {
    pub fn new(target: &Observable<T>) -> ObservablePtr<T> {
        ObservablePtr::<T> {
            cell_ptr: &target.cell,
            valid_handle: Rc::downgrade(&target.get_data().handle),
        }
    }

    fn can_deref(&self) -> bool {
        if let Some(_) = self.valid_handle.upgrade() { true } else { false }
    }

    pub fn try_deref<'a>(&'a self) -> Option<ObservableRef<'a, T>> {
        if self.can_deref() {
            return Some(ObservableRef::new(self));
        }
        None
    }

    pub fn try_deref_mut<'a>(&'a mut self) -> Option<ObservableMutRef<'a, T>> {
        if self.can_deref() {
            return Some(ObservableMutRef::new(self));
        }
        None
    }

    pub fn deref<'a>(&'a self) -> ObservableRef<'a, T> {
        self.try_deref().unwrap()
    }

    pub fn deref_mut<'a>(&'a mut self) -> ObservableMutRef<'a, T> {
        self.try_deref_mut().unwrap()
    }

    // Undefined behavior if `can_deref` is not true
    fn deref_data(&self) -> &mut ObservableData<T> {
        unsafe { &mut *((*self.cell_ptr).get()) }
    }
}

impl<T: PartialEq> Clone for ObservablePtr<T> {
    fn clone(&self) -> Self {
        ObservablePtr {
            cell_ptr: self.cell_ptr,
            valid_handle: self.valid_handle.clone(),
        }
    }
}

impl<T: fmt::Debug + PartialEq> fmt::Debug for Observable<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Observable {{ {:?} }}", self.get())
    }
}

impl<'a, T: fmt::Debug + PartialEq> fmt::Debug for ObservableRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&Observable {{ {:?} }}", self.get())
    }
}

impl<'a, T: fmt::Debug + PartialEq> fmt::Debug for ObservableMutRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&mut Observable {{ {:?} }}", self.get())
    }
}

impl<T: fmt::Debug + PartialEq > fmt::Debug for ObservablePtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self.try_deref() {
            None => write!(f, "*Observable {{ null }}"),
            Some(ref p) => write!(f, "*Observable {{ {:?} }}", p.get())
        };
    }
}
