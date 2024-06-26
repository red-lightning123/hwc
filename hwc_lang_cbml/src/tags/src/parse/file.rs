use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Whitespace, Element };
use crate::parse::{ Optional, Group };

define_parser_combinator! {
    File,
    Group<(
        Optional<Whitespace>,
        Element,
        Optional<Whitespace>
    )>
}
