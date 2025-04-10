use super::{language::{self, Language}, tokenizer::{self, Tokenizer}};

pub struct Interpreter {
    code:String,
}

impl Interpreter {
    pub fn from_code(code:String) -> Self {
        Self { code }
    }

    pub fn from_file(file:String) -> Self {
        let mut inter = Interpreter::from_code(String::new());

        match std::fs::read_to_string(file) {
            Ok(text) => {
                inter.code = text;
            }
            Err(e) => {
                println!("Filed to read file: {}", e);
            }
        }
        
        inter
    }

    pub fn interpret(&self, language:Box<dyn Language>) {
        let mut tokenizer = Tokenizer::new(language);

        tokenizer.tokenize(&self.code);

        tokenizer.print();
    }
}