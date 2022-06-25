#[allow(dead_code)]
mod lexer;
mod parser;
mod eval;
mod env;
use crate::parser::parse;
use crate::eval::eval;
use crate::env::new_env;
use std::io; 
use std::env::args;
use std::fs;

/* TODO:
 * - write tests
 */

fn readline() -> String {
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Error: failed to read from stdin");
   input
}

fn main() {
    let mut mode = &String::from("repl");
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        mode = &args[1];
    }
    match mode.as_str(){
        "repl" => {
            let mut env = new_env(); 
            loop {
                println!("glimpse => ");
                let input: String = readline();
                let program = input.as_str(); 
                let ast = parse(program);
                let result = eval(&ast, &mut env);
                println!("result: {:?}", result);
            }
        },
        "run" => {
            todo!("running files does not work yet :/");
            if args.len() < 3 {
                panic!("error: no input file provided");
            }
            let filename = &args[2].as_str();
            let input = match fs::read_to_string(filename) {
                Ok(i) => i, 
                Err(e) => panic!("could not find file: `{}`", e)
            };
            let program = input.as_str();
            println!("program:\n {}", program);
            let mut env = new_env(); 
            let ast = parse(program);
            println!("ast: {:?}", &ast);
            let result = eval(&ast, &mut env);
            println!("result: {:?}", result);
        }
        _ => {
            println!("mode not recognized: `{}`", mode);
            return;
        }
    }
}
