#[macro_use]
extern crate spectral;
extern crate pebl;

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
fn property_set_box_takes_ownership() {
  let range: Vec<_> = (0..100).collect();
  let range_ptr = Box::new(range);
  let p = Property::new(range_ptr);
  assert_that(&**p.get()).has_length(100);
  assert_that(&**p.get()).has_length(100); // get() is borrow; can read twice
}
