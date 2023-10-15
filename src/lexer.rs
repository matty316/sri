use crate::token::*;
use std::collections::HashMap;

struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
    line: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.as_bytes().to_vec(),
            position: 0,
            read_position: 0,
            ch: b'\0',
            line: 1,
        };

        l.read_char();
        return l
    }

    fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_whitespace();

        match self.ch {
            b'\0' => tok = newToken(TokenType::Eof, "".to_string()),
            _ => {
                print!("CH: {}\n", self.ch);
                if is_alpha(self.ch) {
                    tok = self.read_ident();
                    return tok;
                } else {
                    tok = Token {token_type: TokenType::Illegal, literal: "".to_string() }
                }
            },
        }

        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(& mut self) {
        while self.ch == b' ' || self.ch == b'\r' {
            self.read_char()
        }
    }

    fn read_ident(&mut self) -> Token {
        let start = self.position;

        while is_alpha(self.ch) {
            self.read_char();
        }

        let bytes = &self.input[start..self.position];

        let identifier = match std::str::from_utf8(bytes) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),//TODO: handle err
        };

        let keywords = HashMap::from([
            ("let", TokenType::Let)
        ]);

        match keywords.get(identifier) {
            Some(t) => return Token {token_type: t.clone(), literal: identifier.to_string()},
            _ => return Token {token_type: TokenType::Ident, literal: identifier.to_string()},
        }
    }
}

fn newToken(token_type: TokenType, literal: String) -> Token {
    Token {
        token_type: token_type,
        literal: literal
    }
}

fn is_alpha(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r###"let five = 5
        let ten = 10.5

        let string = "name"; let another = "another name"

        fun add(x, y):
            x + y   

        let result = add(five, ten)
        !-/*5
        5 < 10 > 5

        5 <= 10 >= 5

        if 5 < 10:
            return "if"
        else if:
            return "else if"
        else:
            return "else"

        10 == 10
        10 != 9
        "###;

        let mut l = Lexer::new(input.to_string());

        let expected: [Token; 2]= [
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string()
            },
            Token {
                token_type: TokenType::Ident,
                literal: "five".to_string()
            },
        ];

        for e in expected.iter() {
            let t = l.next_token();
            assert_eq!(e.token_type, t.token_type);
            assert_eq!(e.literal, t.literal);
        }
    }
}
