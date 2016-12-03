pub trait Getter<T> {
  fn get(&self) -> &T;
}

pub trait Observable<T> : Getter<T> {
  fn add_listener(&self);
}

pub trait Setter<T> {
  fn set(&mut self, T);
}

pub struct Property<T> {
  value: T,
}

impl<T> Property<T> {
  pub fn new(value: T) -> Property<T> {
    return Property { value: value };
  }
}

impl<T : Default> Default for Property<T> {
  fn default() -> Self {
    return Property { value: Default::default() };
  }
}


impl<T> Getter<T> for Property<T> {
  fn get(&self) -> &T {
    return &self.value;
  }
}

impl<T> Setter<T> for Property<T> {
  fn set(&mut self, value: T) {
    self.value = value;
  }
}
