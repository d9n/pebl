use std::cmp::PartialOrd;
use std::rc::Rc;

use super::*;

pub fn eq<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 == val2)
}

pub fn eq_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val == rhs)
}

pub fn ne<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 != val2)
}

pub fn ne_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val != rhs)
}

pub fn gt<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 > val2)
}

pub fn gt_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val > rhs)
}

pub fn lt<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 < val2)
}

pub fn lt_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val < rhs)
}

pub fn gte<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 >= val2)
}

pub fn gte_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val >= rhs)
}

pub fn lte<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 <= val2)
}

pub fn lte_val<T, E: IntoExpression<T>>(lhs: E, rhs: T) -> Rc<Expression<bool>>
    where T: 'static + PartialEq + Copy + PartialOrd {
    ::expr::unary(lhs, move |&val| val <= rhs)
}
