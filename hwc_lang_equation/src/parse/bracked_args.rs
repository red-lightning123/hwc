use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::*;
use crate::parse::{Any, Group};

define_parser_combinator! {
    BrackedArgs,
    Any<(
        Group<(LParen, ExprArgs, RParen)>,
        Group<(LBrace, ExprArgs, RBrace)>
    )>
}
