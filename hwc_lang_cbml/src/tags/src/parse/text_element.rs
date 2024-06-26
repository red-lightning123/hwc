use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Text, OpenTag, CloseTag };
use crate::parse::Group;

define_parser_combinator! {
    TextElement,
    Group<(
        OpenTag,
        Text,
        CloseTag
    )>
}
