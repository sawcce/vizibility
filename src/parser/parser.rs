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
    first_token_value: String,
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

    pub fn program(mut self) -> Result<Program, ()> {
        let first = self.expect_ahead(Token::Fn)?;
        let second = self.expect_ahead(Token::Identifier)?;

        Ok(Program {
            first_token_value: first.value,
        })
    }
}

impl Parser for VizibilityParser {
    fn expect_ahead(&mut self, variant: Token) -> Result<TokenMatch, ()> {
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

                let x = Report::build(ReportKind::Error, (), 0)
                    .with_message(message)
                    .with_label(
                        Label::new(token.start..token.start + token.length)
                            .with_message(label)
                            .with_color(Color::Cyan),
                    )
                    .finish()
                    .print(Source::from(self.source))
                    .unwrap();

                return Err(x);
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

                println!("Out of bounds!");
                return Err(());
            }
        }
    }

    fn subrule<ReturnType>(
        &mut self,
        rule: fn(last_token: Option<TokenMatch>) -> ReturnType,
        last_token: Option<TokenMatch>,
    ) -> ReturnType {
        let result = rule(last_token);
        self.current_index += self.skip_amount;
        self.skip_amount = 0;
        result
    }
}

trait Parser {
    fn expect_ahead(&mut self, variant: Token) -> Result<TokenMatch, ()>;
    fn subrule<ReturnType>(
        &mut self,
        rule: fn(last_token: Option<TokenMatch>) -> ReturnType,
        last_token: Option<TokenMatch>,
    ) -> ReturnType;
}
