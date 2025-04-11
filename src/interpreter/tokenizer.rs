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

        if let Some(_min) = separators.iter().min_by_key(|a| a.value.chars().count()) {
            min = _min.value.chars().count();
        }

        if let Some(_max) = separators.iter().max_by_key(|a| a.value.chars().count()) {
            max = _max.value.chars().count();
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

        new_separators.sort_by(|a, b|
            b.value.chars().count().cmp(&a.value.chars().count())
        );

        new_separators
    }

    pub fn tokenize(&mut self, code:&str) {
        self.tokens = vec![];
        self.index = 0;
        
        let separators = self.get_ordered_separators();

        let buffer_range = self.get_max_range(&separators);

        let mut buffer = String::new();
        let mut can_count = false;
        
        for (i, char) in code.char_indices() {
            self.index = i;

            let char_text = String::from(char);
            buffer += &char_text;

            if buffer.chars().count() < buffer_range.max {
                continue;
            }

            self.bufferize(&mut buffer, &separators);

            if buffer.chars().count() >= buffer_range.max {
                let mut chars = buffer.chars();
                if let Some(a_char) = chars.next() {
                    self.push_unknown_character(a_char, chars.clone().count());
                }
                buffer = chars.collect();
            }
        }

        let mut loop_index = 0;
        while loop_index < buffer.chars().count() {
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
            
            if !separator.settings.is_case_sensitive() {
                compare = compare.to_lowercase();
            }
            
            if !compare.starts_with(&value) {
                continue
            }

            let schema = separator.settings.map().clone();

            let value_length = value.chars().count();
            if separator.settings.is_inclusive() {
                let length = compare.chars().count() - 1;
                let mut start = 0;
                if self.index > length {
                    start = self.index - length;
                }
                self.push_token(
                    Token::new(
                        TokenIndex::new(start, start + value_length),
                        value.clone(),
                        schema
                    ),
                    true
                );
            }

            return value_length
        }

        0
    }

    fn push_unknown_character(&mut self, char:char, buffer_length:usize) {
        let last = self.tokens.last();
        let mut new_token:Option<Token> = None;
        let mut index = 0;
        if self.index > buffer_length {
            index = self.index - buffer_length;
        }
        if let Some(token) = last {
            if let TokenTypes::Custom { name } = token.schema() {
                if name == "unknown" {
                    let mut value = token.value().to_string();
                    value.push_str(&String::from(char));
                    new_token = Some(Token::new(
                        TokenIndex::new(token.index().start(), index),
                        value,
                        TokenTypes::Custom { name: String::from("unknown") }
                    ));
                    self.pop_token();
                }
            }
        }

        if new_token.is_none() {
            new_token = Some(Token::new(
                TokenIndex::new(index, index + 1),
                String::from(char),
                TokenTypes::Custom { name: String::from("unknown") }
            ));
        }

        if let Some(token) = new_token {
            self.tokens.push(token);
        }
    }

    fn push_token(&mut self, token:Token, group:bool) {
        if group {
            let last = self.tokens.last();
            if let Some(last_token) = last {
                if last_token.value() == token.value() {
                    let mut popped = self.pop_token().unwrap();
                    popped.increase_count();
                    self.push_token(popped, true);
                    return;
                }
            }
        }
        
        self.tokens.push(token);
    }

    fn pop_token(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub fn print(&self) {
        for t in &self.tokens {
            let mut value = t.value().clone();
            for _ in 1..*t.count() {
                value += &t.value();
            }
            println!("{} [{}] - {}", value, t.schema().to_string(), t.index().to_string());
        }
    }
}