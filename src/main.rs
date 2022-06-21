#[warn(dead_code)]

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen
}

// input: (+ 1 2)
// output: Result<Vec<Token>>
// desc: parses string into token list
fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new(); 
    let program = String::from(input);
    let temp = program.replace("(", " ( ").replace(")", " ) ");
    let words = temp.split_whitespace();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                if let Ok(i) = word.parse::<i64>() {
                    tokens.push(Token::Integer(i))
                } else {
                    tokens.push(Token::Symbol(word.to_string()))
                }
            }
        }
    }
    tokens
}

#[derive(Debug,PartialEq, Clone)]
pub enum Object {
    Void, // an empty object
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>)
}


fn parse(program: &str) -> Object {
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

fn eval_list(list: &Vec<Object>) -> Object{
    todo!("implement list evaluations")
}
fn eval_symbol(sym: &String) -> Object{
    todo!("implement symbol evaluations")
}
fn eval(obj: &Object) -> Object {
    match obj {
        Object::List(list) => eval_list(list),
        Object::Void => Object::Void,
        Object::Lambda(_params, _body) => Object::Void,
        Object::Bool(_) => obj.clone(),
        Object::Integer(n) => Object::Integer(*n), 
        Object::Symbol(s) => eval_symbol(s)
    }
}

/* TODO:
 * - refactor into separate files
 * - implement list evaluations
 * - implement default environment
 * - write tests
 * - write repl
 */
fn main() {
    let ast = parse("(+(+ 1 2)1)");
    println!("ast: {:?}", &ast);
//    eval(&ast);
}
