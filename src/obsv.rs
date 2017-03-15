//! A module supporting the `Observable<T>` struct.
//!
//! Essentially, an `Observable` is a data value which allows the registration of listeners. These
//! listeners are triggered any time the observable's value changes. Like a `RefCell<T>`, an
//! `Observable<T>` can be borrowed immutably or mutably at runtime.
//!
//! One additional trick up this module's sleeve is the inclusion of `ObservablePtr<T>` which can
//! be instantiated with an immutable `Observable<T>` and used to convert it to a mutable one.
//! This is similar to an `UnsafeCell<T>` with an additional set of `try_deref` and `try_deref_mut`
//! methods which can be used to safely deref the pointer.
//!
//! This class acts as the core of the `Property<T>` struct, and the fact it supports a pointer
//! concept makes it easier to create bindings to targets which may get deallocated at any time.

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::fmt;
use std::rc::{Rc, Weak};
use weak::WeakList;

/// A callback which gets fired when a target `Observable<T>` changes.
pub struct InvalidationHandler {
    callback: Rc<Fn()>,
}

impl InvalidationHandler {
    pub fn new<F: 'static + Fn()>(callback: F) -> Self {
        InvalidationHandler { callback: Rc::new(callback) }
    }
}

/// Core data which is wrapped by `Observable<T>`
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

    fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Set this observable's value, triggering invalidation callbacks if this new value is not
    /// equal to the current value.
    fn set(&mut self, value: T) {
        if self.value != value {
            self.value = value;
            self.fire_invalidated();
        }
    }

    /// Trigger the invalidation handlers of any listeners.
    fn fire_invalidated(&self) {
        for callback in self.on_invalidated.upgrade() {
            callback();
        }
    }
}

impl<T: PartialEq> Drop for ObservableData<T> {
    fn drop(&mut self) {
        self.fire_invalidated();
    }
}

/// A data value which can have listeners attached which are triggered when it changes
pub struct Observable<T: PartialEq> {
    cell: UnsafeCell<ObservableData<T>>,
}

impl<T: PartialEq> Observable<T> {
    /// Create a new observable
    ///
    /// # Example
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use pebl::obsv::{Observable, InvalidationHandler};
    ///
    /// let mut val = Observable::new(42);
    /// let times_modified = Rc::new(RefCell::new(0));
    /// let times_modified_clone = times_modified.clone();
    /// let handler = InvalidationHandler::new(move || *times_modified_clone.borrow_mut() += 1);
    /// val.add_invalidation_handler(&handler);
    ///
    /// assert_eq!(42, *val.get());
    /// assert_eq!(0, *times_modified.borrow());
    /// val.set(9001);
    /// assert_eq!(9001, *val.get());
    /// assert_eq!(1, *times_modified.borrow());
    /// ```
    pub fn new(value: T) -> Self {
        let data = ObservableData {
            value: value,
            handle: Rc::new(()),
            borrow_counts: BorrowCounts::new(),
            on_invalidated: WeakList::with_capacity(1),
        };
        Observable { cell: UnsafeCell::new(data) }
    }

    /// Get a reference to the value contained by this observable.
    pub fn get(&self) -> &T {
        &self.get_data().get()
    }

    /// Set the value of this observable. If the value changes, it will trigger any listeners that
    /// have been registered with `add_invalidation_handler`.
    pub fn set(&mut self, value: T) {
        self.get_data().set(value);
    }

    /// Modify the existing value of this observable instead of overwriting it. Note that this
    /// method assumes you will always be updating the value if you use this - otherwise, why not
    /// just use `get`? As a result, it will always fire any listeners that have been registered
    /// with `add_invalidation_handler`.
    ///
    /// # Example
    ///
    /// ```
    /// use pebl::obsv::Observable;
    ///
    /// let mut str_obsv = Observable::new(String::from("Hello"));
    /// str_obsv.modify_inner().push_str(", World");
    /// assert_eq!("Hello, World", str_obsv.get());
    /// ```
    #[must_use]
    pub fn modify_inner(&mut self) -> ModifyInnerRef<T> {
        ModifyInnerRef::new(self.get_data())
    }

