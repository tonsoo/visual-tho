pub struct TokenRule {
    name: String,
    sequence: Vec<TokenRuleItem>,
}

pub enum TokenRuleItem {
    OneOrMore(Vec<TokenRuleItem>),
    All(Vec<TokenRuleItem>),
    Keyword(String), // "inicio"
    Number {
        allow_negative: bool,
        allow_float: bool,
    }, // 123, 20, -10, +10
    Wrapped(TokenWrapperRule), // "aqui", (1 && 2), [3]
    Optional(Box<TokenRuleItem>),
    Custom(Box<dyn Fn(String) -> bool>),
}

pub enum TokenWrapperRule {
    Simple{
        start: String, end: String,
        rules_between: Vec<TokenRule>
    },
    Compose{
        start: Vec<String>, end: Vec<String>,
        rules_between: Vec<TokenRule>
    },
    MultiCompose{
        start: Vec<String>, end: Vec<String>,
        wrapper_between: Vec<TokenWrapperRule>
    },
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
            Self::OneOrMore(rules) => rules.iter().any(|r| r.is_valid(text)),
            Self::Keyword(word) => word == text,
            Self::Number { allow_negative, allow_float } => {
                if !*allow_negative && text.starts_with('-') {
                    return false;
                }

                if *allow_float {
                    text.parse::<f64>().is_ok()
                } else {
                    text.parse::<i64>().is_ok()
                }
            },
            Self::Wrapped(TokenWrapperRule::Simple { start, end, rules_between }) => {
                if !text.starts_with(start) || !text.ends_with(end) {
                    return false;
                }
                let inner = &text[start.len()..text.len() - end.len()];
                // TODO: recursive parsing for `rules_between`
                true
            },

            Self::Wrapped(TokenWrapperRule::Compose { start, end, rules_between })
                => false,

            Self::Wrapped(TokenWrapperRule::MultiCompose { start, end, wrapper_between })
                => false,

            Self::Custom(action) => action(String::from(text)),

            Self::All(rules) => rules.iter().all(|r| r.is_valid(text)),

            Self::Optional(rule) => rule.is_valid(text)
        }
    }
}