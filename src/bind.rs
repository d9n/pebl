use std::collections::HashMap;

use property::*;

pub struct Bindings {
    update_callbacks: HashMap<usize, Box<FnMut()>>,
}

impl Bindings {
    pub fn new() -> Bindings {
        Bindings { update_callbacks: HashMap::new() }
    }

    pub fn bind<T: 'static + PartialEq + Clone>(&mut self, p_dest: &mut Property<T>, p_src: &AsRef<Property<T>>) {
        let mut p_dest_ptr = PropertyPtr::new(p_dest);
        let p_src_ptr = PropertyPtr::new(p_src.as_ref());

        let key = p_dest.id();
        let mut update = Box::new(move || {
            p_dest_ptr.deref_mut().set(p_src_ptr.deref().get().clone());
        });

        (*update)();
        self.update_callbacks.insert(key, update);
    }

    pub fn update(&mut self) {
        for (_, callback) in &mut self.update_callbacks {
            (*callback)();
        }
    }

    pub fn unbind<T: PartialEq>(&mut self, p_dest: &Property<T>) {
        self.update_callbacks.remove(&p_dest.id());
    }

    pub fn clear(&mut self) {
        self.update_callbacks.clear();
    }
}