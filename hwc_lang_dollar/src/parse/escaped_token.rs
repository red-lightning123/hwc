use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::parse::{ Escape, Quote, Dollar };
use crate::parse::{ Any, Group };

define_parser_combinator! {
    EscapedToken,
    Group<(
        Escape,
        Any<(
            Quote,
            Dollar,
            Escape
        )>
    )>
}
