use std::rc::Rc;

use super::*;

pub fn and<E1: IntoExpression<bool>, E2: IntoExpression<bool>>(lhs: E1, rhs: E2) -> Rc<Expression<bool>> {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 && val2)
}

pub fn not<E: IntoExpression<bool>>(value: E) -> Rc<Expression<bool>> {
    ::expr::unary(value, |val| !val)
}

pub fn or<E1: IntoExpression<bool>, E2: IntoExpression<bool>>(lhs: E1, rhs: E2) -> Rc<Expression<bool>> {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 || val2)
}
