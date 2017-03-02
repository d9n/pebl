use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use obsv::{InvalidationHandler, ModifyInnerRef, Observable, ObservablePtr};
use expr::{CoreExpressions, Expression, IntoExpression};

struct Binding<T: PartialEq> {
    pub expr: Rc<Expression<T>>,
    #[allow(dead_code)] // Needed to keep weak ref alive
    handle: InvalidationHandler,
    dirty: Rc<Cell<bool>>,
}

pub struct Property<T: PartialEq> {
    value: Observable<T>,
    bound_to: Option<Binding<T>>,
}

impl<T: 'static + PartialEq> Property<T> {
    pub fn new(value: T) -> Property<T> {
        Property { value: Observable::new(value), bound_to: None }
    }

    pub fn bound_to<E: IntoExpression<T>>(target: E) -> Property<T> {
        let expr = target.into_expr();
        let mut p = Property::new(expr.get());
        p.bind_expr(expr);
        p
    }

    pub fn get(&self) -> &T {
        if let Some(ref binding) = self.bound_to {
            if binding.dirty.get() {
                let mut value_ptr = ObservablePtr::new(&self.value);
                value_ptr.deref_mut().set(binding.expr.get());
                binding.dirty.set(false);
            }
        }
        self.value.get()
    }

    pub fn set(&mut self, value: T) {
        self.value.set(value)
    }

    pub fn modify_inner(&mut self) -> ModifyInnerRef<T> {
        self.value.modify_inner()
    }

    pub fn bind<E: IntoExpression<T>>(&mut self, target: E) {
        self.bind_expr(target.into_expr());
    }

    pub fn unbind(&mut self) {
        self.bound_to = None;
    }

    pub fn is_bound(&self) -> bool {
        self.bound_to.is_some()
    }

    fn bind_expr(&mut self, expr: Rc<Expression<T>>) {
        let dirty = Rc::new(Cell::new(true));
        let dirty_clone = dirty.clone();
        let handle = InvalidationHandler::new(move || dirty_clone.set(true));
        let binding = Binding {
            expr: expr,
            handle: handle,
            dirty: dirty,
        };
        binding.expr.add_invalidation_handler(&binding.handle);
        self.bound_to = Some(binding);
    }
}

struct PassthruExpression<T: PartialEq> {
    src: ObservablePtr<T>,
}

impl<T: PartialEq> PassthruExpression<T> {
    pub fn new(src: &Observable<T>) -> Self {
        PassthruExpression { src: ObservablePtr::new(src) }
    }
}

impl<T: 'static + PartialEq + Clone> IntoExpression<T> for PassthruExpression<T> {
    fn into_expr(self) -> Rc<Expression<T>> {
        Rc::new(self)
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


impl<T: 'static + PartialEq + Default> Default for Property<T> {
    fn default() -> Self {
        Property::new(Default::default())
    }
}

impl<T: PartialEq + Default> Property<T> {
    pub fn clear(&mut self) {
        self.value.clear();
    }
}

impl Property<bool> {
    pub fn invert(&mut self) {
        let val = *self.get();
        self.set(!val);
    }
}

impl<'a, T: 'static + PartialEq + Clone> IntoExpression<T> for &'a Property<T> {
    fn into_expr(self) -> Rc<Expression<T>> {
        Rc::new(PassthruExpression::new(&self.value))
    }
}

impl<'a, T: 'static + PartialEq + Clone> CoreExpressions<T> for &'a Property<T> {
    // Default implementations are fine
}

impl<T: 'static + fmt::Debug + PartialEq> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {{ {:?} }}", self.get())
    }
}
