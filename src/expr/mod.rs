// TODO: macro for creating new expressions?

pub mod logic;
pub mod math;
pub mod text;

use obsv::InvalidationHandler;

pub trait IntoExpression<T: PartialEq + Clone> {
    fn into_expr(self) -> Box<Expression<T>>;
}

pub trait Expression<T: PartialEq + Clone>: IntoExpression<T> {
    fn try_get(&self) -> Option<T>;
    fn get(&self) -> T {
        self.try_get().unwrap()
    }
    fn add_invalidation_handler(&self, handler: &InvalidationHandler);
}

