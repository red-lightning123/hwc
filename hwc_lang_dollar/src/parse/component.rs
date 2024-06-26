use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ InlineText, InlineMath, MultilineMath, Newline };
use crate::parse::Any;

define_parser_combinator! {
    Component,
    Any<(
        InlineText,
        InlineMath,
        MultilineMath,
        Newline
    )>
}
