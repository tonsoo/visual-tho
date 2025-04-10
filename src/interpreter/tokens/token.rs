use super::schema::TokenTypes;

pub struct Token {
    index: TokenIndex,
    value: String,
    schema: TokenTypes
}

pub enum TokenIndex {
    Simple {
        start: usize,
        end: usize
    },

    Group {
        first_start: usize,
        first_end: usize,
        second_start: usize,
        second_end: usize,
    }
}

impl Token {
    pub fn new(index:TokenIndex, value:String, schema:TokenTypes) -> Self {
        Self { index, value, schema }
    }

    pub fn index(&self) -> &TokenIndex {
        &self.index
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn schema(&self) -> &TokenTypes {
        &self.schema
    }
}

impl TokenIndex {
    pub fn to_string(&self) -> String {
        match self {
            TokenIndex::Simple { start, end } => format!("{}:{}", start, end),
            TokenIndex::Group { first_start, first_end, second_start, second_end }
                => format!("{}:{}-{}:{}", first_start, first_end, second_start, second_end),
        }
    }
}