use super::rules::TokenRule;

pub struct Language {
    name: String,
    rules: Vec<TokenRule>
}

impl Language {
    pub fn new(name: String, rules: Vec<TokenRule>) -> Self {
        Language {
            name: name,
            rules: rules
        }
    }

    pub fn get_rules(&self) -> &Vec<TokenRule> {
        &self.rules
    }
}