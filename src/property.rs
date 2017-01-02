use std::fmt;

pub trait Getter<T: Clone> {
    fn get(&self) -> T;
}

pub trait Setter<T> {
    fn set(&mut self, value: T);
}

pub struct Property<T: PartialEq + Clone> {
    value: T,
}

impl<T: PartialEq + Clone> Property<T> {
    pub fn new(value: T) -> Self {
        return Property { value: value };
    }
}

impl<T: PartialEq + Clone + Default> Property<T> {
    pub fn reset(&mut self) {
        self.set(Default::default());
    }
}

impl<T: PartialEq + Clone + Default> Default for Property<T> {
    fn default() -> Self {
        return Property::new(Default::default());
    }
}

impl<T: PartialEq + Clone> Getter<T> for Property<T> {
    fn get(&self) -> T {
        return self.value.clone();
    }
}

impl<T: PartialEq + Clone> Setter<T> for Property<T> {
    fn set(&mut self, value: T) {
        if self.value != value {
            self.value = value;
        }
    }
}

impl<T: PartialEq + Clone + fmt::Debug> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {{ {:?} }}", self.value)
    }
}
