use core::fmt;
use std::fmt::{format, Display, Formatter};

use regex::Regex;

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Fn,
    Do,
    End,
    Identifier,
    LParen,
    RParen,
    Equals,
    Let,
    Mut,
    Int,
    Skipped,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct TokenType(Token, Regex);

pub fn from_str_vec(vec: Vec<(&str, Token)>) -> Vec<TokenType> {
    let mut expr_vec: Vec<TokenType> = Vec::new();

    for (str, token) in vec.iter() {
        let mut prefixed: String = "^".to_owned();
        prefixed.push_str(str);

        let expr = match Regex::new(&prefixed) {
            Ok(expr) => expr,
            Err(_) => panic!("Error while compiling rule: \"{}\"", str),
        };

        expr_vec.push(TokenType(*token, expr));
    }

    expr_vec
}

#[derive(Clone)]
pub struct TokenMatch {
    tokenType: Token,
    value: String,
    line: i32,
    column: i32,
}

impl Display for TokenMatch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //write!(f, "ee")
        write!(
            f,
            "(\"{}\", {} at {}:{})",
            self.value, self.tokenType, self.line, self.column
        )
    }
}

pub fn tokenize(tokens: Vec<TokenType>, text: &str) -> Vec<TokenMatch> {
    let mut current_text = text.to_string();
    let mut matched_tokens: Vec<TokenMatch> = Vec::new();

    let mut at_least_one = false;

    while current_text.len() > 0 {
        at_least_one = false;
        'inner: for tokenType in tokens.iter() {
            let token_variant = tokenType.0;
            let regex_expr = &tokenType.1;

            let owned = &current_text.to_owned();
            let token_match = regex_expr.find(owned);

            match token_match {
                Some(matched) => {
                    at_least_one = true;
                    let value = matched.as_str();
                    current_text = current_text[value.len()..].to_string();

                    if let Token::Skipped = token_variant{
                        break 'inner;
                    }

                    matched_tokens.push(TokenMatch {
                        line: 00,
                        column: 0,
                        value: value.to_string(),
                        tokenType: token_variant,
                    });

                    break 'inner;
                }
                None => {
                    continue 'inner;
                }
            }
        }

        if at_least_one == false {
            panic!("No match for \"{}\"", current_text);
        }
    }

    return matched_tokens;
}
