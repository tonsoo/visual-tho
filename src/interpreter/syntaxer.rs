use super::tokenizer::Tokenizer;

pub struct Syntaxer {
    tokenizer:Tokenizer,
}

impl Syntaxer {
    pub fn new(tokenizer:Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub fn validate_syntax(&self) {
        
    }
}