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

#[derive(Debug,PartialEq)]
pub enum Object {
    Void, // an empty object
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>)
}
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


// input: Vec<tokens>
// output: AST
// desc: recursively parses tokens into an Object
fn parse(tokens: &mut Vec<Token>) -> Object {
    tokens.reverse();
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
                    let sub_list = parse(tokens);
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


fn main() {
    let mut tokens = lex("(+ 1 2)");
    let ast = parse(&mut tokens);
    println!("{:?}", &ast);
}
