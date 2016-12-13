use std::cell::RefCell;
use std::rc::Rc;
use weak::WeakList;

pub trait Observer<A> {
    fn on_invalidated(&mut self, arg: Rc<A>);
}

pub trait Observable<A, O>
where O: Observer<A> {
    fn arg(&self) -> Rc<A>;
    fn add_observer(&mut self, o: &Rc<RefCell<O>>) -> ObserverWrapper<A, O> {
        self.observers_mut().push(o);
        ObserverWrapper::new(self.arg(), o.clone())
    }
    fn observers_mut(&mut self) -> &mut WeakList<RefCell<O>>;
    fn observers(&self) -> &WeakList<RefCell<O>>;
    fn fire(&self) {
        for o in self.observers().iter() {
            o.borrow_mut().on_invalidated(self.arg());
        }
    }
}

pub struct ObserverWrapper<A, O>
    where O: Observer<A> {
    arg: Rc<A>,
    observer: Rc<RefCell<O>>,
}

impl<A, O> ObserverWrapper<A, O>
where O: Observer<A> {
    pub fn new(arg: Rc<A>, observer: Rc<RefCell<O>>) -> ObserverWrapper<A, O> {
        ObserverWrapper { arg: arg, observer: observer }
    }

    pub fn fire(&mut self) {
        self.observer.borrow_mut().on_invalidated(self.arg.clone());
    }
}
