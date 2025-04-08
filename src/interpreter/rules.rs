pub struct TokenRule {
    name: String,
    sequence: Vec<TokenRuleItem>,
}

pub enum TokenRuleItem {
    OneOf(Vec<String>),
    Keyword(String), // "inicio"
    Number(), // 123, 20, -10, +10
    Wrapped(String, String), // "aqui", (1 && 2), [3]
    Custom(Box<dyn Fn(String) -> bool>),
}

impl TokenRule {
    pub fn new(rules: Vec<TokenRuleItem>) -> TokenRule {
        TokenRule { name: String::new(), sequence: rules }
    }

    pub fn with_name(name: String, rules: Vec<TokenRuleItem>) -> TokenRule {
        TokenRule { name: name, sequence: rules }
    }

    pub fn get_sequence(&self) -> &Vec<TokenRuleItem> {
        &self.sequence
    }
}

impl TokenRuleItem {
    fn is_valid(&self, text: &str) -> bool {
        match self {
            Self::OneOf(list) => list.contains(&text.to_string()),
            Self::Keyword(word) => word == text,
            Self::Number() => text.parse::<f64>().is_ok(),
            Self::Wrapped(start, end) =>
                text.starts_with(start) && text.ends_with(end),
            Self::Custom(action) => action(String::from(text))
        }
    }
}