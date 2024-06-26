use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ MinifileName, MinifileContent };
use crate::parse::Group;

define_parser_combinator! {
    Minifile,
    Group<(MinifileName, MinifileContent)>
}
