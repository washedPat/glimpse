
#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(f64),
    Symbol(String),
    LParen,
    RParen
}

// input: (+ 1 2)
// output: Result<Vec<Token>>
// desc: parses string into token list
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new(); 
    let program = String::from(input);
    let temp = program.replace("(", " ( ").replace(")", " ) ");
    let words = temp.split_whitespace();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                if let Ok(i) = word.parse::<f64>() {
                    tokens.push(Token::Integer(i))
                } else {
                    tokens.push(Token::Symbol(word.to_string()))
                }
            }
        }
    }
    tokens
}
