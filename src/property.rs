use std::fmt;
use std::marker::PhantomData;

struct Value<T: PartialEq> {
    value: T,
}

impl<T: PartialEq> Value<T> {
    fn new(value: T) -> Value<T> {
        Value { value: value }
    }
}

pub struct Property<T: PartialEq> {
    inner_value: Value<T>,
}

impl<T: PartialEq> Property<T> {
    pub fn new(value: T) -> Property<T> {
        Property { inner_value: Value::new(value) }
    }

    pub fn get(&self) -> &T {
        &self.inner_value.value
    }

    pub fn set(&mut self, value: T) {
        self.inner_value.value = value;
    }
}

impl<T: PartialEq + Default> Property<T> {
    pub fn clear(&mut self) {
        self.set(Default::default());
    }
}

pub struct PropertyRef<'a, T: 'a + PartialEq> {
    value_ptr: *const Value<T>,
    phantom: PhantomData<&'a T>,

}

impl<'a, T: 'a + PartialEq> PropertyRef<'a, T> {
    pub fn new(p: &'a Property<T>) -> PropertyRef<'a, T> {
        PropertyRef { value_ptr: &p.inner_value, phantom: PhantomData }
    }

    pub fn get(&self) -> &T {
        unsafe { & (*self.value_ptr).value }
    }
}

pub struct PropertyMutRef<'a, T: 'a + PartialEq> {
    value_ptr: *mut Value<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a + PartialEq> PropertyMutRef<'a, T> {
    pub fn new(p: &'a mut Property<T>) -> PropertyMutRef<'a, T> {
        PropertyMutRef { value_ptr: &mut p.inner_value, phantom: PhantomData }
    }

    pub fn get(&self) -> &T {
        unsafe { & (*self.value_ptr).value }
    }

    pub fn set(&mut self, value: T) {
        let value_ref = unsafe { &mut *self.value_ptr };
        value_ref.value = value;
    }
}

impl<T: PartialEq + Default> Default for Property<T> {
    fn default() -> Self {
        Property::new(Default::default())
    }
}

impl<T: PartialEq + fmt::Debug> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {{ {:?} }}", self.get())
    }
}

impl<'a, T: PartialEq + fmt::Debug> fmt::Debug for PropertyRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyRef {{ {:?} }}", self.get())
    }
}

impl<'a, T: PartialEq + fmt::Debug> fmt::Debug for PropertyMutRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyMutRef {{ {:?} }}", self.get())
    }
}
