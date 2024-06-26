pub mod lex;
pub mod parse;
pub mod format;

pub fn parse_file(string : String) -> Result<format::File, String> {
    let tokens = lex::lex_file(&string);
    let parsed = parse::parse_file_tokens(&tokens)?;
    let file = format::File::try_from(parsed)?;
    Ok(file)
}

pub fn highlight_tokens(tokens : &[lex::Token]) -> String {
    let mut s = String::new();
    for token in tokens.iter() {
        match token {
            lex::Token::Quote => { s += "\x1B[1;91m\"\x1B[0m"; }
            lex::Token::Dollar => { s += "\x1B[1;92m$\x1B[0m"; }
            lex::Token::DDollar => { s += "\x1B[1;93m$$\x1B[0m"; }
            lex::Token::Newline => { s += "\x1B[1;94m//\x1B[0m"; }
            lex::Token::Escape => { s += "\x1B[1;95m\\\x1B[0m"; }
            lex::Token::Word(text) => { s += &format!("\x1B[96m{}\x1B[0m", text); }
            lex::Token::Whitespace(whitespace_type) => {
                s +=
                    match whitespace_type {
                        lex::WhitespaceType::Space => "\x1B[1;41m \x1B[0m",
                        lex::WhitespaceType::Tab => "\x1B[1;42m    \x1B[0m",
                        lex::WhitespaceType::Newline => "\x1B[1;43m\n\x1B[0m"
                    };
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
