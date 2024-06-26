use crate::lex::Token;
use crate::parse::Any;
use crate::parse::ParseTokens;
use crate::parse::*;

define_parser_combinator! {
    StmtRel,
    Any<(
        InvRArrow,
        RArrow
    )>
}
