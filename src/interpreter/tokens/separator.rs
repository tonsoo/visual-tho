use super::schema::TokenTypes;

#[derive(Clone)]
pub struct SeparatorSetting {
    case_sensitive:bool,
    include:bool,
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

pub enum TokenSeparators {
    Alpha {
        alpha:char,
        settings: SeparatorSetting
    },

    InAlphaRange {
        alphas:Vec<char>,
        settings: SeparatorSetting
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
            | InAlphaRange { .. } => true,
            _ => false
        }
    }
}