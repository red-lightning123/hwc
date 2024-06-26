use crate::lex::Token;
use crate::parse::ParseTokens;

macro_rules! define_basic_token {
    ($Name:ident, $token_string:expr) => {
        #[derive(Debug)]
        pub struct $Name;

        impl ParseTokens for $Name {
            type Output = Self;
            fn parse_mut_tokens(tokens: &mut &[Token]) -> Result<Self::Output, String> {
                if let Some(Token::$Name) = tokens.get(0) {
                    *tokens = &tokens[1..];
                    Ok($Name)
                } else {
                    Err(format!("unexpected token (\"{}\" expected)", $token_string))
                }
            }
        }
    };
}

define_basic_token! { LParen, "(" }
define_basic_token! { RParen, ")" }
define_basic_token! { LBrace, "{" }
define_basic_token! { RBrace, "}" }
define_basic_token! { InvRArrow, "@=>" }
define_basic_token! { RArrow, "=>" }
define_basic_token! { PlusMinus, "+-" }
define_basic_token! { MinusPlus, "-+" }
define_basic_token! { Plus, "+" }
define_basic_token! { Minus, "-" }
define_basic_token! { At, "@" }
define_basic_token! { Star, "*" }
define_basic_token! { Slash, "/" }
define_basic_token! { Caret, "^" }
define_basic_token! { Tag, "'" }
define_basic_token! { InvEquals, "@=" }
define_basic_token! { Equals, "=" }
define_basic_token! { NotEquals, "!=" }
define_basic_token! { ApproxEquals, "~=" }
define_basic_token! { Lt, "<" }
define_basic_token! { Gt, ">" }
define_basic_token! { Le, "<=" }
define_basic_token! { Ge, ">=" }
define_basic_token! { Comma, "," }
define_basic_token! { Hash, "#" }
define_basic_token! { Newline, "//" }

#[derive(Debug)]
pub struct Is;

impl ParseTokens for Is {
    type Output = Self;
    fn parse_mut_tokens(tokens: &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Value("is")) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Is)
        } else {
            Err("unexpected token (\"is\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Value(pub String);

impl Value {
    fn new(value: String) -> Value {
        Value(value)
    }
}

impl ParseTokens for Value {
    type Output = Self;
    fn parse_mut_tokens(tokens: &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Value(word)) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Value::new(word.to_string()))
        } else {
            Err("unexpected token (value expected)".to_string())
        }
    }
}
