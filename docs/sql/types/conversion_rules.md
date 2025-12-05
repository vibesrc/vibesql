# Conversion Rules

SQL supports type conversion through casting, coercion, and supertyping.

- **Casting** is explicit conversion using the `CAST()` function
- **Coercion** is implicit conversion performed automatically
- **Supertype** is a common type to which multiple expressions can be coerced

## Casting and Coercion Matrix

| From Type          | Can Cast To                       | Implicit Coercion To                                       |
| ------------------ | --------------------------------- | ---------------------------------------------------------- |
| `SMALLINT`         | All numeric, `VARCHAR`, `BOOLEAN` | `INTEGER`, `BIGINT`, `NUMERIC`, `REAL`, `DOUBLE PRECISION` |
| `INTEGER`          | All numeric, `VARCHAR`, `BOOLEAN` | `BIGINT`, `NUMERIC`, `REAL`, `DOUBLE PRECISION`            |
| `BIGINT`           | All numeric, `VARCHAR`, `BOOLEAN` | `NUMERIC`, `DOUBLE PRECISION`                              |
| `NUMERIC`          | All numeric, `VARCHAR`            | `DOUBLE PRECISION`                                         |
| `REAL`             | All numeric, `VARCHAR`            | `DOUBLE PRECISION`                                         |
| `DOUBLE PRECISION` | All numeric, `VARCHAR`            | —                                                          |
| `BOOLEAN`          | `INTEGER`, `BIGINT`, `VARCHAR`    | —                                                          |
| `VARCHAR`          | All types                         | —                                                          |
| `DATE`             | `VARCHAR`, `TIMESTAMP`            | `TIMESTAMP`                                                |
| `TIME`             | `VARCHAR`                         | —                                                          |
| `TIMESTAMP`        | `VARCHAR`, `DATE`, `TIME`         | —                                                          |
| `INTERVAL`         | `VARCHAR`                         | —                                                          |
| `ARRAY`            | `ARRAY` (element coercion)        | —                                                          |
| `ROW`/`STRUCT`     | `ROW` (field coercion)            | —                                                          |
| `JSON`             | `VARCHAR`                         | —                                                          |
| `UUID`             | `VARCHAR`, `VARBINARY`            | —                                                          |

## Casting

Use `CAST(expression AS type)` for explicit conversion:

```sql
CAST(123 AS VARCHAR)           -- '123'
CAST('2024-01-15' AS DATE)     -- DATE value
CAST(3.14159 AS INTEGER)       -- 3 (truncated)
CAST('true' AS BOOLEAN)        -- TRUE
```

### Casting Errors

Invalid casts produce errors:

```sql
CAST('hello' AS INTEGER)       -- Error: invalid input
CAST(NULL AS INTEGER)          -- NULL (valid)
```

## Implicit Coercion

Coercion occurs automatically in these contexts:

### Function Arguments

When a function expects `DOUBLE PRECISION` but receives `INTEGER`:

```sql
SQRT(4)  -- 4 (INTEGER) is coerced to DOUBLE PRECISION
```

### Comparison Operations

```sql
SELECT * FROM t WHERE int_col = 3.14  -- int_col coerced to DOUBLE PRECISION
```

### Set Operations

```sql
SELECT int_col FROM t1
UNION ALL
SELECT bigint_col FROM t2  -- Result type is BIGINT
```

## Literal Coercion

String literals coerce to temporal types when context requires:

| Literal Type    | Coerces To                                 |
| --------------- | ------------------------------------------ |
| String literal  | `DATE`, `TIME`, `TIMESTAMP`, `INTERVAL`    |
| Integer literal | `SMALLINT`, `INTEGER`, `BIGINT`, `NUMERIC` |
| Decimal literal | `NUMERIC`, `REAL`, `DOUBLE PRECISION`      |

```sql
-- String literal coerced to DATE
SELECT * FROM t WHERE date_col > '2024-01-01'

-- Integer literal fits smallest type
SELECT 42         -- INTEGER
SELECT 3000000000 -- BIGINT (too large for INTEGER)
```

## Supertypes

The supertype is the common type for combining expressions:

| Input Types                   | Supertype          |
| ----------------------------- | ------------------ |
| `SMALLINT`, `INTEGER`         | `INTEGER`          |
| `INTEGER`, `BIGINT`           | `BIGINT`           |
| `INTEGER`, `REAL`             | `REAL`             |
| `INTEGER`, `DOUBLE PRECISION` | `DOUBLE PRECISION` |
| `BIGINT`, `DOUBLE PRECISION`  | `DOUBLE PRECISION` |
| `NUMERIC`, `DOUBLE PRECISION` | `DOUBLE PRECISION` |
| `DATE`, `TIMESTAMP`           | `TIMESTAMP`        |

### Exact vs Inexact Types

- **Exact types**: `SMALLINT`, `INTEGER`, `BIGINT`, `NUMERIC`
- **Inexact types**: `REAL`, `DOUBLE PRECISION`

When combining exact types, the result is exact if possible:

```sql
-- Both exact → exact result
SELECT COALESCE(int_col, smallint_col)  -- INTEGER

-- Exact + inexact → inexact result
SELECT COALESCE(int_col, real_col)      -- REAL
```

### NULL Handling

`NULL` literals default to `INTEGER` when no other context:

```sql
SELECT COALESCE(NULL, NULL)  -- Result type is INTEGER
```

## See Also

- [Data Types](data_types.md) - Type definitions
- [Conversion Functions](../functions/conversion_functions.md) - CAST, type-specific parsers
