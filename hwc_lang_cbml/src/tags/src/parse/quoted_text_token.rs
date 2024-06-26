use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ LDBrace, RDBrace, Slash, Equals, Word, WhitespaceToken };
use crate::parse::Any;

define_parser_combinator! {
    QuotedTextToken,
    Any<(
        LDBrace,
        RDBrace,
        Slash,
        Equals,
        Word,
        WhitespaceToken
    )>
}
