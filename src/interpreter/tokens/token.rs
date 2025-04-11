use super::schema::TokenTypes;

pub struct Token {
    index: TokenIndex,
    value: String,
    count: i32,
    schema: TokenTypes
}

#[derive(Clone)]
pub struct  TokenIndex {
    start: usize,
    end: usize
}

impl TokenIndex {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

impl Token {
    pub fn new(index:TokenIndex, value:String, schema:TokenTypes) -> Self {
        Self { index, value, schema, count: 1 }
    }

    pub fn index(&self) -> &TokenIndex {
        &self.index
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn count(&self) -> &i32 {
        &self.count
    }

    pub fn schema(&self) -> &TokenTypes {
        &self.schema
    }

    pub fn increase_count(&mut self) {
        self.count += 1;
        self.index.end += self.value.chars().count();
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