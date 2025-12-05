//! Token definitions for the SQL lexer.
//!
//! This module defines all token types recognized by the SQL lexer,
//! including keywords, operators, literals, and special characters.

use crate::error::Span;
use std::fmt;

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,
    /// The source span of this token.
    pub span: Span,
    /// The original text of this token.
    pub text: String,
}

impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, span: Span, text: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            text: text.into(),
        }
    }

    /// Check if this token is a specific keyword.
    pub fn is_keyword(&self, kw: Keyword) -> bool {
        matches!(&self.kind, TokenKind::Keyword(k) if *k == kw)
    }

    /// Check if this token is any keyword.
    pub fn is_any_keyword(&self) -> bool {
        matches!(&self.kind, TokenKind::Keyword(_))
    }

    /// Check if this token is an identifier.
    pub fn is_identifier(&self) -> bool {
        matches!(&self.kind, TokenKind::Identifier(_))
    }

    /// Check if this token could be used as an identifier (including non-reserved keywords).
    pub fn as_identifier(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Identifier(s) => Some(s),
            TokenKind::Keyword(kw) if !kw.is_reserved() => Some(self.text.as_str()),
            _ => None,
        }
    }

    /// Get the identifier string if this is an identifier or non-reserved keyword.
    pub fn identifier_value(&self) -> Option<String> {
        self.as_identifier().map(|s| s.to_string())
    }

    /// Check if this is an EOF token.
    pub fn is_eof(&self) -> bool {
        matches!(&self.kind, TokenKind::Eof)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

/// The kind of token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Identifiers and keywords
    Identifier(String),
    QuotedIdentifier(String),
    Keyword(Keyword),

    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Boolean(bool),
    Null,

    // Operators
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Caret,      // ^
    Ampersand,  // &
    Pipe,       // |
    Tilde,      // ~
    DoublePipe, // ||
    LeftShift,  // <<
    RightShift, // >>

    // Comparison operators
    Eq,     // =
    NotEq,  // != or <>
    Lt,     // <
    LtEq,   // <=
    Gt,     // >
    GtEq,   // >=
    LtGt,   // <> (not equal)
    SafeEq, // <=> (null-safe equal)

    // Punctuation
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    DoubleColon,  // ::
    Dot,          // .
    DoubleDot,    // ..
    Arrow,        // ->
    FatArrow,     // =>
    At,           // @
    Question,     // ?
    Hash,         // #
    Dollar,       // $
    Backslash,    // \

    // Special tokens
    Eof,
    Error(String),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Identifier(s) => write!(f, "identifier '{}'", s),
            TokenKind::QuotedIdentifier(s) => write!(f, "quoted identifier `{}`", s),
            TokenKind::Keyword(kw) => write!(f, "keyword {}", kw),
            TokenKind::Integer(n) => write!(f, "integer {}", n),
            TokenKind::Float(n) => write!(f, "float {}", n),
            TokenKind::String(s) => write!(f, "string '{}'", s),
            TokenKind::Bytes(_) => write!(f, "bytes literal"),
            TokenKind::Boolean(b) => write!(f, "{}", b),
            TokenKind::Null => write!(f, "NULL"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::DoublePipe => write!(f, "||"),
            TokenKind::LeftShift => write!(f, "<<"),
            TokenKind::RightShift => write!(f, ">>"),
            TokenKind::Eq => write!(f, "="),
            TokenKind::NotEq => write!(f, "!="),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::LtEq => write!(f, "<="),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::GtEq => write!(f, ">="),
            TokenKind::LtGt => write!(f, "<>"),
            TokenKind::SafeEq => write!(f, "<=>"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::DoubleColon => write!(f, "::"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::DoubleDot => write!(f, ".."),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::At => write!(f, "@"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::Hash => write!(f, "#"),
            TokenKind::Dollar => write!(f, "$"),
            TokenKind::Backslash => write!(f, "\\"),
            TokenKind::Eof => write!(f, "end of input"),
            TokenKind::Error(msg) => write!(f, "error: {}", msg),
        }
    }
}

/// SQL keywords recognized by the lexer.
///
/// Keywords are case-insensitive in SQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    // Reserved keywords (cannot be used as identifiers without quoting)
    All,
    And,
    Any,
    Array,
    As,
    Asc,
    AssertRowsModified,
    At,
    Between,
    By,
    Case,
    Cast,
    Collate,
    Contains,
    Create,
    Cross,
    Cube,
    Current,
    Default,
    Define,
    Desc,
    Distinct,
    Else,
    End,
    Enum,
    Escape,
    Except,
    Exclude,
    Exists,
    Extract,
    False,
    Fetch,
    Following,
    For,
    From,
    Full,
    Group,
    Grouping,
    Groups,
    Hash,
    Having,
    If,
    Ignore,
    In,
    Inner,
    Intersect,
    Interval,
    Into,
    Is,
    Join,
    Lateral,
    Left,
    Like,
    Limit,
    Lookup,
    Merge,
    Natural,
    New,
    No,
    Not,
    Null,
    Nulls,
    Of,
    Offset,
    On,
    Or,
    Order,
    Ordinal,
    Outer,
    Over,
    Partition,
    Preceding,
    Qualify,
    Range,
    Recursive,
    Respect,
    Right,
    Rollup,
    Rows,
    Select,
    Semi,
    Set,
    Some,
    Struct,
    Tablesample,
    Then,
    To,
    Treat,
    True,
    Unbounded,
    Union,
    Unnest,
    Using,
    When,
    Where,
    Window,
    With,
    Within,

    // Non-reserved keywords (commonly used but can be identifiers)
    Abort,
    Access,
    Action,
    Add,
    Anti,
    After,
    Aggregate,
    Alter,
    Always,
    Analyze,
    Approx,
    Are,
    Assertion,
    Avg,
    Begin,
    Bigint,
    Binary,
    Bit,
    Blob,
    Bool,
    Boolean,
    Both,
    Breadth,
    Bytes,
    Call,
    Cascade,
    Catalog,
    Chain,
    Char,
    Character,
    Check,
    Clob,
    Clone,
    Close,
    Cluster,
    Coalesce,
    Collation,
    Column,
    Columns,
    Commit,
    Committed,
    Concat,
    Connection,
    Constant,
    Constraint,
    Continue,
    Corresponding,
    Count,
    Cume,
    Data,
    Database,
    Databases,
    Date,
    Datetime,
    Day,
    Days,
    Deallocate,
    Dec,
    Decimal,
    Declare,
    Delete,
    Dense,
    Depth,
    Deref,
    Describe,
    Descriptor,
    Deterministic,
    Diagnostics,
    Disconnect,
    Do,
    Domain,
    Double,
    Drop,
    Dynamic,
    Each,
    Element,
    Elseif,
    Empty,
    Enforced,
    Error,
    Exception,
    Exec,
    Execute,
    Explain,
    Export,
    External,
    Filter,
    First,
    Float,
    Float32,
    Float64,
    Foreign,
    Format,
    Found,
    Free,
    Function,
    Functions,
    Generated,
    Get,
    Global,
    Go,
    Goto,
    Grant,
    Graph,
    GraphTable,
    Greatest,
    Handler,
    Hidden,
    Hold,
    Hour,
    Hours,
    Identity,
    Immediate,
    Import,
    Index,
    Indicator,
    Inout,
    Input,
    Insensitive,
    Insert,
    Int,
    Int32,
    Int64,
    Integer,
    Isolation,
    Iterate,
    Json,
    Key,
    Keys,
    Label,
    Language,
    Large,
    Last,
    Least,
    Leave,
    Level,
    Load,
    Local,
    Localtime,
    Localtimestamp,
    Location,
    Log,
    Loop,
    Map,
    Match,
    Matched,
    Materialized,
    Max,
    Message,
    Microsecond,
    Microseconds,
    Millisecond,
    Milliseconds,
    Min,
    Minute,
    Minutes,
    Mod,
    Mode,
    Model,
    Modifies,
    Module,
    Month,
    Months,
    Names,
    Nanosecond,
    Nanoseconds,
    National,
    Nchar,
    Nclob,
    Next,
    Normalize,
    Nth,
    Ntile,
    Nullif,
    Numeric,
    Object,
    Oid,
    Old,
    Only,
    Open,
    Operator,
    Option,
    Options,
    Out,
    Output,
    Overlaps,
    Overlay,
    Overwrite,
    Pad,
    Parameter,
    Partial,
    Percent,
    PercentRank,
    Percentile,
    Period,
    Pivot,
    Placing,
    Policy,
    Position,
    Precision,
    Prepare,
    Preserve,
    Primary,
    Prior,
    Private,
    Privilege,
    Privileges,
    Procedure,
    Project,
    Public,
    Quarter,
    Quarters,
    Raise,
    Rank,
    Read,
    Reads,
    Real,
    Record,
    Ref,
    References,
    Referencing,
    Relative,
    Release,
    Rename,
    Repeat,
    Replace,
    Replica,
    Restrict,
    Return,
    Returning,
    Returns,
    Rollback,
    Revoke,
    Role,
    Routine,
    Row,
    RowNumber,
    Run,
    Safe,
    SafeCast,
    SafeOffset,
    SafeOrdinal,
    Savepoint,
    Schema,
    Schemas,
    Scope,
    Scroll,
    Search,
    Second,
    Seconds,
    Section,
    Security,
    Sequence,
    Session,
    Sets,
    Show,
    Similar,
    Simple,
    Size,
    Smallint,
    Snapshot,
    Source,
    Space,
    Specific,
    Sql,
    Sqlcode,
    Sqlerror,
    Sqlexception,
    Sqlstate,
    Sqlwarning,
    Start,
    State,
    Statement,
    Static,
    Stddev,
    StddevPop,
    StddevSamp,
    Storage,
    String,
    SubMultiset,
    Submultiset,
    Substring,
    Sum,
    Symmetric,
    System,
    SystemTime,
    SystemUser,
    Table,
    Tables,
    Target,
    Temp,
    Temporary,
    Text,
    Time,
    Timestamp,
    Timezone,
    TimezoneHour,
    TimezoneMinute,
    ToBigint,
    ToDouble,
    ToInt64,
    ToJson,
    Transaction,
    Transform,
    Translate,
    Translation,
    Trigger,
    Trim,
    Truncate,
    Trusted,
    Type,
    Uint32,
    Uint64,
    Under,
    Undo,
    Unique,
    Unknown,
    Unpivot,
    Unsigned,
    Until,
    Update,
    Usage,
    User,
    Uuid,
    Value,
    Values,
    Varbinary,
    Varchar,
    Variable,
    Variables,
    Variance,
    VarPop,
    VarSamp,
    Varying,
    View,
    Virtual,
    Volatile,
    Week,
    Weeks,
    Weight,
    While,
    Work,
    Write,
    Year,
    Years,
    Zone,
}

impl Keyword {
    /// Check if this keyword is reserved (cannot be used as an identifier without quoting).
    /// See: lexical.md#reserved_keywords
    pub fn is_reserved(&self) -> bool {
        use Keyword::*;
        matches!(
            self,
            All | And
                | Any
                | Array
                | As
                | Asc
                | AssertRowsModified
                | At
                | Between
                | By
                | Case
                | Cast
                | Collate
                | Contains
                | Create
                | Cross
                | Cube
                | Current
                | Default
                | Define
                | Desc
                | Distinct
                | Else
                | End
                | Enum
                | Escape
                | Except
                | Exclude
                | Exists
                | Extract
                | False
                | Fetch
                | Following
                | For
                | From
                | Full
                | GraphTable
                | Group
                | Grouping
                | Groups
                | Hash
                | Having
                | If
                | Ignore
                | In
                | Inner
                | Intersect
                | Interval
                | Into
                | Is
                | Join
                | Lateral
                | Left
                | Like
                | Limit
                | Lookup
                | Merge
                | Natural
                | New
                | No
                | Not
                | Null
                | Nulls
                | Of
                | On
                | Or
                | Order
                | Outer
                | Over
                | Partition
                | Preceding
                | Qualify
                | Range
                | Recursive
                | Respect
                | Right
                | Rollup
                | Rows
                | Select
                | Set
                | Some
                | Struct
                | Tablesample
                | Then
                | To
                | Treat
                | True
                | Unbounded
                | Union
                | Unnest
                | Using
                | When
                | Where
                | Window
                | With
                | Within
        )
    }

    /// Try to parse a keyword from a string (case-insensitive).
    pub fn parse(s: &str) -> Option<Keyword> {
        let upper = s.to_uppercase();
        KEYWORD_MAP().get(upper.as_str()).copied()
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Static keyword map for O(1) lookup
lazy_static_keyword_map! {
    KEYWORD_MAP = {
        // Reserved keywords
        "ALL" => All,
        "AND" => And,
        "ANY" => Any,
        "ARRAY" => Array,
        "AS" => As,
        "ASC" => Asc,
        "ASSERT_ROWS_MODIFIED" => AssertRowsModified,
        "AT" => At,
        "BETWEEN" => Between,
        "BY" => By,
        "CASE" => Case,
        "CAST" => Cast,
        "COLLATE" => Collate,
        "CONTAINS" => Contains,
        "CREATE" => Create,
        "CROSS" => Cross,
        "CUBE" => Cube,
        "CURRENT" => Current,
        "DEFAULT" => Default,
        "DEFINE" => Define,
        "DESC" => Desc,
        "DISTINCT" => Distinct,
        "ELSE" => Else,
        "END" => End,
        "ENUM" => Enum,
        "ESCAPE" => Escape,
        "EXCEPT" => Except,
        "EXCLUDE" => Exclude,
        "EXISTS" => Exists,
        "EXTRACT" => Extract,
        "FALSE" => False,
        "FETCH" => Fetch,
        "FOLLOWING" => Following,
        "FOR" => For,
        "FROM" => From,
        "FULL" => Full,
        "GROUP" => Group,
        "GROUPING" => Grouping,
        "GROUPS" => Groups,
        "HASH" => Hash,
        "HAVING" => Having,
        "IF" => If,
        "IGNORE" => Ignore,
        "IN" => In,
        "INNER" => Inner,
        "INTERSECT" => Intersect,
        "INTERVAL" => Interval,
        "INTO" => Into,
        "IS" => Is,
        "JOIN" => Join,
        "LATERAL" => Lateral,
        "LEFT" => Left,
        "LIKE" => Like,
        "LIMIT" => Limit,
        "LOOKUP" => Lookup,
        "MERGE" => Merge,
        "NATURAL" => Natural,
        "NEW" => New,
        "NO" => No,
        "NOT" => Not,
        "NULL" => Null,
        "NULLS" => Nulls,
        "OF" => Of,
        "OFFSET" => Offset,
        "ON" => On,
        "OR" => Or,
        "ORDER" => Order,
        "ORDINAL" => Ordinal,
        "OUTER" => Outer,
        "OVER" => Over,
        "PARTITION" => Partition,
        "PRECEDING" => Preceding,
        "QUALIFY" => Qualify,
        "RANGE" => Range,
        "RECURSIVE" => Recursive,
        "RESPECT" => Respect,
        "RIGHT" => Right,
        "ROLLUP" => Rollup,
        "ROWS" => Rows,
        "SELECT" => Select,
        "SEMI" => Semi,
        "SET" => Set,
        "SOME" => Some,
        "STRUCT" => Struct,
        "TABLESAMPLE" => Tablesample,
        "THEN" => Then,
        "TO" => To,
        "TREAT" => Treat,
        "TRUE" => True,
        "UNBOUNDED" => Unbounded,
        "UNION" => Union,
        "UNNEST" => Unnest,
        "USING" => Using,
        "WHEN" => When,
        "WHERE" => Where,
        "WINDOW" => Window,
        "WITH" => With,
        "WITHIN" => Within,

        // Non-reserved keywords
        "ABORT" => Abort,
        "ACCESS" => Access,
        "ACTION" => Action,
        "ADD" => Add,
        "ANTI" => Anti,
        "AFTER" => After,
        "AGGREGATE" => Aggregate,
        "ALTER" => Alter,
        "ALWAYS" => Always,
        "ANALYZE" => Analyze,
        "APPROX" => Approx,
        "ARE" => Are,
        "ASSERTION" => Assertion,
        "AVG" => Avg,
        "BEGIN" => Begin,
        "BIGINT" => Bigint,
        "BINARY" => Binary,
        "BIT" => Bit,
        "BLOB" => Blob,
        "BOOL" => Bool,
        "BOOLEAN" => Boolean,
        "BOTH" => Both,
        "BREADTH" => Breadth,
        "BYTES" => Bytes,
        "CALL" => Call,
        "CASCADE" => Cascade,
        "CATALOG" => Catalog,
        "CHAIN" => Chain,
        "CHAR" => Char,
        "CHARACTER" => Character,
        "CHECK" => Check,
        "CLOB" => Clob,
        "CLONE" => Clone,
        "CLOSE" => Close,
        "CLUSTER" => Cluster,
        "COALESCE" => Coalesce,
        "COLLATION" => Collation,
        "COLUMN" => Column,
        "COLUMNS" => Columns,
        "COMMIT" => Commit,
        "COMMITTED" => Committed,
        "CONCAT" => Concat,
        "CONNECTION" => Connection,
        "CONSTANT" => Constant,
        "CONSTRAINT" => Constraint,
        "CONTINUE" => Continue,
        "CORRESPONDING" => Corresponding,
        "COUNT" => Count,
        "CUME" => Cume,
        "DATA" => Data,
        "DATABASE" => Database,
        "DATABASES" => Databases,
        "DATE" => Date,
        "DATETIME" => Datetime,
        "DAY" => Day,
        "DAYS" => Days,
        "DEALLOCATE" => Deallocate,
        "DEC" => Dec,
        "DECIMAL" => Decimal,
        "DECLARE" => Declare,
        "DELETE" => Delete,
        "DENSE" => Dense,
        "DEPTH" => Depth,
        "DEREF" => Deref,
        "DESCRIBE" => Describe,
        "DESCRIPTOR" => Descriptor,
        "DETERMINISTIC" => Deterministic,
        "DIAGNOSTICS" => Diagnostics,
        "DISCONNECT" => Disconnect,
        "DO" => Do,
        "DOMAIN" => Domain,
        "DOUBLE" => Double,
        "DROP" => Drop,
        "DYNAMIC" => Dynamic,
        "EACH" => Each,
        "ELEMENT" => Element,
        "ELSEIF" => Elseif,
        "EMPTY" => Empty,
        "ENFORCED" => Enforced,
        "ERROR" => Error,
        "EXCEPTION" => Exception,
        "EXEC" => Exec,
        "EXECUTE" => Execute,
        "EXPLAIN" => Explain,
        "EXPORT" => Export,
        "EXTERNAL" => External,
        "FILTER" => Filter,
        "FIRST" => First,
        "FLOAT" => Float,
        "FLOAT32" => Float32,
        "FLOAT64" => Float64,
        "FOREIGN" => Foreign,
        "FORMAT" => Format,
        "FOUND" => Found,
        "FREE" => Free,
        "FUNCTION" => Function,
        "FUNCTIONS" => Functions,
        "GENERATED" => Generated,
        "GET" => Get,
        "GLOBAL" => Global,
        "GO" => Go,
        "GOTO" => Goto,
        "GRANT" => Grant,
        "GRAPH" => Graph,
        "GRAPH_TABLE" => GraphTable,
        "GREATEST" => Greatest,
        "HANDLER" => Handler,
        "HIDDEN" => Hidden,
        "HOLD" => Hold,
        "HOUR" => Hour,
        "HOURS" => Hours,
        "IDENTITY" => Identity,
        "IMMEDIATE" => Immediate,
        "IMPORT" => Import,
        "INDEX" => Index,
        "INDICATOR" => Indicator,
        "INOUT" => Inout,
        "INPUT" => Input,
        "INSENSITIVE" => Insensitive,
        "INSERT" => Insert,
        "INT" => Int,
        "INT32" => Int32,
        "INT64" => Int64,
        "INTEGER" => Integer,
        "ISOLATION" => Isolation,
        "ITERATE" => Iterate,
        "JSON" => Json,
        "KEY" => Key,
        "KEYS" => Keys,
        "LABEL" => Label,
        "LANGUAGE" => Language,
        "LARGE" => Large,
        "LAST" => Last,
        "LEAST" => Least,
        "LEAVE" => Leave,
        "LEVEL" => Level,
        "LOAD" => Load,
        "LOCAL" => Local,
        "LOCALTIME" => Localtime,
        "LOCALTIMESTAMP" => Localtimestamp,
        "LOCATION" => Location,
        "LOG" => Log,
        "LOOP" => Loop,
        "MAP" => Map,
        "MATCH" => Match,
        "MATCHED" => Matched,
        "MATERIALIZED" => Materialized,
        "MAX" => Max,
        "MESSAGE" => Message,
        "MICROSECOND" => Microsecond,
        "MICROSECONDS" => Microseconds,
        "MILLISECOND" => Millisecond,
        "MILLISECONDS" => Milliseconds,
        "MIN" => Min,
        "MINUTE" => Minute,
        "MINUTES" => Minutes,
        "MOD" => Mod,
        "MODE" => Mode,
        "MODEL" => Model,
        "MODIFIES" => Modifies,
        "MODULE" => Module,
        "MONTH" => Month,
        "MONTHS" => Months,
        "NAMES" => Names,
        "NANOSECOND" => Nanosecond,
        "NANOSECONDS" => Nanoseconds,
        "NATIONAL" => National,
        "NCHAR" => Nchar,
        "NCLOB" => Nclob,
        "NEXT" => Next,
        "NORMALIZE" => Normalize,
        "NTH" => Nth,
        "NTILE" => Ntile,
        "NULLIF" => Nullif,
        "NUMERIC" => Numeric,
        "OBJECT" => Object,
        "OID" => Oid,
        "OLD" => Old,
        "ONLY" => Only,
        "OPEN" => Open,
        "OPERATOR" => Operator,
        "OPTION" => Option,
        "OPTIONS" => Options,
        "OUT" => Out,
        "OUTPUT" => Output,
        "OVERLAPS" => Overlaps,
        "OVERLAY" => Overlay,
        "OVERWRITE" => Overwrite,
        "PAD" => Pad,
        "PARAMETER" => Parameter,
        "PARTIAL" => Partial,
        "PERCENT" => Percent,
        "PERCENT_RANK" => PercentRank,
        "PERCENTILE" => Percentile,
        "PERIOD" => Period,
        "PIVOT" => Pivot,
        "PLACING" => Placing,
        "POLICY" => Policy,
        "POSITION" => Position,
        "PRECISION" => Precision,
        "PREPARE" => Prepare,
        "PRESERVE" => Preserve,
        "PRIMARY" => Primary,
        "PRIOR" => Prior,
        "PRIVATE" => Private,
        "PRIVILEGE" => Privilege,
        "PRIVILEGES" => Privileges,
        "PROCEDURE" => Procedure,
        "PROJECT" => Project,
        "PUBLIC" => Public,
        "QUARTER" => Quarter,
        "QUARTERS" => Quarters,
        "RAISE" => Raise,
        "RANK" => Rank,
        "READ" => Read,
        "READS" => Reads,
        "REAL" => Real,
        "RECORD" => Record,
        "REF" => Ref,
        "REFERENCES" => References,
        "REFERENCING" => Referencing,
        "RELATIVE" => Relative,
        "RELEASE" => Release,
        "RENAME" => Rename,
        "REPEAT" => Repeat,
        "REPLACE" => Replace,
        "REPLICA" => Replica,
        "RESTRICT" => Restrict,
        "RETURN" => Return,
        "RETURNING" => Returning,
        "RETURNS" => Returns,
        "REVOKE" => Revoke,
        "ROLLBACK" => Rollback,
        "ROLE" => Role,
        "ROUTINE" => Routine,
        "ROW" => Row,
        "ROW_NUMBER" => RowNumber,
        "RUN" => Run,
        "SAFE" => Safe,
        "SAFE_CAST" => SafeCast,
        "SAFE_OFFSET" => SafeOffset,
        "SAFE_ORDINAL" => SafeOrdinal,
        "SAVEPOINT" => Savepoint,
        "SCHEMA" => Schema,
        "SCHEMAS" => Schemas,
        "SCOPE" => Scope,
        "SCROLL" => Scroll,
        "SEARCH" => Search,
        "SECOND" => Second,
        "SECONDS" => Seconds,
        "SECTION" => Section,
        "SECURITY" => Security,
        "SEQUENCE" => Sequence,
        "SESSION" => Session,
        "SETS" => Sets,
        "SHOW" => Show,
        "SIMILAR" => Similar,
        "SIMPLE" => Simple,
        "SIZE" => Size,
        "SMALLINT" => Smallint,
        "SNAPSHOT" => Snapshot,
        "SOURCE" => Source,
        "SPACE" => Space,
        "SPECIFIC" => Specific,
        "SQL" => Sql,
        "SQLCODE" => Sqlcode,
        "SQLERROR" => Sqlerror,
        "SQLEXCEPTION" => Sqlexception,
        "SQLSTATE" => Sqlstate,
        "SQLWARNING" => Sqlwarning,
        "START" => Start,
        "STATE" => State,
        "STATEMENT" => Statement,
        "STATIC" => Static,
        "STDDEV" => Stddev,
        "STDDEV_POP" => StddevPop,
        "STDDEV_SAMP" => StddevSamp,
        "STORAGE" => Storage,
        "STRING" => String,
        "SUBMULTISET" => Submultiset,
        "SUBSTRING" => Substring,
        "SUM" => Sum,
        "SYMMETRIC" => Symmetric,
        "SYSTEM" => System,
        "SYSTEM_TIME" => SystemTime,
        "SYSTEM_USER" => SystemUser,
        "TABLE" => Table,
        "TABLES" => Tables,
        "TARGET" => Target,
        "TEMP" => Temp,
        "TEMPORARY" => Temporary,
        "TEXT" => Text,
        "TIME" => Time,
        "TIMESTAMP" => Timestamp,
        "TIMEZONE" => Timezone,
        "TIMEZONE_HOUR" => TimezoneHour,
        "TIMEZONE_MINUTE" => TimezoneMinute,
        "TO_BIGINT" => ToBigint,
        "TO_DOUBLE" => ToDouble,
        "TO_INT64" => ToInt64,
        "TO_JSON" => ToJson,
        "TRANSACTION" => Transaction,
        "TRANSFORM" => Transform,
        "TRANSLATE" => Translate,
        "TRANSLATION" => Translation,
        "TRIGGER" => Trigger,
        "TRIM" => Trim,
        "TRUNCATE" => Truncate,
        "TRUSTED" => Trusted,
        "TYPE" => Type,
        "UINT32" => Uint32,
        "UINT64" => Uint64,
        "UNDER" => Under,
        "UNDO" => Undo,
        "UNIQUE" => Unique,
        "UNKNOWN" => Unknown,
        "UNPIVOT" => Unpivot,
        "UNSIGNED" => Unsigned,
        "UNTIL" => Until,
        "UPDATE" => Update,
        "USAGE" => Usage,
        "USER" => User,
        "UUID" => Uuid,
        "VALUE" => Value,
        "VALUES" => Values,
        "VARBINARY" => Varbinary,
        "VARCHAR" => Varchar,
        "VARIABLE" => Variable,
        "VARIABLES" => Variables,
        "VARIANCE" => Variance,
        "VAR_POP" => VarPop,
        "VAR_SAMP" => VarSamp,
        "VARYING" => Varying,
        "VIEW" => View,
        "VIRTUAL" => Virtual,
        "VOLATILE" => Volatile,
        "WEEK" => Week,
        "WEEKS" => Weeks,
        "WEIGHT" => Weight,
        "WHILE" => While,
        "WORK" => Work,
        "WRITE" => Write,
        "YEAR" => Year,
        "YEARS" => Years,
        "ZONE" => Zone,
    }
}

/// Macro to generate a static keyword map without external dependencies.
macro_rules! lazy_static_keyword_map {
    ($name:ident = { $($key:literal => $value:ident),* $(,)? }) => {
        #[allow(non_snake_case)]
        fn $name() -> &'static std::collections::HashMap<&'static str, Keyword> {
            use std::collections::HashMap;
            use std::sync::OnceLock;

            static MAP: OnceLock<HashMap<&'static str, Keyword>> = OnceLock::new();

            MAP.get_or_init(|| {
                let mut map = HashMap::new();
                $(
                    map.insert($key, Keyword::$value);
                )*
                map
            })
        }
    };
}

use lazy_static_keyword_map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_lookup() {
        assert_eq!(Keyword::parse("SELECT"), Some(Keyword::Select));
        assert_eq!(Keyword::parse("select"), Some(Keyword::Select));
        assert_eq!(Keyword::parse("SeLeCt"), Some(Keyword::Select));
        assert_eq!(Keyword::parse("not_a_keyword"), None);
    }

    #[test]
    fn test_reserved_keywords() {
        assert!(Keyword::Select.is_reserved());
        assert!(Keyword::From.is_reserved());
        assert!(Keyword::Where.is_reserved());
        assert!(!Keyword::Table.is_reserved());
        assert!(!Keyword::Column.is_reserved());
    }

    #[test]
    fn test_token_is_keyword() {
        let token = Token::new(
            TokenKind::Keyword(Keyword::Select),
            Span::new(0, 6),
            "SELECT",
        );
        assert!(token.is_keyword(Keyword::Select));
        assert!(!token.is_keyword(Keyword::From));
    }
}
