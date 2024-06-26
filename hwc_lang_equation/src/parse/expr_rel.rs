use crate::lex::Token;
use crate::parse::Any;
use crate::parse::ParseTokens;
use crate::parse::*;

define_parser_combinator! {
    ExprRel,
    Any<(
        InvEquals,
        Equals,
        NotEquals,
        ApproxEquals,
        Lt,
        Gt,
        Le,
        Ge
    )>
}
