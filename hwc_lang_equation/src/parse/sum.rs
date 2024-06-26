use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::*;
use crate::parse::{Group, Repeat};

define_parser_combinator! {
    Sum,
    Group<(
        Prod,
        Repeat<Group<(SumOp, Prod)>>
    )>
}