use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::OpenCloseTag;

define_parser_combinator! {
    EmptyElement,
    OpenCloseTag
}
