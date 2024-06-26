use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Quote, QuotedText };
use crate::parse::Group;

define_parser_combinator! {
    QuotedPropertyValue,
    Group<(
        Quote,
        QuotedText,
        Quote
    )>
}
