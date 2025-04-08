use super::interpreter::Interpreter;

pub enum TokenTypes {
    Keyword,
    Identifier,
    Number,
    Operator,
    Symbol,
}

pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Operator(char),
    Symbol(char),
}

pub struct TokenRule {
    sequence: Vec<Token>,
}

impl TokenRule {
    pub fn new(rules: Vec<Token>) -> TokenRule {
        TokenRule { sequence: rules }
    }
}

impl Token {
    pub fn get_type(&self) -> TokenTypes {
        match self {
            Token::Keyword(_) => TokenTypes::Keyword,
            Token::Identifier(_) => TokenTypes::Identifier,
            Token::Number(_) => TokenTypes::Number,
            Token::Operator(_) => TokenTypes::Operator,
            Token::Symbol(_) => TokenTypes::Symbol,
        }
    }
}

impl Interpreter {
    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer = String::new();
    
        for char in self.get_code().split("") {
            buffer += char;
        }
    
        tokens
    }
}
