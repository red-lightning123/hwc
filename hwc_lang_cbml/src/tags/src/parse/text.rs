use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::TextToken;
use crate::parse::Repeat;

define_parser_combinator! {
    Text,
    Repeat<TextToken>
}
