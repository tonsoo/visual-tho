pub enum SyntaxType {
    Keyword { key:String }
    OneOf { keys:Vec<String> },
    Numeric { accepts_float:bool, accepts_negative:bool },
    WrappedBy { open:String, close:String },
}

pub struct SyntaxRule {
    syntax:SyntaxType,
    optional:bool,
}

pub struct SyntaxGroup {
    rules:Vect<SyntaxRule>
}

pub impl SyntaxRule {
    fn new(syntax:SyntaxType, { optional:bool = false }) {
        SyntaxRule { syntax, optional }
    }
}

pub impl SyntaxGroup {
    fn new(rules:Vec<SyntaxRule>) {
        SyntaxGroup { rules }
    }
}