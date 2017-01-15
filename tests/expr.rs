#[macro_use]
extern crate spectral;
extern crate pebl;

use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn sum_expr_works_with_int() {
    let p1 = Property::new(10);
    let mut p2 = Property::new(20);
    let p3 = Property::new(30);

    let s = expr::sum(&[&p1, &p2, &p3]);
    assert_that(s.get()).is_equal_to(&60);

    p2.set(100);
    assert_that(s.get()).is_equal_to(&140);
}
#[test]
fn sum_expr_works_with_float() {
    let p1 = Property::new(10.0);
    let mut p2 = Property::new(20.0);
    let p3 = Property::new(30.0);

    let s = expr::sum(&[&p1, &p2, &p3]);
    assert_that(s.get()).is_equal_to(&60.0);

    p2.set(100.0);
    assert_that(s.get()).is_equal_to(&140.0);
}

#[test]
fn expr_can_build_on_other_expr() {
    let p1 = Property::new(10);
    let p2 = Property::new(20);
    let sum1 = expr::sum(&[&p1, &p2]);

    let p3 = Property::new(100);
    let p4 = Property::new(200);
    let sum2 = expr::sum(&[&p3, &p4]);

    let p5 = Property::new(1000);

    let sum3 = expr::sum(&[&sum1, &sum2, &p5]);

    assert_that(sum3.get()).is_equal_to(&1330);
}


#[test]
fn to_string_expr_works() {
    let mut p = Property::new(10);

    let s = expr::to_string(&p);
    assert_that(s.get()).is_equal_to(String::from("10"));

    p.set(-123);
    assert_that(s.get()).is_equal_to(String::from("-123"));
}

#[test]
fn expression_implements_debug() {
    let p1 = Property::new(10);
    let p2 = Property::new(20);
    let s = expr::sum(&[&p1, &p2]);

    let s_string = format!("{:?}", s);

    assert_that(&s_string.len()).is_greater_than(&0);
}