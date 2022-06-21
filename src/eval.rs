use crate::parser::Object;

fn eval_list(list: &Vec<Object>) -> Object{
    todo!("implement list evaluations")
}
fn eval_symbol(sym: &String) -> Object{
    todo!("implement symbol evaluations")
}

pub fn eval(obj: &Object) -> Object {
    match obj {
        Object::List(list) => eval_list(list),
        Object::Void => Object::Void,
        Object::Lambda(_params, _body) => Object::Void,
        Object::Bool(_) => obj.clone(),
        Object::Integer(n) => Object::Integer(*n), 
        Object::Symbol(s) => eval_symbol(s)
    }
}
