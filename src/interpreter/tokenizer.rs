use eframe::glow::TEXTURE_VIEW_MIN_LEVEL;
use egui::util::undoer::Settings;

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
    search_token:Option<TokenizerSearch>
}

struct TokenizerSearch {
    token:Separator,
    skip_content:bool,
}

#[derive(Clone)]
enum Separator {
    Simple {
        value:String,
        settings:SeparatorSetting,
    },

    WithEnd {
        value:String,
        end:Option<String>,
        settings:SeparatorSetting,
    }
}

impl Tokenizer {
    pub fn new(language:Box<dyn Language>) -> Self {
        Self { language, tokens: vec![], index: 0, search_token: None }
    }

    fn get_max_range(&self, separators:&Vec<Separator>) -> Range<usize> {
        if separators.len() == 0 {
            return Range::new(0, 0);
        }
        
        let mut min = 0;
        let mut max = 0;

        let min_fn = |a:&Separator| -> usize {
            match a {
                Separator::Simple { value, .. } => value.chars().count(),
                Separator::WithEnd { value, end, .. } => {
                    let v = value.chars().count();
                    let e = end.as_ref().map_or(v, |e| e.chars().count());
                    v.min(e)
                },
            }
        };

        let max_fn = |a:&Separator| -> usize {
            match a {
                Separator::Simple { value, .. } => value.chars().count(),
                Separator::WithEnd { value, end, .. } => {
                    let v = value.chars().count();
                    let e = end.as_ref().map_or(v, |e| e.chars().count());
                    v.max(e)
                },
            }
        };

        if let Some(_min) = separators.iter().min_by_key(|a| min_fn(a)) {
            min = min_fn(_min);
        }

        if let Some(_max) = separators.iter().max_by_key(|a| max_fn(a)) {
            max = max_fn(_max);
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
                    => new_separators.push(
                        Separator::Simple {
                            value: alpha.to_string(),
                            settings: settings
                        }
                    ),
                InAlphaRange { alphas, settings } => {
                    for a in alphas {
                        new_separators.push(
                            Separator::Simple {
                                value: a.to_string(),
                                settings: settings.clone()
                            }
                        )
                    }
                },
                AlphaUntil { alpha, end, skip_content, settings } => {
                    new_separators.push(
                        Separator::WithEnd {
                            value: alpha.to_string(),
                            end: end.map(|c| c.to_string()),
                            settings: settings
                        }
                    )
                }

                Word { word, settings } => new_separators.push(
                    Separator::Simple {
                        value: word,
                        settings: settings
                    }
                ),
                InWordRange { words, settings } => {
                    for w in words {
                        new_separators.push(
                            Separator::Simple {
                                value: w,
                                settings: settings.clone()
                            }
                        );
                    }
                }
                WordUntil { word, end, skip_content, settings } => new_separators.push(
                    Separator::WithEnd {
                        value: word,
                        end: end,
                        settings: settings
                    }
                )
            }
        }

        new_separators.sort_by(|a, b| {
                let b_value = match b {
                    Separator::Simple { value, .. } => value.chars().count(),
                    Separator::WithEnd { value, end, .. } => {
                        let v = value.chars().count();
                        let e = end.as_ref().map_or(v, |e| e.chars().count());
                        v.max(e)
                    },
                };
                let a_value = match a {
                    Separator::Simple { value, .. } => value.chars().count(),
                    Separator::WithEnd { value, end, .. } => {
                        let v = value.chars().count();
                        let e = end.as_ref().map_or(v, |e| e.chars().count());
                        v.max(e)
                    },
                };
                b_value.cmp(&a_value)
            }
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

            let char_text = char.to_string();
            buffer += &char_text;

            if buffer.chars().count() < buffer_range.max {
                continue;
            }

            let awaiting_token = self.search_token.is_some();
            let skip_content = self.search_token.as_ref().map_or(false, |s| s.skip_content);

            self.bufferize(&mut buffer, &separators);

            if buffer.chars().count() >= buffer_range.max {
                let mut chars = buffer.chars();
                if let Some(a_char) = chars.next() {
                    println!("a: {}", a_char);
                    if !awaiting_token || !skip_content {
                        println!("\tpushed");
                        self.push_unknown_character(a_char, chars.clone().count(), awaiting_token);
                    } else {
                        println!("\tskipped {} - {}", awaiting_token, skip_content);
                    }
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
            let mut is_wrapper = false;
            let mut sep_settings:Option<SeparatorSetting> = None;
            let temp_value:String = if let Some(search) = &self.search_token {
                match &search.token {
                    Separator::WithEnd { value, end, settings } => {
                        sep_settings = Some(settings.clone());
                        is_wrapper = true;
                        match end {
                            Some(v) => v.clone(),
                            None => "\n".to_string()
                        }
                    },
                    Separator::Simple { value, settings } => {
                        sep_settings = Some(settings.clone());
                        value.to_string()
                    }
                }
            } else {
                match separator {
                    Separator::Simple { value, settings } => {
                        sep_settings = Some(settings.clone());
                        value.clone()
                    },
                    Separator::WithEnd { value, end, settings } => {
                        sep_settings = Some(settings.clone());
                        is_wrapper = true;
                        if self.search_token.is_none() {
                            value.clone()
                        } else {
                            end.clone().unwrap_or_else(|| "\n".to_string())
                        }
                    },
                }
            };
            let value = temp_value.clone();
            
            if !sep_settings.clone().map_or(false, |s| s.is_case_sensitive()) {
                compare = compare.to_lowercase();
            }

            if !compare.starts_with(&value) {
                continue
            }

            let awaiting_token = self.search_token.is_some();
            let skip_content = self.search_token.as_ref().map_or(false, |s| s.skip_content);

            if is_wrapper {
                if !awaiting_token {
                    self.search_token = Some(
                        TokenizerSearch {
                            token: separator.clone(),
                            skip_content: false
                        }
                    );
                } else {
                    let chars: Vec<char> = value.chars().collect();
                    let len = chars.len();

                    for &char in chars.iter().take(chars.len().saturating_sub(1 + value.len())) {
                        self.push_unknown_character(char, len, true);
                    }
                    
                    self.search_token = None;
                }
            }

            let schema = sep_settings.clone().map_or(TokenTypes::None, |s| s.map().clone());

            let value_length = value.chars().count();
            if sep_settings.map_or(false, |s| s.is_inclusive()) {
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

    fn merge_last_token(&mut self, new_text:String, index:usize, condition:Box<dyn Fn(&Token) -> bool>) -> Option<Token> {
        let mut new_token:Option<Token> = None;
        let last = self.tokens.last();
        if let Some(token) = last {
            if condition(token) {
                let mut value = token.value().to_string();
                value.push_str(&new_text);
                new_token = Some(Token::new(
                    TokenIndex::new(token.index().start(), index),
                    value,
                    TokenTypes::Custom { name: "unknown".to_string() }
                ));
                self.pop_token();
            }
        }
        new_token
    }

    fn push_unknown_character(&mut self, char:char, buffer_length:usize, merge:bool) {
        let mut index = 0;
        if self.index > buffer_length {
            index = self.index - buffer_length;
        }

        let mut new_token = self.merge_last_token(char.to_string(), index, Box::new(|token| {
            if let TokenTypes::Custom { name } = token.schema() {
                if name == "unknown" {
                    return true;
                }
            }
            false
        }));

        if new_token.is_none() {
            new_token = Some(Token::new(
                TokenIndex::new(index, index + 1),
                char.to_string(),
                TokenTypes::Custom { name: "unknown".to_string() }
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