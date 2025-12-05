# Set Operations

## Set operators

```sql
  query_expr
  [ { INNER | [ { FULL | LEFT } [ OUTER ] ] } ]
  {
    UNION { ALL | DISTINCT } |
    INTERSECT { ALL | DISTINCT } |
    EXCEPT { ALL | DISTINCT }
  }
  [ { BY NAME [ ON (column_list) ] | [ STRICT ] CORRESPONDING [ BY (column_list) ] } ]
  query_expr
```

Set operators combine or filter
results from two or more input queries into a single result set.

**Definitions**

- `query_expr`: One of two input queries whose results are combined or filtered
  into a single result set.
- `UNION`: Returns the combined results of the left and right input queries.
  Values in columns that are matched by position are concatenated vertically.
- `INTERSECT`: Returns rows that are found in the results of both the left and
  right input queries.
- `EXCEPT`: Returns rows from the left input query that aren't present in the
  right input query.
- `ALL`: Executes the set operation on all rows.
- `DISTINCT`: Excludes duplicate rows in the set operation.
- `BY NAME`, `CORRESPONDING`: Matches
  columns by name instead of by position. The `BY NAME` modifier is equivalent to `STRICT CORRESPONDING`.
  For details, see [`BY NAME` or `CORRESPONDING`][by-name-or-corresponding].
- `INNER`, `FULL | LEFT [OUTER]`, `STRICT`, `ON`, `BY`:
  Adjust how the `BY NAME` or `CORRESPONDING` modifier behaves when
  the column names don't match exactly. For details, see
  [`BY NAME` or `CORRESPONDING`][by-name-or-corresponding].

**Positional column matching**

By default, columns are matched positionally and follow these rules. If the
`BY NAME` or `CORRESPONDING` modifier is
used, columns are matched by name, as described in the next section.

- Columns from input queries are matched by their position in the queries. That
  is, the first column in the first input query is paired with the first column
  in the second input query and so on.
- The input queries on each side of the operator must return the same number of
  columns.

**Name-based column matching**

To make set operations match columns by name instead of by column position,
use the [`BY NAME` or `CORRESPONDING`][by-name-or-corresponding] modifier.

With `BY NAME` or `STRICT CORRESPONDING`, the same column names
must exist in each input, but they can be in different orders. Additional
modifiers can be used to handle cases where the columns don't exactly match.

The `BY NAME` modifier is equivalent to `STRICT CORRESPONDING`, but
the `BY NAME` modifier is recommended because it's shorter and clearer.

Example:

```sql
SELECT 1 AS one_digit, 10 AS two_digit
UNION ALL BY NAME
SELECT 20 AS two_digit, 2 AS one_digit;

-- Column values match by name and not position in query.
/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 1         | 10        |
 | 2         | 20        |
 +-----------+-----------*/
```

**Other column-related rules**

- For set operations other than `UNION
ALL`, all column types must support
  equality comparison.
- The results of the set operation always use the column names from the
  first input query.
- The results of the set operation always use the supertypes of input types
  in corresponding columns, so paired columns must also have either the same
  data type or a common supertype.

**Parenthesized set operators**

- Parentheses must be used to separate different set operations.
  Set operations like `UNION ALL` and `UNION DISTINCT` are considered different.
- Parentheses are also used to group set operations and control order of
  operations. In `EXCEPT` set operations, for example,
  query results can vary depending on the operation grouping.

The following examples illustrate the use of parentheses with set
operations:

```sql
-- Same set operations, no parentheses.
query1
UNION ALL
query2
UNION ALL
query3;
```

```sql
-- Different set operations, parentheses needed.
query1
UNION ALL
(
  query2
  UNION DISTINCT
  query3
);
```

```sql {.bad}
-- Invalid
query1
UNION ALL
query2
UNION DISTINCT
query3;
```

