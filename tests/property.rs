#[macro_use]
extern crate spectral;
extern crate pebl;

use std::rc::Rc;
use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn default_properties() {
    let property = Property::<i32>::default();
    assert_that(&property.get()).is_equal_to(0);

    let property = Property::<bool>::default();
    assert_that(&property.get()).is_equal_to(false);

    let property = Property::<f64>::default();
    assert_that(&property.get()).is_equal_to(0.0);

    let property = Property::<Option<i32>>::default();
    assert_that(&property.get()).is_none();

    let property = Property::<Rc<i32>>::default();
    assert_that(& *property.get()).is_equal_to(0);
}

#[test]
fn property_simple_get() {
    let p = Property::new(10);
    assert_that(&p.get()).is_equal_to(10);
    assert_that(&p.get()).is_equal_to(10); // get() is borrow; can read twice
}

#[test]
fn property_simple_set() {
    let mut p = Property::new(10);
    p.set(20);
    assert_that(&p.get()).is_equal_to(20);
}

#[test]
fn property_clear() {
    let mut property = Property::new(10);
    property.reset();
    assert_that(&property.get()).is_equal_to(0);

    let mut property = Property::new(Option::from("Hello"));
    property.reset();
    assert_that(&property.get()).is_none();
}

#[test]
fn property_set_box_takes_ownership() {
    let range: Vec<_> = (0..100).collect();
    let range_box = Box::new(range);
    let p = Property::new(range_box);
    // let take_box_ownership = range_box; // As expected, uncommenting causes compile error
    assert_that(& *p.get()).has_length(100);
    assert_that(& *p.get()).has_length(100); // get() is borrow; can read twice
}

#[test]
fn property_implements_debug() {
    let p = Property::new(42);
    let p_string = format!("{:?}", p);

    assert_that(&p_string.len()).is_greater_than(0);
}