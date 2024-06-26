use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::QuotedTextToken;
use crate::parse::Repeat;

define_parser_combinator! {
    QuotedText,
    Repeat<QuotedTextToken>
}