    /// Register a listener which will be triggered whenever this observable's value is updated.
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
    /// If this observable supports the `Default` trait, call this convenience method to reset it to
    /// its default value.
    ///
    /// # Example
    ///
    /// ```
    /// use pebl::obsv::Observable;
    ///
    /// let mut str_obsv = Observable::new(String::from("Hello"));
    /// let mut int_obsv = Observable::new(123);
    /// str_obsv.clear();
    /// int_obsv.clear();
    ///
    /// assert_eq!("", str_obsv.get());
    /// assert_eq!(0, *int_obsv.get());
    /// ```
    pub fn clear(&mut self) {
        self.set(Default::default());
    }
}

/// Intermediate struct created to handle immutable borrows from an `ObservablePtr`.
pub struct ObservableRef<'a, T: 'a + PartialEq> {
    obsv_ptr: &'a ObservablePtr<T>,
}

impl<'a, T: 'a + PartialEq> ObservableRef<'a, T> {
    // Only call this if you know that `data_ptr` can safely be dereferenced
    fn new(obsv_ptr: &'a ObservablePtr<T>) -> Self {
        // Safe to call during lifetime of ObservableRef
        unsafe { obsv_ptr.deref_data().borrow_counts.count_borrow(); } // Uncounted on Drop
        ObservableRef { obsv_ptr: obsv_ptr }
    }

    pub fn get(&self) -> &T {
        // Safe to call during lifetime of ObservableRef
        unsafe { self.obsv_ptr.get() }
    }

    pub fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        // Safe to call during lifetime of ObservableRef
        unsafe { self.obsv_ptr.add_invalidation_handler(handler); }
    }
}

impl<'a, T: 'a + PartialEq> Drop for ObservableRef<'a, T> {
    fn drop(&mut self) {
        // Safe to call during lifetime of ObservableRef
        unsafe { self.obsv_ptr.deref_data().borrow_counts.count_unborrow(); }
    }
}

/// Intermediate struct created to handle mutable borrows from an `ObservablePtr`.
pub struct ObservableMutRef<'a, T: 'a + PartialEq> {
    obsv_ptr: &'a mut ObservablePtr<T>,
}

impl<'a, T: 'a + PartialEq> ObservableMutRef<'a, T> {
    // Only call this if you know that `data_ptr` can safely be dereferenced
    fn new(obsv_ptr: &'a mut ObservablePtr<T>) -> Self {
        // Safe to call during lifetime of ObservableRef
        unsafe { obsv_ptr.deref_data().borrow_counts.count_borrow_mut(); } // Uncounted on Drop
        ObservableMutRef { obsv_ptr: obsv_ptr }
    }

    pub fn get(&self) -> &T {
        // Safe to call during lifetime of ObservableMutRef
        unsafe { self.obsv_ptr.get() }
    }

    pub fn set(&mut self, value: T) {
        // Safe to call during lifetime of ObservableMutRef
        unsafe { self.obsv_ptr.set(value); }
    }

    pub fn modify_inner(&mut self) -> ModifyInnerRef<T> {
        unsafe { self.obsv_ptr.modify_inner() }
    }

    pub fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        // Safe to call during lifetime of ObservableMutRef
        unsafe { self.obsv_ptr.add_invalidation_handler(handler); }
    }
}

impl<'a, T: 'a + PartialEq> Drop for ObservableMutRef<'a, T> {
    fn drop(&mut self) {
        // Safe to call during lifetime of ObservableMutRef
        unsafe { self.obsv_ptr.deref_data().borrow_counts.count_unborrow_mut(); }
    }
}

/// Intermediate struct used to handle calls to `modify_inner`. It provides access to an
/// `Observable`s inner data for modifying them in place. This is useful for observables that wrap a
/// very large data value, such as a long `String` or large `Vec`. When this struct is dropped, it
/// automatically fires any listeners registered with this observable, ensuring that any change or
/// changes made will be picked up.
pub struct ModifyInnerRef<'a, T: 'a + PartialEq> {
    data: &'a mut ObservableData<T>,
}

impl<'a, T: 'a + PartialEq> ModifyInnerRef<'a, T> {
    fn new(data: &'a mut ObservableData<T>) -> Self {
        data.borrow_counts.count_borrow_mut(); // Uncounted on drop
        ModifyInnerRef { data: data }
    }
}

impl<'a, T: 'a + PartialEq> Deref for ModifyInnerRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data.get()
    }
}

