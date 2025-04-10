use super::tokens::separator::TokenSeparators;

pub trait Language {
    fn escape(&self) -> &str;

    fn name(&self) -> &str;

    fn separators(&self) -> Vec<TokenSeparators>;

    // fn syntax() -> Vec<TokenTypes>;
}