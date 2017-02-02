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

pub fn eq<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 == val2)
}

pub fn eq_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> UnaryExpression<T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val == rhs)
}

pub fn ne<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 != val2)
}

pub fn ne_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> UnaryExpression<T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val != rhs)
}

pub fn gt<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 > val2)
}

pub fn gt_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> UnaryExpression<T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val > rhs)
}

pub fn lt<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 < val2)
}

pub fn lt_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> UnaryExpression<T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val < rhs)
}

pub fn gte<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 >= val2)
}

pub fn gte_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> UnaryExpression<T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val >= rhs)
}

pub fn lte<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 <= val2)
}
pub fn lte_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> UnaryExpression<T, bool>
    where T: PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val <= rhs)
}