```sql
-- Same set operations, no parentheses.
query1
EXCEPT ALL
query2
EXCEPT ALL
query3;

-- Equivalent query with optional parentheses, returns same results.
(
  query1
  EXCEPT ALL
  query2
)
EXCEPT ALL
query3;
```

```sql
-- Different execution order with a subquery, parentheses needed.
query1
EXCEPT ALL
(
  query2
  EXCEPT ALL
  query3
);
```

**Set operator behavior with duplicate rows**

Consider a given row `R` that appears exactly `m` times in the first input query
and `n` times in the second input query, where `m >= 0` and `n >= 0`:

- For `UNION ALL`, row `R` appears exactly `m + n` times in the
  result.
- For `INTERSECT ALL`, row `R` appears exactly `MIN(m, n)` times in the
  result.
- For `EXCEPT ALL`, row `R` appears exactly `MAX(m - n, 0)` times in the
  result.
- For `UNION DISTINCT`, the `DISTINCT`
  is computed after the `UNION` is computed, so row `R` appears exactly
  one time.
- For `INTERSECT DISTINCT`, row `R` appears once in the output if `m > 0` and
  `n > 0`.
- For `EXCEPT DISTINCT`, row `R` appears once in the output if
  `m > 0` and `n = 0`.
- If more than two input queries are used, the above operations generalize
  and the output is the same as if the input queries were combined
  incrementally from left to right.

**BY NAME or CORRESPONDING**

Use the `BY NAME` or `CORRESPONDING` modifier
with set operations to match columns by name instead of by position.
The `BY NAME` modifier is equivalent to `STRICT CORRESPONDING`, but the
`BY NAME` modifier is recommended because it's shorter and clearer.
You can use mode prefixes to adjust how
the `BY NAME` or `CORRESPONDING` modifier
behaves when the column names don't match exactly.

- `BY NAME`: Matches
  columns by name instead of by position.
  - Both input queries must have the same set of column names, but column order
    can be different. If a column in one input query doesn't appear in the other
    query, an error is raised.
  - Input queries can't contain duplicate columns.
  - Input queries that produce [value tables][value-tables] aren't supported.
- `INNER`: Adjusts the `BY NAME` modifier behavior so that columns that appear
  in both input queries are included in the query results and any other
  columns are excluded.
  - No error is raised for the excluded columns that appear in one input
    query but not in the other input query.
  - At least one column must be common in both left and right input queries.
- `FULL [OUTER]`: Adjusts the `BY NAME` modifier behavior so that all columns
  from both input queries are included in the query results, even if some
  columns aren't present in both queries.
  - Columns from the left input query are returned
    first, followed by unique columns from the right input query.
  - For columns in one input query that aren't present in the other query,
    a `NULL` value is added as its column value for the other query in the results.
- `LEFT [OUTER]`: Adjusts the `BY NAME` modifier so that all columns from the
  left input query are included in the results, even if some columns in the
  left query aren't present in the right query.
  - For columns in the left query that aren't in the right query, a `NULL`
    value is added as its column value for the right query in the results.
  - At least one column name must be common in both left and right input queries.
- `OUTER`: If used alone, equivalent to `FULL OUTER`.
- `ON (column_list)`: Used after the `BY NAME` modifier to
  specify a comma-separated list of column names and the column order to
  return from the input queries.
  - If `BY NAME ON (column_list)` is used
    alone without mode prefixes like
    `INNER` or `FULL | LEFT [OUTER]`, then both the left and right
    input queries must contain all the columns in the `column_list`.
  - If any mode prefixes are used, then any column names not in the
    `column_list` are excluded from the results according to the mode used.
- `CORRESPONDING`: Equivalent to `INNER...BY NAME`.
  - Supports `FULL | LEFT [OUTER]` modes the same way they're supported by the `BY NAME` modifier.
  - Supports `INNER` mode, but this mode has no effect. The `INNER` mode is used with the `BY NAME`
    modifier to exclude unmatched columns between input queries, which is
    the default behavior of the `CORRESPONDING` modifier. Therefore, using
    `INNER...CORRESPONDING` produces the same results as `CORRESPONDING`.
