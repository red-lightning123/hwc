use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::*;

define_parser_combinator! {
    DerOp,
    Tag
}
