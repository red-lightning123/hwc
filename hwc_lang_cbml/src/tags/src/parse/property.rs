use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Equals, Word, Whitespace, PropertyValue };
use crate::parse::{ Optional, Group };

define_parser_combinator! {
    Property,
    Group<(
        Word,
        Optional<Whitespace>,
        Equals,
        Optional<Whitespace>,
        PropertyValue
    )>
}
