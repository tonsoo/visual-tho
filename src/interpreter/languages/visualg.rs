use crate::interpreter::tokens::schema::TokenTypes;
use crate::interpreter::tokens::separator::{SeparatorSetting};

use super::super::tokens::separator::TokenSeparators;

use super::super::language::Language;
use super::super::rules::rule::{SyntaxRule, SyntaxType, SyntaxGroup};

pub struct VisuAlg;

impl Language for VisuAlg {
    fn escape(&self) -> &str {
        "\\"
    }

    fn name(&self) -> &str {
        "VisuAlg"
    }

    fn separators(&self) -> Vec<TokenSeparators> {
        vec![
            TokenSeparators::Alpha {
                alpha: ' ',
                settings: SeparatorSetting::new(TokenTypes::None)
                    .gruppable()
            },
            TokenSeparators::Alpha {
                alpha: '\n',
                settings: SeparatorSetting::new(TokenTypes::None)
                    .gruppable()
            },
            TokenSeparators::InAlphaRange {
                alphas: vec![ '.', ',', ':' ],
                settings: SeparatorSetting::new(TokenTypes::Keyword)
            },
            TokenSeparators::InAlphaRange {
                alphas: vec![ '\'', '[', ']', '(', ')', '{', '}' ],
                settings: SeparatorSetting::new(TokenTypes::Groupper)
            },
            TokenSeparators::AlphaUntil {
                alpha: '"',
                end: Some('"'),
                skip_content: false,
                settings: SeparatorSetting::new(TokenTypes::Groupper)
            },
            TokenSeparators::WordUntil {
                word: "//".to_string(),
                end: None,
                skip_content: false,
                settings: SeparatorSetting::new(TokenTypes::LineComment).gruppable()
            },
            TokenSeparators::WordUntil {
                word: "/*".to_string(),
                end: Some("*/".to_string()),
                skip_content: false,
                settings: SeparatorSetting::new(TokenTypes::GroupComment)
            },
            TokenSeparators::InAlphaRange {
                alphas: vec![ '+', '-', '*', '/' ],
                settings: SeparatorSetting::new(TokenTypes::Arithmetic)
            },
            TokenSeparators::InWordRange {
                words: vec![ String::from("and"), String::from("or") ],
                settings: SeparatorSetting::new(TokenTypes::Operator)
            },
            TokenSeparators::Word {
                word: String::from("<-"),
                settings: SeparatorSetting::new(TokenTypes::Assignment)
            },
            TokenSeparators::InWordRange{
                words: vec![
                    String::from("algoritmo"), String::from("fimalgoritmo"),

                    String::from("se"), String::from("fimse"),

                    String::from("enquanto"), String::from("fimenquanto"),

                    String::from("para"), String::from("fimpara"),

                    String::from("repita"), String::from("fimrepita"),

                    String::from("escolha"), String::from("fimescolha"),

                    String::from("procedimento"), String::from("fimprocedimento"),

                    String::from("funcao"), String::from("fimfuncao"),
                ],
                settings: SeparatorSetting::new(TokenTypes::Groupper)
            },
            TokenSeparators::InWordRange{
                words: vec![
                    String::from("inicio"), String::from("var"),

                    String::from("entao"), String::from("senao"),

                    String::from("ate"), String::from("passo"),

                    String::from("ate"),

                    String::from("caso"), String::from("outrocaso"),
                    
                    String::from("retorne"), String::from("verdadeiro"), String::from("falso")
                ],
                settings: SeparatorSetting::new(TokenTypes::Keyword)
            },
            TokenSeparators::InWordRange{
                words: vec![
                    String::from("inteiro"), String::from("real"),
                    String::from("caractere"), String::from("logico")
                ],
                settings: SeparatorSetting::new(TokenTypes::Type)
            },
        ]
    }

    fn syntax() -> Vec<SyntaxGroup> {
        vec![
            SyntaxGroup::new(
                vec![
                    SyntaxRule::new(
                        SyntaxType::Keyword("if".to_string())
                    )
                ]
            )
        ]
    }
}