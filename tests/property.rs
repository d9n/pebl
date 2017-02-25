#[macro_use]
extern crate spectral;
extern crate pebl;

use std::rc::Rc;
use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn default_properties() {
    let property = Property::<i32>::default();
    assert_that(property.get()).is_equal_to(&0);

    let property = Property::<bool>::default();
    assert_that(property.get()).is_false();

    let property = Property::<f64>::default();
    assert_that(property.get()).is_equal_to(&0.0);

    let property = Property::<Option<i32>>::default();
    assert_that(property.get()).is_none();

    let property = Property::<Rc<i32>>::default();
    assert_that(&**property.get()).is_equal_to(&0);
}

#[test]
fn property_simple_get() {
    let p = Property::new(10);
    assert_that(p.get()).is_equal_to(&10);
    assert_that(p.get()).is_equal_to(&10);
}

#[test]
fn property_simple_set() {
    let mut p = Property::new(10);
    p.set(20);
    assert_that(p.get()).is_equal_to(&20);
}

#[test]
fn property_modify_inner() {
    let mut p = Property::new(String::from("Hello"));
    p.modify_inner().push_str(", World");

    assert_that(p.get()).is_equal_to(String::from("Hello, World"));
}

#[test]
fn property_clear() {
    let mut p = Property::new(10);
    p.clear();
    assert_that(p.get()).is_equal_to(&0);

    let mut p = Property::new(Option::from("Hello"));
    p.clear();
    assert_that(p.get()).is_none();
}

#[test]
fn property_takes_ownership() {
    let range: Vec<_> = (0..100).collect();
    let _ = Property::new(range);
//    let range_already_taken = range; // As expected, uncommenting causes compile error
}

#[test]
fn property_class_implements_debug() {
    let p = Property::new(42);

    let p_string = format!("{:?}", p);
    assert_that(&p_string.as_str()).is_equal_to(&"Property { 42 }");
}

#[test]
fn property_can_bind_to_other_property() {
    let mut p_src = Property::new(42);
    let mut p_dest = Property::default();

    assert_that(&p_dest.is_bound()).is_false();
    p_dest.bind(&p_src);
    assert_that(&p_dest.is_bound()).is_true();
    assert_that(p_dest.get()).is_equal_to(&42);

    p_src.set(9000);
    assert_that(p_dest.get()).is_equal_to(&9000);

    assert_that(&p_dest.is_bound()).is_true();
    p_dest.unbind();
    assert_that(&p_dest.is_bound()).is_false();
    p_src.set(123);
    assert_that(p_dest.get()).is_equal_to(&9000);
}

#[test]
fn property_can_bind_to_expression() {
    let mut p1 = Property::new(100);
    let mut p2 = Property::new(10);
    let mut p3 = Property::new(1);
    let sum = p1.plus(&p2).plus(&p3);

    let mut p_dest = Property::default();

    p_dest.bind(sum);
    assert_that(p_dest.get()).is_equal_to(&111);

    p1.set(0);
    assert_that(p_dest.get()).is_equal_to(&11);

    p3.set(0);
    assert_that(p_dest.get()).is_equal_to(&10);

    p2.set(0);
    assert_that(p_dest.get()).is_equal_to(&0);
}

#[test]
fn bool_property_can_invert() {
    let mut p = Property::new(true);

    p.invert();
    assert_that(p.get()).is_false();

    p.invert();
    assert_that(p.get()).is_true();
}
