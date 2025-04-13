#[derive(Clone)]
pub enum TokenTypes {
    Keyword,
    Assignment,
    Operator,
    Arithmetic,
    Groupper,
    Type,
    LineComment,
    GroupComment,
    Custom { name:String },
    None
}

impl TokenTypes {
    pub fn to_string(&self) -> String {
        String::from(
            match self {
                TokenTypes::Keyword => "keyword",
                TokenTypes::Assignment => "assignment",
                TokenTypes::Operator => "operator",
                TokenTypes::Arithmetic => "arithmetic",
                TokenTypes::Groupper => "groupper",
                TokenTypes::Type => "type",
                TokenTypes::LineComment => "line comment",
                TokenTypes::GroupComment => "group comment",
                TokenTypes::Custom { name } => name,
                TokenTypes::None => "none",
            }
        )
    }
}