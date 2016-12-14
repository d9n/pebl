#[macro_use]
extern crate spectral;
extern crate pebl;

use std::rc::Rc;

use spectral::prelude::*;
use pebl::prelude::*;
use pebl::expr;

#[test]
fn sum_expr_works() {
    let p1 = Rc::new(Property::new(10));
    let p2 = Rc::new(Property::new(20));
    let p3 = Rc::new(Property::new(30));

    let s = expr::sum(&[p1.clone(), p2.clone(), p3.clone()]);
    assert_that(&s.get()).is_equal_to(60);
    // TODO:
//    p2.set(100);
//    assert_that(&s.get()).is_equal_to(140);
}

#[test]
fn to_string_expr_works() {
    let p = Rc::new(Property::new(10));

    let s = expr::to_string(p.clone());
    assert_that(&s.get()).is_equal_to(String::from("10"));

    // TODO:
//    p.set(-123);
//    assert_that(&s.get()).is_equal_to(String::from("-123"));
}

#[test]
fn expression_implements_debug() {
    let p1 = Rc::new(Property::new(10));
    let p2 = Rc::new(Property::new(20));
    let s = expr::sum(&[p1, p2]);

    let s_string = format!("{:?}", s);

    assert_that(&s_string.len()).is_greater_than(0);
}