use super::tokens::schema::TokenTypes;
use super::{language::Language, tokens::token::{Token, TokenIndex}};
use super::tokens::separator::{SeparatorMatchSetting, SeparatorSetting, TokenSeparators};

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

#[derive(Clone)]
pub struct TokenMatchItem {
    index:TokenIndex,
    token:Separator,
}

#[derive(Clone)]
pub enum TokenMatch {
    Current { start:TokenMatchItem, end: Option<TokenMatchItem> },
    None
}

pub struct Tokenizer {
    language:Box<dyn Language>,
    tokens:Vec<Token>,
    index:usize,
    current_match:TokenMatch,
}

#[derive(Clone)]
enum Setting {
    Default(SeparatorSetting),
    Match(SeparatorMatchSetting, Box<Separator>),
    MatchEnd(SeparatorMatchSetting),
}

#[derive(Clone)]
struct Separator {
    value:String,
    settings:Setting,
}

impl Separator {
    fn new(value:String, settings:Setting) -> Self {
        Self { value, settings }
    }
}

impl Tokenizer {
    pub fn new(language:Box<dyn Language>) -> Self {
        Self { language, tokens: vec![], index: 0, current_match: TokenMatch::None }
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

        let separators = self.language.sepataros();

        let mut new_separators:Vec<Separator> = vec![];

        for s in separators {
            match s {
                Alpha { alpha, settings }
                    => new_separators.push(Separator::new(String::from(alpha), Setting::Default(settings))),
                AlphaUntilMatch { alpha_start, alpha_end, settings } => {
                    let separator = Separator::new(String::from(alpha_end), Setting::MatchEnd(settings.clone()));
                    new_separators.push(Separator::new(String::from(alpha_start), Setting::Match(settings.clone(), Box::new(separator.clone()))));
                    new_separators.push(separator);
                },
                InAlphaRange { alphas, settings } => {
                    for a in alphas {
                        new_separators.push(Separator::new(String::from(a), Setting::Default(settings.clone())))
                    }
                },

                Word { word, settings } => new_separators.push(Separator::new(word, Setting::Default(settings))),
                WordUntilMatch { word_start, word_end, settings } => {
                    let separator = Separator::new(word_end, Setting::MatchEnd(settings.clone()));
                    new_separators.push(Separator::new(word_start, Setting::Match(settings.clone(), Box::new(separator.clone()))));
                    new_separators.push(separator);
                },
                InWordRange { words, settings } => {
                    for w in words {
                        new_separators.push(Separator::new(w, Setting::Default(settings.clone())));
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
            self.apply_delimeters(&String::from(code));

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
                        TokenIndex::Simple { start: start, end: start + arbitrary_buffer.len() },
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

        let mut skip_for = 0;
        for _ in 0..buffer.len() {
            if skip_for > 0 {
                skip_for -= 1;
                continue;
            }

            skip_for = self.bufferize(&mut buffer, &separators);
            self.apply_delimeters(&String::from(code));
        }
    }

    fn apply_delimeters(&mut self, code:&String) {
        if let TokenMatch::Current { start, end } = self.current_match.clone() {
            if let Some(_end) = end {
                let start_index = match start.index {
                    TokenIndex::Simple { start, end } => end,
                    _ => 0
                };
                let end_index = match _end.index {
                    TokenIndex::Simple { start, end } => start,
                    _ => start_index
                };;
                let wrap_content = String::from(&code[start_index..end_index]);

                println!("Found: {}", wrap_content);
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
        let mut skip_content = false;
        let mut end_separator = Option::None;

        if let TokenMatch::Current { start, end } = self.current_match.clone() {
            skip_content = match start.token.settings {
                Setting::Default(s) => false,
                Setting::Match(s, sep) => {
                    end_separator = Option::Some(sep);
                    s.is_skip_content()
                }
                Setting::MatchEnd(s) => s.is_skip_content()
            };

            if end_separator.is_none() {
                println!("Wtf?");
            }
        }

        for separator in separators {
            let mut compare = buffer.clone();
            let mut value = separator.value.clone();
            let mut include = true;
            let mut schema = TokenTypes::None;
            let mut match_separator:Option<Separator> = Option::None;
            match separator.settings.clone() {
                Setting::Default(s) => {
                    if !s.is_case_sensitive() {
                        compare = compare.to_lowercase();
                    }

                    if !s.is_inclusive() {
                        include = false;
                    }

                    schema = s.map().clone();
                }
                Setting::MatchEnd(s) => {
                    if let Some(end_sep) = end_separator.clone() {
                        assert_eq!(end_sep.value, value);
                        value = end_sep.value;
                    }
                    
                    if !s.is_case_sensitive() {
                        compare = compare.to_lowercase();
                        value = value.to_lowercase();
                    }

                    if !s.is_delimeter_inclusive() {
                        include = false;
                    }
                }
                Setting::Match(s, end) => {
                    if !s.is_case_sensitive() {
                        compare = compare.to_lowercase();
                        value = value.to_lowercase();
                    }

                    if !s.is_delimeter_inclusive() {
                        include = false;
                    }

                    match_separator = Option::Some(*end);
                }
            }
            
            if compare.starts_with(&value) {
                let value_length = separator.value.len();
                if let None = match_separator {
                    if include && !skip_content {
                        let start = (self.index + 1) - compare.len();
                        self.push_token(
                            Token::new(
                                TokenIndex::Simple { start: start, end: start + value_length },
                                separator.value.clone(),
                                schema
                            )
                        );
                    }
                }

                if let Some(separator) = match_separator {
                    println!("separator: {}", separator.value);
                    
                    if let Setting::Match(..) = separator.settings {
                        println!("\tmatch");
                        self.current_match = TokenMatch::Current {
                            start: TokenMatchItem {
                                index: TokenIndex::Simple {
                                    start: self.index - value_length,
                                    end: self.index,
                                },
                                token: separator
                            },
                            end: None
                        };
                    } else {
                        println!("{}", String::from(
                            match separator.settings.clone() {
                                Setting::Default(..) => "\tdefault",
                                Setting::MatchEnd(..) => "\tmatch end",
                                _ => "\t match"
                            }
                        ));
                    }
                }

                return value_length
            }
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