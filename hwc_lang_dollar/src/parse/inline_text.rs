use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Quote, Literal };
use crate::parse::Group;

define_parser_combinator! {
    InlineText,
    Group<(
        Quote,
        Literal,
        Quote
    )>
}
