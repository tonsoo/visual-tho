use super::token::{TokenRule, TokenTypes};

pub struct Interpreter {
    code: String,
    rules: Vec<TokenRule>
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            code: String::from(""),
            rules: Vec::new(),
        }
    }

    pub fn set_rules(&mut self, rules: Vec<TokenRule>) {
        self.rules = rules
    }

    pub fn get_rules(&self) -> &Vec<TokenRule> {
        &self.rules
    }

    fn set_code(&mut self, code: &str) {
        self.code = String::from(code);
    }

    pub fn get_code(&self) -> String {
        self.code.to_string()
    }
    
    pub fn from_file(file_name:&str) -> Interpreter {
        let mut inter = Interpreter::new();

        match std::fs::read_to_string(file_name) {
            Ok(text) => {
                inter.set_code(&text);
            }
            Err(e) => {
                println!("Filed to read file: {}", e);
            }
        }
        
        inter
    }
}