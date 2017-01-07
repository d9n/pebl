use property::*;

use std::boxed::Box;
use std::fmt;
use std::marker::PhantomData;
use std::ops::AddAssign;

type ExprMethod<I, O> = Fn(&Vec<PropertyRef<I>>) -> O;

pub struct Expression<I: PartialEq, O: PartialEq> {
    property: Property<O>,
    targets: Vec<PropertyPtr<I>>,
    map: Box<ExprMethod<I, O>>,
    phantom: PhantomData<I>,
}

impl<I: PartialEq, O: PartialEq> Expression<I, O> {
    pub fn new(targets: &[&AsProperty<I>], map: Box<ExprMethod<I, O>>) -> Self {
        let mut v: Vec<PropertyPtr<I>> = Vec::with_capacity(targets.len());
        for t in targets {
            v.push(PropertyPtr::new(t.as_property()));
        }

        Expression {
            property: Property::new(Expression::run(&v, &map)),
            targets: v,
            map: map,
            phantom: PhantomData,
        }
    }

    pub fn get(&self) -> &O {
        self.property.get()
    }

    pub fn update(&mut self) {
        self.property.set(Expression::run(&self.targets, &self.map));
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

impl<I: PartialEq, O: PartialEq> AsProperty<O> for Expression<I, O> {
    fn as_property(&self) -> &Property<O> {
        &self.property
    }
}

impl<I: PartialEq, O: PartialEq + fmt::Debug> fmt::Debug for Expression<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression {{ {:?} }}", self.property.get())
    }
}

pub fn sum<T: PartialEq + Default + AddAssign + Copy>(targets: &[&AsProperty<T>]) -> Expression<T, T> {
    Expression::<T, T>::new(targets, Box::new(|targets| {
        let mut sum = Default::default();
        for t in targets {
            sum += *t.get();
        }
        sum
    }))
}

pub fn to_string<T: PartialEq + fmt::Display>(target: &AsProperty<T>) -> Expression<T, String> {
    Expression::<T, String>::new(&[target], Box::new(|targets| {
        String::from(format!("{0}", targets[0].get()))
    }))
}
