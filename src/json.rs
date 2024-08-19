
use std::option::Option::*;
use std::fmt;

#[derive(Debug)]
pub enum Token {
    Unknown,
    WS(char),
    Alpha(char),
    Digit(char),
    Sign(char),
}

pub fn tokenize( ) -> Vec<Token> {

    let input = " \n\r\ta93--+4bc";
    let mut chars = input.chars();
    let mut tokens:Vec<Token> = vec![];

    while let Some(c) = chars.next() {
        let e: Token;
        match c {
            '+' | '-' => {
                e = Token::Sign(c);
            },
            '0'..='9' => {
                e = Token::Digit(c);
            },
            'a'..='z' => {
                e = Token::Alpha(c);
            },
            
            '\t'| '\n' | '\r' | ' ' => {
                e = Token::WS(c);
            },
            _ => { 
                e = Token::Unknown;
            }
        }
        tokens.push(e);
    }

    tokens

}

struct Parser {
    tokens: Vec<Token>,
}

#[cfg(test)]
mod tests {

    use self::super::*;

    #[test]
    fn assert_tokens() {
        let mut tokens = tokenize();

        for token in tokens {



        }

        assert!(true); 
    }
}
