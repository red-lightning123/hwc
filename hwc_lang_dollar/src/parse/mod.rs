use crate::lex::Token;

pub fn parse_file_tokens(tokens : &[Token]) -> Result<File, String> {
    let mut tokens = tokens;
    let file = File::parse_mut_tokens(&mut tokens);
    match file {
        Ok(file) => {
            if tokens.is_empty() {
                Ok(file)
            } else {
                Err(format!("while parsing {}: some tokens weren't consumed. text dump:\n{}", "dollar", crate::highlight_tokens(tokens)))
            }
        }
        Err(err) => {
            Err(format!("while parsing {}: parser error: {}. text dump:\n{}", "dollar", err, crate::highlight_tokens(tokens)))
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

mod component;
pub use component::Component;

mod inline_text;
pub use inline_text::InlineText;

mod inline_math;
pub use inline_math::InlineMath;

mod multiline_math;
pub use multiline_math::MultilineMath;

mod literal;
pub use literal::Literal;

mod literal_token;
pub use literal_token::LiteralToken;

mod escaped_token;
pub use escaped_token::EscapedToken;

mod whitespace;
pub use whitespace::Whitespace;

mod single_token;
pub use single_token::{ Quote, Dollar, DDollar, Newline, Escape, Word, WhitespaceToken };
