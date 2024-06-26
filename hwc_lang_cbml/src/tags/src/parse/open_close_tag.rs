use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ LDBrace, RDBrace, Slash, Word, Whitespace, Property };
use crate::parse::{ Optional, Group, Repeat };

define_parser_combinator! {
    OpenCloseTag,
    Group<(
        LDBrace,
        Optional<Whitespace>,
        Word,
        Repeat<Group<(Whitespace, Property)>>,
        Optional<Whitespace>,
        Slash,
        Optional<Whitespace>,
        RDBrace
    )>
}
