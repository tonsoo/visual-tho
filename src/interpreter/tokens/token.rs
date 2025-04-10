use super::schema::TokenTypes;

pub struct Token {
    index: TokenIndex,
    value: String,
    schema: TokenTypes
}

#[derive(Clone)]
pub struct  TokenIndex {
    start: usize,
    end: usize
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
    pub fn new(start:usize, end:usize) -> Self {
        Self { start, end }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.start, self.end)
    }
}