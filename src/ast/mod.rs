//! Abstract Syntax Tree (AST) definitions for SQL statements.
//!
//! This module defines the complete AST structure for representing parsed SQL,
//! following standard SQL conventions.

mod expr;
mod stmt;
mod types;

// Re-export types module first (has DataTypeSpec needed by others)
pub use types::StructField as TypeStructField;
pub use types::{DataTypeKind, DataTypeSpec};

// Re-export expression types
pub use expr::*;

// Re-export statement types
pub use stmt::*;

use crate::error::Span;

/// An identifier (table name, column name, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    /// The identifier value (unquoted or with quotes removed).
    pub value: String,
    /// Whether this identifier was quoted (backticks).
    pub quoted: bool,
    /// Source span.
    pub span: Span,
}

impl Ident {
    pub fn new(value: impl Into<String>, span: Span) -> Self {
        Self {
            value: value.into(),
            quoted: false,
            span,
        }
    }

    pub fn quoted(value: impl Into<String>, span: Span) -> Self {
        Self {
            value: value.into(),
            quoted: true,
            span,
        }
    }

    /// Check if this identifier matches another (case-insensitive for unquoted).
    pub fn matches(&self, other: &str) -> bool {
        if self.quoted {
            self.value == other
        } else {
            self.value.eq_ignore_ascii_case(other)
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.quoted {
            write!(f, "`{}`", self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

/// A qualified name (schema.table, catalog.schema.table, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectName {
    pub parts: Vec<Ident>,
    pub span: Span,
}

impl ObjectName {
    pub fn new(parts: Vec<Ident>, span: Span) -> Self {
        Self { parts, span }
    }

    pub fn simple(name: Ident) -> Self {
        let span = name.span;
        Self {
            parts: vec![name],
            span,
        }
    }

    /// Get the table/object name (last part).
    pub fn name(&self) -> Option<&Ident> {
        self.parts.last()
    }

    /// Get the schema name (second-to-last part, if any).
    pub fn schema(&self) -> Option<&Ident> {
        if self.parts.len() >= 2 {
            Some(&self.parts[self.parts.len() - 2])
        } else {
            None
        }
    }

    /// Get the catalog name (third-to-last part, if any).
    pub fn catalog(&self) -> Option<&Ident> {
        if self.parts.len() >= 3 {
            Some(&self.parts[self.parts.len() - 3])
        } else {
            None
        }
    }
}

impl std::fmt::Display for ObjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self.parts.iter().map(|p| p.to_string()).collect();
        write!(f, "{}", parts.join("."))
    }
}

/// Column definition for CREATE TABLE.
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name: Ident,
    pub data_type: Option<DataTypeSpec>,
    pub constraints: Vec<ColumnConstraint>,
    pub options: Vec<SqlOption>,
    pub span: Span,
}

/// Column constraint.
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnConstraint {
    NotNull,
    Null,
    PrimaryKey,
    Unique,
    Default(Box<Expr>),
    Check(Box<Expr>),
    References {
        table: ObjectName,
        columns: Vec<Ident>,
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
    },
    Generated {
        expr: Box<Expr>,
        always: bool,
    },
    Hidden,
}

/// Referential action for foreign keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferentialAction {
    NoAction,
    Restrict,
    Cascade,
    SetNull,
    SetDefault,
}

/// Table constraint definition.
#[derive(Debug, Clone, PartialEq)]
pub enum TableConstraint {
    PrimaryKey {
        name: Option<Ident>,
        columns: Vec<SortKey>,
        options: Vec<SqlOption>,
    },
    Unique {
        name: Option<Ident>,
        columns: Vec<Ident>,
    },
    ForeignKey {
        name: Option<Ident>,
        columns: Vec<Ident>,
        references_table: ObjectName,
        references_columns: Vec<Ident>,
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
    },
    Check {
        name: Option<Ident>,
        expr: Box<Expr>,
        enforced: Option<bool>,
    },
}

/// Sort key for ORDER BY, PRIMARY KEY, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct SortKey {
    pub column: Ident,
    pub order: Option<SortOrder>,
    pub nulls: Option<NullsOrder>,
}

/// Sort order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// NULL ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullsOrder {
    First,
    Last,
}

/// SQL option (key-value pair).
#[derive(Debug, Clone, PartialEq)]
pub struct SqlOption {
    pub name: Ident,
    pub value: Box<Expr>,
}

/// An alias with optional column aliases.
#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    pub name: Ident,
    pub columns: Vec<Ident>,
}

impl Alias {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            columns: Vec::new(),
        }
    }

    pub fn with_columns(name: Ident, columns: Vec<Ident>) -> Self {
        Self { name, columns }
    }
}

/// WITH clause (Common Table Expressions).
#[derive(Debug, Clone, PartialEq)]
pub struct WithClause {
    pub recursive: bool,
    pub ctes: Vec<Cte>,
    pub span: Span,
}

/// Common Table Expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Cte {
    pub name: Ident,
    pub columns: Vec<Ident>,
    pub query: Box<Query>,
    pub span: Span,
}

/// A complete query expression (may include WITH, set operations, ORDER BY, LIMIT).
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub with: Option<WithClause>,
    pub body: QueryBody,
    pub order_by: Vec<OrderByExpr>,
    pub limit: Option<LimitClause>,
    pub span: Span,
}

