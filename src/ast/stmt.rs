//! Statement AST definitions.
//!
//! This module defines the statement types for SQL DDL and DML.

use super::{
    Alias, ColumnDef, DataTypeSpec, Expr, FromClause, Ident, ObjectName, Query, SelectItem,
    SortKey, SqlOption, TableConstraint, TableRef,
};
use crate::error::Span;

/// A complete SQL statement.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Statement kind.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    // Query
    Query(Box<Query>),

    // DML
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    Merge(MergeStatement),

    // DDL
    CreateDatabase(CreateDatabaseStatement),
    CreateTable(CreateTableStatement),
    CreateView(CreateViewStatement),
    CreateIndex(CreateIndexStatement),
    CreateFunction(CreateFunctionStatement),
    CreateProcedure(CreateProcedureStatement),

    AlterTable(AlterTableStatement),
    AlterView(AlterViewStatement),

    Drop(DropStatement),
    Truncate(TruncateStatement),

    // Transaction control
    Begin(BeginStatement),
    Commit,
    Rollback(RollbackStatement),

    // Utility
    Explain(ExplainStatement),
    Describe(DescribeStatement),
    Show(ShowStatement),
    Set(SetStatement),

    // Empty statement (just a semicolon)
    Empty,
}

// ============================================================================
// DML Statements
// ============================================================================

/// INSERT statement.
#[derive(Debug, Clone, PartialEq)]
pub struct InsertStatement {
    pub or_action: Option<InsertOrAction>,
    pub table: ObjectName,
    pub columns: Vec<Ident>,
    pub source: InsertSource,
    pub returning: Option<ReturningClause>,
}

/// INSERT OR action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertOrAction {
    Ignore,
    Replace,
    Update,
}

/// INSERT source (VALUES or SELECT).
#[derive(Debug, Clone, PartialEq)]
pub enum InsertSource {
    Values(Vec<Vec<Box<Expr>>>),
    Query(Box<Query>),
    DefaultValues,
}

/// UPDATE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateStatement {
    pub table: TableRef,
    pub assignments: Vec<Assignment>,
    pub from: Option<FromClause>,
    pub where_clause: Option<Box<Expr>>,
    pub returning: Option<ReturningClause>,
}

/// Assignment in UPDATE SET clause.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub target: AssignmentTarget,
    pub value: Box<Expr>,
}

/// Assignment target (column or path).
#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentTarget {
    Column(Ident),
    Path(Vec<Ident>),
}

/// DELETE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStatement {
    pub table: ObjectName,
    pub alias: Option<Alias>,
    pub where_clause: Option<Box<Expr>>,
    pub returning: Option<ReturningClause>,
}

/// MERGE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct MergeStatement {
    pub target: TableRef,
    pub source: TableRef,
    pub on: Box<Expr>,
    pub clauses: Vec<MergeClause>,
}

/// MERGE clause.
#[derive(Debug, Clone, PartialEq)]
pub enum MergeClause {
    Matched {
        condition: Option<Box<Expr>>,
        action: MergeMatchedAction,
    },
    NotMatched {
        condition: Option<Box<Expr>>,
        action: MergeNotMatchedAction,
    },
    NotMatchedBySource {
        condition: Option<Box<Expr>>,
        action: MergeMatchedAction,
    },
}

/// Action for WHEN MATCHED.
#[derive(Debug, Clone, PartialEq)]
pub enum MergeMatchedAction {
    Update { assignments: Vec<Assignment> },
    Delete,
}

/// Action for WHEN NOT MATCHED.
#[derive(Debug, Clone, PartialEq)]
pub struct MergeNotMatchedAction {
    pub columns: Vec<Ident>,
    pub values: Vec<Box<Expr>>,
}

/// RETURNING clause.
#[derive(Debug, Clone, PartialEq)]
pub struct ReturningClause {
    pub items: Vec<SelectItem>,
    pub with_action: bool,
}

// ============================================================================
// DDL Statements
// ============================================================================

