use eframe::egui_glow::CallbackFn;

use super::{interpreter::Interpreter, rules::TokenRuleItem};

pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Operator(char),
    Symbol(char),
}
