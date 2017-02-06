use std::ops::{Add, Mul, Neg};
use std::cmp::PartialOrd;

use super::*;

pub fn abs<T, E: IntoExpression<T>>(value: E) -> UnaryExpression<T, T>
    where T: PartialEq + Copy + PartialOrd + Default + Neg<Output=T> {
    ::expr::unary(value, |&val| if val >= Default::default() { val } else { -val })
}

pub fn neg<T, E: IntoExpression<T>>(value: E) -> UnaryExpression<T, T>
    where T: PartialEq + Copy + Neg<Output=T> {
    ::expr::unary(value, |&val| -val)
}

pub fn plus<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, T>
    where T: PartialEq + Copy + Add<Output = T> {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 + val2)
}

pub fn times<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, T>
    where T: PartialEq + Copy + Mul<Output = T> {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 * val2)
}
