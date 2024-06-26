use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ UnquotedPropertyValue, QuotedPropertyValue };
use crate::parse::Any;

define_parser_combinator! {
    PropertyValue,
    Any<(
        UnquotedPropertyValue,
        QuotedPropertyValue
    )>
}
