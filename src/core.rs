pub trait Getter<T> {
  fn get(&self) -> &T;
}

pub trait Observable<T> : Getter<T> {
  fn add_listener(&self);
}

pub trait Setter<T : Clone> {
  fn set(&mut self, &T);
}

pub struct Property<T : Clone> {
  value: T,
}

impl<T : Clone> Property<T> {
  pub fn new(value: &T) -> Property<T> {
    return Property { value: value.clone() };
  }
}

impl<T : Default + Clone> Default for Property<T> {
  fn default() -> Self {
    return Property { value: Default::default() };
  }
}


impl<T : Clone> Getter<T> for Property<T> {
  fn get(&self) -> &T {
    return &self.value;
  }
}

impl<T : Clone> Setter<T> for Property<T> {
  fn set(&mut self, value: &T) {
    self.value = value.clone();
  }
}
