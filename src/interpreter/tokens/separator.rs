use super::schema::TokenTypes;

#[derive(Clone)]
pub struct SeparatorSetting {
    case_sensitive:bool,
    include:bool,
    map:TokenTypes
}

#[derive(Clone)]
pub struct SeparatorMatchSetting {
    case_sensitive:bool,
    include_content:bool,
    include_delimeter:bool,
    skip_content:bool,
    map:TokenTypes
}

impl SeparatorSetting {
    pub fn new(map:TokenTypes) -> Self {
        SeparatorSetting {
            case_sensitive: false,
            include: true,
            map: map,
        }
    }
    pub fn case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }
    pub fn not_inclusive(mut self) -> Self {
        self.include = false;
        self
    }
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }
    pub fn is_inclusive(&self) -> bool {
        self.include
    }
    pub fn map(&self) -> &TokenTypes {
        &self.map
    }
}

impl SeparatorMatchSetting {
    pub fn new(map:TokenTypes) -> Self {
        SeparatorMatchSetting {
            case_sensitive: false,
            include_content: true,
            include_delimeter: true,
            skip_content: false,
            map: map,
        }
    }
    pub fn case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }
    pub fn not_content_inclusive(mut self) -> Self {
        self.include_content = false;
        self
    }
    pub fn not_delimeter_inclusive(mut self) -> Self {
        self.include_delimeter = false;
        self
    }
    pub fn skip_content(mut self) -> Self {
        self.skip_content = true;
        self
    }
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }
    pub fn is_content_inclusive(&self) -> bool {
        self.include_content
    }
    pub fn is_delimeter_inclusive(&self) -> bool {
        self.include_delimeter
    }
    pub fn is_skip_content(&self) -> bool {
        self.skip_content
    }
    pub fn map(&self) -> &TokenTypes {
        &self.map
    }
}

pub enum TokenSeparators {
    AlphaUntilMatch {
        alpha_start:char,
        alpha_end:char,
        settings: SeparatorMatchSetting
    },

    Alpha {
        alpha:char,
        settings: SeparatorSetting
    },

    InAlphaRange {
        alphas:Vec<char>,
        settings: SeparatorSetting
    },

    WordUntilMatch {
        word_start:String,
        word_end:String,
        settings: SeparatorMatchSetting
    },

    Word {
        word:String,
        settings: SeparatorSetting
    },

    InWordRange {
        words:Vec<String>,
        settings: SeparatorSetting
    },
}

impl TokenSeparators {
    pub fn is_alpha(separator:&TokenSeparators) -> bool {
        use TokenSeparators::*;

        match separator {
            Alpha { .. }
            | AlphaUntilMatch { .. }
            | InAlphaRange { .. } => true,
            _ => false
        }
    }
}