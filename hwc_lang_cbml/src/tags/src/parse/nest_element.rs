use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Whitespace, OpenTag, CloseTag, ElementArray };
use crate::parse::{ Optional, Group };

define_parser_combinator! {
    NestElement,
    Group<(
        OpenTag,
        Optional<Whitespace>,
        Box<ElementArray>,
        Optional<Whitespace>,
        CloseTag
    )>
}
