pub trait Getter<T> {
    fn get(&self) -> &T;
}

pub trait Observable<T> {
    fn add_listener<F>(&mut self, l: F) where F: 'static + Fn(&Self);
    fn add_listener_and_fire<F>(&mut self, l: F) where F: 'static + Fn(&Self) {
        l(self);
        self.add_listener(l);
    }
}

pub trait Setter<T: PartialEq> {
    fn set(&mut self, T);
}

pub struct Property<T> {
    listeners: Vec<Box<Fn(&Property<T>)>>,
    value: T,
}

impl<T> Property<T> {
    pub fn new(value: T) -> Self {
        return Property { listeners: Vec::new(), value: value };
    }

    fn notify_invalidated(&self) {
        for l in &self.listeners {
            l(self);
        }
    }
}

impl<T> Getter<T> for Property<T> {
    fn get(&self) -> &T {
        return &self.value;
    }
}

impl<T> Observable<T> for Property<T> {
    fn add_listener<F>(&mut self, l: F) where F: 'static + Fn(&Self) {
        self.listeners.push(Box::new(l));
    }
}

impl<T: PartialEq> Setter<T> for Property<T> {
    fn set(&mut self, value: T) {
        if self.value != value {
            self.value = value;
            self.notify_invalidated();
        }
    }
}

impl<T: Default + PartialEq> Property<T> {
    pub fn reset(&mut self) {
        self.set(Default::default());
    }
}

impl<T: Default> Default for Property<T> {
    fn default() -> Self {
        return Property::new(Default::default());
    }
}