- `STRICT`: Adjusts the `CORRESPONDING` modifier to be equivalent to the default `BY NAME` modifier, where input
  queries must have the same set of column names.
- `BY (column_list)`: Equivalent to `ON (column_list)` with `BY NAME`.

The following table shows the equivalent syntaxes between the `BY NAME` and
`CORRESPONDING` modifiers, using the `UNION ALL` set operator as an example:

| BY NAME syntax                           | Equivalent CORRESPONDING syntax                       |
| ---------------------------------------- | ----------------------------------------------------- | ------ | -------------------------------------- |
| `UNION ALL BY NAME`                      | `UNION ALL STRICT CORRESPONDING`                      |
| `INNER UNION ALL BY NAME`                | `UNION ALL CORRESPONDING`                             |
| `{LEFT                                   | FULL} [OUTER] UNION ALL BY NAME`                      | `{LEFT | FULL} [OUTER] UNION ALL CORRESPONDING` |
| `[FULL] OUTER UNION ALL BY NAME`         | `[FULL] OUTER UNION ALL CORRESPONDING`                |
| `UNION ALL BY NAME ON (col1, col2, ...)` | `UNION ALL STRICT CORRESPONDING BY (col1, col2, ...)` |

The following table shows the behavior of the mode prefixes for the
`BY NAME` and `CORRESPONDING` modifiers
when left and right input columns don't match:

| Mode prefix and modifier                               | Behavior when left and right input columns don't match                                                                                                                                                           |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `BY NAME` (no prefix) or `STRICT CORRESPONDING`        | Error, all columns must match in both inputs.                                                                                                                                                                    |
| `INNER BY NAME` or `CORRESPONDING` (no prefix)         | Drop all unmatched columns in both inputs.                                                                                                                                                                       |
| `FULL [OUTER] BY NAME` or `FULL [OUTER] CORRESPONDING` | Include all columns from both inputs. For column values that exist in one input but not in another, add `NULL` values.                                                                                           |
| `LEFT [OUTER] BY NAME` or `LEFT [OUTER] CORRESPONDING` | Include all columns from the left input. For column values that exist in the left input but not in the right input, add `NULL` values. Drop any columns from the right input that don't exist in the left input. |

For example set operations with modifiers, see the sections for each set
operator, such as [`UNION`][union-syntax].

### `UNION`

The `UNION` operator returns the combined results of the left and right input
queries. Columns are matched according to the rules described previously and
rows are concatenated vertically.

**Examples**

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3]) AS number
UNION ALL
SELECT 1;

/*--------+
 | number |
 +--------+
 | 1      |
 | 2      |
 | 3      |
 | 1      |
 +--------*/
```

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3]) AS number
UNION DISTINCT
SELECT 1;

/*--------+
 | number |
 +--------+
 | 1      |
 | 2      |
 | 3      |
 +--------*/
```

The following example shows multiple chained operators:

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3]) AS number
UNION DISTINCT
SELECT 1
UNION DISTINCT
SELECT 2;

/*--------+
 | number |
 +--------+
 | 1      |
 | 2      |
 | 3      |
 +--------*/
```

The following example shows input queries with multiple columns. Both
queries specify the same column names but in different orders. As a result, the
column values are matched by column position in the input query and the column
names are ignored.

```sql
SELECT 1 AS one_digit, 10 AS two_digit
UNION ALL
SELECT 20 AS two_digit, 2 AS one_digit;

-- Column values are matched by position and not column name.
/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 1         | 10        |
 | 20        | 2         |
 +-----------+-----------*/
```

To resolve this ordering issue, the following example uses the
`BY NAME` modifier to match
the columns by name instead of by position in the query results.

```sql
SELECT 1 AS one_digit, 10 AS two_digit
UNION ALL BY NAME
SELECT 20 AS two_digit, 2 AS one_digit;

