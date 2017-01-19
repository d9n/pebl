use std::fmt;

use super::*;
use obsv::InvalidationHandler;

pub fn to_str<T: PartialEq + Clone, E: IntoExpression<T>>(value: E) -> ToStringExpression<T> {
    ToStringExpression { value_expr: value.into_expr() }
}

pub struct ToStringExpression<T: PartialEq + Clone> {
    value_expr: Box<Expression<T>>,
}

impl<T: 'static + fmt::Display + PartialEq + Clone> IntoExpression<String> for ToStringExpression<T> {
    fn into_expr(self) -> Box<Expression<String>> {
        Box::new(self)
    }
}


impl<T: 'static + fmt::Display + PartialEq + Clone> Expression<String> for ToStringExpression<T> {
    fn try_get(&self) -> Option<String> {
        self.value_expr.try_get().map(|val| String::from(format!("{0}", val)))
    }

    fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        self.value_expr.add_invalidation_handler(handler);
    }
}
