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

pub fn unary<I, O, E, F>(src: E, f: F) -> UnaryExpression<I, O>
    where I: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone, E: IntoExpression<I>, F: 'static + Fn(&I) -> O {
    UnaryExpression {
        src: src.into_expr(),
        f: Box::new(f),
    }
}

pub fn binary<I1, I2, O, E1, E2, F>(lhs: E1, rhs: E2, f: F) -> BinaryExpression<I1, I2, O>
    where I1: 'static + PartialEq + Clone, I2: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone, E1: IntoExpression<I1>, E2: IntoExpression<I2>, F: 'static + Fn(&I1, &I2) -> O {
    BinaryExpression {
        lhs: lhs.into_expr(),
        rhs: rhs.into_expr(),
        f: Box::new(f),
    }
}

pub struct UnaryExpression<I: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone> {
    src: Box<Expression<I>>,
    f: Box<Fn(&I) -> O>,
}

impl<I: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone> IntoExpression<O> for UnaryExpression<I, O> {
    fn into_expr(self) -> Box<Expression<O>> {
        Box::new(self)
    }
}

impl<I: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone> Expression<O> for UnaryExpression<I, O> {
    fn try_get(&self) -> Option<O> {
       self.src.try_get().map(|val| (self.f)(&val))
    }

    fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        self.src.add_invalidation_handler(handler);
    }
}

pub struct BinaryExpression<I1: 'static + PartialEq + Clone, I2: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone> {
    lhs: Box<Expression<I1>>,
    rhs: Box<Expression<I2>>,
    f: Box<Fn(&I1, &I2) -> O>,
}

impl<I1: 'static + PartialEq + Clone, I2: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone> IntoExpression<O> for BinaryExpression<I1, I2, O> {
    fn into_expr(self) -> Box<Expression<O>> {
        Box::new(self)
    }
}

impl<I1: 'static + PartialEq + Clone, I2: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone> Expression<O> for BinaryExpression<I1, I2, O> {
    fn try_get(&self) -> Option<O> {
        let (lhs_opt, rhs_opt) = (self.lhs.try_get(), self.rhs.try_get());
        if lhs_opt.is_none() || rhs_opt.is_none() { return None }
        Some((self.f)(&lhs_opt.unwrap(), &rhs_opt.unwrap()))
    }

    fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        self.lhs.add_invalidation_handler(handler);
        self.rhs.add_invalidation_handler(handler);
    }
}
