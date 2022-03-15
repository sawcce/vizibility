use super::lexer::{Token, TokenMatch};
use std::mem::discriminant;

pub struct VizibilityParser {
    current_index: usize,
    tokens: Vec<TokenMatch>,
    skip_amount: usize,
}

#[derive(Debug)]
pub struct Program {

}

impl VizibilityParser {
    pub fn new(tokens: Vec<TokenMatch>) -> VizibilityParser {
        VizibilityParser {
            current_index: 0,
            skip_amount: 0,
            tokens,
        }
    }

    pub fn program(self) -> Result<Program, String> {
        let first = self.expect_ahead(Token::End)?;
        println!("First token : {}", first);
        Ok(Program {})
    }
}

impl Parser for VizibilityParser {
    fn expect_ahead(&self, variant: Token) -> Result<TokenMatch, String> {
        let tokens = self.tokens.clone();
        let computed = self.current_index + self.skip_amount;

        match self.tokens.get(computed) {
            Some(token) => {
                let token_type = token.token_type;

                if discriminant(&variant) == discriminant(&token_type) {
                    return Ok(token.clone())
                }

                return Err(format!("Err! Expected token of type \"{}\", got \"{}\" of type \"{}\"", variant, token.value, token_type));
            } ,
            None => {
                if let variant = Token::EOF {
                    return Ok(TokenMatch {token_type: Token::EOF, column: 0, line: 0, value: "".to_string()})
                }

                return Err("Out of bounds".to_string())
            },
        }
    }

    fn consume(&mut self) {

    }

    fn subrule<ReturnType>(&mut self, rule: fn(last_token: Option<TokenMatch>) -> ReturnType,  last_token: Option<TokenMatch>) -> ReturnType {
        let result = rule(last_token);
        self.current_index += self.skip_amount;
        self.skip_amount = 0;
        result
    }
}

trait Parser {
    fn expect_ahead(&self, variant: Token) -> Result<TokenMatch, String>;
    fn consume(&mut self);
    fn subrule<ReturnType>(&mut self, rule: fn(last_token: Option<TokenMatch>) -> ReturnType,  last_token: Option<TokenMatch>) -> ReturnType;
}
