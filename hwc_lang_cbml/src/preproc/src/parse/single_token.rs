use crate::lex::Token;
use crate::parse::ParseTokens;

#[derive(Debug)]
pub struct IDHash;

impl ParseTokens for IDHash {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::IDHash) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(IDHash)
        } else {
            Err("unexpected token (\">##\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct DHash;

impl ParseTokens for DHash {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::DHash) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(DHash)
        } else {
            Err("unexpected token (\"##\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Text(pub String);

impl Text {
    fn new(text : String) -> Text {
        Text(text)
    }
}

impl ParseTokens for Text {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Text(text)) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Text::new(text.to_string()))
        } else {
            Err("unexpected token (word expected)".to_string())
        }
    }
}
