# Data Model

This section describes the SQL data model for tables, columns, and values.

## Tables

SQL data is stored in tables. Each table consists of an ordered list of columns and rows. Each column has a name and a data type.

**Example: Singers table**

| Column Name | Data Type      | Default Value    |
| ----------- | -------------- | ---------------- |
| singer_id   | `INTEGER`      | `AUTO_INCREMENT` |
| first_name  | `VARCHAR(100)` |                  |
| last_name   | `VARCHAR(100)` |                  |
| birth_date  | `DATE`         |                  |
| status      | `VARCHAR(20)`  | `'active'`       |

```sql
SELECT * FROM Singers;

/*-----------+------------+-----------+------------+--------+
 | singer_id | first_name | last_name | birth_date | status |
 +-----------+------------+-----------+------------+--------+
 | 1         | Marc       | Richards  | 1970-09-03 | active |
 | 2         | Catalina   | Smith     | 1990-08-17 | active |
 | 3         | Lea        | Martin    | 1991-11-09 | active |
 +-----------+------------+-----------+------------+--------*/
```

See [Data Types](data_types.md) for supported types.

## Constraints

Constraints enforce rules on data modifications (INSERT, UPDATE, DELETE):

### Primary Key

A primary key uniquely identifies each row. It consists of one or more columns.

```sql
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  email VARCHAR(255) NOT NULL
);

-- Composite primary key
CREATE TABLE order_items (
  order_id INTEGER,
  item_id INTEGER,
  quantity INTEGER,
  PRIMARY KEY (order_id, item_id)
);
```

Properties:

- At most one primary key per table
- Values must be unique
- Creates an implicit index
- Can contain NULL (implementation-dependent)

### Unique Constraint

Ensures column values are unique:

```sql
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  email VARCHAR(255) UNIQUE,
  username VARCHAR(50) UNIQUE
);
```

Multiple unique constraints can exist on a table.

### Foreign Key

References a primary key in another table:

```sql
CREATE TABLE orders (
  id INTEGER PRIMARY KEY,
  user_id INTEGER REFERENCES users(id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Check Constraint

Validates values against a condition:

```sql
CREATE TABLE products (
  id INTEGER PRIMARY KEY,
  price NUMERIC(10, 2) CHECK (price > 0),
  quantity INTEGER CHECK (quantity >= 0)
);
```

### NOT NULL

Prevents NULL values:

```sql
CREATE TABLE employees (
  id INTEGER PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  department VARCHAR(50) NOT NULL
);
```

## Indexes

Indexes improve query performance on specific columns:

```sql
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_orders_date ON orders(created_at DESC);
CREATE UNIQUE INDEX idx_users_username ON users(username);
```

## NULL Values

`NULL` represents the absence of a value. It propagates through most expressions:

```sql
SELECT 1 + NULL;           -- NULL
SELECT 'hello' || NULL;    -- NULL
SELECT NULL = NULL;        -- NULL (not TRUE!)
SELECT NULL IS NULL;       -- TRUE
```

## Value Tables

Value tables store rows as single values rather than columns. Each row is a `STRUCT` value:

```sql
SELECT AS VALUE STRUCT(123 AS a, 'hello' AS b)

/*-----+-------+
 | a   | b     |
 +-----+-------+
 | 123 | hello |
 +-----+-------*/
```

Value tables are produced by:

- `SELECT AS VALUE`
- `SELECT AS STRUCT`
- `UNNEST` operator

## Row Constructors

Construct row values explicitly:

```sql
SELECT ROW(1, 'Alice', 25) AS person;
SELECT (1, 'Alice', 25) AS person;  -- Shorthand
```

## See Also

- [Data Types](data_types.md) - Type definitions
- [DDL Statements](../statements/data_definition_language.md) - CREATE TABLE syntax
- [DML Statements](../statements/data_manipulation_language.md) - INSERT, UPDATE, DELETE
