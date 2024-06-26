use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone)]
pub enum Token<'a> {
    LParen,
    RParen,
    LBrace,
    RBrace,

    // statement relations
    InvRArrow,
    RArrow,

    // operators
    PlusMinus,
    MinusPlus,
    Plus,
    Minus,
    At,
    Star,
    Slash,
    Caret,
    Tag,

    // expression relations
    InvEquals,
    Equals,
    NotEquals,
    ApproxEquals,
    Lt,
    Gt,
    Le,
    Ge,

    Comma,
    Hash,

    Newline,

    Value(&'a str),
}

enum LexerState {
    Free,
    LexingWord(usize),
    LexingString(usize),
}

struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
    state: LexerState,
    string: &'a str,
    string_it: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(string: &'a str) -> Lexer {
        Lexer {
            tokens: vec![],
            state: LexerState::Free,
            string,
            string_it: string.char_indices().peekable(),
        }
    }
    fn run(mut self) -> Vec<Token<'a>> {
        while self.consume_next() {}
        self.tokens
    }
    fn try_advance_char_iter_by(
        mut iter: Peekable<CharIndices<'a>>,
        s: &str,
    ) -> Option<Peekable<CharIndices<'a>>> {
        for ch in s.chars() {
            iter.next().filter(|(_, ch2)| ch == *ch2)?;
        }
        Some(iter)
    }

    fn try_advance_char_iter_by_word_char(
        mut iter: Peekable<CharIndices<'a>>,
    ) -> Option<Peekable<CharIndices<'a>>> {
        match iter.next()?.1 {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' | '%' => Some(iter),
            _ => None,
        }
    }

    fn try_advance_char_iter_by_whitespace_char(
        mut iter: Peekable<CharIndices<'a>>,
    ) -> Option<Peekable<CharIndices<'a>>> {
        if iter.next()?.1.is_whitespace() {
            Some(iter)
        } else {
            None
        }
    }

    fn try_advance_char_iter_by_special_token(
        iter: &Peekable<CharIndices<'a>>,
    ) -> Option<(Token<'a>, Peekable<CharIndices<'a>>)> {
        const SPECIAL_TOKENS: &[(Token, &str)] = &[
            (Token::Newline, "//"), // has to be before /
            (Token::LParen, "("),
            (Token::RParen, ")"),
            (Token::LBrace, "{"),
            (Token::RBrace, "}"),
            (Token::InvRArrow, "@=>"),
            (Token::InvEquals, "@="), // has to be before @
            (Token::RArrow, "=>"),
            (Token::PlusMinus, "+-"),
            (Token::MinusPlus, "-+"),
            (Token::Plus, "+"),
            (Token::Minus, "-"),
            (Token::At, "@"),
            (Token::Star, "*"),
            (Token::Slash, "/"),
            (Token::Caret, "^"),
            (Token::Tag, "'"),
            (Token::Equals, "="),
            (Token::NotEquals, "!="),
            (Token::ApproxEquals, "~="),
            (Token::Le, "<="),
            (Token::Ge, ">="),
            (Token::Lt, "<"),
            (Token::Gt, ">"),
            (Token::Comma, ","),
            (Token::Hash, "#"),
        ];
        for (token, chars) in SPECIAL_TOKENS {
            if let Some(advanced_it) = Self::try_advance_char_iter_by(iter.clone(), chars) {
                return Some(((*token).clone(), advanced_it));
            }
        }
        None
    }
    fn consume_next(&mut self) -> bool {
        if let Some((index, _)) = self.string_it.peek() {
            let index = *index; // otherwise uses of index would unnecessarily extend the mutable borrow of string_it
            match self.state {
                LexerState::Free => {
                    if let Some((token, advanced_it)) =
                        Self::try_advance_char_iter_by_special_token(&self.string_it.clone())
                    {
                        self.tokens.push(token);
                        self.string_it = advanced_it;
                    } else if let Some(advanced_it) =
                        Self::try_advance_char_iter_by(self.string_it.clone(), "\"")
                    {
                        self.state = LexerState::LexingString(index + 1);
                        self.string_it = advanced_it;
                    } else if let Some(advanced_it) =
                        Self::try_advance_char_iter_by_word_char(self.string_it.clone())
                    {
                        self.state = LexerState::LexingWord(index);
                        self.string_it = advanced_it;
                    } else if let Some(advanced_it) =
                        Self::try_advance_char_iter_by_whitespace_char(self.string_it.clone())
                    {
                        self.string_it = advanced_it;
                    } else {
                        panic!("equation lexer received unexpected char");
                    }
                }
                LexerState::LexingWord(start) => {
                    if let Some(advanced_it) =
                        Self::try_advance_char_iter_by_word_char(self.string_it.clone())
                    {
                        self.string_it = advanced_it;
                    } else {
                        self.state = LexerState::Free;
                        self.tokens.push(Token::Value(&self.string[start..index]));
                    }
                }
                LexerState::LexingString(start) => {
                    if let Some(advanced_it) =
                        Self::try_advance_char_iter_by(self.string_it.clone(), "\"")
                    {
                        self.state = LexerState::Free;
                        self.tokens.push(Token::Value(&self.string[start..index]));
                        self.string_it = advanced_it;
                    } else {
                        self.string_it.next();
                    }
                }
            }
            true
        } else {
            match self.state {
                LexerState::Free => {}
                LexerState::LexingWord(start) => {
                    self.tokens.push(Token::Value(&self.string[start..]))
                }
                LexerState::LexingString(_) => {
                    panic!("equation lexer received unterminated string")
                }
            }
            false
        }
    }
}

pub fn lex_file(s: &str) -> Vec<Token> {
    let lexer = Lexer::new(s);
    lexer.run()
}
