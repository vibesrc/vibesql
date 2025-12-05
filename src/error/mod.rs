//! Error handling for the VibeSQL parser and analyzer.
//!
//! This module provides error types and utilities for representing and
//! displaying parsing and analysis errors with source location information.

use std::fmt;

/// A span in the source code, represented as byte offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    /// Start byte offset (inclusive)
    pub start: usize,
    /// End byte offset (exclusive)
    pub end: usize,
}

impl Span {
    /// Create a new span from start and end offsets.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Create a span for a single position.
    pub fn point(pos: usize) -> Self {
        Self {
            start: pos,
            end: pos + 1,
        }
    }

    /// Create an empty span at a position.
    pub fn empty(pos: usize) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    /// Merge two spans into one that covers both.
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Get the length of this span in bytes.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Check if this span is empty.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

/// The kind of error that occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    // Lexer errors
    UnexpectedCharacter(char),
    UnterminatedString,
    UnterminatedBlockComment,
    InvalidEscapeSequence(String),
    InvalidNumber(String),
    InvalidHexLiteral,
    InvalidBytesLiteral,

    // Parser errors
    UnexpectedToken {
        expected: String,
        found: String,
    },
    UnexpectedEof,
    ExpectedExpression,
    ExpectedIdentifier,
    ExpectedKeyword(String),
    InvalidSyntax(String),
    UnsupportedFeature(String),

    // Analyzer errors
    UndefinedColumn(String),
    UndefinedTable(String),
    UndefinedFunction(String),
    AmbiguousColumn(String),
    TypeMismatch {
        expected: String,
        found: String,
    },
    InvalidArgumentCount {
        function: String,
        expected: usize,
        found: usize,
    },
    DuplicateColumn(String),
    DuplicateAlias(String),
    InvalidGroupBy(String),
    InvalidOrderBy(String),
    InvalidAggregateUsage(String),
    InvalidWindowFunction(String),

    // General errors
    Internal(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Lexer errors
            ErrorKind::UnexpectedCharacter(c) => write!(f, "unexpected character '{}'", c),
            ErrorKind::UnterminatedString => write!(f, "unterminated string literal"),
            ErrorKind::UnterminatedBlockComment => write!(f, "unterminated block comment"),
            ErrorKind::InvalidEscapeSequence(s) => write!(f, "invalid escape sequence '{}'", s),
            ErrorKind::InvalidNumber(s) => write!(f, "invalid number '{}'", s),
            ErrorKind::InvalidHexLiteral => write!(f, "invalid hexadecimal literal"),
            ErrorKind::InvalidBytesLiteral => write!(f, "invalid bytes literal"),

            // Parser errors
            ErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "expected {}, found {}", expected, found)
            }
            ErrorKind::UnexpectedEof => write!(f, "unexpected end of input"),
            ErrorKind::ExpectedExpression => write!(f, "expected expression"),
            ErrorKind::ExpectedIdentifier => write!(f, "expected identifier"),
            ErrorKind::ExpectedKeyword(kw) => write!(f, "expected keyword '{}'", kw),
            ErrorKind::InvalidSyntax(msg) => write!(f, "invalid syntax: {}", msg),
            ErrorKind::UnsupportedFeature(feat) => write!(f, "unsupported feature: {}", feat),

            // Analyzer errors
            ErrorKind::UndefinedColumn(name) => write!(f, "undefined column '{}'", name),
            ErrorKind::UndefinedTable(name) => write!(f, "undefined table '{}'", name),
            ErrorKind::UndefinedFunction(name) => write!(f, "undefined function '{}'", name),
            ErrorKind::AmbiguousColumn(name) => write!(f, "ambiguous column reference '{}'", name),
            ErrorKind::TypeMismatch { expected, found } => {
                write!(f, "type mismatch: expected {}, found {}", expected, found)
            }
            ErrorKind::InvalidArgumentCount {
                function,
                expected,
                found,
            } => {
                write!(
                    f,
                    "function '{}' expects {} arguments, found {}",
                    function, expected, found
                )
            }
            ErrorKind::DuplicateColumn(name) => write!(f, "duplicate column '{}'", name),
            ErrorKind::DuplicateAlias(name) => write!(f, "duplicate alias '{}'", name),
            ErrorKind::InvalidGroupBy(msg) => write!(f, "invalid GROUP BY: {}", msg),
            ErrorKind::InvalidOrderBy(msg) => write!(f, "invalid ORDER BY: {}", msg),
            ErrorKind::InvalidAggregateUsage(msg) => write!(f, "invalid aggregate usage: {}", msg),
            ErrorKind::InvalidWindowFunction(msg) => write!(f, "invalid window function: {}", msg),

            // General errors
            ErrorKind::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

