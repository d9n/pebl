#[macro_use]
extern crate spectral;
extern crate pebl;

use std::cell::Cell;
use std::rc::Rc;
use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn default_properties() {
    let int_property = Property::<i32>::default();
    assert_that(int_property.get()).is_equal_to(&0);

    let bool_property = Property::<bool>::default();
    assert_that(bool_property.get()).is_equal_to(&false);

    let float_property = Property::<f64>::default();
    assert_that(float_property.get()).is_equal_to(&0.0);

    let float_property = Property::<Option<i32>>::default();
    assert_that(float_property.get()).is_none();
}

#[test]
fn property_simple_get() {
    let p = Property::new(10);
    assert_that(p.get()).is_equal_to(&10);
    assert_that(p.get()).is_equal_to(&10); // get() is borrow; can read twice
}

#[test]
fn property_simple_set() {
    let mut p = Property::new(10);
    p.set(20);
    assert_that(p.get()).is_equal_to(&20);
}

#[test]
fn property_clear() {
    let mut int_property = Property::new(10);
    int_property.reset();
    assert_that(int_property.get()).is_equal_to(0);

    let mut option_property = Property::new(Option::from("Hello"));
    option_property.reset();
    assert_that(option_property.get()).is_none();
}


#[test]
fn property_set_box_takes_ownership() {
    let range: Vec<_> = (0..100).collect();
    let range_ptr = Box::new(range);
    let p = Property::new(range_ptr);
    assert_that(&**p.get()).has_length(100);
    assert_that(&**p.get()).has_length(100); // get() is borrow; can read twice
}

#[test]
fn property_listener_is_notified() {
    let test_val = Rc::new(Cell::new(0));
    let mut p = Property::new(10);
    {
        let test_val = test_val.clone();
        p.add_listener_and_fire(move |sender| test_val.set(*sender.get()));
    }

    assert_that(&test_val.get()).is_equal_to(&10);
    p.set(20);
    assert_that(&test_val.get()).is_equal_to(&20);
    p.reset();
    assert_that(&test_val.get()).is_equal_to(&0);
}
