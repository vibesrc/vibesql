//! Analyzer-specific error types.

use crate::error::Span;
use crate::types::SqlType;
use std::fmt;

/// Analyzer error kinds.
#[derive(Debug, Clone)]
pub enum AnalyzerErrorKind {
    /// Table not found in catalog.
    TableNotFound { name: String },
    /// Column not found.
    ColumnNotFound { name: String, table: Option<String> },
    /// Ambiguous column reference.
    AmbiguousColumn { name: String, tables: Vec<String> },
    /// Function not found.
    FunctionNotFound { name: String },
    /// Wrong number of arguments to function.
    WrongArgumentCount {
        function: String,
        expected_min: usize,
        expected_max: Option<usize>,
        actual: usize,
    },
    /// Type mismatch in expression.
    TypeMismatch {
        expected: SqlType,
        actual: SqlType,
        context: String,
    },
    /// Types are not comparable.
    TypesNotComparable { left: SqlType, right: SqlType },
    /// Invalid use of aggregate function.
    InvalidAggregateUse { function: String, reason: String },
    /// Invalid use of window function.
    InvalidWindowUse { function: String, reason: String },
    /// Duplicate alias.
    DuplicateAlias { name: String },
    /// Duplicate column in GROUP BY.
    DuplicateGroupByColumn { name: String },
    /// Non-aggregated column in SELECT with GROUP BY.
    NonAggregatedColumn { column: String },
    /// ORDER BY column not in SELECT (when DISTINCT is used).
    OrderByNotInSelect { column: String },
    /// Invalid HAVING clause (no GROUP BY).
    HavingWithoutGroupBy,
    /// Invalid subquery.
    InvalidSubquery { reason: String },
    /// Division by zero (constant folding).
    DivisionByZero,
    /// Invalid CAST.
    InvalidCast { from: SqlType, to: SqlType },
    /// Invalid date/time literal.
    InvalidDateTimeLiteral {
        value: String,
        expected_type: String,
    },
    /// CTE name conflict.
    DuplicateCte { name: String },
    /// Recursive CTE without UNION ALL.
    InvalidRecursiveCte { reason: String },
    /// Star (*) not allowed in this context.
    StarNotAllowed { context: String },
    /// EXCEPT/INTERSECT column count mismatch.
    SetOperationColumnMismatch { left: usize, right: usize },
    /// Unknown error.
    Other { message: String },
}

/// An analyzer error with location information.
#[derive(Debug, Clone)]
pub struct AnalyzerError {
    /// The kind of error.
    pub kind: AnalyzerErrorKind,
    /// The source span where the error occurred.
    pub span: Option<Span>,
}

impl AnalyzerError {
    /// Create a new analyzer error.
    pub fn new(kind: AnalyzerErrorKind) -> Self {
        Self { kind, span: None }
    }

