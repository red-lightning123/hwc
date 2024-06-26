use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::*;
use crate::parse::{Group, Optional, Repeat};

define_parser_combinator! {
    Stmts,
    Group<(
        Optional<Stmt>,
        Repeat<Group<(StmtRel, Stmt)>>,
        Optional<StmtRel>
    )>
}
