use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ NestElement, TextElement, EmptyElement };
use crate::parse::Any;

define_parser_combinator! {
    Element,
    Any<(
        NestElement,
        TextElement,
        EmptyElement
    )>
}
