use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ DHash, Text };
use crate::parse::Group;

define_parser_combinator! {
    MinifileName,
    Group<(
        DHash,
        Text,
        DHash
    )>
}
