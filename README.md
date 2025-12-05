# vibesql

A SQL parser and semantic analyzer written in Rust, conforming to modern ISO SQL standards.

## Overview

vibesql provides a complete SQL parsing and analysis pipeline that can be used as a middleware layer between transport and storage layers in database systems. It parses SQL into a rich AST, performs semantic analysis with type checking, and provides a catalog system for schema management.

## Features

- **Full SQL Lexer** - Comprehensive tokenizer with support for SQL keywords, operators, and literals
- **Complete AST** - Rich abstract syntax tree for expressions, queries, and statements
- **Semantic Analyzer** - Type checking, name resolution, and query validation
- **Catalog System** - Schema management with tables, columns, and function signatures
- **Zero Dependencies** - Pure Rust implementation using only the standard library

### Supported SQL

**Queries:**
- SELECT with projections, aliases, and DISTINCT
- FROM with table references and aliases
- JOINs (INNER, LEFT, RIGHT, FULL, CROSS, NATURAL)
- WHERE, GROUP BY, HAVING, ORDER BY, LIMIT, OFFSET
- Window functions (OVER, PARTITION BY, ORDER BY, frame clauses)
- Subqueries and CTEs (WITH clause)
- Set operations (UNION, INTERSECT, EXCEPT)

**DML:**
- INSERT (VALUES and SELECT)
- UPDATE with SET assignments
- DELETE with WHERE
- MERGE (WHEN MATCHED/NOT MATCHED)

**DDL:**
- CREATE/ALTER/DROP TABLE
- CREATE/ALTER/DROP VIEW
- CREATE/DROP INDEX
- CREATE/DROP FUNCTION

**Expressions:**
- Arithmetic, comparison, and logical operators
- CASE expressions
- CAST and type conversions
- Array and struct constructors
- Function calls (scalar and aggregate)
- BETWEEN, IN, LIKE, IS NULL

**Types:**
- Numeric: INTEGER, BIGINT, NUMERIC, REAL, DOUBLE PRECISION
- String: VARCHAR, TEXT, BINARY, BLOB
- Temporal: DATE, TIME, TIMESTAMP, INTERVAL
- Complex: ARRAY, ROW/STRUCT, JSON, UUID

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
vibesql = "0.1"
```

## Usage

### Basic Parsing

```rust
use vibesql::Parser;

let sql = "SELECT id, name FROM users WHERE age > 21";
let mut parser = Parser::new(sql);
let statements = parser.parse().expect("Failed to parse SQL");

for stmt in statements {
    println!("{:#?}", stmt);
}
```

### With Semantic Analysis

```rust
use vibesql::{Parser, Analyzer, CatalogBuilder, SqlType};

// Build a catalog with tables and built-in functions
let catalog = CatalogBuilder::new()
    .with_builtins()
    .add_table("users", |t| {
        t.primary_key("id", SqlType::Int64)
         .column("name", SqlType::Varchar)
         .column("age", SqlType::Int64)
    })
    .build();

// Parse and analyze
let sql = "SELECT id, name FROM users WHERE age > 21";
let mut parser = Parser::new(sql);
let statements = parser.parse().expect("Failed to parse");

let mut analyzer = Analyzer::with_catalog(catalog);
// Analyze queries for type information, column resolution, etc.
```

### Custom Functions and Types

```rust
use vibesql::catalog::{CatalogBuilder, TypeRegistry};
use vibesql::types::SqlType;

// Add custom functions
let catalog = CatalogBuilder::new()
    .with_builtins()
    .add_scalar_function("MY_HASH", SqlType::Int64)
    .add_aggregate_function("CUSTOM_SUM", SqlType::Float64)
    .add_window_function("CUSTOM_RANK", SqlType::Int64)
    .build();

// Add custom type aliases
let mut registry = TypeRegistry::new();
registry.add_alias("SERIAL", SqlType::Int32);
registry.add_alias("MONEY", SqlType::Numeric { precision: Some(19), scale: Some(4) });
```

### Error Handling

```rust
use vibesql::{Parser, Error};

let sql = "SELECT * FORM users";  // Typo: FORM instead of FROM
let mut parser = Parser::new(sql);

match parser.parse() {
    Ok(statements) => println!("Parsed {} statements", statements.len()),
    Err(e) => {
        eprintln!("Parse error: {}", e);
        if let Some(span) = e.span() {
            eprintln!("At position: {:?}", span);
        }
    }
}
```

## Examples

### CSV Database

The `csv_database` example demonstrates using vibesql as a SQL frontend for a simple CSV-backed database:

```bash
# Seed with sample relational data
cargo run --example csv_database seed

# Run queries
cargo run --example csv_database -c "SELECT * FROM employees LIMIT 5"

# Join tables
cargo run --example csv_database -c "SELECT e.name, d.name as department
    FROM employees e
    JOIN departments d ON e.department_id = d.id"

# Aggregations
cargo run --example csv_database -c "SELECT d.name, COUNT(*), AVG(e.salary)
    FROM employees e
    JOIN departments d ON e.department_id = d.id
    GROUP BY d.name"

# Export to CSV
cargo run --example csv_database --csv -c "SELECT * FROM employees" -o employees.csv
```

## Architecture

```
vibesql
├── lexer/      # Tokenizer (keywords, operators, literals)
├── parser/     # SQL parser (expressions, queries, statements)
├── ast/        # Abstract syntax tree definitions
├── analyzer/   # Semantic analysis and type checking
├── catalog/    # Schema management (tables, functions, type registry)
├── types/      # SQL type system
└── error/      # Error types and reporting
```

## License

MIT
