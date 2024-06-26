use crate::lex::Token;

pub trait ParseTokens {
    type Output;
    fn parse_mut_tokens<'a>(tokens : &mut &'a [Token<'a>]) -> Result<Self::Output, String>;
    fn parse_tokens<'a>(tokens : &'a [Token]) -> Result<(&'a [Token<'a>], Self::Output), String> {
        parse_tokens(tokens, Self::parse_mut_tokens)
    }
}

fn parse_tokens<'a, F, T>(tokens : &'a [Token<'a>], parse_mut : F) -> Result<(&'a [Token<'a>], T), String>
where for<'b> F : FnOnce(&mut &'b [Token<'b>]) -> Result<T, String> {
    let mut new_tokens = tokens;
    match parse_mut(&mut new_tokens) {
        Ok(parsed) => { Ok((new_tokens, parsed)) }
        Err(e) => Err(e)
    }
}

