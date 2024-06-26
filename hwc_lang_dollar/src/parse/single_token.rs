use crate::lex::Token;
use crate::parse::ParseTokens;
use crate::lex::WhitespaceType;

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
pub struct Dollar;

impl ParseTokens for Dollar {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Dollar) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Dollar)
        } else {
            Err("unexpected token (\"$\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct DDollar;

impl ParseTokens for DDollar {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::DDollar) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(DDollar)
        } else {
            Err("unexpected token (\"$$\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Newline;

impl ParseTokens for Newline {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Newline) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Newline)
        } else {
            Err("unexpected token (\"//\" expected)".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Escape;

impl ParseTokens for Escape {
    type Output = Self;
    fn parse_mut_tokens(tokens : &mut &[Token]) -> Result<Self::Output, String> {
        if let Some(Token::Escape) = tokens.get(0) {
            *tokens = &tokens[1..];
            Ok(Escape)
        } else {
            Err("unexpected token (\"\\\" expected)".to_string())
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
