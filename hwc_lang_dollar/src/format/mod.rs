use crate::{ lex, parse };

#[derive(Debug)]
pub struct File {
    components : Vec<Component>
}

impl File {
    pub fn components(&self) -> &Vec<Component> {
        &self.components
    }

    fn new(components : Vec<Component>) -> File {
        File {
            components
        }
    }
}

impl TryFrom<parse::File> for File {
    type Error = String;
    fn try_from(file : parse::File) -> Result<Self, Self::Error> {
        let parse::File((_, components_and_whitespaces)) = file;
        let mut components = vec![];
        for (component, _) in components_and_whitespaces {
            components.push(Component::try_from(component)?);
        }
        Ok(File::new(components))
    }
}

#[derive(Debug)]
pub enum Component {
    InlineText(String),
    InlineMath(String),
    MultilineMath(String),
    Newline
}

impl TryFrom<parse::Component> for Component {
    type Error = String;
    fn try_from(component : parse::Component) -> Result<Self, Self::Error> {
        let parse::Component(component_variant) = component;
        match component_variant {
            parse::any_variants::AnyVariants4::V1(parse::InlineText((_, literal, _))) => {
                let string = literal_to_string(literal);
                Ok(Component::InlineText(string))
            }
            parse::any_variants::AnyVariants4::V2(parse::InlineMath((_, literal, _))) => {
                let string = literal_to_string(literal);
                Ok(Component::InlineMath(string))
            }
            parse::any_variants::AnyVariants4::V3(parse::MultilineMath((_, literal, _))) => {
                let string = literal_to_string(literal);
                Ok(Component::MultilineMath(string))
            }
            parse::any_variants::AnyVariants4::V4(parse::Newline) => Ok(Component::Newline)
        }
    }
}

fn literal_to_string(literal : parse::Literal) -> String {
    let parse::Literal(literal_tokens) = literal;
    let mut string = String::new();
    for literal_token in literal_tokens {
        string += &literal_token_to_string(literal_token);
    }
    string
}

fn literal_token_to_string(literal_token : parse::LiteralToken) -> String {
    let parse::LiteralToken(literal_token_variant) = literal_token;
    match literal_token_variant {
        parse::any_variants::AnyVariants4::V1(parse::Newline) => "//".to_string(),
        parse::any_variants::AnyVariants4::V2(parse::Word(string)) => string,
        parse::any_variants::AnyVariants4::V3(escaped_token @ parse::EscapedToken(_)) => escaped_token_as_str(escaped_token).to_string(),
        parse::any_variants::AnyVariants4::V4(parse::WhitespaceToken(whitespace_type)) => match whitespace_type {
            lex::WhitespaceType::Space => " ".to_string(),
            lex::WhitespaceType::Tab => "\t".to_string(),
            lex::WhitespaceType::Newline => "\n".to_string()
        }
    }
}

fn escaped_token_as_str(escaped_token : parse::EscapedToken) -> &'static str {
    let parse::EscapedToken((_, escaped_token_variant)) = escaped_token;
    match escaped_token_variant {
        parse::any_variants::AnyVariants3::V1(parse::Quote) => "\"",
        parse::any_variants::AnyVariants3::V2(parse::Dollar) => "$",
        parse::any_variants::AnyVariants3::V3(parse::Escape) => "\\"
    }
}
