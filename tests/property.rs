#[macro_use]
extern crate spectral;
extern crate pebl;

use std::rc::Rc;
use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn default_properties() {
    let property = Property::<i32>::default();
    property.get(|val| assert_that(val).is_equal_to(&0));

    let property = Property::<bool>::default();
    property.get(|val| assert_that(val).is_equal_to(false));

    let property = Property::<f64>::default();
    property.get(|val| assert_that(val).is_equal_to(0.0));

    let property = Property::<Option<i32>>::default();
    property.get(|val| assert_that(val).is_none());

    let property = Property::<Rc<i32>>::default();
    property.get(|val| assert_that(&**val).is_equal_to(&0));
}

#[test]
fn property_simple_get() {
    let p = Property::new(10);
    p.get(|val| assert_that(&val).is_equal_to(&10));
    p.get(|val| assert_that(&val).is_equal_to(&10));
}

#[test]
fn property_simple_set() {
    let mut p = Property::new(10);
    p.set(20);
    p.get(|val| assert_that(val).is_equal_to(&20));
}

#[test]
fn property_clear() {
    let mut property = Property::new(10);
    property.reset();
    property.get(|val| assert_that(val).is_equal_to(&0));

    let mut property = Property::new(Option::from("Hello"));
    property.reset();
    property.get(|val| assert_that(val).is_none());
}

#[test]
fn property_takes_ownership() {
    let range: Vec<_> = (0..100).collect();
    let p = Property::new(range);
//    let range_already_taken = range; // As expected, uncommenting causes compile error

    p.get(|val| assert_that(& *val).has_length(100));
    // Assert again, verifying vector still available for reading
    p.get(|val| assert_that(& *val).has_length(100));
}

#[test]
fn property_implements_debug() {
    let p = Property::new(42);
    let p_string = format!("{:?}", p);

    assert_that(&p_string.len()).is_greater_than(0);
}