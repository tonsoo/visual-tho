use super::tokens::separator::TokenSeparators;
use super::rules::rule::{SyntaxRule, SyntaxType}

pub trait Language {
    fn escape(&self) -> &str;

    fn name(&self) -> &str;

    fn separators(&self) -> Vec<TokenSeparators>;

    fn syntax() -> Vec<SyntaxRule>;
}