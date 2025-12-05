//! # VibeSQL
//!
//! A SQL parser and semantic analyzer library with standard SQL type system.
//!
//! This library provides a complete SQL parsing and analysis pipeline that can be used
//! as a middleware layer between transport and storage layers in database systems.
//!
//! ## Features
//!
//! - Full SQL lexer/tokenizer
//! - Comprehensive AST representation
//! - Expression, query, and statement parsing
//! - Standard SQL type system (BIGINT, VARCHAR, DOUBLE PRECISION, etc.)
//! - Semantic analysis with type checking
//! - Extensible catalog with custom types, functions, and tables
//! - Zero dependencies (standard library only)
//!
//! ## Quick Start
//!
//! ```rust
//! use vibesql::{Parser, Analyzer};
//!
//! let sql = "SELECT id, name FROM users WHERE age > 21";
//! let mut parser = Parser::new(sql);
//! let ast = parser.parse().expect("Failed to parse SQL");
//! ```
//!
//! ## Extensibility
//!
//! Use [`CatalogBuilder`] to create catalogs with custom functions and types:
//!
//! ```rust
//! use vibesql::catalog::{CatalogBuilder, TypeRegistry};
//! use vibesql::types::SqlType;
//!
//! let catalog = CatalogBuilder::new()
//!     .with_builtins()
//!     .add_scalar_function("MY_HASH", SqlType::Int64)
//!     .add_table("users", |t| {
//!         t.primary_key("id", SqlType::Int64)
//!          .column("name", SqlType::Varchar)
//!          .column("email", SqlType::Varchar)
//!     })
//!     .build();
//! ```
//!
//! ## Type System
//!
//! VibeSQL uses standard SQL type names:
//!
//! | Type | Display Name |
//! |------|--------------|
//! | `SqlType::Bool` | BOOLEAN |
//! | `SqlType::Int32` | INTEGER |
//! | `SqlType::Int64` | BIGINT |
//! | `SqlType::Float32` | REAL |
//! | `SqlType::Float64` | DOUBLE PRECISION |
//! | `SqlType::Varchar` | VARCHAR |
//! | `SqlType::Varbinary` | VARBINARY |

pub mod analyzer;
pub mod ast;
pub mod catalog;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod types;

// Re-export main types for convenience
pub use analyzer::{AnalyzedQuery, Analyzer, AnalyzerError, OutputColumn};
pub use ast::*;
pub use catalog::{
    Catalog, CatalogBuilder, ColumnSchema, FunctionSignature, MemoryCatalog, TableBuilder,
    TableSchema, TableSchemaBuilder, TypeRegistry,
};
pub use error::{Error, Result};
pub use lexer::{Lexer, Token, TokenKind};
pub use parser::Parser;
pub use types::{SqlType, Value};
