pub mod cmp;
pub mod logic;
pub mod math;
pub mod text;

// For CoreExpressions
use std::cmp::PartialOrd;
use std::fmt;
use std::marker::Sized;
use std::ops::{Add, Mul, Neg};
use std::rc::Rc;

use obsv::InvalidationHandler;

pub trait IntoExpression<T: PartialEq> {
    fn into_expr(self) -> Rc<Expression<T>>;
}

impl<T: PartialEq> IntoExpression<T> for Rc<Expression<T>> {
    fn into_expr(self) -> Rc<Expression<T>> {
        self
    }
}

pub trait Expression<T: PartialEq>: IntoExpression<T> {
    fn try_get(&self) -> Option<T>;
    fn get(&self) -> T {
        self.try_get().unwrap()
    }
    fn add_invalidation_handler(&self, handler: &InvalidationHandler);
}

pub trait CoreExpressions<T: PartialEq>: IntoExpression<T> where Self: Sized {
    // logic

    fn and<E: IntoExpression<bool>>(self, rhs: E) -> Rc<Expression<bool>>
        where Self: IntoExpression<bool> {
        logic::and(self, rhs)
    }

    fn not(self) -> Rc<Expression<bool>>
        where Self: IntoExpression<bool> {
        logic::not(self)
    }

    fn or<E: IntoExpression<bool>>(self, rhs: E) -> Rc<Expression<bool>>
        where Self: IntoExpression<bool> {
        logic::or(self, rhs)
    }


    // math

    fn abs(self) -> Rc<Expression<T>>
        where T: 'static + Copy + PartialOrd + Default + Neg<Output=T>, Self: IntoExpression<T> {
        math::abs(self)
    }

    fn neg(self) -> Rc<Expression<T>>
        where T: 'static + Copy + Neg<Output=T>, Self: IntoExpression<T> {
        math::neg(self)
    }

    fn plus<E: IntoExpression<T>>(self, rhs: E) -> Rc<Expression<T>>
        where T: 'static + Copy + Add<Output=T> {
        math::plus(self, rhs)
    }

    fn times<E: IntoExpression<T>>(self, rhs: E) -> Rc<Expression<T>>
        where T: 'static + Copy + Mul<Output=T> {
        math::times(self, rhs)
    }

    // cmp

    fn eq<E: IntoExpression<T>>(self, rhs: E) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::eq(self, rhs)
    }

    fn eq_val(self, val: T) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::eq_val(self, val)
    }

    fn ne<E: IntoExpression<T>>(self, rhs: E) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::ne(self, rhs)
    }

    fn ne_val(self, val: T) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::ne_val(self, val)
    }

    fn gt<E: IntoExpression<T>>(self, rhs: E) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::gt(self, rhs)
    }

    fn gt_val(self, val: T) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::gt_val(self, val)
    }

    fn lt<E: IntoExpression<T>>(self, rhs: E) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::lt(self, rhs)
    }

    fn lt_val(self, val: T) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::lt_val(self, val)
    }

    fn gte<E: IntoExpression<T>>(self, rhs: E) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::gte(self, rhs)
    }

    fn gte_val(self, val: T) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::gte_val(self, val)
    }

    fn lte<E: IntoExpression<T>>(self, rhs: E) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::lte(self, rhs)
    }

    fn lte_val(self, val: T) -> Rc<Expression<bool>>
        where T: 'static + PartialEq + Copy + PartialOrd, Self: IntoExpression<T> {
        cmp::lte_val(self, val)
    }

    // text

    fn len(self) -> Rc<Expression<usize>>
        where Self: IntoExpression<String> {
        text::len(self)
    }

    fn is_empty(self) -> Rc<Expression<bool>>
        where Self: IntoExpression<String> {
        text::is_empty(self)
    }

    fn trim(self) -> Rc<Expression<String>>
        where Self: IntoExpression<String> {
        text::trim(self)
    }

    fn to_string(self) -> Rc<Expression<String>>
        where T: 'static + fmt::Display {
        text::to_string(self)
    }
}

impl<T: PartialEq> CoreExpressions<T> for Rc<Expression<T>> {
    // Default implementation is fine
}

pub fn unary<I, O, E, F>(src: E, f: F) -> Rc<Expression<O>>
    where I: 'static + PartialEq, O: 'static + PartialEq, E: IntoExpression<I>, F: 'static + Fn(&I) -> O {
    UnaryExpression {
        src: src.into_expr(),
        f: Box::new(f),
    }.into_expr()
}

pub fn binary<I1, I2, O, E1, E2, F>(lhs: E1, rhs: E2, f: F) -> Rc<Expression<O>>
    where I1: 'static + PartialEq, I2: 'static + PartialEq, O: 'static + PartialEq, E1: IntoExpression<I1>, E2: IntoExpression<I2>, F: 'static + Fn(&I1, &I2) -> O {
    BinaryExpression {
        lhs: lhs.into_expr(),
        rhs: rhs.into_expr(),
        f: Box::new(f),
    }.into_expr()
}

pub struct UnaryExpression<I: 'static + PartialEq, O: 'static + PartialEq> {
    src: Rc<Expression<I>>,
    f: Box<Fn(&I) -> O>,
}

impl<I: 'static + PartialEq, O: 'static + PartialEq> IntoExpression<O> for UnaryExpression<I, O> {
    fn into_expr(self) -> Rc<Expression<O>> {
        Rc::new(self)
    }
}

impl<I: 'static + PartialEq, O: 'static + PartialEq> Expression<O> for UnaryExpression<I, O> {
    fn try_get(&self) -> Option<O> {
        self.src.try_get().map(|val| (self.f)(&val))
    }

    fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        self.src.add_invalidation_handler(handler);
    }
}

pub struct BinaryExpression<I1: 'static + PartialEq, I2: 'static + PartialEq, O: 'static + PartialEq> {
    lhs: Rc<Expression<I1>>,
    rhs: Rc<Expression<I2>>,
    f: Box<Fn(&I1, &I2) -> O>,
}

impl<I1: 'static + PartialEq, I2: 'static + PartialEq, O: 'static + PartialEq> IntoExpression<O> for BinaryExpression<I1, I2, O> {
    fn into_expr(self) -> Rc<Expression<O>> {
        Rc::new(self)
    }
}

impl<I1: 'static + PartialEq, I2: 'static + PartialEq, O: 'static + PartialEq> Expression<O> for BinaryExpression<I1, I2, O> {
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

