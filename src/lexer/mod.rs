//! SQL Lexer/Tokenizer.
//!
//! This module provides a lexer that tokenizes SQL input into a stream of tokens
//! according to standard SQL lexical conventions.

mod token;

pub use token::{Keyword, Token, TokenKind};

use crate::error::{Error, Result, Span};

/// SQL Lexer that tokenizes input into a stream of tokens.
pub struct Lexer<'a> {
    /// The input source string.
    input: &'a str,
    /// The input as bytes for efficient access.
    bytes: &'a [u8],
    /// Current position in the input (byte offset).
    pos: usize,
    /// Start position of current token.
    start: usize,
    /// Peeked tokens (for lookahead).
    peeked: Vec<Token>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            pos: 0,
            start: 0,
            peeked: Vec::new(),
        }
    }

    /// Get the current position in the input.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Get the remaining input from current position.
    pub fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    /// Peek at the next token without consuming it.
    pub fn peek(&mut self) -> Result<&Token> {
        if self.peeked.is_empty() {
            let token = self.next_token()?;
            self.peeked.push(token);
        }
        Ok(&self.peeked[0])
    }

    /// Peek at the nth token ahead (0 = next token).
    pub fn peek_nth(&mut self, n: usize) -> Result<&Token> {
        while self.peeked.len() <= n {
            let token = self.next_token()?;
            self.peeked.push(token);
        }
        Ok(&self.peeked[n])
    }

    /// Get the next token, consuming it.
    pub fn next_token_result(&mut self) -> Result<Token> {
        if !self.peeked.is_empty() {
            // Remove from front to maintain order
            return Ok(self.peeked.remove(0));
        }
        self.next_token()
    }

    /// Consume the next token if it matches the expected kind.
    pub fn consume(&mut self, expected: &TokenKind) -> Result<Option<Token>> {
        let token = self.peek()?;
        if &token.kind == expected {
            Ok(Some(self.next_token_result()?))
        } else {
            Ok(None)
        }
    }

    /// Consume the next token if it's the specified keyword.
    pub fn consume_keyword(&mut self, keyword: Keyword) -> Result<Option<Token>> {
        let token = self.peek()?;
        if token.is_keyword(keyword) {
            Ok(Some(self.next_token_result()?))
        } else {
            Ok(None)
        }
    }

    /// Expect and consume a specific token kind.
    pub fn expect(&mut self, expected: &TokenKind) -> Result<Token> {
        let token = self.next_token_result()?;
        if &token.kind == expected {
            Ok(token)
        } else {
            Err(Error::unexpected_token(
                format!("{}", expected),
                format!("{}", token.kind),
                token.span,
            ))
        }
    }

    /// Expect and consume a specific keyword.
    pub fn expect_keyword(&mut self, keyword: Keyword) -> Result<Token> {
        let token = self.next_token_result()?;
        if token.is_keyword(keyword) {
            Ok(token)
        } else {
            Err(Error::unexpected_token(
                format!("keyword {}", keyword),
                format!("{}", token.kind),
                token.span,
            ))
        }
    }

    /// Internal: get the next token from input.
    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace_and_comments()?;

        self.start = self.pos;

        if self.is_at_end() {
            return Ok(self.make_token(TokenKind::Eof));
        }

        let c = self.advance();

        match c {
            // Single-character tokens
            '(' => Ok(self.make_token(TokenKind::LeftParen)),
            ')' => Ok(self.make_token(TokenKind::RightParen)),
            '[' => Ok(self.make_token(TokenKind::LeftBracket)),
            ']' => Ok(self.make_token(TokenKind::RightBracket)),
            '{' => Ok(self.make_token(TokenKind::LeftBrace)),
            '}' => Ok(self.make_token(TokenKind::RightBrace)),
            ',' => Ok(self.make_token(TokenKind::Comma)),
            ';' => Ok(self.make_token(TokenKind::Semicolon)),
            '+' => Ok(self.make_token(TokenKind::Plus)),
            '*' => Ok(self.make_token(TokenKind::Star)),
            '%' => Ok(self.make_token(TokenKind::Percent)),
            '^' => Ok(self.make_token(TokenKind::Caret)),
            '~' => Ok(self.make_token(TokenKind::Tilde)),
            '?' => Ok(self.make_token(TokenKind::Question)),
            '@' => Ok(self.make_token(TokenKind::At)),
            '#' => Ok(self.make_token(TokenKind::Hash)),
            '$' => Ok(self.make_token(TokenKind::Dollar)),
            '\\' => Ok(self.make_token(TokenKind::Backslash)),

            // Multi-character tokens
            '-' => {
                if self.matches('>') {
                    Ok(self.make_token(TokenKind::Arrow))
                } else {
                    Ok(self.make_token(TokenKind::Minus))
                }
            }
            '/' => Ok(self.make_token(TokenKind::Slash)),
            '|' => {
                if self.matches('|') {
                    Ok(self.make_token(TokenKind::DoublePipe))
                } else {
                    Ok(self.make_token(TokenKind::Pipe))
                }
            }
            '&' => Ok(self.make_token(TokenKind::Ampersand)),
            ':' => {
                if self.matches(':') {
                    Ok(self.make_token(TokenKind::DoubleColon))
                } else {
                    Ok(self.make_token(TokenKind::Colon))
                }
            }
            '.' => {
                if self.matches('.') {
                    Ok(self.make_token(TokenKind::DoubleDot))
                } else if self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                    self.pos = self.start;
                    self.scan_number()
                } else {
                    Ok(self.make_token(TokenKind::Dot))
                }
            }
            '=' => {
                if self.matches('>') {
                    Ok(self.make_token(TokenKind::FatArrow))
                } else {
                    Ok(self.make_token(TokenKind::Eq))
                }
            }
            '!' => {
                if self.matches('=') {
                    Ok(self.make_token(TokenKind::NotEq))
                } else {
                    Err(Error::unexpected_char('!', self.start))
                }
            }
            '<' => {
                if self.matches('=') {
                    if self.matches('>') {
                        Ok(self.make_token(TokenKind::SafeEq))
                    } else {
                        Ok(self.make_token(TokenKind::LtEq))
                    }
                } else if self.matches('>') {
                    Ok(self.make_token(TokenKind::LtGt))
                } else if self.matches('<') {
                    Ok(self.make_token(TokenKind::LeftShift))
                } else {
                    Ok(self.make_token(TokenKind::Lt))
                }
            }
            '>' => {
                if self.matches('=') {
                    Ok(self.make_token(TokenKind::GtEq))
                } else if self.matches('>') {
                    Ok(self.make_token(TokenKind::RightShift))
                } else {
                    Ok(self.make_token(TokenKind::Gt))
                }
            }

            // String literals
            '\'' | '"' => self.scan_string(c),

            // Backtick quoted identifier
            '`' => self.scan_quoted_identifier(),

            // Numbers
            '0'..='9' => {
                self.pos = self.start;
                self.scan_number()
            }

            // Identifiers, keywords, and special prefixes
            'a'..='z' | 'A'..='Z' | '_' => {
                self.pos = self.start;
                self.scan_identifier_or_keyword()
            }

            _ => Err(Error::unexpected_char(c, self.start)),
        }
    }

    /// Skip whitespace and comments.
    fn skip_whitespace_and_comments(&mut self) -> Result<()> {
        loop {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            // Check for comments
            if self.check('/') && self.check_next('*') {
                self.skip_block_comment()?;
            } else if (self.check('-') && self.check_next('-')) || self.check('#') {
                self.skip_line_comment();
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Skip whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip a line comment (-- or #).
    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek_char() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Skip a block comment (/* ... */).
    fn skip_block_comment(&mut self) -> Result<()> {
        let start = self.pos;
        self.advance(); // consume '/'
        self.advance(); // consume '*'

        let mut depth = 1;
        while depth > 0 {
            if self.is_at_end() {
                return Err(Error::unterminated_comment(Span::new(start, self.pos)));
            }

            if self.check('*') && self.check_next('/') {
                self.advance();
                self.advance();
                depth -= 1;
            } else if self.check('/') && self.check_next('*') {
                self.advance();
                self.advance();
                depth += 1;
            } else {
                self.advance();
            }
        }
        Ok(())
    }

    /// Scan an identifier or keyword.
    fn scan_identifier_or_keyword(&mut self) -> Result<Token> {
        // Check for string prefixes (r, b, rb, br, R, B, etc.)
        let first = self.advance();
        let first_lower = first.to_ascii_lowercase();

        // Check for raw/bytes string prefixes
        if first_lower == 'r' || first_lower == 'b' {
            if let Some(second) = self.peek_char() {
                let second_lower = second.to_ascii_lowercase();

                // r"..." or r'...' (raw string)
                if first_lower == 'r' && (second == '"' || second == '\'') {
                    self.advance();
                    return self.scan_raw_string(second);
                }

                // b"..." or b'...' (bytes)
                if first_lower == 'b' && (second == '"' || second == '\'') {
                    self.advance();
                    return self.scan_bytes_string(second, false);
                }

                // rb"..." or br"..." (raw bytes)
                if (first_lower == 'r' && second_lower == 'b')
                    || (first_lower == 'b' && second_lower == 'r')
                {
                    self.advance();
                    if let Some(quote) = self.peek_char() {
                        if quote == '"' || quote == '\'' {
                            self.advance();
                            return self.scan_bytes_string(quote, true);
                        }
                    }
                }
            }
        }

        // Regular identifier
        while let Some(c) = self.peek_char() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.input[self.start..self.pos];

        // Check if it's a keyword
        if let Some(keyword) = Keyword::parse(text) {
            // Special handling for TRUE, FALSE, NULL
            match keyword {
                Keyword::True => Ok(self.make_token(TokenKind::Boolean(true))),
                Keyword::False => Ok(self.make_token(TokenKind::Boolean(false))),
                Keyword::Null => Ok(self.make_token(TokenKind::Null)),
                _ => Ok(self.make_token(TokenKind::Keyword(keyword))),
            }
        } else {
            Ok(self.make_token(TokenKind::Identifier(text.to_string())))
        }
    }

    /// Scan a regular string literal.
    fn scan_string(&mut self, quote: char) -> Result<Token> {
        // Check for triple-quoted string
        let triple = self.matches(quote) && self.matches(quote);
        let mut value = String::new();

        loop {
            if self.is_at_end() {
                return Err(Error::unterminated_string(Span::new(self.start, self.pos)));
            }

            let c = self.advance();

            if c == quote {
                if triple {
                    if self.matches(quote) && self.matches(quote) {
                        break;
                    } else {
                        value.push(c);
                    }
                } else {
                    break;
                }
            } else if c == '\\' && !self.is_at_end() {
                let escaped = self.scan_escape_sequence()?;
                value.push_str(&escaped);
            } else if c == '\n' && !triple {
                return Err(Error::unterminated_string(Span::new(self.start, self.pos)));
            } else {
                value.push(c);
            }
        }

        Ok(self.make_token(TokenKind::String(value)))
    }

    /// Scan a raw string literal (r"..." or r'...').
    fn scan_raw_string(&mut self, quote: char) -> Result<Token> {
        let triple = self.matches(quote) && self.matches(quote);
        let mut value = String::new();

        loop {
            if self.is_at_end() {
                return Err(Error::unterminated_string(Span::new(self.start, self.pos)));
            }

            let c = self.advance();

            if c == quote {
                if triple {
                    if self.matches(quote) && self.matches(quote) {
                        break;
                    } else {
                        value.push(c);
                    }
                } else {
                    break;
                }
            } else {
                value.push(c);
            }
        }

        Ok(self.make_token(TokenKind::String(value)))
    }

    /// Scan a bytes literal (b"..." or b'...').
    fn scan_bytes_string(&mut self, quote: char, raw: bool) -> Result<Token> {
        let triple = self.matches(quote) && self.matches(quote);
        let mut bytes = Vec::new();

        loop {
            if self.is_at_end() {
                return Err(Error::unterminated_string(Span::new(self.start, self.pos)));
            }

            let c = self.advance();

            if c == quote {
                if triple {
                    if self.matches(quote) && self.matches(quote) {
                        break;
                    } else {
                        bytes.push(c as u8);
                    }
                } else {
                    break;
                }
            } else if c == '\\' && !raw && !self.is_at_end() {
                let escaped = self.scan_bytes_escape_sequence()?;
                bytes.extend(escaped);
            } else if c == '\n' && !triple {
                return Err(Error::unterminated_string(Span::new(self.start, self.pos)));
            } else {
                bytes.push(c as u8);
            }
        }

        Ok(self.make_token(TokenKind::Bytes(bytes)))
    }

    /// Scan an escape sequence in a string.
    fn scan_escape_sequence(&mut self) -> Result<String> {
        if self.is_at_end() {
            return Err(Error::invalid_escape(
                "\\",
                Span::new(self.pos - 1, self.pos),
            ));
        }

        let c = self.advance();
        match c {
            'a' => Ok("\x07".to_string()),
            'b' => Ok("\x08".to_string()),
            'f' => Ok("\x0C".to_string()),
            'n' => Ok("\n".to_string()),
            'r' => Ok("\r".to_string()),
            't' => Ok("\t".to_string()),
            'v' => Ok("\x0B".to_string()),
            '\\' => Ok("\\".to_string()),
            '?' => Ok("?".to_string()),
            '"' => Ok("\"".to_string()),
            '\'' => Ok("'".to_string()),
            '`' => Ok("`".to_string()),
            '0'..='7' => {
                // Octal escape: exactly 3 digits
                let mut value = u32::from(c as u8 - b'0');
                for _ in 0..2 {
                    if let Some(d) = self.peek_char() {
                        if ('0'..='7').contains(&d) {
                            self.advance();
                            value = value * 8 + u32::from(d as u8 - b'0');
                        } else {
                            return Err(Error::invalid_escape(
                                format!("\\{}", c),
                                Span::new(self.pos - 2, self.pos),
                            ));
                        }
                    } else {
                        return Err(Error::invalid_escape(
                            format!("\\{}", c),
                            Span::new(self.pos - 2, self.pos),
                        ));
                    }
                }
                if let Some(ch) = char::from_u32(value) {
                    Ok(ch.to_string())
                } else {
                    Err(Error::invalid_escape(
                        format!("\\{:o}", value),
                        Span::new(self.pos - 4, self.pos),
                    ))
                }
            }
            'x' | 'X' => {
                // Hex escape: exactly 2 hex digits
                let mut value = 0u32;
                for _ in 0..2 {
                    if let Some(d) = self.peek_char() {
                        if d.is_ascii_hexdigit() {
                            self.advance();
                            value = value * 16 + d.to_digit(16).unwrap();
                        } else {
                            return Err(Error::invalid_escape(
                                format!("\\{}", c),
                                Span::new(self.pos - 2, self.pos),
                            ));
                        }
                    } else {
                        return Err(Error::invalid_escape(
                            format!("\\{}", c),
                            Span::new(self.pos - 2, self.pos),
                        ));
                    }
                }
                if let Some(ch) = char::from_u32(value) {
                    Ok(ch.to_string())
                } else {
                    Err(Error::invalid_escape(
                        format!("\\x{:02x}", value),
                        Span::new(self.pos - 4, self.pos),
                    ))
                }
            }
            'u' => {
                // Unicode escape: exactly 4 hex digits
                let mut value = 0u32;
                for _ in 0..4 {
                    if let Some(d) = self.peek_char() {
                        if d.is_ascii_hexdigit() {
                            self.advance();
                            value = value * 16 + d.to_digit(16).unwrap();
                        } else {
                            return Err(Error::invalid_escape(
                                "\\u",
                                Span::new(self.pos - 2, self.pos),
                            ));
                        }
                    } else {
                        return Err(Error::invalid_escape(
                            "\\u",
                            Span::new(self.pos - 2, self.pos),
                        ));
                    }
                }
                // Check for surrogate pairs (D800-DFFF)
                if (0xD800..=0xDFFF).contains(&value) {
                    return Err(Error::invalid_escape(
                        format!("\\u{:04x}", value),
                        Span::new(self.pos - 6, self.pos),
                    ));
                }
                if let Some(ch) = char::from_u32(value) {
                    Ok(ch.to_string())
                } else {
                    Err(Error::invalid_escape(
                        format!("\\u{:04x}", value),
                        Span::new(self.pos - 6, self.pos),
                    ))
                }
            }
            'U' => {
                // Unicode escape: exactly 8 hex digits
                let mut value = 0u32;
                for _ in 0..8 {
                    if let Some(d) = self.peek_char() {
                        if d.is_ascii_hexdigit() {
                            self.advance();
                            value = value * 16 + d.to_digit(16).unwrap();
                        } else {
                            return Err(Error::invalid_escape(
                                "\\U",
                                Span::new(self.pos - 2, self.pos),
                            ));
                        }
                    } else {
                        return Err(Error::invalid_escape(
                            "\\U",
                            Span::new(self.pos - 2, self.pos),
                        ));
                    }
                }
                // Check for surrogate pairs and values > 10FFFF
                if (0xD800..=0xDFFF).contains(&value) || value > 0x10FFFF {
                    return Err(Error::invalid_escape(
                        format!("\\U{:08x}", value),
                        Span::new(self.pos - 10, self.pos),
                    ));
                }
                if let Some(ch) = char::from_u32(value) {
                    Ok(ch.to_string())
                } else {
                    Err(Error::invalid_escape(
                        format!("\\U{:08x}", value),
                        Span::new(self.pos - 10, self.pos),
                    ))
                }
            }
            _ => Err(Error::invalid_escape(
                format!("\\{}", c),
                Span::new(self.pos - 2, self.pos),
            )),
        }
    }

    /// Scan an escape sequence in bytes literal.
    fn scan_bytes_escape_sequence(&mut self) -> Result<Vec<u8>> {
        if self.is_at_end() {
            return Err(Error::invalid_escape(
                "\\",
                Span::new(self.pos - 1, self.pos),
            ));
        }

        let c = self.advance();
        match c {
            'a' => Ok(vec![0x07]),
            'b' => Ok(vec![0x08]),
            'f' => Ok(vec![0x0C]),
            'n' => Ok(vec![0x0A]),
            'r' => Ok(vec![0x0D]),
            't' => Ok(vec![0x09]),
            'v' => Ok(vec![0x0B]),
            '\\' => Ok(vec![b'\\']),
            '?' => Ok(vec![b'?']),
            '"' => Ok(vec![b'"']),
            '\'' => Ok(vec![b'\'']),
            '`' => Ok(vec![b'`']),
            '0'..='7' => {
                let mut value = c as u8 - b'0';
                for _ in 0..2 {
                    if let Some(d) = self.peek_char() {
                        if ('0'..='7').contains(&d) {
                            self.advance();
                            value = value * 8 + (d as u8 - b'0');
                        } else {
                            return Err(Error::invalid_escape(
                                format!("\\{}", c),
                                Span::new(self.pos - 2, self.pos),
                            ));
                        }
                    } else {
                        return Err(Error::invalid_escape(
                            format!("\\{}", c),
                            Span::new(self.pos - 2, self.pos),
                        ));
                    }
                }
                Ok(vec![value])
            }
            'x' | 'X' => {
                let mut value = 0u8;
                for _ in 0..2 {
                    if let Some(d) = self.peek_char() {
                        if d.is_ascii_hexdigit() {
                            self.advance();
                            value = value * 16 + d.to_digit(16).unwrap() as u8;
                        } else {
                            return Err(Error::invalid_escape(
                                format!("\\{}", c),
                                Span::new(self.pos - 2, self.pos),
                            ));
                        }
                    } else {
                        return Err(Error::invalid_escape(
                            format!("\\{}", c),
                            Span::new(self.pos - 2, self.pos),
                        ));
                    }
                }
                Ok(vec![value])
            }
            _ => Err(Error::invalid_escape(
                format!("\\{}", c),
                Span::new(self.pos - 2, self.pos),
            )),
        }
    }

    /// Scan a quoted identifier (`identifier`).
    fn scan_quoted_identifier(&mut self) -> Result<Token> {
        let mut value = String::new();

        loop {
            if self.is_at_end() {
                return Err(Error::unterminated_string(Span::new(self.start, self.pos)));
            }

            let c = self.advance();

            if c == '`' {
                break;
            } else if c == '\\' && !self.is_at_end() {
                let escaped = self.scan_escape_sequence()?;
                value.push_str(&escaped);
            } else {
                value.push(c);
            }
        }

        if value.is_empty() {
            return Err(Error::invalid_syntax(
                "empty quoted identifier",
                Span::new(self.start, self.pos),
            ));
        }

        Ok(self.make_token(TokenKind::QuotedIdentifier(value)))
    }

    /// Scan a number literal.
    fn scan_number(&mut self) -> Result<Token> {
        // Check for hex literal
        if self.check('0') && self.check_next_char(|c| c == 'x' || c == 'X') {
            self.advance(); // '0'
            self.advance(); // 'x'
            return self.scan_hex_number();
        }

        // Scan integer part
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point or exponent
        let has_decimal = self.check('.') && self.check_next_char(|c| c.is_ascii_digit());
        let has_exponent = self.peek_char().is_some_and(|c| c == 'e' || c == 'E');

        if has_decimal || has_exponent {
            // Floating point number
            if has_decimal {
                self.advance(); // consume '.'
                while let Some(c) = self.peek_char() {
                    if c.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }

            // Exponent part
            if let Some(c) = self.peek_char() {
                if c == 'e' || c == 'E' {
                    self.advance();
                    if let Some(sign) = self.peek_char() {
                        if sign == '+' || sign == '-' {
                            self.advance();
                        }
                    }
                    while let Some(c) = self.peek_char() {
                        if c.is_ascii_digit() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }

            let text = &self.input[self.start..self.pos];
            match text.parse::<f64>() {
                Ok(value) => Ok(self.make_token(TokenKind::Float(value))),
                Err(_) => Err(Error::invalid_number(text, Span::new(self.start, self.pos))),
            }
        } else {
            // Integer
            let text = &self.input[self.start..self.pos];
            match text.parse::<i64>() {
                Ok(value) => Ok(self.make_token(TokenKind::Integer(value))),
                Err(_) => Err(Error::invalid_number(text, Span::new(self.start, self.pos))),
            }
        }
    }

    /// Scan a hexadecimal number.
    fn scan_hex_number(&mut self) -> Result<Token> {
        let hex_start = self.pos;

        while let Some(c) = self.peek_char() {
            if c.is_ascii_hexdigit() {
                self.advance();
            } else {
                break;
            }
        }

        if self.pos == hex_start {
            return Err(Error::with_span(
                crate::error::ErrorKind::InvalidHexLiteral,
                Span::new(self.start, self.pos),
            ));
        }

        let hex_str = &self.input[hex_start..self.pos];
        match i64::from_str_radix(hex_str, 16) {
            Ok(value) => Ok(self.make_token(TokenKind::Integer(value))),
            Err(_) => Err(Error::invalid_number(
                &self.input[self.start..self.pos],
                Span::new(self.start, self.pos),
            )),
        }
    }

    // Helper methods

    fn is_at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> char {
        let c = self.input[self.pos..].chars().next().unwrap();
        self.pos += c.len_utf8();
        c
    }

    fn check(&self, expected: char) -> bool {
        self.peek_char() == Some(expected)
    }

    fn check_next(&self, expected: char) -> bool {
        self.input[self.pos..].chars().nth(1) == Some(expected)
    }

    fn check_next_char<F: Fn(char) -> bool>(&self, f: F) -> bool {
        self.input[self.pos..].chars().nth(1).is_some_and(f)
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        Token::new(
            kind,
            Span::new(self.start, self.pos),
            &self.input[self.start..self.pos],
        )
    }
}

/// Iterator implementation for the lexer.
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token_result() {
            Ok(token) if token.kind == TokenKind::Eof => None,
            result => Some(result),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token_result() {
                Ok(token) => {
                    if token.kind == TokenKind::Eof {
                        break;
                    }
                    tokens.push(token.kind);
                }
                Err(e) => panic!("Lexer error: {}", e),
            }
        }
        tokens
    }

    #[test]
    fn test_simple_select() {
        let tokens = tokenize("SELECT * FROM table1");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Keyword(Keyword::Select),
                TokenKind::Star,
                TokenKind::Keyword(Keyword::From),
                TokenKind::Identifier("table1".to_string()),
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let tokens = tokenize("123 45.67 0x1A 1e10 .5");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Integer(123),
                TokenKind::Float(45.67),
                TokenKind::Integer(0x1A),
                TokenKind::Float(1e10),
                TokenKind::Float(0.5),
            ]
        );
    }

    #[test]
    fn test_strings() {
        let tokens = tokenize(
            r#"'hello' "world" '''multi
line'''"#,
        );
        assert_eq!(
            tokens,
            vec![
                TokenKind::String("hello".to_string()),
                TokenKind::String("world".to_string()),
                TokenKind::String("multi\nline".to_string()),
            ]
        );
    }

    #[test]
    fn test_operators() {
        let tokens = tokenize("+ - * / = != < > <= >= <> || << >>");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Eq,
                TokenKind::NotEq,
                TokenKind::Lt,
                TokenKind::Gt,
                TokenKind::LtEq,
                TokenKind::GtEq,
                TokenKind::LtGt,
                TokenKind::DoublePipe,
                TokenKind::LeftShift,
                TokenKind::RightShift,
            ]
        );
    }

    #[test]
    fn test_comments() {
        let tokens = tokenize("SELECT -- comment\n* /* block */ FROM");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Keyword(Keyword::Select),
                TokenKind::Star,
                TokenKind::Keyword(Keyword::From),
            ]
        );
    }

    #[test]
    fn test_quoted_identifier() {
        let tokens = tokenize("`my table` `column-name`");
        assert_eq!(
            tokens,
            vec![
                TokenKind::QuotedIdentifier("my table".to_string()),
                TokenKind::QuotedIdentifier("column-name".to_string()),
            ]
        );
    }

    #[test]
    fn test_escape_sequences() {
        let tokens = tokenize(r"'hello\nworld' '\x41\x42'");
        assert_eq!(
            tokens,
            vec![
                TokenKind::String("hello\nworld".to_string()),
                TokenKind::String("AB".to_string()),
            ]
        );
    }

    #[test]
    fn test_raw_string() {
        let tokens = tokenize(r#"r'\n\t'"#);
        assert_eq!(tokens, vec![TokenKind::String(r"\n\t".to_string())]);
    }

    #[test]
    fn test_bytes_literal() {
        let tokens = tokenize(r#"b'ABC' b'\x41'"#);
        assert_eq!(
            tokens,
            vec![
                TokenKind::Bytes(vec![b'A', b'B', b'C']),
                TokenKind::Bytes(vec![0x41]),
            ]
        );
    }

    #[test]
    fn test_boolean_and_null() {
        let tokens = tokenize("TRUE FALSE NULL true false null");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Boolean(true),
                TokenKind::Boolean(false),
                TokenKind::Null,
                TokenKind::Boolean(true),
                TokenKind::Boolean(false),
                TokenKind::Null,
            ]
        );
    }

    #[test]
    fn test_complex_query() {
        let tokens = tokenize(
            "SELECT a.id, b.name FROM table_a a JOIN table_b b ON a.id = b.id WHERE a.value > 10",
        );
        assert!(tokens.len() > 10);
        assert!(matches!(tokens[0], TokenKind::Keyword(Keyword::Select)));
        assert!(matches!(tokens[tokens.len() - 1], TokenKind::Integer(10)));
    }
}
