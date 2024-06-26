use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::lex::WhitespaceType;

#[derive(Debug)]
pub struct LDBrace;

impl ParseTokens for LDBrace {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::LDBrace) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(LDBrace)
        } else {
            Err("unexpected token (\"{{\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct RDBrace;

impl ParseTokens for RDBrace {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::RDBrace) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(RDBrace)
        } else {
            Err("unexpected token (\"}}\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Slash;

impl ParseTokens for Slash {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Slash) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Slash)
        } else {
            Err("unexpected token (\"/\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Quote;

impl ParseTokens for Quote {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Quote) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Quote)
        } else {
            Err("unexpected token (\"\\\"\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Equals;

impl ParseTokens for Equals {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Equals) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Equals)
        } else {
            Err("unexpected token (\"=\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Word(pub String);

impl Word {
    fn new(word : String) -> Word {
        Word(word)
    }
}

impl ParseTokens for Word {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Word(word)) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Word::new(word.to_string()))
        } else {
            Err("unexpected token (word expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct WhitespaceToken(pub WhitespaceType);

impl WhitespaceToken {
    fn new(whitespace_type : WhitespaceType) -> WhitespaceToken {
        WhitespaceToken(whitespace_type)
    }
}

impl ParseTokens for WhitespaceToken {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Whitespace(whitespace_type)) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(WhitespaceToken::new(whitespace_type.clone()))
        } else {
            Err("unexpected token (whitespace expected)".to_string())
        }
    }
}
