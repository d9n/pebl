#[macro_use]
extern crate spectral;
extern crate pebl;

use std::cell::RefCell;
use std::rc::Rc;
use pebl::prelude::*;
use pebl::weak::*;
use spectral::prelude::*;

struct _IntObserver {
    val: i32,
}

struct IntObserver {
    inner_observer: Rc<RefCell<_IntObserver>>,
}

impl IntObserver {
    pub fn new() -> IntObserver {
        IntObserver { inner_observer: Rc::new(RefCell::new(_IntObserver { val: 0 })) }
    }

    pub fn val(&self) -> i32 {
        self.inner_observer.borrow().val
    }
}

impl Observer<i32> for _IntObserver {
    fn on_invalidated(&mut self, arg: Rc<i32>) {
        self.val = *arg;
    }
}

struct _IntObservable {
    val: i32,
    observers: WeakList<RefCell<_IntObserver>>
}

struct IntObservable {
    inner_observable: _IntObservable,
}

impl Observable<i32, _IntObserver> for _IntObservable {
    fn arg(&self) -> Rc<i32> {
        Rc::new(self.val)
    }

    fn observers(&self) -> &WeakList<RefCell<_IntObserver>> {
        &self.observers
    }

    fn observers_mut(&mut self) -> &mut WeakList<RefCell<_IntObserver>> {
        &mut self.observers
    }
}

impl IntObservable {
    pub fn new() -> Self {
        IntObservable { inner_observable: _IntObservable { val: 0, observers: WeakList::new() } }
    }

    pub fn set(&mut self, val: i32) {
        self.inner_observable.val = val;
        self.inner_observable.fire();
    }

    pub fn add_observer(&mut self, o: &IntObserver) -> ObserverWrapper<i32, _IntObserver> {
        self.inner_observable.add_observer(&o.inner_observer)
    }
}


#[test]
fn simple_observer_list() {
    let mut src = IntObservable::new();
    let dest1 = IntObserver::new();
    let dest2 = IntObserver::new();

    src.set(5); // Initial value not automatically picked up by observers
    src.add_observer(&dest1);
    src.add_observer(&dest2);

    assert_that(&dest1.val()).is_equal_to(0);
    assert_that(&dest2.val()).is_equal_to(0);

    src.set(10);
    assert_that(&dest1.val()).is_equal_to(10);
    assert_that(&dest2.val()).is_equal_to(10);
}

#[test]
fn add_and_fire_observer() {
    let mut src = IntObservable::new();
    let dest1 = IntObserver::new();
    let dest2 = IntObserver::new();

    src.set(5); // Initial value not automatically picked up by observers
    src.add_observer(&dest1).fire(); // but calling "fire" forces a refresh
    src.add_observer(&dest2);

    assert_that(&dest1.val()).is_equal_to(5);
    assert_that(&dest2.val()).is_equal_to(0);
}

#[test]
fn observer_scope_works() {
    let mut src = IntObservable::new();
    let dest1 = IntObserver::new();
    {
        let dest2 = IntObserver::new();

        src.add_observer(&dest1);
        src.add_observer(&dest2);
    }

    src.set(10);
    assert_that(&dest1.val()).is_equal_to(10);
}
