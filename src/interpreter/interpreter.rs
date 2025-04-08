use super::{language::Language, rules::{TokenRule, TokenRuleItem}, token::Token};

pub struct Interpreter {
    language: Option<Language>,
    code: String,
    pub tokens: Vec<Token>,
}

struct Range {
    min: usize,
    max: usize
}

impl Range {
    fn min(&mut self, value: usize) {
        if value < self.min {
            self.min = value;
        }
    }

    fn max(&mut self, value: usize) {
        if value > self.max {
            self.max = value;
        }
    }
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

    pub fn get_rules(&self) -> Option<&Vec<TokenRule>> {
        match self.get_language() {
            Some(lang) => Some(lang.get_rules()),
            None => None
        }
    }


    pub fn get_formatted_code(&mut self) -> String {
        let mut code = self.get_code();

        // TODO: Manually replace to avoid errors like "\n" inside strings
        code.replace("\n", " ")
            .replace("  ", " ")
    }

    pub fn get_buffer_range(&self) -> Range {
        let mut range = Range { min: usize::MAX, max: 0 };

        let rules = self.get_rules();

        if rules.is_none() {
            return range;
        }

        for rule in rules.unwrap() {
            for token in rule.get_sequence() {
                let mut compare_text = String::new();
                match token {
                    TokenRuleItem::Keyword(text) => {
                        compare_text = text.to_string();
                    }
                    TokenRuleItem::OneOf(list) => {
                        let min = list.iter().min_by_key(|s| s.len());
                        if min.is_some() {
                            compare_text = min.unwrap().to_string();
                        }
                    }
                    TokenRuleItem::Wrapped(start, end) => {
                        if start.len() < end.len() {
                            compare_text = start.to_string();
                        } else {
                            compare_text = end.to_string();
                        }
                    }
                    _ => {}
                }

                range.max(compare_text.len());
                range.min(compare_text.len());
            }
        }

        range
    }

    pub fn tokenize(&mut self) {
        if self.get_language().is_none() {
            // TODO: Error
            println!("Select a language to continue.");
            return;
        }

        let Range { min: min_buffer_size, max: max_buffer_size }
            = self.get_buffer_range();
        
        self.tokens = Vec::new();
        let mut buffer = String::new();
    
        let mut code = self.get_formatted_code();
        let rules = self.get_rules();

        for char in code.chars() {
            buffer += &char.to_string();

            if buffer.len() < min_buffer_size {
                continue;
            }

            

            if buffer.len() > max_buffer_size {
                print!("Previous: {}, ", buffer);

                let mut buffer_chars = buffer.chars();
                buffer_chars.next();
                buffer = buffer_chars.collect();
                
                println!("New: {}", buffer);
            }
        }
    }
}