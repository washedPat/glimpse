#[allow(dead_code)]
mod lexer;
mod parser;
mod eval;
mod env;
use crate::parser::parse;
use crate::eval::eval;
use crate::env::new_env;
use std::io; 

/* TODO:
 * - implement list evaluations
 * - implement default environment
 * - write tests
 * - write repl
 */

fn readline() -> String {
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Error: failed to read from stdin");
   input
}

fn main() {
    let mut env = new_env(); 
    loop {
        println!("glimpse => ");
        let input: String = readline();
        let program = input.as_str(); 
        let ast = parse(program);
        let result = eval(&ast, &mut env);
        println!("result: {:?}", result);
    }
}