-- Column values now match.
/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 1         | 10        |
 | 2         | 20        |
 +-----------+-----------*/
```

The previous set operation with `BY NAME` is equivalent to using the `STRICT
CORRESPONDING` modifier. The `BY NAME` modifier is recommended because it's
shorter and clearer than the `STRICT CORRESPONDING` modifier.

```sql
SELECT 1 AS one_digit, 10 AS two_digit
UNION ALL STRICT CORRESPONDING
SELECT 20 AS two_digit, 2 AS one_digit;

/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 1         | 10        |
 | 2         | 20        |
 +-----------+-----------*/
```

The following example adds a `three_digit` column to the left input query and a
`four_digit` column to the right input query. Because these columns aren't
present in both queries, the `BY NAME`
modifier would trigger an error. Therefore, the example
adds the `INNER`
mode prefix so that the new columns are excluded from the results, executing the
query successfully.

```sql
SELECT 1 AS one_digit, 10 AS two_digit, 100 AS three_digit
INNER UNION ALL BY NAME
SELECT 20 AS two_digit, 2 AS one_digit, 1000 AS four_digit;

/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 1         | 10        |
 | 2         | 20        |
 +-----------+-----------*/
```

To include the differing columns in the results, the following example uses
the `FULL OUTER` mode prefix to populate `NULL` values for the missing column in
each query.

```sql
SELECT 1 AS one_digit, 10 AS two_digit, 100 AS three_digit
FULL OUTER UNION ALL BY NAME
SELECT 20 AS two_digit, 2 AS one_digit, 1000 AS four_digit;

/*-----------+-----------+-------------+------------+
 | one_digit | two_digit | three_digit | four_digit |
 +-----------+-----------+-------------+------------+
 | 1         | 10        | 100         | NULL       |
 | 2         | 20        | NULL        | 1000       |
 +-----------+-----------+-------------+------------*/
```

Similarly, the following example uses the `LEFT OUTER` mode prefix to include
the new column from only the left input query and populate a `NULL` value for
the missing column in the right input query.

```sql
SELECT 1 AS one_digit, 10 AS two_digit, 100 AS three_digit
LEFT OUTER UNION ALL BY NAME
SELECT 20 AS two_digit, 2 AS one_digit, 1000 AS four_digit;

/*-----------+-----------+-------------+
 | one_digit | two_digit | three_digit |
 +-----------+-----------+-------------+
 | 1         | 10        | 100         |
 | 2         | 20        | NULL        |
 +-----------+-----------+-------------*/
```

The following example adds the modifier `ON (column_list)`
to return only the specified columns in the specified order.

```sql
SELECT 1 AS one_digit, 10 AS two_digit, 100 AS three_digit
FULL OUTER UNION ALL BY NAME ON (three_digit, two_digit)
SELECT 20 AS two_digit, 2 AS one_digit, 1000 AS four_digit;

/*-------------+-----------+
 | three_digit | two_digit |
 +-------------+-----------+
 | 100         | 10        |
 | NULL        | 20        |
 +-----------+-------------*/
```

### `INTERSECT`

The `INTERSECT` operator returns rows that are found in the results of both the
left and right input queries.

**Examples**

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3, 3, 4]) AS number
INTERSECT ALL
SELECT * FROM UNNEST(ARRAY[2, 3, 3, 5]) AS number;

/*--------+
 | number |
 +--------+
 | 2      |
 | 3      |
 | 3      |
 +--------*/
```

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3, 3, 4]) AS number
INTERSECT DISTINCT
SELECT * FROM UNNEST(ARRAY[2, 3, 3, 5]) AS number;

/*--------+
 | number |
 +--------+
 | 2      |
 | 3      |
 +--------*/
```

The following example shows multiple chained operations:

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3, 3, 4]) AS number
INTERSECT DISTINCT
SELECT * FROM UNNEST(ARRAY[2, 3, 3, 5]) AS number
INTERSECT DISTINCT
SELECT * FROM UNNEST(ARRAY[3, 3, 4, 5]) AS number;

/*--------+
 | number |
 +--------+
 | 3      |
 +--------*/
```

