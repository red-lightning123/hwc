pub mod lex;
pub mod parse;
pub mod format;
pub mod resolved_file;

pub fn parse_file(string : String) -> Result<format::File, String> {
    let tokens = lex::lex_file(&string);
    let parsed = parse::parse_file_tokens(&tokens)?;
    let file = format::File::try_from(parsed)?;
    Ok(file)
}

pub fn resolve_file(file : format::File) -> Result<resolved_file::ResolvedFile, String> {
    resolved_file::ResolvedFile::try_from(file)
}

fn highlight_tokens(tokens : &[lex::Token]) -> String {
    let mut s = String::new();
    for token in tokens {
        match token {
            lex::Token::DHash => { s += "\x1B[1;91m##\x1B[0m"; }
            lex::Token::IDHash => { s += "\x1B[1;92m>##\x1B[0m"; }
            lex::Token::Text(text) => { s += &format!("\x1B[93m{}\x1B[0m", text); }
        };
    }
    s
}

pub fn highlight(data : &str) -> String {
    let tokens = lex::lex_file(data);
    highlight_tokens(&tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
