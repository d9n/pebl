use std::rc::Rc;

use expr::IntoExpression;
use obsv::InvalidationHandler;

pub struct Listeners {
    handlers: Vec<InvalidationHandler>,
}

pub struct ListenChain<'a> {
    owner: &'a mut Listeners,
    register_callbacks: Vec<Box<Fn(&InvalidationHandler)>>,
}

impl Listeners {
    pub fn new() -> Listeners {
        Listeners { handlers: Vec::new() }
    }
    
    #[must_use]
    pub fn listen_to<T: 'static + PartialEq, E: IntoExpression<T>>(&mut self, target: E) -> ListenChain {
        let expr = target.into_expr();
        let mut lc = ListenChain { owner: self, register_callbacks: Vec::with_capacity(1) };
        lc.register_callbacks.push(Box::new(move |handler| expr.add_invalidation_handler(handler)));
        lc
    }

    pub fn release_all(&mut self) {
        self.handlers.clear();
    }
}

impl<'a> ListenChain<'a> {
    #[must_use]
    pub fn and<T: 'static + PartialEq, E: IntoExpression<T>>(self, target: E) -> ListenChain<'a> {
        let expr = target.into_expr();
        let mut lc = ListenChain { owner: self.owner, register_callbacks: self.register_callbacks };
        lc.register_callbacks.push(Box::new(move |handler| expr.add_invalidation_handler(handler)));
        lc
    }

    pub fn with<F: 'static + Fn()>(self, f: F) {
        let mut handler = InvalidationHandler::new(f);
        for r in &self.register_callbacks {
            r(&handler);
        }
        self.owner.handlers.push(handler);
    }
}
