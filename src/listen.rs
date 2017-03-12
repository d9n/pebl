use std::rc::Rc;

use expr::IntoExpression;
use obsv::InvalidationHandler;

pub struct Listeners {
    handlers: Vec<Rc<InvalidationHandler>>,
}

pub struct ListenTo {
    handler: Rc<InvalidationHandler>,
}

pub struct ListenAnd {
    handler: Rc<InvalidationHandler>,
}

impl Listeners {
    pub fn new() -> Listeners {
        Listeners { handlers: Vec::new() }
    }
    
    #[must_use]
    pub fn listen_with<F: 'static + Fn()>(&mut self, f: F) -> ListenTo {
        let handler = Rc::new(InvalidationHandler::new(f));
        self.handlers.push(handler.clone());
        ListenTo { handler: handler }
    }

    pub fn release_all(&mut self) {
        self.handlers.clear();
    }
}

impl ListenTo {
    pub fn to<T: PartialEq, E: IntoExpression<T>>(&self, target: E) -> ListenAnd {
        let expr = target.into_expr();
        expr.add_invalidation_handler(&*self.handler);
        ListenAnd { handler: self.handler.clone() }
    }
}

impl ListenAnd {
    pub fn and<T: PartialEq, E: IntoExpression<T>>(&self, target: E) -> ListenAnd {
        let expr = target.into_expr();
        expr.add_invalidation_handler(&*self.handler);
        ListenAnd { handler: self.handler.clone() }
    }
}
