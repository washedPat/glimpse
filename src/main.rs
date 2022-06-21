#[allow(dead_code)]
mod lexer;
mod parser;
mod eval;
mod env;
use crate::parser::parse;
use crate::eval::eval;

/* TODO:
 * - implement list evaluations
 * - implement default environment
 * - write tests
 * - write repl
 */
fn main() {
    let ast = parse("(+ 1.5 2)");
    println!("ast: {:?}", &ast);
    eval(&ast);
}
