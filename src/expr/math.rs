use std::ops::Add;

use super::*;
use obsv::InvalidationHandler;


pub fn plus<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> PlusExpression<T>
    where T: PartialEq + Copy + Add + Default {
    PlusExpression { lhs: lhs.into_expr(), rhs: rhs.into_expr() }
}

pub struct PlusExpression<T: PartialEq + Copy + Add + Default> {
    lhs: Box<Expression<T>>,
    rhs: Box<Expression<T>>,
}

impl<T: 'static + PartialEq + Copy + Add<Output = T> + Default> IntoExpression<T> for PlusExpression<T> {
    fn into_expr(self) -> Box<Expression<T>> {
        Box::new(self)
    }
}

impl<T: 'static + PartialEq + Copy + Add<Output = T> + Default> Expression<T> for PlusExpression<T> {
    fn try_get(&self) -> Option<T> {
        let (lhs_opt, rhs_opt) = (self.lhs.try_get(), self.rhs.try_get());
        if lhs_opt.is_none() && rhs_opt.is_none() { return None }
        Some(lhs_opt.unwrap_or(Default::default()) + rhs_opt.unwrap_or(Default::default()))
    }

    fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        self.lhs.add_invalidation_handler(handler);
        self.rhs.add_invalidation_handler(handler);
    }
}
