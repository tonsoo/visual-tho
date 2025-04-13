pub enum SyntaxRule {
    OneOf { keys:Vec<String> },
    Numeric { accepts_float:bool, accepts_negative:bool },
    FollowedBy { key:String },
    WrappedBy { open:String, close:String },
}