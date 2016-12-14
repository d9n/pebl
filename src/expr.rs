use property::*;
use weak::*;

use std::boxed::Box;
use std::fmt;
use std::ops::Add;
use std::rc::Rc;

pub struct Expression<I: Clone, O: Clone> {
    targets: WeakList<Getter<I>>,
    calc: Box<Fn(&[I]) -> O>,
}

impl<I: Clone, O: Clone> Expression<I, O> {
    pub fn new<F>(targets: &[Rc<Getter<I>>], calc: F) -> Expression<I, O>
    where F: 'static + Fn(&[I]) -> O
    {
        Expression { targets: WeakList::of(targets), calc: Box::new(calc) }
    }

    pub fn delete_me_len(&self) -> usize {
        self.targets.len()
    }
}

impl<I: Clone, O: Clone> Getter<O> for Expression<I, O> {
    fn get(&self) -> O {
        let values: Vec<I> = self.targets.iter().map(|t| t.get()).collect();
        (*self.calc)(&values).clone()
    }
}

impl<I: Clone + fmt::Debug, O: Clone> fmt::Debug for Expression<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let values: Vec<I> = self.targets.iter().map(|t| t.get()).collect();
        write!(f, "Expression {{ {:?} }}", values)
    }
}

pub fn sum<T: Default + Add<Output=T> + Copy>(targets: &[Rc<Getter<T>>]) -> Expression<T, T> {
    Expression::new(targets, |seq| seq.into_iter().fold(Default::default(), |a, &b| a + b))
}

pub fn to_string<T: fmt::Display + Clone>(target: Rc<Getter<T>>) -> Expression<T, String> {
    Expression::new(&[target], |seq| if let Some(val) = seq.into_iter().nth(0) { val.to_string() } else { String::from("") })
}
