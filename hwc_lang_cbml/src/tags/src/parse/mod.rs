use crate::lex::Token;

pub fn parse_file_tokens(tokens : &[Token]) -> Result<File, String> {
    let mut new_tokens = tokens;
    let file = File::parse_mut_tokens(&mut new_tokens);
    match file {
        Ok(file) => {
            if new_tokens.is_empty() {
                Ok(file)
            } else {
                Err(format!("while parsing {}: some tokens weren't consumed. text dump:\n{}", "cbml::tags", crate::highlight_tokens(tokens)))
            }
        }
        Err(err) => {
            Err(format!("while parsing {}: parser error: {}. text dump:\n{}", "cbml::tags", err, crate::highlight_tokens(tokens)))
        }
    }
}

macro_rules! define_parser_combinator {
    ( $name:ident, $parse_type:ty ) => {
        type ParseType = $parse_type;

        #[derive(Debug)]
        pub struct $name(pub <ParseType as ParseTokens>::Output);

        impl ParseTokens for $name {
            type Output = Self;
            fn parse_mut_tokens<'a>(tokens : &mut &'a [Token<'a>]) -> Result<Self::Output, String> {
                Ok(Self(ParseType::parse_mut_tokens(tokens)?))
            }
        }
    };
}

mod combinators;
use combinators::{ Optional, Any, Group, Repeat };
pub use combinators::any_variants;

mod parse_tokens;
use parse_tokens::ParseTokens;

mod file;
pub use file::File;

mod element;
pub use element::Element;

mod nest_element;
pub use nest_element::NestElement;

mod text_element;
pub use text_element::TextElement;

mod empty_element;
pub use empty_element::EmptyElement;

mod element_array;
pub use element_array::ElementArray;

mod open_tag;
pub use open_tag::OpenTag;

mod close_tag;
pub use close_tag::CloseTag;

mod open_close_tag;
pub use open_close_tag::OpenCloseTag;

mod property;
pub use property::Property;

mod property_value;
pub use property_value::PropertyValue;

mod unquoted_property_value;
pub use unquoted_property_value::UnquotedPropertyValue;

mod quoted_property_value;
pub use quoted_property_value::QuotedPropertyValue;

mod text;
pub use text::Text;

mod text_token;
pub use text_token::TextToken;

mod quoted_text;
pub use quoted_text::QuotedText;

mod quoted_text_token;
pub use quoted_text_token::QuotedTextToken;

mod whitespace;
pub use whitespace::Whitespace;

mod single_token;
pub use single_token::{ LDBrace, RDBrace, Slash, Quote, Equals, Word, WhitespaceToken };
