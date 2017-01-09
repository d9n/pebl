#[macro_use]
extern crate spectral;
extern crate pebl;

use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn bind_property_to_another() {
    let mut p_dest = Property::new(10);
    let mut p_src = Property::new(20);
    let mut b = Bindings::new();

    assert_that(p_dest.get()).is_equal_to(&10);

    b.bind(&mut p_dest, &p_src);
    assert_that(p_dest.get()).is_equal_to(&20);

    p_src.set(30);
    assert_that(p_dest.get()).is_equal_to(&20);

    b.update();
    assert_that(p_dest.get()).is_equal_to(&30);
}

#[test]
fn bind_property_to_expression() {
    let mut p_dest = Property::new(10);
    let mut p1 = Property::new(100);
    let mut p2 = Property::new(200);
    let mut sum = expr::sum(&[&p1, &p2]);

    let mut b = Bindings::new();

    assert_that(p_dest.get()).is_equal_to(&10);

    b.bind(&mut p_dest, &sum);
    assert_that(p_dest.get()).is_equal_to(&300);

    p1.set(30);
    p2.set(50);
    sum.update();
    assert_that(p_dest.get()).is_equal_to(&300);
    b.update();
    assert_that(p_dest.get()).is_equal_to(&80);
}

#[test]
fn release_binding_works() {
    let mut b = Bindings::new();
    let mut p_dest = Property::<i32>::default();
    let mut p_src = Property::new(20);

    b.bind(&mut p_dest, &p_src);
    b.unbind(&p_dest);

    p_src.set(30);
    b.update();
    assert_that(p_dest.get()).is_not_equal_to(&30);
}

#[test]
fn release_all_bindings_works() {
    let mut b = Bindings::new();
    let mut p_dest = Property::<i32>::default();
    let mut p_src = Property::new(20);

    b.bind(&mut p_dest, &p_src);
    b.clear();

    p_src.set(30);
    b.update();
    assert_that(p_dest.get()).is_not_equal_to(&30);
}