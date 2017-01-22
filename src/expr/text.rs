use std::fmt;

use super::*;

pub fn to_string<T: fmt::Display + PartialEq + Clone, E: IntoExpression<T>>(value: E) -> UnaryExpression<T, String> {
    ::expr::unary(value, |val| String::from(format!("{0}", val)))
}
