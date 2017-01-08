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
fn property_classes_implement_debug() {
    let p = Property::new(42);

    let p_string = format!("{:?}", p);
    assert_that(&p_string.as_str()).is_equal_to(&"Property { 42 }");

    let mut p_ptr = PropertyPtr::new(&p);
    let p_string = format!("{:?}", p_ptr);
    assert_that(&p_string.as_str()).is_equal_to(&"*Property { 42 }");

    {
        let p_ref = p_ptr.get().unwrap();
        let p_string = format!("{:?}", p_ref);
        assert_that(&p_string.as_str()).is_equal_to(&"&Property { 42 }");
    }

    {
        let p_ref_mut = p_ptr.get_mut().unwrap();
        let p_string = format!("{:?}", p_ref_mut);
        assert_that(&p_string.as_str()).is_equal_to(&"&mut Property { 42 }");
    }

    drop(p);
    let p_string = format!("{:?}", p_ptr);
    assert_that(&p_string.as_str()).is_equal_to(&"*Property { null }");
}

#[test]
fn property_ptr_wraps_target_property() {
    let p = Property::new(String::from("Hello"));
    let mut p_ptr = PropertyPtr::new(&p);
    {
        let p_ref = p_ptr.get().unwrap();
        assert_that(p_ref.get()).is_equal_to(String::from("Hello"));
    }

    {
        let mut p_ref = p_ptr.get_mut().unwrap();
        p_ref.set(String::from("World"));
        assert_that(p_ref.get()).is_equal_to(String::from("World"));
    }

    assert_that(p.get()).is_equal_to(String::from("World"));

    drop(p);
    assert_that(&p_ptr.get()).is_none();
    assert_that(&p_ptr.get_mut()).is_none();
}

#[test]
#[should_panic]
#[allow(unused_variables)] // Variables needed to keep property references alive
fn getting_read_and_write_properties_from_ptr_panics() {
    let p = Property::new(10);
    let p_ptr1 = PropertyPtr::new(&p);
    let mut p_ptr2 = PropertyPtr::new(&p);

    let p_val_imm = p_ptr1.get().unwrap();
    let p_val_mut = p_ptr2.get_mut().unwrap();
}

#[test]
#[should_panic]
#[allow(unused_variables)] // Variables needed to keep property references alive
fn getting_multiple_write_properties_from_ptr_panics() {
    let p = Property::new(10);
    let mut p_ptr1 = PropertyPtr::new(&p);
    let mut p_ptr2 = PropertyPtr::new(&p);

    let p_val_mut1 = p_ptr1.get_mut().unwrap();
    let p_val_mut2 = p_ptr2.get_mut().unwrap();
}
