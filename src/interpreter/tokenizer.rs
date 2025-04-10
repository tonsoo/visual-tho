use super::tokens::schema::TokenTypes;
use super::{language::Language, tokens::token::{Token, TokenIndex}};
use super::tokens::separator::{SeparatorSetting, TokenSeparators};

struct Range<T> {
    min:T,
    max:T,
}

impl<T: PartialOrd + Clone> Range<T> {
    fn new(min:T, max:T) -> Self {
        Self { min, max }
    }

    fn min(&mut self, min:T) {
        if min < self.min {
            self.min = min;
        }
    }
    fn max(&mut self, max:T) {
        if max > self.max {
            self.max = max;
        }
    }
    fn range(&mut self, value:T) {
        self.max(value.clone());
        self.min(value);
    }
}

pub struct Tokenizer {
    language:Box<dyn Language>,
    tokens:Vec<Token>,
    index:usize,
}

#[derive(Clone)]
struct Separator {
    value:String,
    settings:SeparatorSetting,
}

impl Separator {
    fn new(value:String, settings:SeparatorSetting) -> Self {
        Self { value, settings }
    }
}

impl Tokenizer {
    pub fn new(language:Box<dyn Language>) -> Self {
        Self { language, tokens: vec![], index: 0 }
    }

    fn get_max_range(&self, separators:&Vec<Separator>) -> Range<usize> {
        if separators.len() == 0 {
            return Range::new(0, 0);
        }
        
        let mut min = 0;
        let mut max = 0;

        if let Some(_min) = separators.iter().min_by_key(|a| a.value.len()) {
            min = _min.value.len();
        }

        if let Some(_max) = separators.iter().max_by_key(|a| a.value.len()) {
            max = _max.value.len();
        }

        Range::new(min, max)
    }

    fn get_ordered_separators(&self) -> Vec<Separator> {
        use TokenSeparators::*;

        let separators = self.language.separators();

        let mut new_separators:Vec<Separator> = vec![];

        for s in separators {
            match s {
                Alpha { alpha, settings }
                    => new_separators.push(Separator::new(String::from(alpha), settings)),
                InAlphaRange { alphas, settings } => {
                    for a in alphas {
                        new_separators.push(Separator::new(String::from(a), settings.clone()))
                    }
                },

                Word { word, settings } => new_separators.push(Separator::new(word, settings)),
                InWordRange { words, settings } => {
                    for w in words {
                        new_separators.push(Separator::new(w, settings.clone()));
                    }
                }
            }
        }

        new_separators.sort_by(|a, b| b.value.len().cmp(&a.value.len()));

        new_separators
    }

    pub fn tokenize(&mut self, code:&str) {
        self.tokens = vec![];
        self.index = 0;
        
        let separators = self.get_ordered_separators();

        let buffer_range = self.get_max_range(&separators);

        let mut buffer = String::new();
        let mut arbitrary_buffer = String::new();
        let mut can_count = false;
        let mut skip_logic_for = 0;
        for char in code.chars() {
            if skip_logic_for > 0 {
                skip_logic_for -= 1;
            }
            
            if can_count {
                self.index += 1;
            } else {
                can_count = true;
            }

            let char_text = String::from(char);
            buffer += &char_text.to_string();

            if buffer.len() < buffer_range.max {
                continue;
            }

            self.bufferize(&mut buffer, &separators);

            if buffer.len() >= buffer_range.max {
                let mut chars = buffer.chars();
                
                if let Some(a_char) = chars.next() {
                    arbitrary_buffer += &String::from(a_char);
                }
                
                buffer = chars.collect();
            } else if arbitrary_buffer.len() > 0 {
                let popped = self.pop_token();

                let start = (self.index + 1) - buffer_range.max - arbitrary_buffer.len();
                self.push_token(
                    Token::new(
                        TokenIndex::new(start, start + arbitrary_buffer.len()),
                        arbitrary_buffer.clone(),
                        TokenTypes::Custom { name: arbitrary_buffer.clone() },
                    )
                );

                if let Some(token) = popped {
                    self.push_token(token);
                }

                arbitrary_buffer.clear();
            }
        }

        let mut loop_index = 0;
        while loop_index < buffer.len() {
            let skip = self.bufferize(&mut buffer, &separators);

            if skip > 0 {
                loop_index += skip;
            } else {
                loop_index += 1;
            }
        }
    }

    fn bufferize(&mut self, buffer:&mut String, separators:&Vec<Separator>) -> usize {
        let skip_chars = self.tokenize_buffer_piece(buffer.clone(), &separators);

        if skip_chars > 0 {
            self.index += skip_chars;

            let mut chars = buffer.chars();

            for _ in 0..skip_chars {
                chars.next();
            }

            *buffer = chars.collect();
        }

        skip_chars
    }

    fn tokenize_buffer_piece(&mut self, buffer:String, separators:&Vec<Separator>) -> usize {
        for separator in separators {
            let mut compare = buffer.clone();
            let value = separator.value.clone();
            let schema = separator.settings.map().clone();
            
            if !separator.settings.is_case_sensitive() {
                compare = compare.to_lowercase();
            }
            
            if !compare.starts_with(&value) {
                continue
            }

            let value_length = value.len();
            if separator.settings.is_inclusive() {
                let start = (self.index + 1) - compare.len();
                self.push_token(
                    Token::new(
                        TokenIndex::new(start, start + value_length),
                        value.clone(),
                        schema
                    )
                );
            }

            return value_length
        }

        0
    }

    fn push_token(&mut self, token:Token) {
        self.tokens.push(token);
    }

    fn pop_token(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub fn print(&self) {
        for t in &self.tokens {
            println!("{}[{}]-{}", t.value(), t.schema().to_string(), t.index().to_string());
        }
    }
}