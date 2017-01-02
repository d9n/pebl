use rcref::RcRef;

use std::fmt;

pub trait Getter<T> {
    fn get<F: Fn(&T)>(&self, f: F);
}

pub trait Setter<T> {
    fn set(&mut self, value: T);
}

pub struct Property<T: PartialEq> {
    value: RcRef<T>,
}

impl<T: PartialEq> Property<T> {
    pub fn new(value: T) -> Self {
        return Property { value: RcRef::new(value) };
    }
}

impl<T: PartialEq + Default> Property<T> {
    pub fn reset(&mut self) {
        self.set(Default::default());
    }
}

impl<T: PartialEq + Default> Default for Property<T> {
    fn default() -> Self {
        return Property::new(Default::default());
    }
}

impl<T: PartialEq> Getter<T> for Property<T> {
    fn get<F: Fn(&T)>(&self, f: F) {
        self.value.peek(f);
    }
}

impl<T: PartialEq> Setter<T> for Property<T> {
    fn set(&mut self, value: T) {
        if *self.value.borrow() != value {
            *self.value.borrow_mut() = value;
        }
    }
}

impl<T: PartialEq + fmt::Debug> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {{ {:?} }}", self.value)
    }
}
