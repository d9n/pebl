use std::boxed::Box;
use std::cell::Cell;
use std::fmt;
use std::ops::AddAssign;
use std::rc::Rc;

use obsv::{InvalidationHandler, Observable, ObservableRef, ObservablePtr};
use property::ToObservablePtr;


type ExprMethod<I, O> = Fn(&Vec<ObservableRef<I>>) -> O;

pub struct Expression<I: PartialEq, O: PartialEq> {
    value: Observable<O>,
    targets: Vec<ObservablePtr<I>>,
    resolve: Box<ExprMethod<I, O>>,
    #[allow(dead_code)] // Handle reference needed to keep weak reference alive
    invalidation_handler: InvalidationHandler,
    dirty: Rc<Cell<bool>>,
}

impl<I: PartialEq, O: PartialEq> Expression<I, O> {
    pub fn new(targets: &[&ToObservablePtr<I>], resolve: Box<ExprMethod<I, O>>) -> Self {
        let dirty = Rc::new(Cell::new(false));
        let handler;
        {
            let dirty = dirty.clone();
            handler = InvalidationHandler::new(move || dirty.set(true));
        }

        let mut target_ptrs: Vec<ObservablePtr<I>> = Vec::with_capacity(targets.len());
        for t in targets {
            let ptr = t.to_obsv_ptr();
            ptr.deref().add_invalidation_handler(&handler);
            target_ptrs.push(ptr);
        }

        Expression {
            value: Observable::new(Expression::run(&target_ptrs, &resolve)),
            targets: target_ptrs,
            resolve: resolve,
            dirty: dirty,
            invalidation_handler: handler,
        }
    }

    pub fn get(&self) -> &O {
        if self.dirty.get() {
            let mut value_ptr = ObservablePtr::new(&self.value);
            value_ptr.deref_mut().set(Expression::run(&self.targets, &self.resolve));
            self.dirty.set(false);
        }

        self.value.get()
    }

    fn run(vec_ptrs: &Vec<ObservablePtr<I>>, f: &Box<ExprMethod<I, O>>) -> O {
        let mut vec: Vec<ObservableRef<I>> = Vec::with_capacity(vec_ptrs.len());
        for v in vec_ptrs {
            if let Some(p_ref) = v.try_deref() {
                vec.push(p_ref);
            }
        }

        f(&vec)
    }
}

impl<I: PartialEq, O: PartialEq> ToObservablePtr<O> for Expression<I, O> {
    fn to_obsv_ptr(&self) -> ObservablePtr<O> {
        ObservablePtr::new(&self.value)
    }
}

impl<I: PartialEq, O: PartialEq + fmt::Debug> fmt::Debug for Expression<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression {{ {:?} }}", self.value.get())
    }
}

pub fn sum<T: PartialEq + Default + AddAssign + Copy>(targets: &[&ToObservablePtr<T>]) -> Expression<T, T> {
    Expression::<T, T>::new(targets, Box::new(|targets| {
        let mut sum = Default::default();
        for t in targets {
            sum += *t.get();
        }
        sum
    }))
}

pub fn to_string<T: PartialEq + fmt::Display>(target: &ToObservablePtr<T>) -> Expression<T, String> {
    // TODO: new_unary
    Expression::<T, String>::new(&[target], Box::new(|targets| {
        return String::from(format!("{0}", *targets[0].get()));
    }))
}
