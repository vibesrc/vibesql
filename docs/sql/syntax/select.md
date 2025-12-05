# Select

## `SELECT` statement

```sql
SELECT
  [ WITH differential_privacy_clause ]
  [ { ALL | DISTINCT } ]
  [ AS { typename | STRUCT | VALUE } ]
  select_list

select_list:
  { select_all | select_expression } [, ...]

select_all:
  [ expression. ]*
  [ EXCEPT ( column_name [, ...] ) ]
  [ REPLACE ( expression AS column_name [, ...] ) ]

select_expression:
  expression [ [ AS ] alias ]
```

The `SELECT` list defines the columns that the query will return. Expressions in
the `SELECT` list can refer to columns in any of the `from_item`s in its
corresponding `FROM` clause.

Each item in the `SELECT` list is one of:

- `*`
- `expression`
- `expression.*`

### `SELECT *`

`SELECT *`, often referred to as _select star_, produces one output column for
each column that's visible after executing the full query.

```sql
SELECT * FROM (SELECT "apple" AS fruit, "carrot" AS vegetable);

/*-------+-----------+
 | fruit | vegetable |
 +-------+-----------+
 | apple | carrot    |
 +-------+-----------*/
```

### `SELECT expression`

Items in a `SELECT` list can be expressions. These expressions evaluate to a
single value and produce one output column, with an optional explicit `alias`.

If the expression doesn't have an explicit alias, it receives an implicit alias
according to the rules for [implicit aliases][implicit-aliases], if possible.
Otherwise, the column is anonymous and you can't refer to it by name elsewhere
in the query.

### `SELECT expression.*`

An item in a `SELECT` list can also take the form of `expression.*`. This
produces one output column for each column or top-level field of `expression`.
The expression must either be a table alias or evaluate to a single value of a
data type with fields, such as a STRUCT.

The following query produces one output column for each column in the table
`groceries`, aliased as `g`.

```sql
WITH groceries AS
  (SELECT "milk" AS dairy,
   "eggs" AS protein,
   "bread" AS grain)
SELECT g.*
FROM groceries AS g;

/*-------+---------+-------+
 | dairy | protein | grain |
 +-------+---------+-------+
 | milk  | eggs    | bread |
 +-------+---------+-------*/
```

More examples:

```sql
WITH locations AS
  (SELECT STRUCT("Seattle" AS city, "Washington" AS state) AS location
  UNION ALL
  SELECT STRUCT("Phoenix" AS city, "Arizona" AS state) AS location)
SELECT l.location.*
FROM locations l;

/*---------+------------+
 | city    | state      |
 +---------+------------+
 | Seattle | Washington |
 | Phoenix | Arizona    |
 +---------+------------*/
```

```sql
WITH locations AS
  (SELECT ARRAY>[("Seattle", "Washington"),
    ("Phoenix", "Arizona")] AS location)
SELECT l.LOCATION[offset(0)].*
FROM locations l;

/*---------+------------+
 | city    | state      |
 +---------+------------+
 | Seattle | Washington |
 +---------+------------*/
```

### `SELECT * EXCEPT`

A `SELECT * EXCEPT` statement specifies the names of one or more columns to
exclude from the result. All matching column names are omitted from the output.

```sql
WITH orders AS
  (SELECT 5 as order_id,
  "sprocket" as item_name,
  200 as quantity)
SELECT * EXCEPT (order_id)
FROM orders;

/*-----------+----------+
 | item_name | quantity |
 +-----------+----------+
 | sprocket  | 200      |
 +-----------+----------*/
```

Note: `SELECT * EXCEPT` doesn't exclude columns that don't have names.

### `SELECT * REPLACE`

A `SELECT * REPLACE` statement specifies one or more
`expression AS identifier` clauses. Each identifier must match a column name
from the `SELECT *` statement. In the output column list, the column that
matches the identifier in a `REPLACE` clause is replaced by the expression in
that `REPLACE` clause.

A `SELECT * REPLACE` statement doesn't change the names or order of columns.
However, it can change the value and the value type.

```sql
WITH orders AS
  (SELECT 5 as order_id,
  "sprocket" as item_name,
  200 as quantity)
SELECT * REPLACE ("widget" AS item_name)
FROM orders;

/*----------+-----------+----------+
 | order_id | item_name | quantity |
 +----------+-----------+----------+
 | 5        | widget    | 200      |
 +----------+-----------+----------*/

WITH orders AS
  (SELECT 5 as order_id,
  "sprocket" as item_name,
  200 as quantity)
SELECT * REPLACE (quantity/2 AS quantity)
FROM orders;

/*----------+-----------+----------+
 | order_id | item_name | quantity |
 +----------+-----------+----------+
 | 5        | sprocket  | 100      |
 +----------+-----------+----------*/
```

Note: `SELECT * REPLACE` doesn't replace columns that don't have names.

### `SELECT DISTINCT`

A `SELECT DISTINCT` statement discards duplicate rows and returns only the
remaining rows. `SELECT DISTINCT` can't return columns of the following types:

-
- `GRAPH_ELEMENT`

In the following example, `SELECT DISTINCT` is used to produce distinct arrays:

