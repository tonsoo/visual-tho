use std::vec;

use crate::interpreter::rules::TokenWrapperRule;

use super::super::rules::{TokenRule, TokenRuleItem};

use super::super::language::Language;

pub fn visual_g() -> Language {
    Language::new(
        String::from("VisualG"),
        vec![

            // Algoritmo
            TokenRule::with_name(
                String::from("Algoritmo"),
                vec![
                    TokenRuleItem::Wrapped(
                        TokenWrapperRule::Simple {
                            start: String::from("algoritmo"),
                            end: String::from("fimalgoritmo"),
                            rules_between: vec![
                                inicio(),
                            ]
                        }
                    ),
                ]
            ),
        ]
    )
}

fn inicio() -> TokenRule {
    TokenRule::with_name(
        String::from("Ínicio"),
        vec![
            TokenRuleItem::Wrapped(
                TokenWrapperRule::Simple {
                    start: String::from("inicio"),
                    end: String::new(),
                    rules_between: vec![
                        escolha(),

                        se()
                    ],
                }
            ),
        ]
    )
}

fn se() -> TokenRule {
    TokenRule::with_name(
        String::from("Se"),
        vec![
            TokenRuleItem::Wrapped(
                TokenWrapperRule::Simple {
                    start: String::from("se"),
                    end: String::from("entao"),
                    rules_between: vec![
                    
                    ],
                }
            ),
        ]
    )
}

fn escolha() -> TokenRule {
    TokenRule::with_name(
        String::from("Escolha"),
        vec![
            TokenRuleItem::Wrapped(
                TokenWrapperRule::Simple {
                    start: String::from("escolha"),
                    end: String::from("fimescolha"),
                    rules_between: vec![
                    
                    ],
                }
            ),
        ]
    )
}