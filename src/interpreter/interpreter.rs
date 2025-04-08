use super::{language::Language, token::Token};

pub struct Interpreter {
    language: Option<Language>,
    code: String,
    pub tokens: Vec<Token>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            code: String::from(""),
            language: None,
            tokens: vec![]
        }
    }
    
    pub fn from_file(file_name:&str) -> Interpreter {
        let mut inter = Interpreter::new();

        match std::fs::read_to_string(file_name) {
            Ok(text) => {
                inter.load_code(&text);
            }
            Err(e) => {
                println!("Filed to read file: {}", e);
            }
        }
        
        inter
    }

    pub fn get_language(&self) -> &Option<Language> {
        &self.language
    }

    pub fn get_mut_language(&mut self) -> &mut Option<Language> {
        &mut self.language
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = Some(language)
    }

    pub fn load_code(&mut self, code: &str) {
        self.code = String::from(code);
    }

    pub fn get_code(&self) -> String {
        self.code.to_string()
    }
}