    /// Create a new analyzer error with a span.
    pub fn with_span(kind: AnalyzerErrorKind, span: Span) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }

    /// Table not found.
    pub fn table_not_found(name: impl Into<String>) -> Self {
        Self::new(AnalyzerErrorKind::TableNotFound { name: name.into() })
    }

    /// Column not found.
    pub fn column_not_found(name: impl Into<String>, table: Option<String>) -> Self {
        Self::new(AnalyzerErrorKind::ColumnNotFound {
            name: name.into(),
            table,
        })
    }

    /// Ambiguous column.
    pub fn ambiguous_column(name: impl Into<String>, tables: Vec<String>) -> Self {
        Self::new(AnalyzerErrorKind::AmbiguousColumn {
            name: name.into(),
            tables,
        })
    }

    /// Function not found.
    pub fn function_not_found(name: impl Into<String>) -> Self {
        Self::new(AnalyzerErrorKind::FunctionNotFound { name: name.into() })
    }

    /// Wrong argument count.
    pub fn wrong_argument_count(
        function: impl Into<String>,
        expected_min: usize,
        expected_max: Option<usize>,
        actual: usize,
    ) -> Self {
        Self::new(AnalyzerErrorKind::WrongArgumentCount {
            function: function.into(),
            expected_min,
            expected_max,
            actual,
        })
    }

    /// Type mismatch.
    pub fn type_mismatch(expected: SqlType, actual: SqlType, context: impl Into<String>) -> Self {
        Self::new(AnalyzerErrorKind::TypeMismatch {
            expected,
            actual,
            context: context.into(),
        })
    }

    /// Types not comparable.
    pub fn types_not_comparable(left: SqlType, right: SqlType) -> Self {
        Self::new(AnalyzerErrorKind::TypesNotComparable { left, right })
    }

    /// Non-aggregated column.
    pub fn non_aggregated_column(column: impl Into<String>) -> Self {
        Self::new(AnalyzerErrorKind::NonAggregatedColumn {
            column: column.into(),
        })
    }

    /// Invalid aggregate use.
    pub fn invalid_aggregate_use(function: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::new(AnalyzerErrorKind::InvalidAggregateUse {
            function: function.into(),
            reason: reason.into(),
        })
    }

    /// Set operation column mismatch.
    pub fn set_operation_column_mismatch(left: usize, right: usize) -> Self {
        Self::new(AnalyzerErrorKind::SetOperationColumnMismatch { left, right })
    }
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            AnalyzerErrorKind::TableNotFound { name } => {
                write!(f, "table '{}' not found", name)
            }
            AnalyzerErrorKind::ColumnNotFound { name, table } => {
                if let Some(t) = table {
                    write!(f, "column '{}' not found in table '{}'", name, t)
                } else {
                    write!(f, "column '{}' not found", name)
                }
            }
            AnalyzerErrorKind::AmbiguousColumn { name, tables } => {
                write!(
                    f,
                    "ambiguous column '{}' found in tables: {}",
                    name,
                    tables.join(", ")
                )
            }
            AnalyzerErrorKind::FunctionNotFound { name } => {
                write!(f, "function '{}' not found", name)
            }
            AnalyzerErrorKind::WrongArgumentCount {
                function,
                expected_min,
                expected_max,
                actual,
            } => {
                if let Some(max) = expected_max {
                    if expected_min == max {
                        write!(
                            f,
                            "function '{}' expects {} arguments, got {}",
                            function, expected_min, actual
                        )
                    } else {
                        write!(
                            f,
                            "function '{}' expects {}-{} arguments, got {}",
                            function, expected_min, max, actual
                        )
                    }
                } else {
                    write!(
                        f,
                        "function '{}' expects at least {} arguments, got {}",
                        function, expected_min, actual
                    )
                }
            }
            AnalyzerErrorKind::TypeMismatch {
                expected,
                actual,
                context,
            } => {
                write!(
                    f,
                    "type mismatch in {}: expected {}, got {}",
                    context, expected, actual
                )
            }
            AnalyzerErrorKind::TypesNotComparable { left, right } => {
                write!(f, "cannot compare {} with {}", left, right)
            }
            AnalyzerErrorKind::InvalidAggregateUse { function, reason } => {
                write!(
                    f,
                    "invalid use of aggregate function '{}': {}",
                    function, reason
                )
            }
            AnalyzerErrorKind::InvalidWindowUse { function, reason } => {
                write!(
                    f,
                    "invalid use of window function '{}': {}",
                    function, reason
                )
            }
            AnalyzerErrorKind::DuplicateAlias { name } => {
                write!(f, "duplicate alias '{}'", name)
            }
            AnalyzerErrorKind::DuplicateGroupByColumn { name } => {
                write!(f, "duplicate column '{}' in GROUP BY", name)
            }
            AnalyzerErrorKind::NonAggregatedColumn { column } => {
                write!(
                    f,
                    "column '{}' must appear in GROUP BY clause or be used in an aggregate function",
                    column
                )
            }
            AnalyzerErrorKind::OrderByNotInSelect { column } => {
                write!(
                    f,
                    "ORDER BY column '{}' must appear in SELECT list when DISTINCT is used",
                    column
                )
            }
            AnalyzerErrorKind::HavingWithoutGroupBy => {
                write!(f, "HAVING clause requires GROUP BY clause")
            }
            AnalyzerErrorKind::InvalidSubquery { reason } => {
                write!(f, "invalid subquery: {}", reason)
            }
            AnalyzerErrorKind::DivisionByZero => {
                write!(f, "division by zero")
            }
            AnalyzerErrorKind::InvalidCast { from, to } => {
                write!(f, "cannot cast {} to {}", from, to)
            }
            AnalyzerErrorKind::InvalidDateTimeLiteral {
                value,
                expected_type,
            } => {
                write!(f, "invalid {} literal: '{}'", expected_type, value)
            }
            AnalyzerErrorKind::DuplicateCte { name } => {
                write!(f, "duplicate CTE name '{}'", name)
            }
            AnalyzerErrorKind::InvalidRecursiveCte { reason } => {
                write!(f, "invalid recursive CTE: {}", reason)
            }
            AnalyzerErrorKind::StarNotAllowed { context } => {
                write!(f, "* not allowed in {}", context)
            }
            AnalyzerErrorKind::SetOperationColumnMismatch { left, right } => {
                write!(
                    f,
                    "set operations require the same number of columns ({} vs {})",
                    left, right
                )
            }
            AnalyzerErrorKind::Other { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl std::error::Error for AnalyzerError {}
