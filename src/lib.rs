extern crate ucd;

mod tokenstate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[derive(Debug)]
pub struct CharRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct PositionRange {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub range: CharRange,
    pub position: PositionRange,
    pub raw: String,
    pub tok: TokenType<'a>,

    // Strings stored in the token so that they are persistent and won't trigger
    // tons of new allocations, should be accessed via the string slices in TokenType.
    s1: String,
    s2: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
    Whitespace,
    LineTerminator,

    LCurly,
    LParen,
    RParen,
    LSquare,
    RSquare,
    Semicolon,
    Comma,
    Tilde,
    Quest,
    Colon,

    Period,
    Ellipsis,

    RCurly,
    IdentifierName,

    LAngle,
    LessEq,
    LAngleAngle,
    LAngleAngleEq,

    RAngle,
    GreaterEq,
    RAngleAngle,
    RAngleAngleAngle,
    RAngleAngleEq,
    RAngleAngleAngleEq,

    NEq,
    NEqEq,
    Exclam,

    Eq,
    EqEq,
    EqEqEq,
    Arrow,

    Plus,
    PlusPlus,
    PlusEq,

    Minus,
    MinusMinus,
    MinusEq,

    Mod,
    ModEq,

    Star,
    StarEq,
    StarStar,
    StarStarEq,

    Amp,
    AmpAmp,
    AmpEq,

    Bar,
    BarBar,
    BarEq,

    Caret,
    CaretEq,

    Comment,

    Div,
    DivEq,
    RegularExpressionLiteral(&'a str, &'a str),

    NumericLiteral(NumberType, &'a str),
    StringLiteral(&'a str),

    TemplateTick,
    TemplateClose,
    TemplateOpen,
    TemplateChars(&'a str),

    Unknown,
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NumberType {
    Hex,
    Octal,
    Binary,
    Float,
}




fn read_tokens<'a, 'b, 'c>(
    chars: &'a str,
    tokens: &'b mut [Token<'c>],
) -> (&'a str, &'b [Token<'c>]) {
    let mut c = chars;
    let mut i = 0;
    for token in tokens.iter_mut() {
        let next = read_token(c, token);

        if next as *const _ == c as *const _ {
            break;
        }
        if token.tok == TokenType::Unknown {
            break;
        }

        c = next;
        i += 1;
    }

    (c, &tokens[..i])
}

fn read_token<'a>(chars: &'a str, token: &mut Token) -> &'a str {


    // if same slice is returned, it means "not enough chars"
    return chars;
}
