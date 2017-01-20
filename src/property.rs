use std::fmt;
use obsv::{InvalidationHandler, Observable, ObservablePtr};
use expr::{Expression, IntoExpression};

pub struct Property<T: PartialEq> {
    value: Observable<T>,
}

impl<T: 'static + PartialEq> Property<T> {
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

struct PassthruExpression<T: PartialEq + Clone> {
    src: ObservablePtr<T>,
}

impl<T: PartialEq + Clone> PassthruExpression<T> {
    pub fn new(src: &Observable<T>) -> Self {
        PassthruExpression { src: ObservablePtr::new(src) }
    }
}

impl<T: 'static + PartialEq + Clone> IntoExpression<T> for PassthruExpression<T> {
    fn into_expr(self) -> Box<Expression<T>> {
        Box::new(self)
    }
}

impl<T: 'static + PartialEq + Clone> Expression<T> for PassthruExpression<T> {
    fn try_get(&self) -> Option<T> {
        self.src.try_deref().map(|obsv| obsv.get().clone())
    }

    fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        if let Some(ref obsv) = self.src.try_deref() {
            obsv.add_invalidation_handler(handler);
        }
    }
}


impl<T: 'static + PartialEq + Clone + Default> Default for Property<T> {
    fn default() -> Self {
        Property::new(Default::default())
    }
}

impl<T: PartialEq + Clone + Default> Property<T> {
    pub fn clear(&mut self) {
        self.value.clear();
    }
}

impl<'a, T: 'static + PartialEq + Clone> IntoExpression<T> for &'a Property<T> {
    fn into_expr(self) -> Box<Expression<T>> {
        Box::new(PassthruExpression::new(&self.value))
    }
}

impl<T: 'static + fmt::Debug + Clone + PartialEq> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {{ {:?} }}", self.get())
    }
}
