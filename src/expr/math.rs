use std::ops::{Add, Mul, Neg};
use std::cmp::PartialOrd;
use std::rc::Rc;

use super::*;

pub fn abs<T, E: IntoExpression<T>>(value: E) -> Rc<Expression<T>>
    where T: 'static + PartialEq + Copy + PartialOrd + Default + Neg<Output=T> {
    ::expr::unary(value, |&val| if val >= Default::default() { val } else { -val })
}

pub fn neg<T, E: IntoExpression<T>>(value: E) -> Rc<Expression<T>>
    where T: 'static + PartialEq + Copy + Neg<Output=T> {
    ::expr::unary(value, |&val| -val)
}

pub fn plus<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> Rc<Expression<T>>
    where T: 'static + PartialEq + Copy + Add<Output=T> {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 + val2)
}

pub fn times<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> Rc<Expression<T>>
    where T: 'static + PartialEq + Copy + Mul<Output=T> {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 * val2)
}
