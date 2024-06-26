use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::Minifile;
use crate::parse::Repeat;

define_parser_combinator! {
    File,
    Repeat<Minifile>
}
