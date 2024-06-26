use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::LiteralToken;
use crate::parse::Repeat;

define_parser_combinator! {
    Literal,
    Repeat<LiteralToken>
}
