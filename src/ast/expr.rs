//! Expression AST definitions.
//!
//! This module defines the expression types for the SQL AST.

use super::{DataTypeSpec, Ident, ObjectName, Query, WindowSpec};
use crate::error::Span;

/// An SQL expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Create a boxed expression.
    pub fn boxed(kind: ExprKind, span: Span) -> Box<Self> {
        Box::new(Self::new(kind, span))
    }
}

/// Expression kind.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    // Literals
    /// NULL literal
    Null,
    /// Boolean literal (TRUE/FALSE)
    Boolean(bool),
    /// Integer literal
    Integer(i64),
    /// Floating-point literal
    Float(f64),
    /// String literal
    String(String),
    /// Bytes literal
    Bytes(Vec<u8>),
    /// Array literal: [1, 2, 3] or ARRAY[1, 2, 3]
    Array {
        element_type: Option<Box<DataTypeSpec>>,
        elements: Vec<Box<Expr>>,
    },
    /// Struct literal: STRUCT(1, 'a') or (1, 'a')
    Struct { fields: Vec<StructField> },

    // Identifiers and references
    /// Simple identifier (column name)
    Identifier(Ident),
    /// Compound identifier (table.column or schema.table.column)
    CompoundIdentifier(Vec<Ident>),
    /// Parameter reference (@param or ?)
    Parameter(Parameter),

    // Operators
    /// Unary operator
    UnaryOp { op: UnaryOp, expr: Box<Expr> },
    /// Binary operator
    BinaryOp {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // Comparisons
    /// BETWEEN expression
    Between {
        expr: Box<Expr>,
        low: Box<Expr>,
        high: Box<Expr>,
        negated: bool,
    },
    /// IN expression (expr IN (values) or expr IN (subquery))
    In {
        expr: Box<Expr>,
        list: InList,
        negated: bool,
    },
    /// LIKE expression
    Like {
        expr: Box<Expr>,
        pattern: Box<Expr>,
        escape: Option<Box<Expr>>,
        negated: bool,
    },
    /// IS NULL / IS NOT NULL / IS TRUE / IS FALSE / IS UNKNOWN
    IsExpr {
        expr: Box<Expr>,
        test: IsTest,
        negated: bool,
    },
    /// IS DISTINCT FROM
    IsDistinct {
        left: Box<Expr>,
        right: Box<Expr>,
        negated: bool,
    },

    // Functions
    /// Function call
    Function(FunctionCall),
    /// Aggregate function call
    Aggregate(AggregateCall),
    /// Window function call
    WindowFunction(WindowFunctionCall),

    // Type operations
    /// CAST(expr AS type)
    Cast {
        expr: Box<Expr>,
        data_type: DataTypeSpec,
        safe: bool, // SAFE_CAST
    },
    /// EXTRACT(part FROM expr)
    Extract {
        field: DateTimePart,
        from: Box<Expr>,
    },

    // Conditional expressions
    /// CASE expression
    Case {
        operand: Option<Box<Expr>>,
        conditions: Vec<(Box<Expr>, Box<Expr>)>,
        else_result: Option<Box<Expr>>,
    },
    /// IF(condition, then_expr, else_expr)
    If {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    /// COALESCE(expr1, expr2, ...)
    Coalesce(Vec<Box<Expr>>),
    /// NULLIF(expr1, expr2)
    Nullif { left: Box<Expr>, right: Box<Expr> },
    /// IFNULL(expr1, expr2)
    IfNull {
        expr: Box<Expr>,
        null_replacement: Box<Expr>,
    },

    // Subqueries
    /// Scalar subquery
    Subquery(Box<Query>),
    /// EXISTS subquery
    Exists { subquery: Box<Query>, negated: bool },
    /// ANY/SOME/ALL subquery
    SubqueryOp {
        left: Box<Expr>,
        op: BinaryOp,
        modifier: SubqueryModifier,
        subquery: Box<Query>,
    },
    /// IN subquery
    InSubquery {
        expr: Box<Expr>,
        subquery: Box<Query>,
        negated: bool,
    },

    // Array operations
    /// Array subscript: `array[index]`
    ArraySubscript {
        array: Box<Expr>,
        index: ArraySubscriptKind,
    },
    /// Array element access with SAFE: `array[SAFE_OFFSET(index)]`
    SafeArraySubscript {
        array: Box<Expr>,
        index: Box<Expr>,
        offset_type: ArrayOffsetType,
    },

    // Struct/JSON operations
    /// Field access: expr.field
    FieldAccess { expr: Box<Expr>, field: Ident },
    /// JSON subscript: `json['field']` or `json[index]`
    JsonSubscript { expr: Box<Expr>, key: JsonKey },

    // Special expressions
    /// Interval literal: INTERVAL expr UNIT
    Interval {
        value: Box<Expr>,
        unit: IntervalUnit,
    },
    /// Date/Time/Timestamp/Datetime literal with type prefix
    TypedLiteral {
        data_type: TypedLiteralType,
        value: String,
    },
    /// Parenthesized expression
    Parenthesized(Box<Expr>),
    /// Row constructor: ROW(expr1, expr2, ...)
    Row(Vec<Box<Expr>>),
}

/// Struct field in struct literal.
#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: Option<Ident>,
    pub value: Box<Expr>,
}

