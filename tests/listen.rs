#[macro_use]
extern crate spectral;
extern crate pebl;

use std::rc::Rc;
use std::cell::Cell;
use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn listener_with_multiple_types_works() {
    let mut l = Listeners::new();
    let mut p_int = Property::<i32>::default();
    let mut p_str = Property::<String>::default();
    let mut p_bool = Property::<bool>::default();
    let listen_count = Rc::new(Cell::new(0));
    {
        let listen_count = listen_count.clone();
        l.listen_to(&p_int).and(&p_str).and(&p_bool).with(move || listen_count.set(listen_count.get() + 1));
    }

    assert_that(&listen_count.get()).is_equal_to(&0);
    p_int.set(10);
    assert_that(&listen_count.get()).is_equal_to(&1);
    p_str.set(String::from("Ten"));
    assert_that(&listen_count.get()).is_equal_to(&2);
    p_bool.set(true);
    assert_that(&listen_count.get()).is_equal_to(&3);

    l.release_all();

    p_int.set(99);
    assert_that(&listen_count.get()).is_equal_to(&3);
    p_str.set(String::from(":("));
    assert_that(&listen_count.get()).is_equal_to(&3);
    p_bool.set(false);
    assert_that(&listen_count.get()).is_equal_to(&3);
}