```sql
WITH PlayerStats AS (
  SELECT ['Coolidge', 'Adams'] as Name, 3 as PointsScored UNION ALL
  SELECT ['Adams', 'Buchanan'], 0 UNION ALL
  SELECT ['Coolidge', 'Adams'], 1 UNION ALL
  SELECT ['Kiran', 'Noam'], 1)
SELECT DISTINCT Name
FROM PlayerStats;

/*------------------+
 | Name             |
 +------------------+
 | [Coolidge,Adams] |
 | [Adams,Buchanan] |
 | [Kiran,Noam]     |
 +------------------*/
```

In the following example, `SELECT DISTINCT` is used to produce distinct structs:

```sql
WITH
  PlayerStats AS (
    SELECT
      STRUCT(
        'Adams', 'Noam', 20) AS Player,
      3 AS PointsScored UNION ALL
    SELECT ('Buchanan', 'Jie', 19), 0 UNION ALL
    SELECT ('Adams', 'Noam', 20), 4 UNION ALL
    SELECT ('Buchanan', 'Jie', 19), 13
  )
SELECT DISTINCT Player
FROM PlayerStats;

/*--------------------------+
 | player                   |
 +--------------------------+
 | {                        |
 |   last_name: "Adams",    |
 |   first_name: "Noam",    |
 |   age: 20                |
 |  }                       |
 +--------------------------+
 | {                        |
 |   last_name: "Buchanan", |
 |   first_name: "Jie",     |
 |   age: 19                |
 |  }                       |
 +---------------------------*/
```

### `SELECT ALL`

A `SELECT ALL` statement returns all rows, including duplicate rows.
`SELECT ALL` is the default behavior of `SELECT`.

### `SELECT AS STRUCT`

```sql
SELECT AS STRUCT expr [[AS] struct_field_name1] [,...]
```

This produces a [value table][value-tables] with a
STRUCT row type, where the
STRUCT field names and types match the column names
and types produced in the `SELECT` list.

Example:

```sql
SELECT ARRAY(SELECT AS STRUCT 1 a, 2 b)
```

`SELECT AS STRUCT` can be used in a scalar or array subquery to produce a single
STRUCT type grouping multiple values together. Scalar
and array subqueries (see [Subqueries][subquery-concepts]) are normally not
allowed to return multiple columns, but can return a single column with
STRUCT type.

Anonymous columns are allowed.

Example:

```sql
SELECT AS STRUCT 1 x, 2, 3
```

The query above produces STRUCT values of type
`STRUCT<int64 x, int64, int64>.` The first field has the name `x` while the
second and third fields are anonymous.

The example above produces the same result as this `SELECT AS VALUE` query using
a struct constructor:

```sql
SELECT AS VALUE STRUCT(1 AS x, 2, 3)
```

Duplicate columns are allowed.

Example:

```sql
SELECT AS STRUCT 1 x, 2 y, 3 x
```

The query above produces STRUCT values of type
`STRUCT<int64 x, int64 y, int64 x>.` The first and third fields have the same
name `x` while the second field has the name `y`.

The example above produces the same result as this `SELECT AS VALUE` query
using a struct constructor:

```sql
SELECT AS VALUE STRUCT(1 AS x, 2 AS y, 3 AS x)
```

### `SELECT AS typename`

```sql
SELECT AS typename
  expr [[AS] field]
  [, ...]
```

A `SELECT AS typename` statement produces a value table where the row type
is a specific named type. Currently, [][proto-buffers] are the
only supported type that can be used with this syntax.

When selecting as a type that has fields, such as a proto message type,
the `SELECT` list may produce multiple columns. Each produced column must have
an explicit or [implicit][implicit-aliases] alias that matches a unique field of
the named type.

When used with `SELECT DISTINCT`, or `GROUP BY` or `ORDER BY` using column
ordinals, these operators are first applied on the columns in the `SELECT` list.
The value construction happens last. This means that `DISTINCT` can be applied
on the input columns to the value construction, including in
cases where `DISTINCT` wouldn't be allowed after value construction because
grouping isn't supported on the constructed type.

The following is an example of a `SELECT AS typename` query.

```sql
SELECT AS tests.TestProtocolBuffer mytable.key int64_val, mytable.name string_val
FROM mytable;
```

The query returns the output as a `tests.TestProtocolBuffer` protocol
buffer. `mytable.key int64_val` means that values from the `key` column are
stored in the `int64_val` field in the . Similarly, values from
the `mytable.name` column are stored in the `string_val` field.

To learn more about , see
[Work with ][proto-buffers].

### `SELECT AS VALUE`

`SELECT AS VALUE` produces a [value table][value-tables] from any
`SELECT` list that produces exactly one column. Instead of producing an
output table with one column, possibly with a name, the output will be a
value table where the row type is just the value type that was produced in the
one `SELECT` column. Any alias the column had will be discarded in the
value table.

Example:

```sql
SELECT AS VALUE 1
```

The query above produces a table with row type BIGINT.

Example:

```sql
SELECT AS VALUE STRUCT(1 AS a, 2 AS b) xyz
```

The query above produces a table with row type `STRUCT<a int64, b int64>`.

Example:

```sql
SELECT AS VALUE v FROM (SELECT AS STRUCT 1 a, true b) v WHERE v.b
```

Given a value table `v` as input, the query above filters out certain values in
the `WHERE` clause, and then produces a value table using the exact same value
that was in the input table. If the query above didn't use `SELECT AS VALUE`,
then the output table schema would differ from the input table schema because
the output table would be a regular table with a column named `v` containing the
input value.
