# Data Types

This page provides an overview of all SQL data types supported by vibesql, including information about their value domains. The type system is based on ISO SQL with common extensions.

## Data Type List

| Name                                    | SQL Standard | Summary                                                         |
| --------------------------------------- | ------------ | --------------------------------------------------------------- |
| [BOOLEAN](#boolean-type)                | SQL-92       | A value that can be `TRUE`, `FALSE`, or `NULL`. Aliases: `BOOL` |
| [INTEGER types](#integer-types)         | SQL-92       | Signed integers. Types: `SMALLINT`, `INTEGER`, `BIGINT`         |
| [Decimal types](#decimal-types)         | SQL-92       | Exact numeric with fixed precision. Types: `NUMERIC`, `DECIMAL` |
| [Floating point](#floating-point-types) | SQL-92       | Approximate numeric. Types: `REAL`, `DOUBLE PRECISION`          |
| [CHARACTER types](#character-types)     | SQL-92       | Character strings. Types: `CHAR`, `VARCHAR`, `TEXT`             |
| [Binary types](#binary-types)           | SQL:1999     | Binary data. Types: `BINARY`, `VARBINARY`, `BLOB`               |
| [DATE](#date-type)                      | SQL-92       | Calendar date (year, month, day)                                |
| [TIME](#time-type)                      | SQL-92       | Time of day (hour, minute, second)                              |
| [TIMESTAMP](#timestamp-type)            | SQL-92       | Date and time, with or without time zone                        |
| [INTERVAL](#interval-type)              | SQL-92       | Duration of time                                                |
| [ARRAY](#array-type)                    | SQL:2003     | Ordered collection of elements                                  |
| [ROW/STRUCT](#rowstruct-type)           | SQL:1999     | Composite type with named fields                                |
| [JSON](#json-type)                      | SQL:2016     | JSON data                                                       |
| [UUID](#uuid-type)                      | Extension    | Universally unique identifier                                   |

## Data Type Properties

### Nullable Types

All data types are nullable. `NULL` represents the absence of a value.

### Orderable Types

Orderable types can be used in `ORDER BY` clauses. All scalar types are orderable. Complex types (ARRAY, ROW) have defined ordering rules:

- **Arrays**: Ordered lexicographically by elements
- **ROW/STRUCT**: Not directly orderable

#### NULL Ordering

In `ORDER BY`:

- `NULLS FIRST`: NULL values appear first
- `NULLS LAST`: NULL values appear last
- Default: NULL values are treated as the smallest value (first in ASC, last in DESC)

### Groupable Types

Groupable types can appear in `GROUP BY`, `DISTINCT`, and `PARTITION BY`. All types except JSON are groupable.

### Comparable Types

Comparable types support comparison operators (`=`, `<>`, `<`, `>`, `<=`, `>=`). All scalar types are comparable. Arrays support equality comparison only.

---

## Boolean Type

**SQL Standard**: SQL-92

```sql
BOOLEAN
BOOL  -- alias
```

Boolean values: `TRUE`, `FALSE`, or `NULL`.

Boolean values are sorted: `NULL` < `FALSE` < `TRUE`

---

## Integer Types

**SQL Standard**: SQL-92

| Type              | Range                                                   | Storage |
| ----------------- | ------------------------------------------------------- | ------- |
| `SMALLINT`        | -32,768 to 32,767                                       | 16-bit  |
| `INTEGER` / `INT` | -2,147,483,648 to 2,147,483,647                         | 32-bit  |
| `BIGINT`          | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 | 64-bit  |

### Integer Literals

```sql
42
-17
+100
```

---

## Decimal Types

**SQL Standard**: SQL-92

```sql
NUMERIC
NUMERIC(precision)
NUMERIC(precision, scale)
DECIMAL              -- alias for NUMERIC
DECIMAL(p)
DECIMAL(p, s)
```

- **precision**: Total number of digits (1-38)
- **scale**: Digits after decimal point (0-precision)

Decimal types provide exact numeric representation, suitable for financial calculations.

### Examples

```sql
NUMERIC(10, 2)    -- 8 digits before decimal, 2 after: 12345678.99
DECIMAL(5)        -- 5 digits, no decimal places
```

---

## Floating Point Types

**SQL Standard**: SQL-92

| Type                          | Precision          | Storage         |
| ----------------------------- | ------------------ | --------------- |
| `REAL` / `FLOAT`              | ~7 decimal digits  | 32-bit IEEE 754 |
| `DOUBLE PRECISION` / `DOUBLE` | ~15 decimal digits | 64-bit IEEE 754 |

### Special Values

Floating point types support:

- `NaN` (Not a Number)
- `+Infinity` / `-Infinity`
- Positive and negative zero (treated as equal)

### Floating Point Ordering

1. `NULL`
2. `NaN`
3. `-Infinity`
4. Negative numbers
5. Zero
6. Positive numbers
7. `+Infinity`

---

## Character Types

**SQL Standard**: SQL-92

| Type                                  | Description                             |
| ------------------------------------- | --------------------------------------- |
| `CHAR(n)` / `CHARACTER(n)`            | Fixed-length, padded with spaces        |
| `VARCHAR(n)` / `CHARACTER VARYING(n)` | Variable-length with max length         |
| `TEXT`                                | Variable-length, no maximum (extension) |

All character types use UTF-8 encoding.

### String Literals

```sql
'Hello, World!'
'It''s escaped'     -- single quote escaped by doubling
```

---

## Binary Types

**SQL Standard**: SQL:1999

| Type           | Description                     |
| -------------- | ------------------------------- |
| `BINARY(n)`    | Fixed-length binary             |
| `VARBINARY(n)` | Variable-length binary with max |
| `BLOB`         | Binary large object (extension) |

### Binary Literals

```sql
X'48454C4C4F'       -- hex notation
```

---

## Date Type

**SQL Standard**: SQL-92

```sql
DATE
```

Represents a calendar date (year, month, day) without time zone.

### Format

```sql
YYYY-MM-DD
```

### Date Literals

```sql
DATE '2024-01-15'
```

---

## Time Type

**SQL Standard**: SQL-92

```sql
TIME
TIME WITH TIME ZONE
```

Represents time of day (hour, minute, second, optional fractional seconds).

### Format

```sql
HH:MM:SS[.fraction]
HH:MM:SS[.fraction][±HH:MM]   -- with time zone
```

### Time Literals

```sql
TIME '14:30:00'
TIME '14:30:00.123456'
```

---

## Timestamp Type

**SQL Standard**: SQL-92

```sql
TIMESTAMP
TIMESTAMP WITH TIME ZONE
```

Represents a point in time (date and time combined).

### Format

```sql
YYYY-MM-DD HH:MM:SS[.fraction]
YYYY-MM-DDTHH:MM:SS[.fraction][±HH:MM|Z]
```

### Timestamp Literals

```sql
TIMESTAMP '2024-01-15 14:30:00'
TIMESTAMP '2024-01-15T14:30:00Z'
TIMESTAMP '2024-01-15 14:30:00-08:00'
```

### Time Zones

Time zones can be specified as:

- UTC offset: `+05:30`, `-08:00`
- UTC indicator: `Z`
- Time zone name: `America/New_York` (extension)

---

## Interval Type

**SQL Standard**: SQL-92

```sql
INTERVAL
```

Represents a duration of time.

### Interval Fields

- Year-Month intervals: `YEAR`, `MONTH`
- Day-Time intervals: `DAY`, `HOUR`, `MINUTE`, `SECOND`

### Interval Literals

```sql
INTERVAL '1' YEAR
INTERVAL '3' MONTH
INTERVAL '7' DAY
INTERVAL '2:30' HOUR TO MINUTE
INTERVAL '1 12:30:00' DAY TO SECOND
```

---

## Array Type

**SQL Standard**: SQL:2003

```sql
ARRAY
type ARRAY
type ARRAY[n]
```

An ordered collection of elements of the same type.

### Array Literals

```sql
ARRAY[1, 2, 3]
ARRAY['a', 'b', 'c']
```

### Array Access

```sql
my_array[1]           -- 1-based indexing (SQL standard)
```

### Array Properties

- Arrays can be `NULL`
- Array elements can be `NULL`
- Nested arrays are not allowed (use ROW/STRUCT to wrap)
- Empty arrays: `ARRAY[]`

---

## ROW/STRUCT Type

**SQL Standard**: SQL:1999 (as ROW)

```sql
ROW(field1 type1, field2 type2, ...)
STRUCT  -- alternative syntax
```

A composite type containing named fields.

### STRUCT Literals

```sql
ROW(1, 'hello', TRUE)
STRUCT(1 AS id, 'hello' AS name)
(1, 'hello', TRUE)    -- anonymous tuple syntax
```

### Field Access

```sql
my_struct.field_name
```

### Comparison

ROW/STRUCT types support equality comparison (`=`, `<>`) by comparing fields in order.

---

## JSON Type

**SQL Standard**: SQL:2016

```sql
JSON
```

Stores JSON (JavaScript Object Notation) data.

### JSON Literals

```sql
JSON '{"name": "John", "age": 30}'
JSON '[1, 2, 3]'
```

### JSON Properties

- Preserves object member order
- Duplicate keys: first occurrence is kept
- Numbers stored with double precision
- Whitespace is not preserved

---

## UUID Type

**Extension** (widely supported: PostgreSQL, MySQL 8+, SQL Server)

```sql
UUID
```

A 128-bit universally unique identifier.

### UUID Format

```sql
xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```

Example: `f81d4fae-7dec-11d0-a765-00a0c91e6bf6`

### UUID Literals

```sql
UUID 'f81d4fae-7dec-11d0-a765-00a0c91e6bf6'
```

---

## Type Coercion

Implicit type coercion follows these rules:

### Numeric Coercion

```sql
SMALLINT → INTEGER → BIGINT → NUMERIC → DOUBLE PRECISION
```

### Date/Time Coercion

```sql
DATE → TIMESTAMP
TIME → TIMESTAMP (with current date)
```

### String Coercion

Character types can be implicitly cast to each other.

---

## See Also

- [Conversion Rules](conversion_rules.md) - Explicit CAST operations
- [Lexical Structure](../syntax/lexical.md) - Literal syntax
- [Arrays](arrays.md) - Working with arrays
