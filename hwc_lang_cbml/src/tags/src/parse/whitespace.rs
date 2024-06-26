use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::WhitespaceToken;
use crate::parse::{ Group, Repeat };

define_parser_combinator! {
    Whitespace,
    Group<(
        WhitespaceToken,
        Repeat<WhitespaceToken>
    )>
}
