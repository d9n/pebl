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
    assert_that(& **property.get()).is_equal_to(&0);
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
fn property_clear() {
    let mut p = Property::new(10);
    p.clear();
    assert_that(p.get()).is_equal_to(&0);

    let mut p = Property::new(Option::from("Hello"));
    p.clear();
    assert_that(p.get()).is_none();
}

#[test]
fn property_ref() {
    let p = Property::new(123);
    let p_ref = PropertyRef::new(&p);

    assert_that(p_ref.get()).is_equal_to(&123);
}

#[test]
fn property_mut_ref() {
    let mut p = Property::new(123);
    let mut p_mut_ref = PropertyMutRef::new(&mut p);

    assert_that(p_mut_ref.get()).is_equal_to(&123);
    p_mut_ref.set(321);
    assert_that(p_mut_ref.get()).is_equal_to(&321);
}

#[test]
fn property_takes_ownership() {
    let range: Vec<_> = (0..100).collect();
    let _ = Property::new(range);
//    let range_already_taken = range; // As expected, uncommenting causes compile error
}

#[test]
fn property_classes_implement_debug() {
    let mut p = Property::new(42);

    let p_string = format!("{:?}", p);
    assert_that(&p_string.len()).is_greater_than(0);
    let p_string = format!("{:?}", PropertyRef::new(&p));
    assert_that(&p_string.len()).is_greater_than(0);
    let p_string = format!("{:?}", PropertyMutRef::new(&mut p));
    assert_that(&p_string.len()).is_greater_than(0);
}