/// Query parameter.
#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    Named(String),
    Positional(u32),
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    BitwiseNot,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Plus => write!(f, "+"),
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::Not => write!(f, "NOT"),
            UnaryOp::BitwiseNot => write!(f, "~"),
        }
    }
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,

    // Logical
    And,
    Or,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,

    // String/Array
    Concat,
}

impl BinaryOp {
    /// Get the precedence of this operator (higher = binds tighter).
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::BitwiseOr => 3,
            BinaryOp::BitwiseXor => 4,
            BinaryOp::BitwiseAnd => 5,
            BinaryOp::LeftShift | BinaryOp::RightShift => 6,
            BinaryOp::Plus | BinaryOp::Minus => 7,
            BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo | BinaryOp::Concat => 8,
            BinaryOp::Eq
            | BinaryOp::NotEq
            | BinaryOp::Lt
            | BinaryOp::LtEq
            | BinaryOp::Gt
            | BinaryOp::GtEq => 9,
        }
    }

    /// Check if this operator is left-associative.
    pub fn is_left_associative(&self) -> bool {
        true // All binary operators in SQL are left-associative
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Plus => write!(f, "+"),
            BinaryOp::Minus => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::Eq => write!(f, "="),
            BinaryOp::NotEq => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::LtEq => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::GtEq => write!(f, ">="),
            BinaryOp::And => write!(f, "AND"),
            BinaryOp::Or => write!(f, "OR"),
            BinaryOp::BitwiseAnd => write!(f, "&"),
            BinaryOp::BitwiseOr => write!(f, "|"),
            BinaryOp::BitwiseXor => write!(f, "^"),
            BinaryOp::LeftShift => write!(f, "<<"),
            BinaryOp::RightShift => write!(f, ">>"),
            BinaryOp::Concat => write!(f, "||"),
        }
    }
}

/// IN list variants.
#[derive(Debug, Clone, PartialEq)]
pub enum InList {
    Values(Vec<Box<Expr>>),
    Subquery(Box<Query>),
}

/// IS test type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsTest {
    Null,
    True,
    False,
    Unknown,
}

/// Subquery modifier for ANY/SOME/ALL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubqueryModifier {
    Any,
    Some,
    All,
}

/// Array subscript kind.
#[derive(Debug, Clone, PartialEq)]
pub enum ArraySubscriptKind {
    /// Simple index: `array[0]`
    Index(Box<Expr>),
    /// OFFSET: `array[OFFSET(0)]`
    Offset(Box<Expr>),
    /// ORDINAL: `array[ORDINAL(1)]`
    Ordinal(Box<Expr>),
    /// SAFE_OFFSET: `array[SAFE_OFFSET(0)]`
    SafeOffset(Box<Expr>),
    /// SAFE_ORDINAL: `array[SAFE_ORDINAL(1)]`
    SafeOrdinal(Box<Expr>),
}

/// Array offset type for safe access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayOffsetType {
    Offset,
    Ordinal,
}

/// JSON key type.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonKey {
    String(String),
    Index(Box<Expr>),
}

/// Function call.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: ObjectName,
    pub args: Vec<FunctionArg>,
    pub distinct: bool,
    pub null_treatment: Option<NullTreatment>,
    pub order_by: Vec<super::OrderByExpr>,
    pub limit: Option<Box<Expr>>,
}

/// Function argument.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionArg {
    /// Unnamed argument
    Unnamed(Box<Expr>),
    /// Named argument: name => value
    Named { name: Ident, value: Box<Expr> },
    /// Star argument: COUNT(*)
    Star,
}

/// Null treatment in window/aggregate functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullTreatment {
    RespectNulls,
    IgnoreNulls,
}

/// Aggregate function call.
#[derive(Debug, Clone, PartialEq)]
pub struct AggregateCall {
    pub function: FunctionCall,
    pub filter: Option<Box<Expr>>,
}

/// Window function call.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowFunctionCall {
    pub function: FunctionCall,
    pub window: WindowSpecOrRef,
}

