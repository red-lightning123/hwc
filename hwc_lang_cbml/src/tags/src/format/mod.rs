use crate::{ lex, parse };
use std::collections::HashMap;

#[derive(Debug)]
pub struct File {
    root_element : Element
}

impl File {
    pub fn root_element(&self) -> &Element {
        &self.root_element
    }

    fn new(root_element : Element) -> File {
        File {
            root_element
        }
    }
}

impl TryFrom<parse::File> for File {
    type Error = String;
    fn try_from(file : parse::File) -> Result<Self, Self::Error> {
        let parse::File((_, root_element, _)) = file;
        Ok(File::new(Element::try_from(root_element)?))
    }
}

#[derive(Debug)]
pub struct Element {
    name : String,
    properties : HashMap<String, String>,
    children : Vec<Element>,
    text : String
}

impl Element {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    pub fn children(&self) -> &Vec<Element> {
        &self.children
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    fn new(name : String, properties : HashMap<String, String>, children : Vec<Element>, text : String) -> Element {
        Element {
            name,
            properties,
            children,
            text
        }
    }

    fn quoted_text_token_to_string(quoted_text_token : parse::QuotedTextToken) -> String {
        let parse::QuotedTextToken(quoted_text_token_variant) = quoted_text_token;
        match quoted_text_token_variant {
            parse::any_variants::AnyVariants6::V1(parse::LDBrace) => "{{".to_string(),
            parse::any_variants::AnyVariants6::V2(parse::RDBrace) => "}}".to_string(),
            parse::any_variants::AnyVariants6::V3(parse::Slash) => "/".to_string(),
            parse::any_variants::AnyVariants6::V4(parse::Equals) => "=".to_string(),
            parse::any_variants::AnyVariants6::V5(parse::Word(string)) => string,
            parse::any_variants::AnyVariants6::V6(parse::WhitespaceToken(whitespace_type)) => match whitespace_type {
                lex::WhitespaceType::Space => " ".to_string(),
                lex::WhitespaceType::Tab => "\t".to_string(),
                lex::WhitespaceType::Newline => "\n".to_string()
            }
        }
    }

    fn text_token_to_string(text_token : parse::TextToken) -> String {
        let parse::TextToken(text_token_variant) = text_token;
        match text_token_variant {
            parse::any_variants::AnyVariants5::V1(parse::Quote) => "\"".to_string(),
            parse::any_variants::AnyVariants5::V2(parse::Slash) => "/".to_string(),
            parse::any_variants::AnyVariants5::V3(parse::Equals) => "=".to_string(),
            parse::any_variants::AnyVariants5::V4(parse::Word(string)) => string,
            parse::any_variants::AnyVariants5::V5(parse::WhitespaceToken(whitespace_type)) => match whitespace_type {
                lex::WhitespaceType::Space => " ".to_string(),
                lex::WhitespaceType::Tab => "\t".to_string(),
                lex::WhitespaceType::Newline => "\n".to_string()
            }
        }
    }
    
    fn word_to_string(word : parse::Word) -> String {
        let parse::Word(string) = word;
        string
    }


    fn quoted_text_to_string(quoted_text : parse::QuotedText) -> String {
        let parse::QuotedText(quoted_text_tokens) = quoted_text;
        let mut string = String::new();
        for quoted_text_token in quoted_text_tokens {
            string += &Self::quoted_text_token_to_string(quoted_text_token);
        }
        string
    }

    fn text_to_string(text : parse::Text) -> String {
        let parse::Text(text_tokens) = text;
        let mut string = String::new();
        for text_token in text_tokens {
            string += &Self::text_token_to_string(text_token);
        }
        string
    }

    fn property_value_to_string(property_value : parse::PropertyValue) -> String {
        let parse::PropertyValue(property_value_variant) = property_value;
        match property_value_variant {
            parse::any_variants::AnyVariants2::V1(parse::UnquotedPropertyValue(word)) => Self::word_to_string(word),
            parse::any_variants::AnyVariants2::V2(parse::QuotedPropertyValue((_, quoted_text, _))) => Self::quoted_text_to_string(quoted_text)
        }
    }

    fn properties_to_hash_map(properties : Vec<parse::Property>) -> HashMap<String, String> {
        let mut properties_hash_map = HashMap::new();
        for property in properties {
            let parse::Property((key, _, _, _, value)) = property;
            let key = Self::word_to_string(key);
            let value = Self::property_value_to_string(value);
            properties_hash_map.insert(key, value);
        }
        properties_hash_map
    }

    fn whitespace_properties_to_hash_map(properties : Vec<(parse::Whitespace, parse::Property)>) -> HashMap<String, String> {
        Self::properties_to_hash_map(properties.into_iter().map(|(_, property)| property).collect::<Vec<_>>())
    }

    fn element_array_to_vec(element_array : parse::ElementArray) -> Result<Vec<Element>, String> {
        let parse::ElementArray((first_element, rest)) = element_array;
        let mut elements = vec![Element::try_from(first_element)?];
        for (_, element) in rest {
            elements.push(Element::try_from(element)?);
        }
        Ok(elements)
    }
}

impl TryFrom<parse::Element> for Element {
    type Error = String;
    fn try_from(element : parse::Element) -> Result<Self, Self::Error> {
        let parse::Element(element_variant) = element;
        match element_variant {
            parse::any_variants::AnyVariants3::V1(parse::NestElement((open_tag, _, element_array_boxed, _, close_tag))) => {
                let parse::OpenTag((_, _, open_tag_name, properties, _, _)) = open_tag;
                let children = Self::element_array_to_vec(*element_array_boxed)?;
                let parse::CloseTag((_, _, _, _, close_tag_name, _, _)) = close_tag;

                let open_tag_name = Self::word_to_string(open_tag_name);
                let close_tag_name = Self::word_to_string(close_tag_name);

                if open_tag_name != close_tag_name {
                    Err("open/close tag pair didn't match".to_string())
                } else {
                    let properties_hash_map = Self::whitespace_properties_to_hash_map(properties);
                    Ok(Element::new(open_tag_name, properties_hash_map, children, String::new()))
                }
            }
            parse::any_variants::AnyVariants3::V2(parse::TextElement((open_tag, text, close_tag))) => {
                let parse::OpenTag((_, _, open_tag_name, properties, _, _)) = open_tag;
                let text = Self::text_to_string(text);
                let parse::CloseTag((_, _, _, _, close_tag_name, _, _)) = close_tag;

                let open_tag_name = Self::word_to_string(open_tag_name);
                let close_tag_name = Self::word_to_string(close_tag_name);

                if open_tag_name != close_tag_name {
                    Err("open/close tag pair didn't match".to_string())
                } else {
                    let properties_hash_map = Self::whitespace_properties_to_hash_map(properties);
                    Ok(Element::new(open_tag_name, properties_hash_map, vec![], text))
                }
            }
            parse::any_variants::AnyVariants3::V3(parse::EmptyElement(open_close_tag)) => {
                let parse::OpenCloseTag((_, _, name, properties, _, _, _, _)) = open_close_tag;

                let name = Self::word_to_string(name);

                let properties_hash_map = Self::whitespace_properties_to_hash_map(properties);
                Ok(Element::new(name, properties_hash_map, vec![], String::new()))
            }
        }
    }
}
