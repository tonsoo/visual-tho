use super::{rules::rule::SyntaxRule, tokenizer::Tokenizer};

pub struct Syntaxer {
    tokenizer:Tokenizer,
}

impl Syntaxer {
    pub fn new(tokenizer:Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub fn validate_syntax(&self, rules:Vec<SyntaxRule>) {
        
    }
}