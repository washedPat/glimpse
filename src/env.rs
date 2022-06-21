use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::parser::Object;

#[derive(Default)]
pub struct Env {
   parent: Option<Rc<RefCell<Env>>>,
   vars: HashMap<String, Object>
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn get(&self, name: &str) -> Option<Object> {
        // look in current scope
        // if not found in current scope look in parent
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone())
        }
    }
}

pub fn new_env() -> Rc<RefCell<Env>> {
    Rc::new(RefCell::new(Env::new()))
}
