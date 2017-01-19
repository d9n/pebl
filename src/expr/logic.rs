use super::*;
use obsv::InvalidationHandler;

pub fn and<E1: IntoExpression<bool>, E2: IntoExpression<bool>>(lhs: E1, rhs: E2) -> AndExpression {
    AndExpression { lhs: lhs.into_expr(), rhs: rhs.into_expr() }
}

pub struct AndExpression {
    lhs: Box<Expression<bool>>,
    rhs: Box<Expression<bool>>,
}

impl IntoExpression<bool> for AndExpression {
    fn into_expr(self) -> Box<Expression<bool>> {
        Box::new(self)
    }
}

impl Expression<bool> for AndExpression {
    fn try_get(&self) -> Option<bool> {
        let (lhs_opt, rhs_opt) = (self.lhs.try_get(), self.rhs.try_get());
        if lhs_opt.is_none() || rhs_opt.is_none() { return None }
        Some(lhs_opt.unwrap()    && rhs_opt.unwrap())
    }

    fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        self.lhs.add_invalidation_handler(handler);
        self.rhs.add_invalidation_handler(handler);
    }
}

