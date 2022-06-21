mod lexer;
mod parser;
mod eval;
use crate::parser::parse;
use crate::eval::eval;

/* TODO:
 * - implement list evaluations
 * - implement default environment
 * - write tests
 * - write repl
 */
fn main() {
    let ast = parse("(+(+ 1 2)1)");
    println!("ast: {:?}", &ast);
    eval(&ast);
}
