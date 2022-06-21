use crate::parser::Object;
use std::rc::Rc;
use std::cell::RefCell;
use crate::env::Env;

fn eval_binary_op(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Object {
    if list.len() != 3 {
        panic!("invalid number of args for infix op")
    }
    let op = list[0].clone();
    let left = eval(&list[1].clone(), env);
    let right = eval(&list[2].clone(), env);
    let left_val = match left {
        Object::Integer(n) => n,
        _ => panic!("left operand must be integer")
    };
    let right_val = match right  {
        Object::Integer(n) => n,
        _ => panic!("right opernad must be integer")
    };

    match op {
        Object::Symbol(s) => match s.as_str() {
            "+" => return Object::Integer(left_val + right_val),
            "-" => return Object::Integer(left_val - right_val),
            "*" => return Object::Integer(left_val * right_val),
            "/" => return Object::Integer(left_val / right_val),
            ">" => return Object::Bool(left_val > right_val),
            ">=" => return Object::Bool(left_val>= right_val),
            "<" => return Object::Bool(left_val < right_val),
            "<=" => return Object::Bool(left_val <= right_val),
            _ => todo!("operator `{}` not implemented yet", s)
        },
        _ => panic!("operator must be symbol")
    };
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Object{
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | ">" | ">=" | "<" | "<=" => {
                return eval_binary_op(&list, env);
            }
           _ => todo!() 
        },
        _ => todo!()
    }
}

fn eval_symbol(sym: &String, env: &mut Rc<RefCell<Env>>) -> Object{
    let val = env.borrow_mut().get(sym);
    if val.is_none() {
        panic!("symbol not found")
    }
    val.unwrap().clone()
}

pub fn eval(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Object {
    match obj {
        Object::List(list) => eval_list(list, env),
        Object::Void => Object::Void,
        Object::Lambda(_params, _body) => Object::Void,
        Object::Bool(_) => obj.clone(),
        Object::Integer(n) => Object::Integer(*n), 
        Object::Symbol(s) => eval_symbol(s, env)
    }
}
