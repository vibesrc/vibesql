//! SQL Parser.
//!
//! This module provides a recursive descent parser that converts a stream of tokens
//! into an Abstract Syntax Tree (AST).

mod expr;
mod query;
mod stmt;

use crate::ast::*;
use crate::error::{Error, Result, Span};
use crate::lexer::{Keyword, Lexer, Token, TokenKind};

/// SQL Parser.
///
/// The parser converts SQL text into an Abstract Syntax Tree (AST).
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    #[allow(dead_code)]
    input: &'a str,
}

impl<'a> Parser<'a> {
    /// Create a new parser for the given input.
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            input,
        }
    }

    /// Parse all statements from the input.
    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        loop {
            // Skip empty statements (standalone semicolons)
            while self.consume(&TokenKind::Semicolon)?.is_some() {}

            if self.check_eof()? {
                break;
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);

            // Optional semicolon between statements
            self.consume(&TokenKind::Semicolon)?;
        }

        Ok(statements)
    }

    /// Parse a single statement.
    pub fn parse_statement(&mut self) -> Result<Statement> {
        let token = self.peek()?;
        let start = token.span.start;

        let kind = match &token.kind {
            TokenKind::Keyword(kw) => match kw {
                Keyword::Select | Keyword::With => {
                    let query = self.parse_query()?;
                    StatementKind::Query(Box::new(query))
                }
                Keyword::Insert => self.parse_insert()?,
                Keyword::Update => self.parse_update()?,
                Keyword::Delete => self.parse_delete()?,
                Keyword::Merge => self.parse_merge()?,
                Keyword::Create => self.parse_create()?,
                Keyword::Alter => self.parse_alter()?,
                Keyword::Drop => self.parse_drop()?,
                Keyword::Truncate => self.parse_truncate()?,
                Keyword::Begin => self.parse_begin()?,
                Keyword::Commit => {
                    self.advance()?;
                    StatementKind::Commit
                }
                Keyword::Rollback => self.parse_rollback()?,
                Keyword::Explain => self.parse_explain()?,
                Keyword::Describe => self.parse_describe()?,
                Keyword::Show => self.parse_show()?,
                Keyword::Set => self.parse_set()?,
                _ => {
                    return Err(Error::unexpected_token(
                        "statement",
                        format!("{}", token.kind),
                        token.span,
                    ));
                }
            },
            TokenKind::LeftParen => {
                // Parenthesized query
                let query = self.parse_query()?;
                StatementKind::Query(Box::new(query))
            }
            _ => {
                return Err(Error::unexpected_token(
                    "statement",
                    format!("{}", token.kind),
                    token.span,
                ));
            }
        };

        let end = self.current_position();
        Ok(Statement::new(kind, Span::new(start, end)))
    }

    // ========================================================================
    // Parser utilities
    // ========================================================================

    /// Get the current position in the input.
    fn current_position(&self) -> usize {
        self.lexer.position()
    }

    /// Peek at the next token.
    fn peek(&mut self) -> Result<&Token> {
        self.lexer.peek()
    }

    /// Peek at the nth token ahead.
    fn peek_nth(&mut self, n: usize) -> Result<&Token> {
        self.lexer.peek_nth(n)
    }

    /// Advance to the next token and return it.
    fn advance(&mut self) -> Result<Token> {
        self.lexer.next_token_result()
    }

    /// Check if the next token is EOF.
    fn check_eof(&mut self) -> Result<bool> {
        Ok(self.peek()?.is_eof())
    }

    /// Check if the next token matches the expected kind.
    fn check(&mut self, expected: &TokenKind) -> Result<bool> {
        Ok(&self.peek()?.kind == expected)
    }

    /// Check if the next token is a specific keyword.
    fn check_keyword(&mut self, keyword: Keyword) -> Result<bool> {
        Ok(self.peek()?.is_keyword(keyword))
    }

    /// Consume the next token if it matches.
    fn consume(&mut self, expected: &TokenKind) -> Result<Option<Token>> {
        self.lexer.consume(expected)
    }

    /// Consume a keyword if it matches.
    fn consume_keyword(&mut self, keyword: Keyword) -> Result<Option<Token>> {
        self.lexer.consume_keyword(keyword)
    }

    /// Expect and consume a specific token.
    fn expect(&mut self, expected: &TokenKind) -> Result<Token> {
        self.lexer.expect(expected)
    }

    /// Expect and consume a specific keyword.
    fn expect_keyword(&mut self, keyword: Keyword) -> Result<Token> {
        self.lexer.expect_keyword(keyword)
    }

    /// Parse an identifier.
    fn parse_identifier(&mut self) -> Result<Ident> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::Identifier(name) => Ok(Ident::new(name, token.span)),
            TokenKind::QuotedIdentifier(name) => Ok(Ident::quoted(name, token.span)),
            TokenKind::Keyword(kw) if !kw.is_reserved() => Ok(Ident::new(token.text, token.span)),
            _ => Err(Error::expected_identifier(token.span)),
        }
    }

    /// Parse an identifier, allowing reserved keywords (for aliases).
    fn parse_identifier_allow_reserved(&mut self) -> Result<Ident> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::Identifier(name) => Ok(Ident::new(name, token.span)),
            TokenKind::QuotedIdentifier(name) => Ok(Ident::quoted(name, token.span)),
            TokenKind::Keyword(_) => Ok(Ident::new(token.text, token.span)),
            _ => Err(Error::expected_identifier(token.span)),
        }
    }

    /// Parse an object name (possibly qualified: schema.table).
    fn parse_object_name(&mut self) -> Result<ObjectName> {
        let mut parts = vec![self.parse_identifier()?];
        let start = parts[0].span.start;

        while self.consume(&TokenKind::Dot)?.is_some() {
            parts.push(self.parse_identifier()?);
        }

        let end = parts.last().map(|p| p.span.end).unwrap_or(start);
        Ok(ObjectName::new(parts, Span::new(start, end)))
    }

    /// Parse a comma-separated list.
    fn parse_comma_separated<T, F>(&mut self, mut parse_fn: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let mut items = vec![parse_fn(self)?];

        while self.consume(&TokenKind::Comma)?.is_some() {
            items.push(parse_fn(self)?);
        }

        Ok(items)
    }

    /// Parse an optional alias (AS name or just name).
    fn parse_optional_alias(&mut self) -> Result<Option<Ident>> {
        if self.consume_keyword(Keyword::As)?.is_some() {
            Ok(Some(self.parse_identifier_allow_reserved()?))
        } else if self.peek()?.is_identifier() || !self.peek()?.is_any_keyword() {
            // Check if next token could be an alias
            let token = self.peek()?;
            if matches!(
                &token.kind,
                TokenKind::Identifier(_) | TokenKind::QuotedIdentifier(_)
            ) || matches!(&token.kind, TokenKind::Keyword(kw) if !kw.is_reserved())
            {
                Ok(Some(self.parse_identifier_allow_reserved()?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Parse an optional table alias with column aliases.
    fn parse_optional_table_alias(&mut self) -> Result<Option<Alias>> {
        if let Some(name) = self.parse_optional_alias()? {
            let columns = if self.consume(&TokenKind::LeftParen)?.is_some() {
                let cols = self.parse_comma_separated(|p| p.parse_identifier())?;
                self.expect(&TokenKind::RightParen)?;
                cols
            } else {
                Vec::new()
            };
            Ok(Some(Alias::with_columns(name, columns)))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn parse_stmt(sql: &str) -> Statement {
        let mut parser = Parser::new(sql);
        parser.parse_statement().expect("Failed to parse")
    }

    fn parse_all(sql: &str) -> Vec<Statement> {
        let mut parser = Parser::new(sql);
        parser.parse().expect("Failed to parse")
    }

    #[test]
    fn test_empty_input() {
        let stmts = parse_all("");
        assert!(stmts.is_empty());
    }

    #[test]
    fn test_semicolons() {
        let stmts = parse_all(";;;");
        assert!(stmts.is_empty());
    }

    #[test]
    fn test_multiple_statements() {
        let stmts = parse_all("SELECT 1; SELECT 2; SELECT 3");
        assert_eq!(stmts.len(), 3);
    }
}
