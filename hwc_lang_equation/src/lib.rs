pub mod format;
pub mod lex;
pub mod parse;

pub fn parse_geo_proof(string: String) -> Result<format::GeoProof, String> {
    let tokens = lex::lex_file(&string);
    let parsed = parse::parse_geo_proof_tokens(&tokens)?;
    let file = format::GeoProof::try_from(parsed)?;
    Ok(file)
}

pub fn parse_multiline_statements(string: String) -> Result<format::MultilineStatements, String> {
    let tokens = lex::lex_file(&string);
    let parsed = parse::parse_multiline_stmts_tokens(&tokens)?;
    let file = format::MultilineStatements::try_from(parsed)?;
    Ok(file)
}

pub fn parse_statements(string: String) -> Result<format::Statements, String> {
    let tokens = lex::lex_file(&string);
    let parsed = parse::parse_stmts_tokens(&tokens)?;
    let file = format::Statements::try_from(parsed)?;
    Ok(file)
}

pub fn parse_expression(string: String) -> Result<format::Expression, String> {
    let tokens = lex::lex_file(&string);
    let parsed = parse::parse_expr_tokens(&tokens)?;
    let file = format::Expression::try_from(parsed)?;
    Ok(file)
}

pub fn highlight_tokens(tokens: &[lex::Token]) -> String {
    let mut s = String::new();
    for token in tokens.iter() {
        let token_color = match token {
            lex::Token::LBrace | lex::Token::RBrace => "1;91m",
            lex::Token::LParen | lex::Token::RParen => "1;92m",
            lex::Token::InvRArrow | lex::Token::RArrow => "1;93m",
            lex::Token::PlusMinus | lex::Token::MinusPlus => "1;94m",
            lex::Token::Plus
            | lex::Token::Minus
            | lex::Token::At
            | lex::Token::Star
            | lex::Token::Slash
            | lex::Token::Caret
            | lex::Token::Tag => "1;95m",
            lex::Token::InvEquals
            | lex::Token::Equals
            | lex::Token::NotEquals
            | lex::Token::ApproxEquals
            | lex::Token::Lt
            | lex::Token::Gt
            | lex::Token::Le
            | lex::Token::Ge => "1;96m",
            lex::Token::Comma | lex::Token::Hash => "1;31m",
            lex::Token::Newline => "1;32m",
            lex::Token::Value(_) => "1m",
        };

        let token_text = match token {
            lex::Token::LBrace => "{",
            lex::Token::RBrace => "}",
            lex::Token::LParen => "(",
            lex::Token::RParen => ")",
            lex::Token::InvRArrow => "@=>",
            lex::Token::RArrow => "=>",
            lex::Token::PlusMinus => "+-",
            lex::Token::MinusPlus => "-+",
            lex::Token::Plus => "+",
            lex::Token::Minus => "-",
            lex::Token::At => "@",
            lex::Token::Star => "*",
            lex::Token::Slash => "/",
            lex::Token::Caret => "^",
            lex::Token::Tag => "'",
            lex::Token::InvEquals => "@=",
            lex::Token::Equals => "=",
            lex::Token::NotEquals => "!=",
            lex::Token::ApproxEquals => "~=",
            lex::Token::Lt => "<",
            lex::Token::Gt => ">",
            lex::Token::Le => "<=",
            lex::Token::Ge => ">=",
            lex::Token::Comma => ",",
            lex::Token::Hash => "#",
            lex::Token::Newline => "//",
            lex::Token::Value(value) => value,
        };
        s += &format!("\x1B[{}{}\x1B[0m", token_color, token_text);
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //crate::parse::parse_stmts_tokens(&crate::lex::lex_file("{v_1 +- x}, = 1.5 @ (v_1 - x) =>")).unwrap();
        //println!("{:?}", crate::parse::parse_stmts_tokens(&crate::lex::lex_file("{v_1 +- x} = 1.5 @ (v_1 - x) =>")).unwrap());
        println!(
            "{:?}",
            crate::parse_statements("{v_1 +- x} = 1.5 @ (v_1 - x) =>".to_string()).unwrap()
        );
        //panic!("");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