/// An error with source location information.
#[derive(Debug, Clone)]
pub struct Error {
    /// The kind of error.
    pub kind: ErrorKind,
    /// The source span where the error occurred.
    span: Option<Span>,
    /// Optional context message.
    context: Option<String>,
}

impl Error {
    /// Create a new error with the given kind.
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            span: None,
            context: None,
        }
    }

    /// Create a new error with source location.
    pub fn with_span(kind: ErrorKind, span: Span) -> Self {
        Self {
            kind,
            span: Some(span),
            context: None,
        }
    }

    /// Add context to this error.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Get the span of this error, if any.
    pub fn span(&self) -> Option<Span> {
        self.span
    }

    /// Get the error kind.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    // Convenience constructors for common errors
    pub fn unexpected_char(c: char, pos: usize) -> Self {
        Self::with_span(ErrorKind::UnexpectedCharacter(c), Span::point(pos))
    }

    pub fn unexpected_token(
        expected: impl Into<String>,
        found: impl Into<String>,
        span: Span,
    ) -> Self {
        Self::with_span(
            ErrorKind::UnexpectedToken {
                expected: expected.into(),
                found: found.into(),
            },
            span,
        )
    }

    pub fn unexpected_eof(pos: usize) -> Self {
        Self::with_span(ErrorKind::UnexpectedEof, Span::point(pos))
    }

    pub fn expected_expression(span: Span) -> Self {
        Self::with_span(ErrorKind::ExpectedExpression, span)
    }

    pub fn expected_identifier(span: Span) -> Self {
        Self::with_span(ErrorKind::ExpectedIdentifier, span)
    }

    pub fn expected_keyword(keyword: impl Into<String>, span: Span) -> Self {
        Self::with_span(ErrorKind::ExpectedKeyword(keyword.into()), span)
    }

    pub fn invalid_syntax(msg: impl Into<String>, span: Span) -> Self {
        Self::with_span(ErrorKind::InvalidSyntax(msg.into()), span)
    }

    pub fn unsupported(feature: impl Into<String>, span: Span) -> Self {
        Self::with_span(ErrorKind::UnsupportedFeature(feature.into()), span)
    }

    pub fn unterminated_string(span: Span) -> Self {
        Self::with_span(ErrorKind::UnterminatedString, span)
    }

    pub fn unterminated_comment(span: Span) -> Self {
        Self::with_span(ErrorKind::UnterminatedBlockComment, span)
    }

    pub fn invalid_escape(seq: impl Into<String>, span: Span) -> Self {
        Self::with_span(ErrorKind::InvalidEscapeSequence(seq.into()), span)
    }

    pub fn invalid_number(num: impl Into<String>, span: Span) -> Self {
        Self::with_span(ErrorKind::InvalidNumber(num.into()), span)
    }

    /// Create an analyzer error.
    pub fn analyzer(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::Internal(msg.into()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        if let Some(ref ctx) = self.context {
            write!(f, " ({})", ctx)?;
        }
        if let Some(span) = self.span {
            write!(f, " at position {}", span.start)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

/// A specialized Result type for VibeSQL operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_merge() {
        let span1 = Span::new(10, 20);
        let span2 = Span::new(15, 30);
        let merged = span1.merge(span2);
        assert_eq!(merged.start, 10);
        assert_eq!(merged.end, 30);
    }

    #[test]
    fn test_error_display() {
        let err = Error::unexpected_char('$', 5);
        let msg = format!("{}", err);
        assert!(msg.contains("unexpected character"));
        assert!(msg.contains("$"));
    }

    #[test]
    fn test_error_with_context() {
        let err = Error::new(ErrorKind::UnexpectedEof).with_context("parsing SELECT clause");
        let msg = format!("{}", err);
        assert!(msg.contains("parsing SELECT clause"));
    }
}