The following example shows input queries that specify multiple columns. Both
queries specify the same column names but in different orders. As a result, the
same columns in differing order are considered different columns, so the query
doesn't detect any intersecting row values. Therefore, no results are returned.

```sql
WITH
  NumbersTable AS (
    SELECT 1 AS one_digit, 10 AS two_digit
    UNION ALL
    SELECT 2, 20
    UNION ALL
    SELECT 3, 30
  )
SELECT one_digit, two_digit FROM NumbersTable
INTERSECT DISTINCT
SELECT 10 AS two_digit, 1 AS one_digit;

-- No intersecting values detected because columns aren't recognized as the same.
/*-----------+-----------+

 +-----------+-----------*/
```

To resolve this ordering issue, the following example uses the
`BY NAME` modifier to match
the columns by name instead of by position in the query results.

```sql
WITH
  NumbersTable AS (
    SELECT 1 AS one_digit, 10 AS two_digit
    UNION ALL
    SELECT 2, 20
    UNION ALL
    SELECT 3, 30
  )
SELECT one_digit, two_digit FROM NumbersTable
INTERSECT DISTINCT BY NAME
SELECT 10 AS two_digit, 1 AS one_digit;

/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 1         | 10        |
 +-----------+-----------*/
```

The previous set operation with `BY NAME` is equivalent to using the `STRICT
CORRESPONDING` modifier. The `BY NAME` modifier is recommended because it's
shorter and clearer than the `STRICT CORRESPONDING` modifier.

```sql
WITH
  NumbersTable AS (
    SELECT 1 AS one_digit, 10 AS two_digit
    UNION ALL
    SELECT 2, 20
    UNION ALL
    SELECT 3, 30
  )
SELECT one_digit, two_digit FROM NumbersTable
INTERSECT DISTINCT STRICT CORRESPONDING
SELECT 10 AS two_digit, 1 AS one_digit;

/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 1         | 10        |
 +-----------+-----------*/
```

For more syntax examples with the `BY NAME`
modifier, see the [`UNION`][union] set operator.

### `EXCEPT`

The `EXCEPT` operator returns rows from the left input query that aren't present
in the right input query.

**Examples**

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3, 3, 4]) AS number
EXCEPT ALL
SELECT * FROM UNNEST(ARRAY[1, 2]) AS number;

/*--------+
 | number |
 +--------+
 | 3      |
 | 3      |
 | 4      |
 +--------*/
```

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3, 3, 4]) AS number
EXCEPT DISTINCT
SELECT * FROM UNNEST(ARRAY[1, 2]) AS number;

/*--------+
 | number |
 +--------+
 | 3      |
 | 4      |
 +--------*/
```

The following example shows multiple chained operations:

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3, 3, 4]) AS number
EXCEPT DISTINCT
SELECT * FROM UNNEST(ARRAY[1, 2]) AS number
EXCEPT DISTINCT
SELECT * FROM UNNEST(ARRAY[1, 4]) AS number;

/*--------+
 | number |
 +--------+
 | 3      |
 +--------*/
```

The following example modifies the execution behavior of the set operations. The
first input query is used against the result of the last two input queries
instead of the values of the last two queries individually. In this example,
the `EXCEPT` result of the last two input queries is `2`. Therefore, the
`EXCEPT` results of the entire query are any values other than `2` in the first
input query.

```sql
SELECT * FROM UNNEST(ARRAY[1, 2, 3, 3, 4]) AS number
EXCEPT DISTINCT
(
  SELECT * FROM UNNEST(ARRAY[1, 2]) AS number
  EXCEPT DISTINCT
  SELECT * FROM UNNEST(ARRAY[1, 4]) AS number
);

/*--------+
 | number |
 +--------+
 | 1      |
 | 3      |
 | 4      |
 +--------*/