impl<'a, T: 'a + PartialEq> DerefMut for ModifyInnerRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.get_mut()
    }
}

impl<'a, T: 'a + PartialEq> Drop for ModifyInnerRef<'a, T> {
    fn drop(&mut self) {
        self.data.borrow_counts.count_unborrow_mut();
        self.data.fire_invalidated();
    }
}

/// A pointer to an `Observable` which can safely be dereferenced as long as the target observable
/// is still valid.
///
/// # Examples
///
/// ```
/// use pebl::obsv::{Observable, ObservablePtr};
///
/// let o = Observable::new(123);
/// let op = ObservablePtr::new(&o);
///
/// assert!(op.try_deref().is_some());
/// assert_eq!(123, *op.deref().get());
///
/// drop(o);
/// assert!(op.try_deref().is_none());
/// ```
///
/// A very powerful feature exposed by this class is the ability to modify immutable `Observable`s.
/// This is occasionally necessary but can panic if you're not careful. So remember, with great
/// power comes great responsibility.
///
/// ```
/// use pebl::obsv::{Observable, ObservablePtr};
///
/// let o_imm = Observable::new(123); // immutable!!
/// let mut op = ObservablePtr::new(&o_imm);
/// op.deref_mut().set(999); // Legal but dangerous if you're not careful
/// assert_eq!(999, *o_imm.get());
/// ```
pub struct ObservablePtr<T: PartialEq> {
    cell_ptr: *const UnsafeCell<ObservableData<T>>,
    valid_handle: Weak<()>,
}

impl<T: PartialEq> ObservablePtr<T> {
    /// Create a pointer to a target `Observable<T>`
    pub fn new(target: &Observable<T>) -> ObservablePtr<T> {
        ObservablePtr::<T> {
            cell_ptr: &target.cell,
            valid_handle: Rc::downgrade(&target.get_data().handle),
        }
    }

    /// Attempt to dereference this pointer. If the target observable has been dropped, this
    /// optional will return `None`.
    pub fn try_deref<'a>(&'a self) -> Option<ObservableRef<'a, T>> {
        if self.can_deref() {
            return Some(ObservableRef::new(self));
        }
        None
    }

    /// Attempt to dereference this pointer mutably. If the target observable has been dropped, this
    /// optional will return `None`.
    pub fn try_deref_mut<'a>(&'a mut self) -> Option<ObservableMutRef<'a, T>> {
        if self.can_deref() {
            return Some(ObservableMutRef::new(self));
        }
        None
    }

    /// Convenience method for unwrapping the optional returned by `try_deref`, when you are sure
    /// the value will be available.
    pub fn deref<'a>(&'a self) -> ObservableRef<'a, T> {
        self.try_deref().unwrap()
    }

    /// Convenience method for unwrapping the optional returned by `try_deref_mut`, when you are
    /// sure the value will be available.
    pub fn deref_mut<'a>(&'a mut self) -> ObservableMutRef<'a, T> {
        self.try_deref_mut().unwrap()
    }

    fn can_deref(&self) -> bool {
        self.valid_handle.upgrade().is_some()
    }

    // Undefined behavior if `can_deref` is not true
    unsafe fn deref_data(&self) -> &mut ObservableData<T> {
        &mut *((*self.cell_ptr).get())
    }

    // Undefined behavior if `can_deref` is not true
    unsafe fn get(&self) -> &T {
        &self.deref_data().get()
    }

    // Undefined behavior if `can_deref` is not true
    unsafe fn set(&mut self, value: T) {
        self.deref_data().set(value);
    }
    // Undefined behavior if `can_deref` is not true
    unsafe fn modify_inner(&mut self) -> ModifyInnerRef<T> {
        ModifyInnerRef::new(self.deref_data())
    }

    // Undefined behavior if `can_deref` is not true
    unsafe fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        self.deref_data().on_invalidated.push(&handler.callback);
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

impl<T: fmt::Debug + PartialEq> fmt::Debug for ObservablePtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self.try_deref() {
            None => write!(f, "*Observable {{ null }}"),
            Some(ref p) => write!(f, "*Observable {{ {:?} }}", p.get())
        };
    }
}

/// A struct for detailing how many current borrows are being made on an intermediate reference.
/// This is useful to provide runtime panic behavior similar to Rust's normal compile-time
/// restrictions.
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

