use std::collections::HashMap;
use crate::parser::Object;

pub fn default_env() -> HashMap<String, Object> {
    let env: HashMap<String, Object> = HashMap::new();
    env
}
