use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ LDBrace, RDBrace, Slash, Word, Whitespace };
use crate::parse::{ Optional, Group };

define_parser_combinator! {
    CloseTag,
    Group<(
        LDBrace,
        Optional<Whitespace>,
        Slash,
        Optional<Whitespace>,
        Word,
        Optional<Whitespace>,
        RDBrace
    )>
}
