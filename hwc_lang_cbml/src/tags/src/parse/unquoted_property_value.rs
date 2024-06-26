use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::Word;

define_parser_combinator! {
    UnquotedPropertyValue,
    Word
}
