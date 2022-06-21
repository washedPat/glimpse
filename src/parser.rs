use crate::lexer::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void, // an empty object
    Integer(f64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>)
}


pub fn parse(program: &str) -> Object {
    let mut tokens = lex(program);
    tokens.reverse();
    let parsed_list = parse_list(&mut tokens);
    parsed_list
}
// input: Vec<tokens>
// output: AST
// desc: recursively parses tokens into an Object
fn parse_list(tokens: &mut Vec<Token>) -> Object {
    if let Some(token) = tokens.pop() {
        if token != Token::LParen {
            panic!("Error: Expected LParen") 
        }
    } else {
        panic!("Error: no tokens!")
    };

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        if let Some(token) = tokens.pop() {
            match token {
                Token::Integer(n) => list.push(Object::Integer(n)),
                Token::Symbol(s) => list.push(Object::Symbol(s)),
                Token::LParen => {
                    tokens.push(Token::LParen);
                    let sub_list = parse_list(tokens);
                    list.push(sub_list);
                },
                Token::RParen => return Object::List(list),
            }
        }else {
            panic!("Error: Insufficient tokens")
        }
    }

    Object::List(list)
}
