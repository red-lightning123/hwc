use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Whitespace, Element };
use crate::parse::{ Optional, Group, Repeat };

define_parser_combinator! {
    ElementArray,
    Group<(
        Element,
        Repeat<Group<(Optional<Whitespace>, Element)>>
    )>
}