/// CREATE DATABASE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateDatabaseStatement {
    pub name: Ident,
    pub if_not_exists: bool,
    pub options: Vec<SqlOption>,
}

/// CREATE TABLE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateTableStatement {
    pub or_replace: bool,
    pub temporary: bool,
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
    pub partition_by: Vec<Box<Expr>>,
    pub cluster_by: Vec<Box<Expr>>,
    pub options: Vec<SqlOption>,
    pub as_query: Option<Box<Query>>,
    pub like: Option<ObjectName>,
    pub clone: Option<ObjectName>,
}

/// CREATE VIEW statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateViewStatement {
    pub or_replace: bool,
    pub materialized: bool,
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub columns: Vec<Ident>,
    pub query: Box<Query>,
    pub options: Vec<SqlOption>,
}

/// CREATE INDEX statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateIndexStatement {
    pub unique: bool,
    pub if_not_exists: bool,
    pub name: Option<Ident>,
    pub table: ObjectName,
    pub columns: Vec<SortKey>,
    pub options: Vec<SqlOption>,
}

/// CREATE FUNCTION statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateFunctionStatement {
    pub or_replace: bool,
    pub temporary: bool,
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub params: Vec<FunctionParam>,
    pub returns: Option<DataTypeSpec>,
    pub language: Option<String>,
    pub body: FunctionBody,
    pub options: Vec<SqlOption>,
}

/// Function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParam {
    pub name: Option<Ident>,
    pub data_type: DataTypeSpec,
    pub default: Option<Box<Expr>>,
}

/// Function body.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBody {
    Expr(Box<Expr>),
    Statements(Vec<Statement>),
    External(String),
}

/// CREATE PROCEDURE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateProcedureStatement {
    pub or_replace: bool,
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub params: Vec<ProcedureParam>,
    pub body: Vec<Statement>,
    pub options: Vec<SqlOption>,
}

/// Procedure parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureParam {
    pub mode: ParamMode,
    pub name: Option<Ident>,
    pub data_type: DataTypeSpec,
}

/// Parameter mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamMode {
    In,
    Out,
    InOut,
}

/// ALTER TABLE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct AlterTableStatement {
    pub if_exists: bool,
    pub name: ObjectName,
    pub action: AlterTableAction,
}

/// ALTER TABLE action.
#[derive(Debug, Clone, PartialEq)]
pub enum AlterTableAction {
    AddColumn {
        if_not_exists: bool,
        column: ColumnDef,
    },
    DropColumn {
        if_exists: bool,
        column: Ident,
    },
    AlterColumn {
        column: Ident,
        action: AlterColumnAction,
    },
    AddConstraint(TableConstraint),
    DropConstraint {
        if_exists: bool,
        name: Ident,
    },
    RenameColumn {
        from: Ident,
        to: Ident,
    },
    RenameTable(ObjectName),
    SetOptions(Vec<SqlOption>),
}

/// ALTER COLUMN action.
#[derive(Debug, Clone, PartialEq)]
pub enum AlterColumnAction {
    SetDataType(DataTypeSpec),
    SetDefault(Box<Expr>),
    DropDefault,
    SetNotNull,
    DropNotNull,
    SetOptions(Vec<SqlOption>),
}

/// ALTER VIEW statement.
#[derive(Debug, Clone, PartialEq)]
pub struct AlterViewStatement {
    pub if_exists: bool,
    pub name: ObjectName,
    pub action: AlterViewAction,
}

/// ALTER VIEW action.
#[derive(Debug, Clone, PartialEq)]
pub enum AlterViewAction {
    SetOptions(Vec<SqlOption>),
    SetQuery(Box<Query>),
}

/// DROP statement.
#[derive(Debug, Clone, PartialEq)]
pub struct DropStatement {
    pub object_type: ObjectType,
    pub if_exists: bool,
    pub names: Vec<ObjectName>,
    pub cascade: bool,
}

/// Object type for DROP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Table,
    View,
    MaterializedView,
    Index,
    Function,
    Procedure,
    Database,
    Schema,
}

