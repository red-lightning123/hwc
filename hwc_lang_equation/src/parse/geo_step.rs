use crate::lex::Token;
use crate::parse::Group;
use crate::parse::ParseTokens;
use crate::parse::*;

define_parser_combinator! {
    GeoStep,
    Group<(
        GeoExpl,
        RArrow,
        MultilineStmts
    )>
}
