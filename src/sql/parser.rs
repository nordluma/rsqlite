use anyhow::Context;

use crate::sql::tokenizer::Token;

#[derive(Debug)]
struct ParserState {
    tokens: Vec<Token>,
    pos: usize,
}

impl ParserState {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn next_token_is(&self, expected: Token) -> bool {
        self.tokens.get(self.pos) == Some(&expected)
    }

    fn expect_identifier(&mut self) -> Result<&str, anyhow::Error> {
        self.expect_matching(|t| matches!(t, Token::Identifier(_)))
            .map(|t| t.as_identifier().unwrap())
    }

    fn expect_eq(&mut self, expected: Token) -> Result<&Token, anyhow::Error> {
        self.expect_matching(|t| *t == expected)
    }

    fn expect_matching(&mut self, f: impl Fn(&Token) -> bool) -> Result<&Token, anyhow::Error> {
        match self.next_token() {
            Some(token) if f(token) => Ok(token),
            Some(token) => anyhow::bail!("unexpected token: {token:?}"),
            None => anyhow::bail!("unexpected end of input"),
        }
    }

    fn peek_next_token(&self) -> Result<&Token, anyhow::Error> {
        self.tokens.get(self.pos).context("unexpected end of input")
    }

    fn next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            // advance posititon
            self.pos += 1;
        }

        token
    }
}