/// TRUNCATE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct TruncateStatement {
    pub table: ObjectName,
}

// ============================================================================
// Transaction Control
// ============================================================================

/// BEGIN statement.
#[derive(Debug, Clone, PartialEq)]
pub struct BeginStatement {
    pub mode: Option<TransactionMode>,
}

/// Transaction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionMode {
    ReadOnly,
    ReadWrite,
}

/// ROLLBACK statement.
#[derive(Debug, Clone, PartialEq)]
pub struct RollbackStatement {
    pub savepoint: Option<Ident>,
}

// ============================================================================
// Utility Statements
// ============================================================================

/// EXPLAIN statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplainStatement {
    pub analyze: bool,
    pub format: Option<ExplainFormat>,
    pub statement: Box<Statement>,
}

/// EXPLAIN format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplainFormat {
    Text,
    Json,
}

/// DESCRIBE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct DescribeStatement {
    pub object: ObjectName,
}

/// SHOW statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ShowStatement {
    pub object_type: ShowObjectType,
    pub filter: Option<ShowFilter>,
}

/// SHOW object type.
#[derive(Debug, Clone, PartialEq)]
pub enum ShowObjectType {
    Tables { from: Option<Ident> },
    Databases,
    Schemas { from: Option<Ident> },
    Columns { from: ObjectName },
    Functions { from: Option<Ident> },
    Variables,
}

/// SHOW filter.
#[derive(Debug, Clone, PartialEq)]
pub enum ShowFilter {
    Like(String),
    Where(Box<Expr>),
}

/// SET statement.
#[derive(Debug, Clone, PartialEq)]
pub struct SetStatement {
    pub variable: Ident,
    pub value: SetValue,
}

/// SET value.
#[derive(Debug, Clone, PartialEq)]
pub enum SetValue {
    Expr(Box<Expr>),
    Default,
}

// ============================================================================
// Procedural Language
// ============================================================================

/// Procedural statement (for stored procedures).
#[derive(Debug, Clone, PartialEq)]
pub enum ProceduralStatement {
    Declare(DeclareStatement),
    Set(SetVariableStatement),
    If(IfStatement),
    Loop(LoopStatement),
    While(WhileStatement),
    For(ForStatement),
    Break,
    Continue,
    Return(Option<Box<Expr>>),
    Raise(RaiseStatement),
    Call(CallStatement),
    Begin(BeginEndStatement),
}

/// DECLARE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclareStatement {
    pub names: Vec<Ident>,
    pub data_type: Option<DataTypeSpec>,
    pub default: Option<Box<Expr>>,
}

/// SET variable statement.
#[derive(Debug, Clone, PartialEq)]
pub struct SetVariableStatement {
    pub names: Vec<Ident>,
    pub value: Box<Expr>,
}

/// IF statement.
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Box<Expr>,
    pub then_body: Vec<Statement>,
    pub elseif_clauses: Vec<(Box<Expr>, Vec<Statement>)>,
    pub else_body: Option<Vec<Statement>>,
}

/// LOOP statement.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement {
    pub body: Vec<Statement>,
}

/// WHILE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Box<Expr>,
    pub body: Vec<Statement>,
}

/// FOR statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub variable: Ident,
    pub query: Box<Query>,
    pub body: Vec<Statement>,
}

/// RAISE statement.
#[derive(Debug, Clone, PartialEq)]
pub struct RaiseStatement {
    pub message: Option<Box<Expr>>,
}

/// CALL statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CallStatement {
    pub name: ObjectName,
    pub args: Vec<Box<Expr>>,
}

/// BEGIN...END block.
#[derive(Debug, Clone, PartialEq)]
pub struct BeginEndStatement {
    pub statements: Vec<Statement>,
    pub exception_handlers: Vec<ExceptionHandler>,
}

/// Exception handler.
#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionHandler {
    pub when: ExceptionWhen,
    pub body: Vec<Statement>,
}

/// Exception condition.
#[derive(Debug, Clone, PartialEq)]
pub enum ExceptionWhen {
    Error,
    Named(String),
}