```

The following example shows input queries that specify multiple columns. Both
queries specify the same column names but in different orders. As a result, the
same columns in differing order are considered different columns, so the query
doesn't detect any common rows that should be excluded. Therefore, all column
values from the left input query are returned with no exclusions.

```sql
WITH
  NumbersTable AS (
    SELECT 1 AS one_digit, 10 AS two_digit
    UNION ALL
    SELECT 2, 20
    UNION ALL
    SELECT 3, 30
  )
SELECT one_digit, two_digit FROM NumbersTable
EXCEPT DISTINCT
SELECT 10 AS two_digit, 1 AS one_digit;

-- No values excluded because columns aren't recognized as the same.
/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 1         | 10        |
 | 2         | 20        |
 | 3         | 30        |
 +-----------+-----------*/
```

To resolve this ordering issue, the following example uses the
`BY NAME` modifier to match
the columns by name instead of by position in the query results.

```sql
WITH
  NumbersTable AS (
    SELECT 1 AS one_digit, 10 AS two_digit
    UNION ALL
    SELECT 2, 20
    UNION ALL
    SELECT 3, 30
  )
SELECT one_digit, two_digit FROM NumbersTable
EXCEPT DISTINCT BY NAME
SELECT 10 AS two_digit, 1 AS one_digit;

/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 2         | 20        |
 | 3         | 30        |
 +-----------+-----------*/
```

The previous set operation with `BY NAME` is equivalent to using the `STRICT
CORRESPONDING` modifier. The `BY NAME` modifier is recommended because it's
shorter and clearer than the `STRICT CORRESPONDING` modifier.

```sql
WITH
  NumbersTable AS (
    SELECT 1 AS one_digit, 10 AS two_digit
    UNION ALL
    SELECT 2, 20
    UNION ALL
    SELECT 3, 30
  )
SELECT one_digit, two_digit FROM NumbersTable
EXCEPT DISTINCT STRICT CORRESPONDING
SELECT 10 AS two_digit, 1 AS one_digit;

/*-----------+-----------+
 | one_digit | two_digit |
 +-----------+-----------+
 | 2         | 20        |
 | 3         | 30        |
 +-----------+-----------*/
```

For more syntax examples with the `BY NAME`
modifier, see the [`UNION`][union] set operator.

## `LIMIT` and `OFFSET` clause

```sql
LIMIT count [ OFFSET skip_rows ]
```

Limits the number of rows to return in a query. Optionally includes
the ability to skip over rows.

**Definitions**

- `LIMIT`: Limits the number of rows to produce.

  `count` is an `BIGINT` constant expression that represents the
  non-negative, non-`NULL` limit. No more than `count` rows are produced.
  `LIMIT 0` returns 0 rows.

  If there is a set operation, `LIMIT` is applied after the set operation is
  evaluated.

- `OFFSET`: Skips a specific number of rows before applying `LIMIT`.

  `skip_rows` is an `BIGINT` constant expression that represents
  the non-negative, non-`NULL` number of rows to skip.

**Details**

The rows that are returned by `LIMIT` and `OFFSET` have undefined order unless
these clauses are used after `ORDER BY`.

A constant expression can be represented by a general expression, literal, or
parameter value.

Note: Although the `LIMIT` clause limits the rows that a query produces, it
doesn't limit the amount of data processed by that query.

**Examples**

```sql
SELECT *
FROM UNNEST(ARRAY['a', 'b', 'c', 'd', 'e']) AS letter
ORDER BY letter ASC LIMIT 2;

/*---------+
 | letter  |
 +---------+
 | a       |
 | b       |
 +---------*/
```

```sql
SELECT *
FROM UNNEST(ARRAY['a', 'b', 'c', 'd', 'e']) AS letter
ORDER BY letter ASC LIMIT 3 OFFSET 1;

/*---------+
 | letter  |
 +---------+
 | b       |
 | c       |
 | d       |
 +---------*/
```
