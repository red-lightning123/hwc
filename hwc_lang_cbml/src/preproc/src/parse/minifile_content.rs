use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Text, IncludeCluster };
use crate::parse::{ Optional, Group, Repeat };

define_parser_combinator! {
    MinifileContent,
    Group<(
        Optional<Text>,
        Repeat<
            Group<(
                IncludeCluster,
                Text
            )>
        >,
        Optional<IncludeCluster>
    )>
}
