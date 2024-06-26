use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ DDollar, Literal };
use crate::parse::Group;

define_parser_combinator! {
    MultilineMath,
    Group<(
        DDollar,
        Literal,
        DDollar
    )>
}
