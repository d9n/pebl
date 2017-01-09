use property::*;

use std::boxed::Box;
use std::fmt;
use std::ops::AddAssign;

type ExprMethod<I, O> = Fn(&Vec<PropertyRef<I>>) -> O;

pub struct Expression<I: PartialEq, O: PartialEq> {
    property: Property<O>,
    targets: Vec<PropertyPtr<I>>,
    resolve: Box<ExprMethod<I, O>>,
}

impl<I: PartialEq, O: PartialEq> Expression<I, O> {
    pub fn new(targets: &[&AsRef<Property<I>>], resolve: Box<ExprMethod<I, O>>) -> Self {
        let mut v: Vec<PropertyPtr<I>> = Vec::with_capacity(targets.len());
        for t in targets {
            v.push(PropertyPtr::new(t.as_ref()));
        }

        Expression {
            property: Property::new(Expression::run(&v, &resolve)),
            targets: v,
            resolve: resolve,
        }
    }

    pub fn get(&self) -> &O {
        self.property.get()
    }

    pub fn update(&mut self) {
        self.property.set(Expression::run(&self.targets, &self.resolve));
    }

    fn run(vec_ptrs: &Vec<PropertyPtr<I>>, f: &Box<ExprMethod<I, O>>) -> O {
        let mut vec: Vec<PropertyRef<I>> = Vec::with_capacity(vec_ptrs.len());
        for v in vec_ptrs {
            if let Some(p_ref) = v.get() {
                vec.push(p_ref);
            }
        }

        f(&vec)
    }
}

impl<I: PartialEq, O: PartialEq> AsRef<Property<O>> for Expression<I, O> {
    fn as_ref(&self) -> &Property<O> {
        &self.property
    }
}

impl<I: PartialEq, O: PartialEq + fmt::Debug> fmt::Debug for Expression<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression {{ {:?} }}", self.property.get())
    }
}

pub fn sum<T: PartialEq + Default + AddAssign + Copy>(targets: &[&AsRef<Property<T>>]) -> Expression<T, T> {
    Expression::<T, T>::new(targets, Box::new(|targets| {
        let mut sum = Default::default();
        for t in targets {
            sum += *t.get();
        }
        sum
    }))
}

pub fn to_string<T: PartialEq + fmt::Display>(target: &AsRef<Property<T>>) -> Expression<T, String> {
    Expression::<T, String>::new(&[target], Box::new(|targets| {
        String::from(format!("{0}", targets[0].get()))
    }))
}
