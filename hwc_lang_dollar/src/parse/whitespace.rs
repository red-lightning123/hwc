use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::WhitespaceToken;
use crate::parse::Repeat;

define_parser_combinator! {
    Whitespace,
    Repeat<WhitespaceToken>
}
