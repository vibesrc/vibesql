//! Catalog abstraction for schema metadata.
//!
//! This module provides traits that storage backends implement to provide
//! schema information for query analysis. The catalog is the bridge between
//! the SQL analyzer and the underlying storage system.
//!
//! # Extensibility
//!
//! Use the [`CatalogBuilder`] to create customized catalogs with:
//! - Custom functions (scalar, aggregate, window)
//! - Custom tables and schemas
//! - Custom type aliases via [`TypeRegistry`]
//!
//! ```
//! use vibesql::catalog::{CatalogBuilder, TypeRegistry};
//! use vibesql::types::SqlType;
//!
//! let catalog = CatalogBuilder::new()
//!     .with_builtins()
//!     .add_scalar_function("MY_HASH", SqlType::Int64)
//!     .add_table("users", |t| {
//!         t.primary_key("id", SqlType::Int64)
//!          .column("name", SqlType::Varchar)
//!     })
//!     .build();
//! ```

mod builder;
mod function;
mod schema;
mod type_registry;

pub use builder::*;
pub use function::*;
pub use schema::*;
pub use type_registry::*;

use crate::error::Result;
use crate::types::SqlType;

/// A catalog provides access to database schema information.
///
/// Storage backends implement this trait to provide table, column, and
/// function metadata to the query analyzer.
pub trait Catalog: Send + Sync {
    /// Resolve a table by name, returning its schema.
    ///
    /// The name parts are: `[catalog].[schema].table`
    fn resolve_table(&self, name: &[String]) -> Result<Option<TableSchema>>;

    /// Resolve a function by name.
    fn resolve_function(&self, name: &[String]) -> Result<Option<FunctionSignature>>;

    /// Get all tables in a schema.
    fn list_tables(&self, schema: Option<&str>) -> Result<Vec<String>>;

    /// Get all schemas in the catalog.
    fn list_schemas(&self) -> Result<Vec<String>>;

    /// Check if a table exists.
    fn table_exists(&self, name: &[String]) -> Result<bool> {
        Ok(self.resolve_table(name)?.is_some())
    }

    /// Get the default schema name.
    fn default_schema(&self) -> &str {
        "default"
    }
}

/// An in-memory catalog for testing and simple use cases.
#[derive(Debug, Default, Clone)]
pub struct MemoryCatalog {
    schemas: std::collections::HashMap<String, SchemaDefinition>,
    functions: std::collections::HashMap<String, FunctionSignature>,
}

/// A schema containing tables.
#[derive(Debug, Default, Clone)]
pub struct SchemaDefinition {
    pub name: String,
    pub tables: std::collections::HashMap<String, TableSchema>,
}

impl MemoryCatalog {
    /// Create a new empty catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a schema to the catalog.
    pub fn add_schema(&mut self, name: impl Into<String>) -> &mut SchemaDefinition {
        let name = name.into();
        self.schemas
            .entry(name.clone())
            .or_insert_with(|| SchemaDefinition {
                name,
                tables: std::collections::HashMap::new(),
            })
    }

    /// Add a table to the default schema.
    pub fn add_table(&mut self, table: TableSchema) {
        let schema = self.add_schema("default");
        schema.tables.insert(table.name.clone(), table);
    }

    /// Add a function to the catalog.
    pub fn add_function(&mut self, func: FunctionSignature) {
        self.functions.insert(func.name.clone(), func);
    }

