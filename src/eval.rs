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


fn eval_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Object {
    todo!();
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Object {
    if list.len() != 3 {
        panic!("invalid number of args for def");
    }

    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => panic!("invalid define")
    };

    let val = eval(&list[2], env);
    env.borrow_mut().set(&sym, val);
    Object::Void
}

fn eval_func_def(list: &Vec<Object>) -> Object {
    todo!();
}

fn eval_func_call(sym: String, list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Object {
    todo!();
}
 
fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Object {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | ">" | ">=" | "<" | "<=" => {
                return eval_binary_op(&list, env);
            }
            "if" => eval_if(&list, env),
            "def" => eval_define(&list, env),
            "lambda" => eval_func_def(&list),
           _ => todo!() 
        },
        _ => {
            let mut sub_list = Vec::new(); 
            for obj in list {
                let result = eval(obj, env);             
                match result {
                    Object::Void => {},
                    _ => sub_list.push(result),
                }
            }
            Object::List(sub_list)
        }
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
