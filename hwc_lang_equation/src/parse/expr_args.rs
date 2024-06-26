use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::*;
use crate::parse::{Group, Optional, Repeat};

define_parser_combinator! {
    ExprArgs,
    Optional<
        Group<(
            Box<Stmts>,
            Repeat<Group<(Comma, Box<Stmts>)>>
        )>
    >
}
