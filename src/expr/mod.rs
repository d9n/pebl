pub mod cmp;
pub mod logic;
pub mod math;
pub mod text;

// For CoreExpressions
use std::marker::Sized;
use std::ops::{Add, Mul, Neg};
use std::cmp::PartialOrd;
use std::fmt;

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

pub trait CoreExpressions<T: PartialEq + Clone>: IntoExpression<T> where Self: Sized {
    // logic

    fn and<E: IntoExpression<bool>>(self, rhs: E) -> BinaryExpression<bool, bool, bool>
        where Self: IntoExpression<bool> {
        logic::and(self, rhs)
    }

    fn not(self) -> UnaryExpression<bool, bool>
        where Self: IntoExpression<bool> {
        logic::not(self)
    }

    fn or<E: IntoExpression<bool>>(self, rhs: E) -> BinaryExpression<bool, bool, bool>
        where Self: IntoExpression<bool> {
        logic::or(self, rhs)
    }


    // math

    fn abs(self) -> UnaryExpression<T, T>
        where T: Copy + PartialOrd + Default + Neg<Output=T>, Self: IntoExpression<T> {
        math::abs(self)
    }

    fn neg(self) -> UnaryExpression<T, T>
        where T: Copy + Neg<Output=T>, Self: IntoExpression<T> {
        math::neg(self)
    }

    fn plus<E: IntoExpression<T>>(self, rhs: E) -> BinaryExpression<T, T, T>
        where T: Copy + Add<Output = T> {
        math::plus(self, rhs)
    }

    fn times<E: IntoExpression<T>>(self, rhs: E) -> BinaryExpression<T, T, T>
        where T: Copy + Mul<Output = T> {
        math::times(self, rhs)
    }

    // cmp

    fn eq<E: IntoExpression<T>>(self, rhs: E) -> BinaryExpression<T, T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::eq(self, rhs)
    }

    fn eq_val(self, val: T) -> UnaryExpression<T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::eq_val(self, val)
    }

    fn ne<E: IntoExpression<T>>(self, rhs: E) -> BinaryExpression<T, T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::ne(self, rhs)
    }

    fn ne_val(self, val: T) -> UnaryExpression<T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::ne_val(self, val)
    }

    fn gt<E: IntoExpression<T>>(self, rhs: E) -> BinaryExpression<T, T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::gt(self, rhs)
    }

    fn gt_val(self, val: T) -> UnaryExpression<T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::gt_val(self, val)
    }

    fn lt<E: IntoExpression<T>>(self, rhs: E) -> BinaryExpression<T, T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::lt(self, rhs)
    }

    fn lt_val(self, val: T) -> UnaryExpression<T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::lt_val(self, val)
    }

    fn gte<E: IntoExpression<T>>(self, rhs: E) -> BinaryExpression<T, T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::gte(self, rhs)
    }

    fn gte_val(self, val: T) -> UnaryExpression<T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::gte_val(self, val)
    }

    fn lte<E: IntoExpression<T>>(self, rhs: E) -> BinaryExpression<T, T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::lte(self, rhs)
    }

    fn lte_val(self, val: T) -> UnaryExpression<T, bool>
        where T: PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::lte_val(self, val)
    }

    // text

    fn len(self) -> UnaryExpression<String, usize>
        where Self: IntoExpression<String> {
        text::len(self)
    }

    fn is_empty(self) -> UnaryExpression<String, bool>
        where Self: IntoExpression<String> {
        text::is_empty(self)
    }

    fn to_string(self) -> UnaryExpression<T, String>
        where T: fmt::Display {
        text::to_string(self)
    }
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

impl<I: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone> CoreExpressions<O> for UnaryExpression<I, O> {}

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

impl<I1: 'static + PartialEq + Clone, I2: 'static + PartialEq + Clone, O: 'static + PartialEq + Clone> CoreExpressions<O> for BinaryExpression<I1, I2, O> {}