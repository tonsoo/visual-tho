use super::{language::Language, tokenizer::Tokenizer};

pub struct Interpreter {
    code:Option<String>,
    file_path:Option<String>
}

impl Interpreter {
    pub fn from_code(code:String) -> Self {
        Self { code: Some(code), file_path: None }
    }

    pub fn from_file(file:String) -> Self {
        Self { code: None, file_path: Some(file) }
    }

    fn load_file(&mut self, file:String) {
        match std::fs::read_to_string(file) {
            Ok(text) => {
                self.code = Some(text);
            }
            Err(e) => {
                println!("Filed to read file: {}", e);
            }
        }
    }

    pub fn interpret(&mut self, language:Box<dyn Language>) {
        let mut has_file = false;
        if let Some(file) = self.file_path.clone() {
            has_file = true;
            self.load_file(file);
        }

        let mut tokenizer = Tokenizer::new(language);

        if let Some(code) = self.code.clone() {
            tokenizer.tokenize(&code);

            tokenizer.print();
        }

        if has_file {
            self.code = None;
        }
    }
}