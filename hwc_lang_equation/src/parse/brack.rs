use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::*;
use crate::parse::{Any, Group};

define_parser_combinator! {
    Brack,
    Any<(
        Group<(
            Value,
            BrackedArgs
        )>,
        BrackedArgs,
        Value
    )>
}
