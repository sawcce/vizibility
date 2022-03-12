use super::lexer::{Token, TokenMatch};

pub struct VizibilityParser {
    current_index: usize,
    tokens: Vec<TokenMatch>,
}

impl VizibilityParser {
    fn program(self) {
        self.look_ahead(0);
    }
}

impl Parser for VizibilityParser {
    fn look_ahead(&self, skip_amount: usize) -> Result<TokenMatch, &'static str> {
        let tokens = self.tokens.clone();
        let computed = self.current_index + skip_amount;

        match self.tokens.get(computed) {
            Some(value) => Ok(value.clone()),
            None => Ok(TokenMatch {
                token_type: Token::EOF,
                value: "Out of bounds".to_string(),
                line: 0,
                column: 0,
            }),
        }
    }
}

trait Parser {
    fn look_ahead(&self, skip_amount: usize) -> Result<TokenMatch, &'static str>;
}
