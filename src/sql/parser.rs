use anyhow::Context;

use crate::sql::{
    ast::{Column, Expr},
    tokenizer::Token,
};

use super::ast::{
    ExprResultColumn, ResultColumn, SelectCore, SelectFrom, SelectStatement, Statement,
};

#[derive(Debug)]
struct ParserState {
    tokens: Vec<Token>,
    pos: usize,
}

impl ParserState {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_statement(&mut self) -> Result<Statement, anyhow::Error> {
        Ok(Statement::Select(self.parse_select()?))
    }

    fn parse_select(&mut self) -> Result<SelectStatement, anyhow::Error> {
        self.expect_eq(Token::Select)?;
        let result_columns = self.parse_result_columns()?;
        self.expect_eq(Token::From)?;
        let from = self.parse_select_from()?;

        Ok(SelectStatement {
            core: SelectCore {
                result_columns,
                from,
            },
        })
    }

    fn parse_select_from(&mut self) -> Result<SelectFrom, anyhow::Error> {
        Ok(SelectFrom::Table(self.expect_identifier()?.to_string()))
    }

    fn parse_expr(&mut self) -> Result<Expr, anyhow::Error> {
        Ok(Expr::Column(Column {
            name: self.expect_identifier()?.to_string(),
        }))
    }

    fn parse_expr_result_column(&mut self) -> Result<ExprResultColumn, anyhow::Error> {
        let expr = self.parse_expr()?;
        let alias = if self.next_token_is(Token::As) {
            self.pos += 1;
            Some(self.expect_identifier()?.to_string())
        } else {
            None
        };

        Ok(ExprResultColumn { expr, alias })
    }

    fn parse_result_column(&mut self) -> Result<ResultColumn, anyhow::Error> {
        if self.peek_next_token()? == &Token::Star {
            self.pos += 1;
            return Ok(ResultColumn::Star);
        }

        Ok(ResultColumn::Expr(self.parse_expr_result_column()?))
    }

    fn parse_result_columns(&mut self) -> Result<Vec<ResultColumn>, anyhow::Error> {
        let mut result_columns = vec![self.parse_result_column()?];
        while self.next_token_is(Token::Comma) {
            self.pos += 1;
            result_columns.push(self.parse_result_column()?);
        }

        Ok(result_columns)
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
