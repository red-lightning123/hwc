use crate::lex::Token;
use crate::parse::Group;
use crate::parse::ParseTokens;
use crate::parse::*;

define_parser_combinator! {
    IsStmt,
    Group<(
        Expr,
        Is,
        Expr
    )>
}
