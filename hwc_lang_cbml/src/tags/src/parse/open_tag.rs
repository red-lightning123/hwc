use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ LDBrace, RDBrace, Word, Whitespace, Property };
use crate::parse::{ Optional, Group, Repeat };

define_parser_combinator! {
    OpenTag,
    Group<(
        LDBrace,
        Optional<Whitespace>,
        Word,
        Repeat<Group<(Whitespace, Property)>>,
        Optional<Whitespace>,
        RDBrace
    )>
}
