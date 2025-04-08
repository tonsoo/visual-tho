use eframe::egui_glow::CallbackFn;

use super::interpreter::Interpreter;

pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Operator(char),
    Symbol(char),
}

impl Token {
    
}

impl Interpreter {
    pub fn tokenize(&mut self) {
        let mut min_buffer_size:i32 = 0;
        let mut max_buffer_size:i32 = 0;

        if self.get_language().is_none() {
            // TODO: Error
            println!("Select a language to continue.");
            return;
        }
        
        for rule in self.get_language().as_ref().unwrap().get_rules() {
            for token in rule.get_sequence() {
                
            }
        }
        
        self.tokens = Vec::new();
        let mut buffer = String::new();
    
        for char in self.get_code().split("") {
            buffer += char;
        }
    }
}
