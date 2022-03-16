use super::lexer::{Token, TokenMatch};
use ariadne::{Color, Label, Report, ReportKind, Source};
use std::io::Error;
use std::mem::discriminant;

pub struct VizibilityParser {
    source: &'static str,
    current_index: usize,
    tokens: Vec<TokenMatch>,
    skip_amount: usize,
}

#[derive(Debug)]
pub struct Program {
    func_name: String,
}



pub struct ErrorReport {
    offset: usize,
    message: String,
    label: String,
    length: usize,
}

pub fn id(parser: &mut impl Parser) -> Result<String, String> {
    let id_token = parser.expect_ahead(Token::Identifier)?;
    Ok(id_token.value)
}

pub fn program(mut parser: impl Parser) -> Result<Program, String> {
    parser.expect_ahead(Token::Fn)?;
    let name = parser.subrule(id, None)?;

    parser.expect_ahead(Token::LParen)?;
    parser.expect_ahead(Token::RParen)?;
    parser.expect_ahead(Token::End)?;

    Ok(Program {
        func_name: name,
    })
}

impl VizibilityParser {
    pub fn new(tokens: Vec<TokenMatch>, source: &'static str) -> VizibilityParser {
        VizibilityParser {
            source,
            current_index: 0,
            skip_amount: 0,
            tokens,
        }
    }
}

impl Parser for VizibilityParser {
    fn expect_ahead(&mut self, variant: Token) -> Result<TokenMatch, String> {
        let tokens = self.tokens.clone();
        let computed = self.current_index + self.skip_amount;

        match self.tokens.get(computed) {
            Some(token) => {
                let token_type = token.token_type;

                if discriminant(&variant) == discriminant(&token_type) {
                    self.skip_amount += 1;
                    return Ok(token.clone());
                }

                let message = format!(
                    "Err! Expected token of type \"{}\", got \"{}\" of type \"{}\"",
                    variant, token.value, token_type
                );
                let label = format!("This is of type {}", token_type);

                return Err(message);
            }
            None => {
                if let variant = Token::EOF {
                    let source_length = self.source.len();
                    return Ok(TokenMatch {
                        token_type: Token::EOF,
                        column: 0,
                        line: 0,
                        value: "".to_string(),
                        start: source_length - 1,
                        length: 1,
                    });
                }

                return Err("Out of bounds!".to_string());
            }
        }
    }

    fn subrule<ReturnType>(
        &mut self,
        rule: impl Fn(&mut VizibilityParser) -> ReturnType,
        last_token: Option<TokenMatch>,
    ) -> ReturnType {
        let result = rule(self);
        self.current_index += self.skip_amount;
        self.skip_amount = 0;
        result
    }
}

pub trait Parser {
    fn expect_ahead(&mut self, variant: Token) -> Result<TokenMatch, String>;
    fn subrule<ReturnType>(
        &mut self,
        rule: impl Fn(&mut VizibilityParser) -> ReturnType,
        last_token: Option<TokenMatch>,
    ) -> ReturnType;
}
