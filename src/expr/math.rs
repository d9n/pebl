use std::ops::Add;

use super::Expression;
use ::property::ToObservablePtr;

pub fn sum<T: PartialEq + Default + Add<Output = T> + Copy>(targets: &[&ToObservablePtr<T>]) -> Expression<T, T> {
    Expression::<T, T>::new(targets, Box::new(|targets| {
        targets.iter().fold(Default::default(), |sum, val| sum + *val.get())
    }))
}
