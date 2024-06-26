use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone)]
pub enum Token<'a> {
    DHash,
    IDHash,
    Text(&'a str)
}

enum LexerState {
    Free,
    LexingText(usize),
}

struct Lexer<'a> {
    tokens : Vec<Token<'a>>,
    state : LexerState,
    string : &'a str,
    string_it : Peekable<CharIndices<'a>>
}

impl<'a> Lexer<'a> {
    fn new(string : &'a str) -> Lexer {
        Lexer {
            tokens: vec![],
            state: LexerState::Free,
            string,
            string_it: string.char_indices().peekable()
        }
    }
    fn run(mut self) -> Vec<Token<'a>> {
        while self.consume_next() {
        }
        self.tokens
    }
    fn try_advance_char_iter_by(mut iter : Peekable<CharIndices<'a>>, s : &str) -> Option<Peekable<CharIndices<'a>>> {
        for ch in s.chars() {
            iter.next().filter(|(_, ch2)| ch == *ch2)?;
        }
        Some(iter)
    }
    fn try_advance_char_iter_by_special_token(iter : &Peekable<CharIndices<'a>>) -> Option<(Token<'a>, Peekable<CharIndices<'a>>)> {
        const SPECIAL_TOKENS : &[(Token, &str)] =
            &[
                (Token::DHash, "##"),
                (Token::IDHash, ">##")
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
            if let Some((token, advanced_it)) = Self::try_advance_char_iter_by_special_token(&self.string_it) {
                match self.state {
                    LexerState::Free => { }
                    LexerState::LexingText(start) => {
                        self.state = LexerState::Free;
                        self.tokens.push(Token::Text(&self.string[start..index]));
                    }
                }
                self.tokens.push(token);
                self.string_it = advanced_it;
            } else {
                match self.state {
                    LexerState::Free => {
                        self.state = LexerState::LexingText(index);
                    }
                    LexerState::LexingText(_) => { }
                }
                self.string_it.next();
            }
            true
        } else {
            match self.state {
                LexerState::Free => { },
                LexerState::LexingText(start) => self.tokens.push(Token::Text(&self.string[start..]))
            }
            false
        }
    }
}

pub fn lex_file(s : &str) -> Vec<Token> {
    let lexer = Lexer::new(s);
    lexer.run()
}