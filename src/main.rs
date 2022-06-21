#[allow(dead_code)]
mod lexer;
mod parser;
mod eval;
mod env;
use crate::parser::parse;
use crate::eval::eval;
use crate::env::new_env;
use std::io;
use promptly::prompt;

/* TODO:
 * - implement list evaluations
 * - implement default environment
 * - write tests
 * - write repl
 */

fn readline(s: &mut String) -> io::Result<()> {
    io::stdin().read_line(s)?;
    return Ok(())
}

fn main() {
    loop {
        let input: String = prompt("=> ");
        let program = input.as_str(); 
        let ast = parse(program);
        let mut env = new_env(); 
        let result = eval(&ast, &mut env);
        println!("result: {:?}", result);
    }
}
