use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Newline, Word, EscapedToken, WhitespaceToken };
use crate::parse::Any;

define_parser_combinator! {
    LiteralToken,
    Any<(
        Newline,
        Word,
        EscapedToken,
        WhitespaceToken
    )>
}
