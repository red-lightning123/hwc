use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::*;
use crate::parse::{Group, Repeat};

define_parser_combinator! {
    Prod,
    Group<(
        Neg,
        Repeat<Group<(ProdOp, Neg)>>
    )>
}
