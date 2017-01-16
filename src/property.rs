use std::fmt;
use obsv::{Observable, ObservablePtr};

pub trait ToObservablePtr<T: PartialEq> {
    fn to_obsv_ptr(&self) -> ObservablePtr<T>;
}

pub struct Property<T: PartialEq> {
    value: Observable<T>,
}

impl<T: PartialEq> Property<T> {
    pub fn new(value: T) -> Property<T> {
        Property { value: Observable::new(value) }
    }

    pub fn get(&self) -> &T {
        self.value.get()
    }

    pub fn set(&mut self, value: T) {
        self.value.set(value)
    }
}

impl<T: PartialEq + Default> Default for Property<T> {
    fn default() -> Self {
        Property::new(Default::default())
    }
}

impl<T: PartialEq + Default> Property<T> {
    pub fn clear(&mut self) {
        self.value.clear();
    }
}

impl<T: PartialEq> ToObservablePtr<T> for Property<T> {
    fn to_obsv_ptr(&self) -> ObservablePtr<T> {
        ObservablePtr::new(&self.value)
    }
}

impl<T: fmt::Debug + PartialEq> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {{ {:?} }}", self.get())
    }
}