/// Window specification or reference.
#[derive(Debug, Clone, PartialEq)]
pub enum WindowSpecOrRef {
    Spec(WindowSpec),
    Ref(Ident),
}

/// Date/time part for EXTRACT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimePart {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Dayofweek,
    Dayofyear,
    Week,
    Quarter,
    Date,
    Time,
    Datetime,
    Isoweek,
    Isoyear,
}

impl DateTimePart {
    /// Parse a datetime part from a string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "YEAR" => Some(DateTimePart::Year),
            "MONTH" => Some(DateTimePart::Month),
            "DAY" => Some(DateTimePart::Day),
            "HOUR" => Some(DateTimePart::Hour),
            "MINUTE" => Some(DateTimePart::Minute),
            "SECOND" => Some(DateTimePart::Second),
            "MILLISECOND" => Some(DateTimePart::Millisecond),
            "MICROSECOND" => Some(DateTimePart::Microsecond),
            "NANOSECOND" => Some(DateTimePart::Nanosecond),
            "DAYOFWEEK" => Some(DateTimePart::Dayofweek),
            "DAYOFYEAR" => Some(DateTimePart::Dayofyear),
            "WEEK" => Some(DateTimePart::Week),
            "QUARTER" => Some(DateTimePart::Quarter),
            "DATE" => Some(DateTimePart::Date),
            "TIME" => Some(DateTimePart::Time),
            "DATETIME" => Some(DateTimePart::Datetime),
            "ISOWEEK" => Some(DateTimePart::Isoweek),
            "ISOYEAR" => Some(DateTimePart::Isoyear),
            _ => None,
        }
    }
}

/// Interval unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntervalUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Week,
    Quarter,
}

impl IntervalUnit {
    /// Parse an interval unit from a string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "YEAR" | "YEARS" => Some(IntervalUnit::Year),
            "MONTH" | "MONTHS" => Some(IntervalUnit::Month),
            "DAY" | "DAYS" => Some(IntervalUnit::Day),
            "HOUR" | "HOURS" => Some(IntervalUnit::Hour),
            "MINUTE" | "MINUTES" => Some(IntervalUnit::Minute),
            "SECOND" | "SECONDS" => Some(IntervalUnit::Second),
            "MILLISECOND" | "MILLISECONDS" => Some(IntervalUnit::Millisecond),
            "MICROSECOND" | "MICROSECONDS" => Some(IntervalUnit::Microsecond),
            "NANOSECOND" | "NANOSECONDS" => Some(IntervalUnit::Nanosecond),
            "WEEK" | "WEEKS" => Some(IntervalUnit::Week),
            "QUARTER" | "QUARTERS" => Some(IntervalUnit::Quarter),
            _ => None,
        }
    }
}

/// Typed literal type (DATE, TIME, TIMESTAMP, DATETIME, JSON, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedLiteralType {
    Date,
    Time,
    Timestamp,
    Datetime,
    Json,
    Numeric,
    Bignumeric,
    Range,
}

impl TypedLiteralType {
    /// Parse a typed literal type from a string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "DATE" => Some(TypedLiteralType::Date),
            "TIME" => Some(TypedLiteralType::Time),
            "TIMESTAMP" => Some(TypedLiteralType::Timestamp),
            "DATETIME" => Some(TypedLiteralType::Datetime),
            "JSON" => Some(TypedLiteralType::Json),
            "NUMERIC" | "DECIMAL" => Some(TypedLiteralType::Numeric),
            "BIGNUMERIC" | "BIGDECIMAL" => Some(TypedLiteralType::Bignumeric),
            "RANGE" => Some(TypedLiteralType::Range),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_op_precedence() {
        assert!(BinaryOp::Multiply.precedence() > BinaryOp::Plus.precedence());
        assert!(BinaryOp::And.precedence() > BinaryOp::Or.precedence());
        assert!(BinaryOp::Eq.precedence() > BinaryOp::And.precedence());
    }

    #[test]
    fn test_datetime_part_from_str() {
        assert_eq!(DateTimePart::parse("YEAR"), Some(DateTimePart::Year));
        assert_eq!(DateTimePart::parse("year"), Some(DateTimePart::Year));
        assert_eq!(DateTimePart::parse("invalid"), None);
    }

    #[test]
    fn test_interval_unit_from_str() {
        assert_eq!(IntervalUnit::parse("DAY"), Some(IntervalUnit::Day));
        assert_eq!(IntervalUnit::parse("DAYS"), Some(IntervalUnit::Day));
        assert_eq!(IntervalUnit::parse("invalid"), None);
    }
}
