use super::super::rules::{TokenRule, TokenRuleItem};

use super::super::language::Language;

pub fn visual_g() -> Language {
    Language::new(
        String::from("VisualG"),
        vec![
            TokenRule::with_name(
                String::from("Ínicio"),
                vec![
                    TokenRuleItem::Keyword(String::from("Inicio"))
                ]
            ),
        ]
    )
}