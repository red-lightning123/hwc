use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Dollar, Literal };
use crate::parse::Group;

define_parser_combinator! {
    InlineMath,
    Group<(
        Dollar,
        Literal,
        Dollar
    )>
}