    /// Register built-in functions.
    ///
    /// Note: Some aggregate functions like ARRAY_AGG use `Array<Any>` as return type.
    /// Proper polymorphic type inference (e.g., ARRAY_AGG on INT64 returns `ARRAY<INT64>`)
    /// requires the type checker to specialize return types based on input types at analysis time.
    pub fn register_builtins(&mut self) {
        // ===== AGGREGATE FUNCTIONS =====
        self.add_function(FunctionSignature::aggregate("COUNT", SqlType::Int64));
        self.add_function(FunctionSignature::aggregate("COUNTIF", SqlType::Int64));
        self.add_function(FunctionSignature::aggregate("SUM", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate("AVG", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate("MIN", SqlType::Any));
        self.add_function(FunctionSignature::aggregate("MAX", SqlType::Any));
        self.add_function(FunctionSignature::aggregate("ANY_VALUE", SqlType::Any));
        self.add_function(FunctionSignature::aggregate(
            "ARRAY_AGG",
            SqlType::Array(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::aggregate(
            "ARRAY_CONCAT_AGG",
            SqlType::Array(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::aggregate("STRING_AGG", SqlType::Varchar));

        // Bitwise aggregate functions
        self.add_function(FunctionSignature::aggregate("BIT_AND", SqlType::Int64));
        self.add_function(FunctionSignature::aggregate("BIT_OR", SqlType::Int64));
        self.add_function(FunctionSignature::aggregate("BIT_XOR", SqlType::Int64));

        // Logical aggregate functions
        self.add_function(FunctionSignature::aggregate("LOGICAL_AND", SqlType::Bool));
        self.add_function(FunctionSignature::aggregate("LOGICAL_OR", SqlType::Bool));

        // Grouping function (for ROLLUP/CUBE)
        self.add_function(FunctionSignature::aggregate("GROUPING", SqlType::Int64));

        // Statistical aggregate functions
        self.add_function(FunctionSignature::aggregate("STDDEV", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate("STDDEV_POP", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate(
            "STDDEV_SAMP",
            SqlType::Float64,
        ));
        self.add_function(FunctionSignature::aggregate("VARIANCE", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate("VAR_POP", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate("VAR_SAMP", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate("CORR", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate("COVAR_POP", SqlType::Float64));
        self.add_function(FunctionSignature::aggregate("COVAR_SAMP", SqlType::Float64));

        // ===== WINDOW FUNCTIONS =====
        self.add_function(FunctionSignature::window("ROW_NUMBER", SqlType::Int64));
        self.add_function(FunctionSignature::window("RANK", SqlType::Int64));
        self.add_function(FunctionSignature::window("DENSE_RANK", SqlType::Int64));
        self.add_function(FunctionSignature::window("NTILE", SqlType::Int64));
        self.add_function(FunctionSignature::window("LAG", SqlType::Any));
        self.add_function(FunctionSignature::window("LEAD", SqlType::Any));
        self.add_function(FunctionSignature::window("FIRST_VALUE", SqlType::Any));
        self.add_function(FunctionSignature::window("LAST_VALUE", SqlType::Any));
        self.add_function(FunctionSignature::window("NTH_VALUE", SqlType::Any));
        self.add_function(FunctionSignature::window("CUME_DIST", SqlType::Float64));
        self.add_function(FunctionSignature::window("PERCENT_RANK", SqlType::Float64));
        self.add_function(FunctionSignature::window(
            "PERCENTILE_CONT",
            SqlType::Float64,
        ));
        self.add_function(FunctionSignature::window("PERCENTILE_DISC", SqlType::Any));

        // ===== STRING FUNCTIONS =====
        self.add_function(FunctionSignature::scalar("CONCAT", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("LENGTH", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("CHAR_LENGTH", SqlType::Int64));
        self.add_function(FunctionSignature::scalar(
            "CHARACTER_LENGTH",
            SqlType::Int64,
        ));
        self.add_function(FunctionSignature::scalar("BYTE_LENGTH", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("UPPER", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("LOWER", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("TRIM", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("LTRIM", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("RTRIM", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("LPAD", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("RPAD", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("SUBSTR", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("SUBSTRING", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("LEFT", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("RIGHT", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("REPLACE", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("REVERSE", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("REPEAT", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar(
            "SPLIT",
            SqlType::Array(Box::new(SqlType::Varchar)),
        ));
        self.add_function(FunctionSignature::scalar("STRPOS", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("INSTR", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("STARTS_WITH", SqlType::Bool));
        self.add_function(FunctionSignature::scalar("ENDS_WITH", SqlType::Bool));
        self.add_function(FunctionSignature::scalar("CONTAINS_SUBSTR", SqlType::Bool));
        self.add_function(FunctionSignature::scalar("REGEXP_CONTAINS", SqlType::Bool));
        self.add_function(FunctionSignature::scalar(
            "REGEXP_EXTRACT",
            SqlType::Varchar,
        ));
        self.add_function(FunctionSignature::scalar(
            "REGEXP_EXTRACT_ALL",
            SqlType::Array(Box::new(SqlType::Varchar)),
        ));
        self.add_function(FunctionSignature::scalar(
            "REGEXP_REPLACE",
            SqlType::Varchar,
        ));
        self.add_function(FunctionSignature::scalar("REGEXP_INSTR", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("FORMAT", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("NORMALIZE", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar(
            "NORMALIZE_AND_CASEFOLD",
            SqlType::Varchar,
        ));
        self.add_function(FunctionSignature::scalar("TO_BASE32", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("TO_BASE64", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("FROM_BASE32", SqlType::Varbinary));
        self.add_function(FunctionSignature::scalar("FROM_BASE64", SqlType::Varbinary));
        self.add_function(FunctionSignature::scalar("TO_HEX", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("FROM_HEX", SqlType::Varbinary));
        self.add_function(FunctionSignature::scalar("ASCII", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("CHR", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("UNICODE", SqlType::Int64));
        self.add_function(FunctionSignature::scalar(
            "TO_CODE_POINTS",
            SqlType::Array(Box::new(SqlType::Int64)),
        ));
        self.add_function(FunctionSignature::scalar(
            "CODE_POINTS_TO_STRING",
            SqlType::Varchar,
        ));
        self.add_function(FunctionSignature::scalar(
            "CODE_POINTS_TO_BYTES",
            SqlType::Varbinary,
        ));
        self.add_function(FunctionSignature::scalar("SOUNDEX", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("TRANSLATE", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("INITCAP", SqlType::Varchar));

        // ===== MATH FUNCTIONS =====
        self.add_function(FunctionSignature::scalar("ABS", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("SIGN", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("CEIL", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("CEILING", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("FLOOR", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("ROUND", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("TRUNC", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("TRUNCATE", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("DIV", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("MOD", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("SQRT", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("CBRT", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("POW", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("POWER", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("EXP", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("LN", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("LOG", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("LOG10", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("LOG2", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("GREATEST", SqlType::Any));
        self.add_function(FunctionSignature::scalar("LEAST", SqlType::Any));

        // Trigonometric functions
        self.add_function(FunctionSignature::scalar("SIN", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("COS", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("TAN", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("ASIN", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("ACOS", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("ATAN", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("ATAN2", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("SINH", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("COSH", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("TANH", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("ASINH", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("ACOSH", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("ATANH", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("COT", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("CSC", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("SEC", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("COTH", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("CSCH", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("SECH", SqlType::Float64));

        // IEEE floating point functions
        self.add_function(FunctionSignature::scalar("IEEE_DIVIDE", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("IS_INF", SqlType::Bool));
        self.add_function(FunctionSignature::scalar("IS_NAN", SqlType::Bool));

        // Random and range
        self.add_function(FunctionSignature::scalar("RAND", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("RANDOM", SqlType::Float64));
        self.add_function(FunctionSignature::scalar("RANGE_BUCKET", SqlType::Int64));

        // Bitwise functions
        self.add_function(FunctionSignature::scalar("BIT_COUNT", SqlType::Int64));

        // ===== DATE/TIME FUNCTIONS =====
        self.add_function(FunctionSignature::scalar("CURRENT_DATE", SqlType::Date));
        self.add_function(FunctionSignature::scalar("CURRENT_TIME", SqlType::Time));
        self.add_function(FunctionSignature::scalar(
            "CURRENT_DATETIME",
            SqlType::Datetime,
        ));
        self.add_function(FunctionSignature::scalar(
            "CURRENT_TIMESTAMP",
            SqlType::Timestamp,
        ));

        // Date functions
        self.add_function(FunctionSignature::scalar("DATE", SqlType::Date));
        self.add_function(FunctionSignature::scalar("DATE_ADD", SqlType::Date));
        self.add_function(FunctionSignature::scalar("DATE_SUB", SqlType::Date));
        self.add_function(FunctionSignature::scalar("DATE_DIFF", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("DATE_TRUNC", SqlType::Date));
        self.add_function(FunctionSignature::scalar(
            "DATE_FROM_UNIX_DATE",
            SqlType::Date,
        ));
        self.add_function(FunctionSignature::scalar("FORMAT_DATE", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("PARSE_DATE", SqlType::Date));
        self.add_function(FunctionSignature::scalar("UNIX_DATE", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("LAST_DAY", SqlType::Date));

        // Time functions
        self.add_function(FunctionSignature::scalar("TIME", SqlType::Time));
        self.add_function(FunctionSignature::scalar("TIME_ADD", SqlType::Time));
        self.add_function(FunctionSignature::scalar("TIME_SUB", SqlType::Time));
        self.add_function(FunctionSignature::scalar("TIME_DIFF", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("TIME_TRUNC", SqlType::Time));
        self.add_function(FunctionSignature::scalar("FORMAT_TIME", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar("PARSE_TIME", SqlType::Time));

        // Datetime functions
        self.add_function(FunctionSignature::scalar("DATETIME", SqlType::Datetime));
        self.add_function(FunctionSignature::scalar("DATETIME_ADD", SqlType::Datetime));
        self.add_function(FunctionSignature::scalar("DATETIME_SUB", SqlType::Datetime));
        self.add_function(FunctionSignature::scalar("DATETIME_DIFF", SqlType::Int64));
        self.add_function(FunctionSignature::scalar(
            "DATETIME_TRUNC",
            SqlType::Datetime,
        ));
        self.add_function(FunctionSignature::scalar(
            "FORMAT_DATETIME",
            SqlType::Varchar,
        ));
        self.add_function(FunctionSignature::scalar(
            "PARSE_DATETIME",
            SqlType::Datetime,
        ));

        // Timestamp functions
        self.add_function(FunctionSignature::scalar("TIMESTAMP", SqlType::Timestamp));
        self.add_function(FunctionSignature::scalar(
            "TIMESTAMP_ADD",
            SqlType::Timestamp,
        ));
        self.add_function(FunctionSignature::scalar(
            "TIMESTAMP_SUB",
            SqlType::Timestamp,
        ));
        self.add_function(FunctionSignature::scalar("TIMESTAMP_DIFF", SqlType::Int64));
        self.add_function(FunctionSignature::scalar(
            "TIMESTAMP_TRUNC",
            SqlType::Timestamp,
        ));
        self.add_function(FunctionSignature::scalar(
            "FORMAT_TIMESTAMP",
            SqlType::Varchar,
        ));
        self.add_function(FunctionSignature::scalar(
            "PARSE_TIMESTAMP",
            SqlType::Timestamp,
        ));
        self.add_function(FunctionSignature::scalar(
            "TIMESTAMP_SECONDS",
            SqlType::Timestamp,
        ));
        self.add_function(FunctionSignature::scalar(
            "TIMESTAMP_MILLIS",
            SqlType::Timestamp,
        ));
        self.add_function(FunctionSignature::scalar(
            "TIMESTAMP_MICROS",
            SqlType::Timestamp,
        ));
        self.add_function(FunctionSignature::scalar("UNIX_SECONDS", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("UNIX_MILLIS", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("UNIX_MICROS", SqlType::Int64));
        self.add_function(FunctionSignature::scalar("STRING", SqlType::Varchar));

        // Interval functions
        self.add_function(FunctionSignature::scalar(
            "MAKE_INTERVAL",
            SqlType::Interval,
        ));
        self.add_function(FunctionSignature::scalar("JUSTIFY_DAYS", SqlType::Interval));
        self.add_function(FunctionSignature::scalar(
            "JUSTIFY_HOURS",
            SqlType::Interval,
        ));
        self.add_function(FunctionSignature::scalar(
            "JUSTIFY_INTERVAL",
            SqlType::Interval,
        ));

        // Extract (handled specially but good to have)
        self.add_function(FunctionSignature::scalar("EXTRACT", SqlType::Int64));

        // Time series functions
        self.add_function(FunctionSignature::scalar("DATE_BUCKET", SqlType::Date));
        self.add_function(FunctionSignature::scalar(
            "DATETIME_BUCKET",
            SqlType::Datetime,
        ));
        self.add_function(FunctionSignature::scalar(
            "TIMESTAMP_BUCKET",
            SqlType::Timestamp,
        ));

        // ===== TYPE CONVERSION =====
        self.add_function(FunctionSignature::scalar("CAST", SqlType::Any));
        self.add_function(FunctionSignature::scalar("TRY_CAST", SqlType::Any));
        self.add_function(FunctionSignature::scalar(
            "PARSE_NUMERIC",
            SqlType::Numeric {
                precision: None,
                scale: None,
            },
        ));

        // ===== CONDITIONAL FUNCTIONS =====
        self.add_function(FunctionSignature::scalar("IF", SqlType::Any));
        self.add_function(FunctionSignature::scalar("IFNULL", SqlType::Any));
        self.add_function(FunctionSignature::scalar("NULLIF", SqlType::Any));
        self.add_function(FunctionSignature::scalar("COALESCE", SqlType::Any));
        self.add_function(FunctionSignature::scalar("NVL", SqlType::Any));
        self.add_function(FunctionSignature::scalar("ZeroIfNull", SqlType::Any));

        // ===== ARRAY FUNCTIONS =====
        self.add_function(FunctionSignature::scalar("ARRAY_LENGTH", SqlType::Int64));
        self.add_function(FunctionSignature::scalar(
            "ARRAY_TO_STRING",
            SqlType::Varchar,
        ));
        self.add_function(FunctionSignature::scalar(
            "ARRAY_CONCAT",
            SqlType::Array(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::scalar(
            "ARRAY_REVERSE",
            SqlType::Array(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::scalar(
            "ARRAY_FILTER",
            SqlType::Array(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::scalar(
            "ARRAY_TRANSFORM",
            SqlType::Array(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::scalar(
            "ARRAY_SLICE",
            SqlType::Array(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::scalar("ARRAY_FIRST", SqlType::Any));
        self.add_function(FunctionSignature::scalar("ARRAY_LAST", SqlType::Any));
        self.add_function(FunctionSignature::scalar("ARRAY_INCLUDES", SqlType::Bool));
        self.add_function(FunctionSignature::scalar(
            "ARRAY_INCLUDES_ANY",
            SqlType::Bool,
        ));
        self.add_function(FunctionSignature::scalar(
            "ARRAY_INCLUDES_ALL",
            SqlType::Bool,
        ));
        self.add_function(FunctionSignature::scalar(
            "GENERATE_ARRAY",
            SqlType::Array(Box::new(SqlType::Int64)),
        ));
        self.add_function(FunctionSignature::scalar(
            "GENERATE_DATE_ARRAY",
            SqlType::Array(Box::new(SqlType::Date)),
        ));
        self.add_function(FunctionSignature::scalar(
            "GENERATE_TIMESTAMP_ARRAY",
            SqlType::Array(Box::new(SqlType::Timestamp)),
        ));
        self.add_function(FunctionSignature::scalar(
            "FLATTEN",
            SqlType::Array(Box::new(SqlType::Any)),
        ));

        // ===== JSON FUNCTIONS =====
        // Standard extractors
        self.add_function(FunctionSignature::scalar("JSON_QUERY", SqlType::Json));
        self.add_function(FunctionSignature::scalar("JSON_VALUE", SqlType::Varchar));
        self.add_function(FunctionSignature::scalar(
            "JSON_QUERY_ARRAY",
            SqlType::Array(Box::new(SqlType::Json)),
        ));
        self.add_function(FunctionSignature::scalar(
            "JSON_VALUE_ARRAY",
            SqlType::Array(Box::new(SqlType::Varchar)),
        ));

        // JSON constructors
        self.add_function(FunctionSignature::scalar("JSON_ARRAY", SqlType::Json));
        self.add_function(FunctionSignature::scalar("JSON_OBJECT", SqlType::Json));

        // JSON mutators
        self.add_function(FunctionSignature::scalar("JSON_SET", SqlType::Json));
        self.add_function(FunctionSignature::scalar("JSON_REMOVE", SqlType::Json));
        self.add_function(FunctionSignature::scalar(
            "JSON_ARRAY_APPEND",
            SqlType::Json,
        ));
        self.add_function(FunctionSignature::scalar(
            "JSON_ARRAY_INSERT",
            SqlType::Json,
        ));
        self.add_function(FunctionSignature::scalar("JSON_STRIP_NULLS", SqlType::Json));

        // JSON other
        self.add_function(FunctionSignature::scalar("PARSE_JSON", SqlType::Json));
        self.add_function(FunctionSignature::scalar("TO_JSON", SqlType::Json));
        self.add_function(FunctionSignature::scalar(
            "TO_JSON_STRING",
            SqlType::Varchar,
        ));
        self.add_function(FunctionSignature::scalar("JSON_TYPE", SqlType::Varchar));

        // ===== RANGE FUNCTIONS =====
        self.add_function(FunctionSignature::scalar(
            "RANGE",
            SqlType::Range(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::scalar("RANGE_START", SqlType::Any));
        self.add_function(FunctionSignature::scalar("RANGE_END", SqlType::Any));
        self.add_function(FunctionSignature::scalar("RANGE_CONTAINS", SqlType::Bool));
        self.add_function(FunctionSignature::scalar("RANGE_OVERLAPS", SqlType::Bool));
        self.add_function(FunctionSignature::scalar(
            "RANGE_INTERSECT",
            SqlType::Range(Box::new(SqlType::Any)),
        ));
        self.add_function(FunctionSignature::scalar(
            "GENERATE_RANGE_ARRAY",
            SqlType::Array(Box::new(SqlType::Range(Box::new(SqlType::Any)))),
        ));

        // ===== HASH FUNCTIONS =====
        self.add_function(FunctionSignature::scalar("MD5", SqlType::Varbinary));
        self.add_function(FunctionSignature::scalar("SHA1", SqlType::Varbinary));
        self.add_function(FunctionSignature::scalar("SHA256", SqlType::Varbinary));
        self.add_function(FunctionSignature::scalar("SHA512", SqlType::Varbinary));

        // ===== UUID FUNCTIONS =====
        self.add_function(FunctionSignature::scalar("GENERATE_UUID", SqlType::Uuid));

        // ===== ERROR HANDLING =====
        self.add_function(FunctionSignature::scalar("ERROR", SqlType::Unknown));
        self.add_function(FunctionSignature::scalar("IFERROR", SqlType::Any));
        self.add_function(FunctionSignature::scalar("ISERROR", SqlType::Bool));
    }
}

impl Catalog for MemoryCatalog {
    fn resolve_table(&self, name: &[String]) -> Result<Option<TableSchema>> {
        let (schema_name, table_name) = match name.len() {
            1 => ("default", name[0].as_str()),
            2 => (name[0].as_str(), name[1].as_str()),
            3 => (name[1].as_str(), name[2].as_str()), // Ignore catalog for now
            _ => return Ok(None),
        };

        Ok(self
            .schemas
            .get(schema_name)
            .and_then(|s| s.tables.get(table_name))
            .cloned())
    }

    fn resolve_function(&self, name: &[String]) -> Result<Option<FunctionSignature>> {
        // For now, just use the last part (function name)
        let func_name = name.last().map(|s| s.to_uppercase()).unwrap_or_default();
        Ok(self.functions.get(&func_name).cloned())
    }

    fn list_tables(&self, schema: Option<&str>) -> Result<Vec<String>> {
        let schema_name = schema.unwrap_or("default");
        Ok(self
            .schemas
            .get(schema_name)
            .map(|s| s.tables.keys().cloned().collect())
            .unwrap_or_default())
    }

    fn list_schemas(&self) -> Result<Vec<String>> {
        Ok(self.schemas.keys().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_catalog() {
        let mut catalog = MemoryCatalog::new();

        catalog.add_table(TableSchema {
            name: "users".to_string(),
            columns: vec![
                ColumnSchema::new("id", SqlType::Int64).not_null(),
                ColumnSchema::new("name", SqlType::Varchar),
                ColumnSchema::new("email", SqlType::Varchar),
            ],
        });

        let table = catalog.resolve_table(&["users".to_string()]).unwrap();
        assert!(table.is_some());

        let table = table.unwrap();
        assert_eq!(table.name, "users");
        assert_eq!(table.columns.len(), 3);
    }

    #[test]
    fn test_builtin_functions() {
        let mut catalog = MemoryCatalog::new();
        catalog.register_builtins();

        let count = catalog.resolve_function(&["COUNT".to_string()]).unwrap();
        assert!(count.is_some());
        assert!(count.unwrap().is_aggregate);
    }
}
