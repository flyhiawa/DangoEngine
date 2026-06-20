pub enum Tkw{
    // Keywords
    Fn,
    AsLong,
    Tie,
    Concept,
    Match,

    //Identifiers
    Identifier(String),
    Number(f64),

    // Signals
    Equal,
    Underscore,
    DoubleEqual,
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
    CloseBrace
}

fn main(){}
