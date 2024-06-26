use crate::lex::Token;
use crate::parse::Any;
use crate::parse::ParseTokens;
use crate::parse::*;

define_parser_combinator! {
    ProdOp,
    Any<(
        At,
        Star,
        Slash
    )>
}
