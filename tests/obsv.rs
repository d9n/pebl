#[macro_use]
extern crate spectral;
extern crate pebl;

use std::cell::Cell;
use std::rc::Rc;
use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn default_values() {
    let value = Observable::<i32>::default();
    assert_that(value.get()).is_equal_to(&0);

    let value = Observable::<bool>::default();
    assert_that(value.get()).is_false();

    let value = Observable::<f64>::default();
    assert_that(value.get()).is_equal_to(&0.0);

    let value = Observable::<Option<i32>>::default();
    assert_that(value.get()).is_none();

    let value = Observable::<Rc<i32>>::default();
    assert_that(&**value.get()).is_equal_to(&0);
}

#[test]
fn value_simple_get() {
    let p = Observable::new(10);
    assert_that(p.get()).is_equal_to(&10);
    assert_that(p.get()).is_equal_to(&10);
}

#[test]
fn value_simple_set() {
    let mut p = Observable::new(10);
    p.set(20);
    assert_that(p.get()).is_equal_to(&20);
}

#[test]
fn value_clear() {
    let mut p = Observable::new(10);
    p.clear();
    assert_that(p.get()).is_equal_to(&0);

    let mut p = Observable::new(Option::from("Hello"));
    p.clear();
    assert_that(p.get()).is_none();
}

#[test]
fn value_takes_ownership() {
    let range: Vec<_> = (0..100).collect();
    let _ = Observable::new(range);
//    let range_already_taken = range; // As expected, uncommenting causes compile error
}

#[test]
fn value_can_be_observed_for_changes() {
    let mut o = Observable::new(10);
    let count = Rc::new(Cell::new(0));
    let handler;
    {
        let count = count.clone();
        handler = InvalidationHandler::new(move || count.set(count.get() + 1));
    }
    o.add_invalidation_handler(&handler);

    assert_that(&count.get()).is_equal_to(&0);

    o.set(20);
    assert_that(&count.get()).is_equal_to(&1);

    o.set(30);
    assert_that(&count.get()).is_equal_to(&2);

    o.set(30);
    assert_that(&count.get()).is_equal_to(&2);
}

#[test]
fn invalidation_handler_not_called_after_dropping() {
    let mut o = Observable::new(10);
    let count = Rc::new(Cell::new(0));

    let handler;
    {
        let count = count.clone();
        handler = InvalidationHandler::new(move || count.set(count.get() + 1));
    }
    o.add_invalidation_handler(&handler);

    o.set(20);
    assert_that(&count.get()).is_equal_to(&1);

    drop(handler);

    o.set(30);
    assert_that(&count.get()).is_equal_to(&1);
}

#[test]
fn value_classes_implement_debug() {
    let p = Observable::new(42);

    let p_string = format!("{:?}", p);
    assert_that(&p_string.as_str()).is_equal_to(&"Observable { 42 }");

    let mut p_ptr = ObservablePtr::new(&p);
    let p_string = format!("{:?}", p_ptr);
    assert_that(&p_string.as_str()).is_equal_to(&"*Observable { 42 }");

    {
        let p_ref = p_ptr.deref();
        let p_string = format!("{:?}", p_ref);
        assert_that(&p_string.as_str()).is_equal_to(&"&Observable { 42 }");
    }

    {
        let p_ref_mut = p_ptr.deref_mut();
        let p_string = format!("{:?}", p_ref_mut);
        assert_that(&p_string.as_str()).is_equal_to(&"&mut Observable { 42 }");
    }

    drop(p);
    let p_string = format!("{:?}", p_ptr);
    assert_that(&p_string.as_str()).is_equal_to(&"*Observable { null }");
}

#[test]
fn value_ptr_wraps_target_value() {
    let p = Observable::new(String::from("Hello"));
    let mut p_ptr = ObservablePtr::new(&p);
    {
        let p_ref = p_ptr.deref();
        assert_that(p_ref.get()).is_equal_to(String::from("Hello"));
    }

    {
        let mut p_ref = p_ptr.deref_mut();
        p_ref.set(String::from("World"));
        assert_that(p_ref.get()).is_equal_to(String::from("World"));
    }

    assert_that(p.get()).is_equal_to(String::from("World"));

    drop(p);
    assert_that(&p_ptr.try_deref()).is_none();
    assert_that(&p_ptr.try_deref_mut()).is_none();
}

#[test]
#[should_panic(expected = "value already immutably borrowed")]
#[allow(unused_variables)] // Variables needed to keep value references alive
fn getting_read_then_write_values_from_ptr_panics() {
    let p = Observable::new(10);
    let p_ptr1 = ObservablePtr::new(&p);
    let mut p_ptr2 = ObservablePtr::new(&p);

    let p_val_imm = p_ptr1.deref();
    let p_val_mut = p_ptr2.deref_mut();
}

#[test]
#[should_panic(expected = "value already mutably borrowed")]
#[allow(unused_variables)] // Variables needed to keep value references alive
fn getting_write_then_read_values_from_ptr_panics() {
    let p = Observable::new(10);
    let p_ptr1 = ObservablePtr::new(&p);
    let mut p_ptr2 = ObservablePtr::new(&p);

    let p_val_mut = p_ptr2.deref_mut();
    let p_val_imm = p_ptr1.deref();
}

#[test]
#[should_panic(expected = "value already mutably borrowed")]
#[allow(unused_variables)] // Variables needed to keep value references alive
fn getting_multiple_write_values_from_ptr_panics() {
    let p = Observable::new(10);
    let mut p_ptr1 = ObservablePtr::new(&p);
    let mut p_ptr2 = ObservablePtr::new(&p);

    let p_val_mut1 = p_ptr1.deref_mut();
    let p_val_mut2 = p_ptr2.deref_mut();
}
