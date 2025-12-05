# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build
cargo build

# Run tests
cargo test

# Run a single test
cargo test test_name

# Run tests in a specific module
cargo test parser::tests

# Run the CLI
cargo run -- "SELECT * FROM users"

# Run the REPL
cargo run

# Run the CSV database example
cargo run --example csv_database seed                    # Seed sample data
cargo run --example csv_database -c "SELECT * FROM employees LIMIT 5"
```

## Architecture

vibesql is a SQL parser and semantic analyzer conforming to ISO SQL standards with zero dependencies (standard library only).

### Pipeline Flow

```sql
SQL text → Lexer → Tokens → Parser → AST → Analyzer → Typed/Validated AST
                                              ↑
                                           Catalog (schema metadata)
```

### Module Structure

- **lexer/** - Tokenizer that produces `Token` with `TokenKind` (keywords, operators, literals). Keywords defined in `token.rs` with reserved/non-reserved distinction.

- **parser/** - Recursive descent parser split by SQL construct:
  - `expr.rs` - Expression parsing (operators, functions, CASE, etc.)
  - `query.rs` - SELECT, FROM, JOIN, GROUP BY, HAVING, ORDER BY, window functions
  - `stmt.rs` - DDL/DML statements (CREATE, INSERT, UPDATE, DELETE, MERGE)

- **ast/** - AST node definitions:
  - `expr.rs` - Expression nodes (`ExprKind` enum)
  - `stmt.rs` - Statement nodes (`StatementKind` enum)
  - `types.rs` - Common AST types (Ident, ObjectName, Span)

- **analyzer/** - Semantic analysis:
  - `mod.rs` - Main `Analyzer` struct, query/statement analysis
  - `scope.rs` - Name resolution scopes (`Scope`, `ScopeTable`, `ScopeColumn`)
  - `type_checker.rs` - Expression type inference
  - `error.rs` - Analyzer-specific errors

- **catalog/** - Schema metadata abstraction:
  - `Catalog` trait - Interface for storage backends
  - `MemoryCatalog` - In-memory implementation with builtin functions
  - `CatalogBuilder` - Fluent API for building custom catalogs
  - `TypeRegistry` - Type alias management
  - `TableSchema`, `ColumnSchema` - Table/column definitions
  - `FunctionSignature` - Function metadata (scalar/aggregate/window)

- **types/** - SQL type system:
  - `SqlType` enum - All SQL types (INTEGER, BIGINT, VARCHAR, ARRAY, STRUCT, etc.)
  - `Value` - Runtime values

### Extensibility

Use `CatalogBuilder` to create catalogs with custom functions, tables, and type aliases:

```rust
use vibesql::catalog::{CatalogBuilder, TypeRegistry};
use vibesql::types::SqlType;

let catalog = CatalogBuilder::new()
    .with_builtins()                                      // Include standard functions
    .add_scalar_function("MY_HASH", SqlType::Int64)       // Custom function
    .add_aggregate_function("MY_AGG", SqlType::Float64)   // Custom aggregate
    .add_table("users", |t| {                             // Custom table
        t.primary_key("id", SqlType::Int64)
         .column("name", SqlType::Varchar)
    })
    .build();
```

Use `TypeRegistry` for custom type aliases:

```rust
let mut registry = TypeRegistry::new();
registry.add_alias("SERIAL", SqlType::Int32);
registry.add_alias("MONEY", SqlType::Numeric { precision: Some(19), scale: Some(4) });
```

### Key Design Patterns

1. **Parser utilities** in `parser/mod.rs`: `parse_comma_separated`, `parse_optional_alias`, `expect_keyword` - reused across all parsing modules.

2. **Scope stack** in analyzer: Pushed for subqueries/CTEs, enables proper name resolution.

3. **Catalog trait**: Storage backends implement this to provide schema info to the analyzer.

### Reference Documentation

The `docs/sql/` directory contains SQL language specification based on ISO/IEC 9075 (SQL standard):

- `syntax/` - Lexical structure, query syntax, operators, subqueries
- `types/` - Data types, arrays, conversion rules, collation
- `statements/` - DDL (CREATE/ALTER/DROP) and DML (INSERT/UPDATE/DELETE)
- `functions/` - Scalar, aggregate, window, and time series functions

Each directory has a README.md index.
