use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::*;
use crate::parse::{Group, Optional, Repeat};

define_parser_combinator! {
    RelStmt,
    Group<(
        Optional<Expr>,
        Repeat<Group<(ExprRel, Expr)>>,
        Optional<ExprRel>
    )>
}