/// The body of a query (SELECT, set operations, or subquery).
#[derive(Debug, Clone, PartialEq)]
pub enum QueryBody {
    Select(Box<Select>),
    SetOperation {
        op: SetOperator,
        all: bool,
        left: Box<QueryBody>,
        right: Box<QueryBody>,
    },
    Parenthesized(Box<Query>),
}

/// Set operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetOperator {
    Union,
    Intersect,
    Except,
}

/// LIMIT clause.
#[derive(Debug, Clone, PartialEq)]
pub struct LimitClause {
    pub count: Option<Box<Expr>>,
    pub offset: Option<Box<Expr>>,
}

/// ORDER BY expression.
#[derive(Debug, Clone, PartialEq)]
pub struct OrderByExpr {
    pub expr: Box<Expr>,
    pub order: Option<SortOrder>,
    pub nulls: Option<NullsOrder>,
}

/// SELECT statement.
#[derive(Debug, Clone, PartialEq)]
pub struct Select {
    pub distinct: Option<Distinct>,
    /// SELECT AS modifier (STRUCT, VALUE, or type name)
    pub select_as: Option<SelectAs>,
    pub projection: Vec<SelectItem>,
    pub from: Option<FromClause>,
    pub where_clause: Option<Box<Expr>>,
    pub group_by: Option<GroupByClause>,
    pub having: Option<Box<Expr>>,
    pub qualify: Option<Box<Expr>>,
    pub window: Vec<WindowDef>,
    pub span: Span,
}

/// SELECT AS modifier for value tables.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectAs {
    /// SELECT AS STRUCT - returns each row as a STRUCT
    Struct,
    /// SELECT AS VALUE - returns a single-column value table
    Value,
    /// SELECT AS <type_name> - returns value table with specific type
    TypeName(ObjectName),
}

/// DISTINCT specification.
#[derive(Debug, Clone, PartialEq)]
pub enum Distinct {
    All,
    Distinct,
}

/// SELECT list item.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectItem {
    /// An expression, optionally with an alias: `expr [AS alias]`
    Expr {
        expr: Box<Expr>,
        alias: Option<Ident>,
    },
    /// Wildcard: `*`
    Wildcard,
    /// Qualified wildcard: `table.*`
    QualifiedWildcard { qualifier: ObjectName },
    /// Wildcard with EXCEPT: `* EXCEPT (col1, col2)`
    WildcardExcept {
        qualifier: Option<ObjectName>,
        except: Vec<Ident>,
    },
    /// Wildcard with REPLACE: `* REPLACE (expr AS col)`
    WildcardReplace {
        qualifier: Option<ObjectName>,
        replace: Vec<(Box<Expr>, Ident)>,
    },
}

/// FROM clause.
#[derive(Debug, Clone, PartialEq)]
pub struct FromClause {
    pub tables: Vec<TableRef>,
}

/// Table reference in FROM clause.
#[derive(Debug, Clone, PartialEq)]
pub enum TableRef {
    /// Simple table reference: `table [AS alias]`
    Table {
        name: ObjectName,
        alias: Option<Alias>,
        hints: Vec<SqlOption>,
    },
    /// Subquery: `(SELECT ...) AS alias`
    Subquery {
        query: Box<Query>,
        alias: Option<Alias>,
    },
    /// UNNEST: `UNNEST(array) [AS alias] [WITH OFFSET [AS offset_alias]]`
    Unnest {
        expr: Box<Expr>,
        alias: Option<Alias>,
        with_offset: bool,
        offset_alias: Option<Ident>,
    },
    /// Join: `table1 JOIN table2 ON condition`
    Join {
        left: Box<TableRef>,
        right: Box<TableRef>,
        join_type: JoinType,
        condition: Option<JoinCondition>,
    },
    /// Parenthesized table reference
    Parenthesized(Box<TableRef>),
    /// Table function: `TABLE_FUNCTION(...)`
    TableFunction {
        name: ObjectName,
        args: Vec<FunctionArg>,
        alias: Option<Alias>,
    },
}

/// Type of JOIN.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
    Natural,
    LeftSemi,
    RightSemi,
    LeftAnti,
    RightAnti,
}

/// JOIN condition.
#[derive(Debug, Clone, PartialEq)]
pub enum JoinCondition {
    On(Box<Expr>),
    Using(Vec<Ident>),
}

/// GROUP BY clause.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupByClause {
    pub items: Vec<GroupByItem>,
}

/// GROUP BY item.
#[derive(Debug, Clone, PartialEq)]
pub enum GroupByItem {
    Expr(Box<Expr>),
    Rollup(Vec<Box<Expr>>),
    Cube(Vec<Box<Expr>>),
    GroupingSets(Vec<Vec<Box<Expr>>>),
}

/// Named window definition.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowDef {
    pub name: Ident,
    pub spec: WindowSpec,
}

/// Window specification.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowSpec {
    pub partition_by: Vec<Box<Expr>>,
    pub order_by: Vec<OrderByExpr>,
    pub frame: Option<WindowFrame>,
}

/// Window frame specification.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowFrame {
    pub unit: WindowFrameUnit,
    pub start: WindowFrameBound,
    pub end: Option<WindowFrameBound>,
}

/// Window frame unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowFrameUnit {
    Rows,
    Range,
    Groups,
}

/// Window frame bound.
#[derive(Debug, Clone, PartialEq)]
pub enum WindowFrameBound {
    CurrentRow,
    Preceding(Option<Box<Expr>>), // None = UNBOUNDED
    Following(Option<Box<Expr>>), // None = UNBOUNDED
}
