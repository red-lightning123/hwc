use crate::lex::Token;

pub fn parse_file_tokens(tokens : &[Token]) -> Result<File, String> {
    let mut new_tokens = tokens;
    let file = File::parse_mut_tokens(&mut new_tokens);
    match file {
        Ok(file) => {
            if new_tokens.is_empty() {
                Ok(file)
            } else {
                Err(format!("while parsing {}: some tokens weren't consumed. text dump:\n{}", "cbml::preproc", crate::highlight_tokens(tokens)))
            }
        }
        Err(err) => {
            Err(format!("while parsing {}: parser error: {}. text dump:\n{}", "cbml::preproc", err, crate::highlight_tokens(tokens)))
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
use combinators::{ Optional, Group, Repeat };
pub use combinators::any_variants;

mod parse_tokens;
use parse_tokens::ParseTokens;

mod file;
pub use file::File;

mod minifile;
pub use minifile::Minifile;

mod minifile_name;
pub use minifile_name::MinifileName;

mod minifile_content;
pub use minifile_content::MinifileContent;

mod include_cluster;
pub use include_cluster::IncludeCluster;

mod include;
pub use include::Include;

mod single_token;
pub use single_token::{ IDHash, DHash, Text };
