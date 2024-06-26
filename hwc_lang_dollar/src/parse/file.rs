use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Whitespace, Component };
use crate::parse::{ Optional, Group, Repeat };

define_parser_combinator! {
    File,
    Group<(Optional<Whitespace>, Repeat<Group<(Component, Optional<Whitespace>)>>)>
}
