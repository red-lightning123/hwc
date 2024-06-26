use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::Include;
use crate::parse::Repeat;

define_parser_combinator! {
    IncludeCluster,
    Repeat<Include>
}
