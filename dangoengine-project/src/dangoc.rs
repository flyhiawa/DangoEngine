pub enum Tkw{
    // Keywords
    Fn,
    AsLong,
    Tie,
    Concept,
    Match,
    Import,
    //Identifiers
    Identifier(String),
    Number(f64),
    StringI(String),
    Bool(bool),
    Char(char),
    // Signals
    Equal,
    Underscore,
    DoubleEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Semicolon,
    Colon,
    DoubleColon,
    Arrow,
    FatArrow,
    Plus,
    Minus,
    Times,
    DividedBy,
    Comma,
    // Blocks
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    // Especials
    Eof,
    UnknownError,
}

fn main(){}
