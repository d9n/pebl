use std::fmt;
use std::rc::Rc;

use super::*;

pub fn is_empty<E: IntoExpression<String>>(value: E) -> Rc<Expression<bool>> {
    ::expr::unary(value, |val| val.is_empty())
}

pub fn len<E: IntoExpression<String>>(value: E) -> Rc<Expression<usize>> {
    ::expr::unary(value, |val| val.len())
}

pub fn to_string<T: 'static + fmt::Display + PartialEq, E: IntoExpression<T>>(value: E) -> Rc<Expression<String>> {
    ::expr::unary(value, |val| String::from(format!("{0}", val)))
}

pub fn trim<E: IntoExpression<String>>(value: E) -> Rc<Expression<String>> {
    ::expr::unary(value, |val| String::from(val.trim()))
}
