use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ IDHash, DHash, Text };
use crate::parse::Group;

define_parser_combinator! {
    Include,
    Group<(
        IDHash,
        Text,
        DHash
    )>
}
