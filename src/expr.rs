use std::boxed::Box;
use std::cell::Cell;
use std::fmt;
use std::ops::Add;
use std::rc::Rc;

use obsv::{InvalidationHandler, Observable, ObservableRef, ObservablePtr};
use property::ToObservablePtr;


type ExprMethod<I, O> = Fn(&Vec<ObservableRef<I>>) -> O;
type UnaryExprMethod<I, O> = Fn(Option<&ObservableRef<I>>) -> O;

pub struct Expression<I: 'static + PartialEq, O: 'static + PartialEq> {
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

    pub fn new_unary(target: &ToObservablePtr<I>, resolve: Box<UnaryExprMethod<I, O>>) -> Self {
        Expression::new(&[target], Box::new(move |targets| {
            if targets.len() > 0 { resolve(Option::from(&targets[0])) } else { resolve(None) }
        }))
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

pub fn sum<T: PartialEq + Default + Add<Output = T> + Copy>(targets: &[&ToObservablePtr<T>]) -> Expression<T, T> {
    Expression::<T, T>::new(targets, Box::new(|targets| {
        targets.iter().fold(Default::default(), |sum, val| sum + *val.get())
    }))
}

pub fn and(targets: &[&ToObservablePtr<bool>]) -> Expression<bool, bool> {
    Expression::<bool, bool>::new(targets, Box::new(|targets| {
        if targets.len() == 0 {
            return false;
        }
        targets.iter().fold(true, |result, val| result && *val.get())
    }))
}

pub fn to_string<T: PartialEq + fmt::Display>(target: &ToObservablePtr<T>) -> Expression<T, String> {
    Expression::<T, String>::new_unary(target, Box::new(|opt| {
        match opt {
            Some(value) => String::from(format!("{0}", value.get())),
            None => String::from(""),
        }
    }))
}
