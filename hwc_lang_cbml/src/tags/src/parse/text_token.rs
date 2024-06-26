use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Quote, Slash, Equals, Word, WhitespaceToken };
use crate::parse::Any;

define_parser_combinator! {
    TextToken,
    Any<(
        Quote,
        Slash,
        Equals,
        Word,
        WhitespaceToken
    )>
}
