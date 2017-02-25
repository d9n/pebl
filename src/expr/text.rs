use std::fmt;

use super::*;

pub fn is_empty<E: IntoExpression<String>>(value: E) -> UnaryExpression<String, bool> {
    ::expr::unary(value, |val| val.is_empty())
}

pub fn len<E: IntoExpression<String>>(value: E) -> UnaryExpression<String, usize> {
    ::expr::unary(value, |val| val.len())
}

pub fn to_string<T: fmt::Display + PartialEq + Clone, E: IntoExpression<T>>(value: E) -> UnaryExpression<T, String> {
    ::expr::unary(value, |val| String::from(format!("{0}", val)))
}

pub fn trim<E: IntoExpression<String>>(value: E) -> UnaryExpression<String, String> {
    ::expr::unary(value, |val| String::from(val.trim()))
}
