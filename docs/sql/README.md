# SQL Reference

SQL language specification for vibesql parser conformance. Based on ISO/IEC 9075 (SQL standard) with common extensions.

## Sections

| Section                                  | Description                                                |
| ---------------------------------------- | ---------------------------------------------------------- |
| [Syntax](syntax/README.md)               | Lexical structure, query syntax, operators                 |
| [Types](types/README.md)                 | Data types, arrays, conversion rules                       |
| [Statements](statements/README.md)       | DDL (CREATE, ALTER, DROP) and DML (INSERT, UPDATE, DELETE) |
| [Functions](functions/README.md)         | Scalar, aggregate, and window functions                    |

## Quick Links

- [Query Syntax](syntax/query_syntax.md) - SELECT statement reference
- [Data Types](types/data_types.md) - All supported types
- [Operators](syntax/operators.md) - Operator precedence
- [Functions Reference](functions/functions_reference.md) - Function call syntax

## SQL Standard Conformance

This specification aims to conform to:

- **SQL:2016** (ISO/IEC 9075:2016) - Core features
- **SQL:2011** - Window functions, CTEs
- **SQL:2003** - ARRAY types, enhanced datetime

Common extensions from PostgreSQL and other databases are noted where applicable.

## License

[Apache License 2.0](../../LICENSE)

## Attribution

This documentation is derived from [ZetaSQL](https://github.com/google/zetasql), Copyright Google LLC, licensed under the Apache License 2.0. The original documentation has been modified to use standard SQL terminology and reorganized for vibesql.
