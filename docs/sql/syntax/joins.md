# Joins

## Join operation

```sql
join_operation:
  { cross_join_operation | condition_join_operation }

cross_join_operation:
  from_item cross_join_operator [ LATERAL ] from_item

condition_join_operation:
  from_item condition_join_operator [ LATERAL ] from_item join_condition

cross_join_operator:
  { CROSS JOIN | , }

condition_join_operator:
  {
    [INNER] JOIN
    | FULL [OUTER] JOIN
    | LEFT [OUTER] JOIN
    | RIGHT [OUTER] JOIN
  }

join_condition:
  { on_clause | using_clause }

on_clause:
  ON bool_expression

using_clause:
  USING ( column_list )
```

The `JOIN` operation merges two `from_item`s so that the `SELECT` clause can
query them as one source. The join operator and join condition specify how to
combine and discard rows from the two `from_item`s to form a single source.

### `[INNER] JOIN`

An `INNER JOIN`, or simply `JOIN`, effectively calculates the Cartesian product
of the two `from_item`s and discards all rows that don't meet the join
condition. _Effectively_ means that it's possible to implement an `INNER JOIN`
without actually calculating the Cartesian product.

```sql
FROM A INNER JOIN B ON A.w = B.y

/*
Table A       Table B       Result
+-------+     +-------+     +---------------+
| w | x |  *  | y | z |  =  | w | x | y | z |
+-------+     +-------+     +---------------+
| 1 | a |     | 2 | k |     | 2 | b | 2 | k |
| 2 | b |     | 3 | m |     | 3 | c | 3 | m |
| 3 | c |     | 3 | n |     | 3 | c | 3 | n |
| 3 | d |     | 4 | p |     | 3 | d | 3 | m |
+-------+     +-------+     | 3 | d | 3 | n |
                            +---------------+
*/
```

```sql
FROM A INNER JOIN B USING (x)

/*
Table A       Table B       Result
+-------+     +-------+     +-----------+
| x | y |  *  | x | z |  =  | x | y | z |
+-------+     +-------+     +-----------+
| 1 | a |     | 2 | k |     | 2 | b | k |
| 2 | b |     | 3 | m |     | 3 | c | m |
| 3 | c |     | 3 | n |     | 3 | c | n |
| 3 | d |     | 4 | p |     | 3 | d | m |
+-------+     +-------+     | 3 | d | n |
                            +-----------+
*/
```

**Example**

This query performs an `INNER JOIN` on the [`Roster`][roster-table]
and [`TeamMascot`][teammascot-table] tables.

```sql
SELECT Roster.LastName, TeamMascot.Mascot
FROM Roster JOIN TeamMascot ON Roster.SchoolID = TeamMascot.SchoolID;

/*---------------------------+
 | LastName   | Mascot       |
 +---------------------------+
 | Adams      | Jaguars      |
 | Buchanan   | Lakers       |
 | Coolidge   | Lakers       |
 | Davis      | Knights      |
 +---------------------------*/
```

You can use a [correlated][correlated-join] `INNER JOIN` to flatten an array
into a set of rows. To learn more, see
[Convert elements in an array to rows in a table][flattening-arrays].

### `CROSS JOIN`

`CROSS JOIN` returns the Cartesian product of the two `from_item`s. In other
words, it combines each row from the first `from_item` with each row from the
second `from_item`.

If the rows of the two `from_item`s are independent, then the result has
_M \* N_ rows, given _M_ rows in one `from_item` and _N_ in the other. Note that
this still holds for the case when either `from_item` has zero rows.

In a `FROM` clause, a `CROSS JOIN` can be written like this:

```sql
FROM A CROSS JOIN B

/*
Table A       Table B       Result
+-------+     +-------+     +---------------+
| w | x |  *  | y | z |  =  | w | x | y | z |
+-------+     +-------+     +---------------+
| 1 | a |     | 2 | c |     | 1 | a | 2 | c |
| 2 | b |     | 3 | d |     | 1 | a | 3 | d |
+-------+     +-------+     | 2 | b | 2 | c |
                            | 2 | b | 3 | d |
                            +---------------+
*/
```

You can use a [correlated][correlated-join] cross join to convert or
flatten an array into a set of rows, though the (equivalent) `INNER JOIN` is
preferred over `CROSS JOIN` for this case. To learn more, see
[Convert elements in an array to rows in a table][flattening-arrays].

**Examples**

This query performs an `CROSS JOIN` on the [`Roster`][roster-table]
and [`TeamMascot`][teammascot-table] tables.

```sql
SELECT Roster.LastName, TeamMascot.Mascot
FROM Roster CROSS JOIN TeamMascot;

/*---------------------------+
 | LastName   | Mascot       |
 +---------------------------+
 | Adams      | Jaguars      |
 | Adams      | Knights      |
 | Adams      | Lakers       |
 | Adams      | Mustangs     |
 | Buchanan   | Jaguars      |
 | Buchanan   | Knights      |
 | Buchanan   | Lakers       |
 | Buchanan   | Mustangs     |
 | ...                       |
 +---------------------------*/
```

### Comma cross join (,)

[`CROSS JOIN`][cross-join]s can be written implicitly with a comma. This is
called a comma cross join.

A comma cross join looks like this in a `FROM` clause:

```sql
FROM A, B

/*
Table A       Table B       Result
+-------+     +-------+     +---------------+
| w | x |  *  | y | z |  =  | w | x | y | z |
+-------+     +-------+     +---------------+
| 1 | a |     | 2 | c |     | 1 | a | 2 | c |
| 2 | b |     | 3 | d |     | 1 | a | 3 | d |
+-------+     +-------+     | 2 | b | 2 | c |
                            | 2 | b | 3 | d |
                            +---------------+
*/
```

You can't write comma cross joins inside parentheses. To learn more, see
[Join operations in a sequence][sequences-of-joins].

```sql {.bad}
FROM (A, B)  // INVALID
```

You can use a [correlated][correlated-join] comma cross join to convert or
flatten an array into a set of rows. To learn more, see
[Convert elements in an array to rows in a table][flattening-arrays].

**Examples**

This query performs a comma cross join on the [`Roster`][roster-table]
and [`TeamMascot`][teammascot-table] tables.

```sql
SELECT Roster.LastName, TeamMascot.Mascot
FROM Roster, TeamMascot;

/*---------------------------+
 | LastName   | Mascot       |
 +---------------------------+
 | Adams      | Jaguars      |
 | Adams      | Knights      |
 | Adams      | Lakers       |
 | Adams      | Mustangs     |
 | Buchanan   | Jaguars      |
 | Buchanan   | Knights      |
 | Buchanan   | Lakers       |
 | Buchanan   | Mustangs     |
 | ...                       |
 +---------------------------*/
```

### `FULL [OUTER] JOIN`

A `FULL OUTER JOIN` (or simply `FULL JOIN`) returns all fields for all matching
rows in both `from_items` that meet the join condition. If a given row from one
`from_item` doesn't join to any row in the other `from_item`, the row returns
with `NULL` values for all columns from the other `from_item`.

```sql
FROM A FULL OUTER JOIN B ON A.w = B.y

/*
Table A       Table B       Result
+-------+     +-------+     +---------------------------+
| w | x |  *  | y | z |  =  | w    | x    | y    | z    |
+-------+     +-------+     +---------------------------+
| 1 | a |     | 2 | k |     | 1    | a    | NULL | NULL |
| 2 | b |     | 3 | m |     | 2    | b    | 2    | k    |
| 3 | c |     | 3 | n |     | 3    | c    | 3    | m    |
| 3 | d |     | 4 | p |     | 3    | c    | 3    | n    |
+-------+     +-------+     | 3    | d    | 3    | m    |
                            | 3    | d    | 3    | n    |
                            | NULL | NULL | 4    | p    |
                            +---------------------------+
*/
```

```sql
FROM A FULL OUTER JOIN B USING (x)

/*
Table A       Table B       Result
+-------+     +-------+     +--------------------+
| x | y |  *  | x | z |  =  | x    | y    | z    |
+-------+     +-------+     +--------------------+
| 1 | a |     | 2 | k |     | 1    | a    | NULL |
| 2 | b |     | 3 | m |     | 2    | b    | k    |
| 3 | c |     | 3 | n |     | 3    | c    | m    |
| 3 | d |     | 4 | p |     | 3    | c    | n    |
+-------+     +-------+     | 3    | d    | m    |
                            | 3    | d    | n    |
                            | 4    | NULL | p    |
                            +--------------------+
*/
```

**Example**

This query performs a `FULL JOIN` on the [`Roster`][roster-table]
and [`TeamMascot`][teammascot-table] tables.

```sql
SELECT Roster.LastName, TeamMascot.Mascot
FROM Roster FULL JOIN TeamMascot ON Roster.SchoolID = TeamMascot.SchoolID;

/*---------------------------+
 | LastName   | Mascot       |
 +---------------------------+
 | Adams      | Jaguars      |
 | Buchanan   | Lakers       |
 | Coolidge   | Lakers       |
 | Davis      | Knights      |
 | Eisenhower | NULL         |
 | NULL       | Mustangs     |
 +---------------------------*/
```

### `LEFT [OUTER] JOIN`

The result of a `LEFT OUTER JOIN` (or simply `LEFT JOIN`) for two
`from_item`s always retains all rows of the left `from_item` in the
`JOIN` operation, even if no rows in the right `from_item` satisfy the join
predicate.

All rows from the _left_ `from_item` are retained;
if a given row from the left `from_item` doesn't join to any row
in the _right_ `from_item`, the row will return with `NULL` values for all
columns exclusively from the right `from_item`. Rows from the right
`from_item` that don't join to any row in the left `from_item` are discarded.

```sql
FROM A LEFT OUTER JOIN B ON A.w = B.y

/*
Table A       Table B       Result
+-------+     +-------+     +---------------------------+
| w | x |  *  | y | z |  =  | w    | x    | y    | z    |
+-------+     +-------+     +---------------------------+
| 1 | a |     | 2 | k |     | 1    | a    | NULL | NULL |
| 2 | b |     | 3 | m |     | 2    | b    | 2    | k    |
| 3 | c |     | 3 | n |     | 3    | c    | 3    | m    |
| 3 | d |     | 4 | p |     | 3    | c    | 3    | n    |
+-------+     +-------+     | 3    | d    | 3    | m    |
                            | 3    | d    | 3    | n    |
                            +---------------------------+
*/
```

```sql
FROM A LEFT OUTER JOIN B USING (x)

/*
Table A       Table B       Result
+-------+     +-------+     +--------------------+
| x | y |  *  | x | z |  =  | x    | y    | z    |
+-------+     +-------+     +--------------------+
| 1 | a |     | 2 | k |     | 1    | a    | NULL |
| 2 | b |     | 3 | m |     | 2    | b    | k    |
| 3 | c |     | 3 | n |     | 3    | c    | m    |
| 3 | d |     | 4 | p |     | 3    | c    | n    |
+-------+     +-------+     | 3    | d    | m    |
                            | 3    | d    | n    |
                            +--------------------+
*/
```

**Example**

This query performs a `LEFT JOIN` on the [`Roster`][roster-table]
and [`TeamMascot`][teammascot-table] tables.

```sql
SELECT Roster.LastName, TeamMascot.Mascot
FROM Roster LEFT JOIN TeamMascot ON Roster.SchoolID = TeamMascot.SchoolID;

/*---------------------------+
 | LastName   | Mascot       |
 +---------------------------+
 | Adams      | Jaguars      |
 | Buchanan   | Lakers       |
 | Coolidge   | Lakers       |
 | Davis      | Knights      |
 | Eisenhower | NULL         |
 +---------------------------*/
```

### `RIGHT [OUTER] JOIN`

The result of a `RIGHT OUTER JOIN` (or simply `RIGHT JOIN`) for two
`from_item`s always retains all rows of the right `from_item` in the
`JOIN` operation, even if no rows in the left `from_item` satisfy the join
predicate.

All rows from the _right_ `from_item` are returned;
if a given row from the right `from_item` doesn't join to any row
in the _left_ `from_item`, the row will return with `NULL` values for all
columns exclusively from the left `from_item`. Rows from the left `from_item`
that don't join to any row in the right `from_item` are discarded.

```sql
FROM A RIGHT OUTER JOIN B ON A.w = B.y

/*
Table A       Table B       Result
+-------+     +-------+     +---------------------------+
| w | x |  *  | y | z |  =  | w    | x    | y    | z    |
+-------+     +-------+     +---------------------------+
| 1 | a |     | 2 | k |     | 2    | b    | 2    | k    |
| 2 | b |     | 3 | m |     | 3    | c    | 3    | m    |
| 3 | c |     | 3 | n |     | 3    | c    | 3    | n    |
| 3 | d |     | 4 | p |     | 3    | d    | 3    | m    |
+-------+     +-------+     | 3    | d    | 3    | n    |
                            | NULL | NULL | 4    | p    |
                            +---------------------------+
*/
```

```sql
FROM A RIGHT OUTER JOIN B USING (x)

/*
Table A       Table B       Result
+-------+     +-------+     +--------------------+
| x | y |  *  | x | z |  =  | x    | y    | z    |
+-------+     +-------+     +--------------------+
| 1 | a |     | 2 | k |     | 2    | b    | k    |
| 2 | b |     | 3 | m |     | 3    | c    | m    |
| 3 | c |     | 3 | n |     | 3    | c    | n    |
| 3 | d |     | 4 | p |     | 3    | d    | m    |
+-------+     +-------+     | 3    | d    | n    |
                            | 4    | NULL | p    |
                            +--------------------+
*/
```

**Example**

This query performs a `RIGHT JOIN` on the [`Roster`][roster-table]
and [`TeamMascot`][teammascot-table] tables.

```sql
SELECT Roster.LastName, TeamMascot.Mascot
FROM Roster RIGHT JOIN TeamMascot ON Roster.SchoolID = TeamMascot.SchoolID;

/*---------------------------+
 | LastName   | Mascot       |
 +---------------------------+
 | Adams      | Jaguars      |
 | Buchanan   | Lakers       |
 | Coolidge   | Lakers       |
 | Davis      | Knights      |
 | NULL       | Mustangs     |
 +---------------------------*/
```

### `LATERAL` join

```sql
from_item { CROSS JOIN | [INNER] JOIN | LEFT [OUTER] JOIN } LATERAL from_item [ join_condition ]
from_item , LATERAL from_item
```

A `LATERAL` join enables a right `from_item`
(typically a subquery, an
[`UNNEST` operator][unnest-operator] operation, or a
[table-valued function (TVF)][tvf-concepts])
to reference columns from a left `from_item` that precedes
it in the `FROM` clause. The right `from_item` is evaluated for each row of the
left `from_item`.

**Key Characteristics:**

- **Correlation**: The primary purpose of `LATERAL` is to enable correlated
  subqueries in the `FROM` clause. The subquery or TVF on the right side of
  the `LATERAL` join can depend on values from the current row of the table on
  its left.
- **Row-wise evaluation**: The right side is logically re-evaluated for each
  row of the left side. Note that re-evaluation is not guaranteed. For
  example, when multiple rows from the left input provide identical values for
  the columns referenced by the right input, engines are free to choose
  whether to re-evaluate the computed right side or reuse the same computed
  relation. In other words, the computed right input is not guaranteed to
  reuse or regenerate volatile expressions such as RAND().
- **Join types**: You can use `LATERAL` with `INNER JOIN`, `LEFT OUTER JOIN`,
  and `CROSS JOIN` (often implied by a comma). `LATERAL` is **not** allowed
  with `RIGHT OUTER JOIN` nor `FULL OUTER JOIN`.

**Behavior with join types:**

- `CROSS JOIN LATERAL` (or with comma: `, LATERAL`): If the lateral
  subquery or TVF produces no rows for a given row from the left input, that
  row is excluded from the final result.
- `INNER JOIN LATERAL`: Similar to `CROSS JOIN`, but applies the condition in
  the `ON` clause as a filter on the `LATERAL` join.
- `LEFT [OUTER] JOIN LATERAL`: If the lateral subquery/TVF produces no rows
  for a given row, the row is included in the result, with `NULL`s for columns
  originating from the lateral subquery/TVF. `LATERAL` allows `LEFT JOIN` to
  omit the `ON` clause (which is equivalent to `LEFT JOIN LATERAL ... ON
true`)

**Example**

These examples include statements which perform queries on the
[`Roster`][roster-table] and [`TeamMascot`][teammascot-table], and
[`PlayerStats`][playerstats-table] tables.

The first query aims to find, for each school, the opponent player who scored
the highest points against this school.

```

SELECT R.SchoolID, OP.LastName AS TopOpPlayer, OP.PointsScored
FROM Roster AS R,
     LATERAL (
      SELECT PS.LastName, PS.PointsScored
      FROM PlayerStats AS PS
      WHERE PS.OpponentID = R.SchoolID
      ORDER BY PointsScored DESC
      LIMIT 1
     ) AS OP
ORDER BY R.SchoolID

/*
Result (using implicit CROSS JOIN with LATERAL):
+----------+---------------+--------------+
| SchoolID | TopOpPlayer   | PointsScored |
+----------+---------------+--------------+
| 50       | Buchanan      | 13           |
| 51       | Adams         | 3            |
| 52       | Adams         | 4            |
| 57       | Coolidge      | 1            |
+----------+---------------+--------------+
*/
```

Using `LEFT JOIN LATERAL`:

```

SELECT R.LastName, R.SchoolID, M.Mascot FROM Roster AS R LEFT JOIN LATERAL (
SELECT Mascot FROM TeamMascot m WHERE m.SchoolID = R.SchoolI ) AS M ORDER BY
R.LastName;

/* SchoolID 77 has no mascot listed in the TeamMascot table. Because the join is
`LEFT OUTER`, players from schoolID 77 still shows up in the output, with `NULL`
padding.

Result: +------------+----------+---------+ | LastName | SchoolID | Mascot |
+------------+----------+---------+ | Adams | 50 | Jaguars | | Buchanan | 52 |
Lakers | | Coolidge | 52 | Lakers | | Davis | 51 | Knights | | Eisenhower | 77 |
NULL | +------------+----------+---------+ */
```

**Restrictions and notes:**

- The `LATERAL` keyword is necessary to enable the correlation for the
  subquery or TVF on the right.
- The right side of `LATERAL` is typically a subquery or a TVF call. It can
  also be an [`UNNEST` operator][unnest-operator] referencing columns from the
  left side.
- Ensure that the correlated columns are correctly scoped and available from
  the left `from_item`.
- `LATERAL` cannot be used on the first or leftmost item in a join or a
  parenthesized join.
- `LATERAL` cannot be used with RIGHT or FULL join.
- The `LATERAL` input on the right side can't be followed by a postfix
  operator (`TABLESAMPLE`, `PIVOT`, etc.)

### Join conditions

In a [join operation][query-joins], a join condition helps specify how to
combine rows in two `from_items` to form a single source.

The two types of join conditions are the [`ON` clause][on-clause] and
[`USING` clause][using-clause]. You must use a join condition when you perform a
conditional join operation. You can't use a join condition when you perform a
cross join operation.

#### `ON` clause

```sql
ON bool_expression
```

**Description**

Given a row from each table, if the `ON` clause evaluates to `TRUE`, the query
generates a consolidated row with the result of combining the given rows.

Definitions:

- `bool_expression`: The boolean expression that specifies the condition for
  the join. This is frequently a [comparison operation][comparison-operators] or
  logical combination of comparison operators.

Details:

Similarly to `CROSS JOIN`, `ON` produces a column once for each column in each
input table.

A `NULL` join condition evaluation is equivalent to a `FALSE` evaluation.

If a column-order sensitive operation such as `UNION` or `SELECT *` is used with
the `ON` join condition, the resulting table contains all of the columns from
the left input in order, and then all of the columns from the right input in
order.

**Examples**

The following examples show how to use the `ON` clause:

```sql
WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3),
  B AS ( SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4)
SELECT * FROM A INNER JOIN B ON A.x = B.x;

WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3),
  B AS ( SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4)
SELECT A.x, B.x FROM A INNER JOIN B ON A.x = B.x;

/*
Table A   Table B   Result (A.x, B.x)
+---+     +---+     +-------+
| x |  *  | x |  =  | x | x |
+---+     +---+     +-------+
| 1 |     | 2 |     | 2 | 2 |
| 2 |     | 3 |     | 3 | 3 |
| 3 |     | 4 |     +-------+
+---+     +---+
*/
```

```sql
WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT * FROM A LEFT OUTER JOIN B ON A.x = B.x;

WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT A.x, B.x FROM A LEFT OUTER JOIN B ON A.x = B.x;

/*
Table A    Table B   Result
+------+   +---+     +-------------+
| x    | * | x |  =  | x    | x    |
+------+   +---+     +-------------+
| 1    |   | 2 |     | 1    | NULL |
| 2    |   | 3 |     | 2    | 2    |
| 3    |   | 4 |     | 3    | 3    |
| NULL |   | 5 |     | NULL | NULL |
+------+   +---+     +-------------+
*/
```

```sql
WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT * FROM A FULL OUTER JOIN B ON A.x = B.x;

WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT A.x, B.x FROM A FULL OUTER JOIN B ON A.x = B.x;

/*
Table A    Table B   Result
+------+   +---+     +-------------+
| x    | * | x |  =  | x    | x    |
+------+   +---+     +-------------+
| 1    |   | 2 |     | 1    | NULL |
| 2    |   | 3 |     | 2    | 2    |
| 3    |   | 4 |     | 3    | 3    |
| NULL |   | 5 |     | NULL | NULL |
+------+   +---+     | NULL | 4    |
                     | NULL | 5    |
                     +-------------+
*/
```

#### `USING` clause

```sql
USING ( column_name_list )

column_name_list:
    column_name[, ...]
```

**Description**

When you are joining two tables, `USING` performs an
[equality comparison operation][comparison-operators] on the columns named in
`column_name_list`. Each column name in `column_name_list` must appear in both
input tables. For each pair of rows from the input tables, if the
equality comparisons all evaluate to `TRUE`, one row is added to the resulting
column.

Definitions:

- `column_name_list`: A list of columns to include in the join condition.
- `column_name`: The column that exists in both of the tables that you are
  joining.

Details:

A `NULL` join condition evaluation is equivalent to a `FALSE` evaluation.

If a column-order sensitive operation such as `UNION` or `SELECT *` is used
with the `USING` join condition, the resulting table contains columns in this
order:

- The columns from `column_name_list` in the order they appear in the `USING`
  clause.
- All other columns of the left input in the order they appear in the input.
- All other columns of the right input in the order they appear in the input.

A column name in the `USING` clause must not be qualified by a
table name.

If the join is an `INNER JOIN` or a `LEFT OUTER JOIN`, the output
columns are populated from the values in the first table. If the
join is a `RIGHT OUTER JOIN`, the output columns are populated from the values
in the second table. If the join is a `FULL OUTER JOIN`, the output columns
are populated by [coalescing][coalesce] the values from the left and right
tables in that order.

**Examples**

The following example shows how to use the `USING` clause with one
column name in the column name list:

```sql
WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 9 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 9 UNION ALL SELECT 9 UNION ALL SELECT 5)
SELECT * FROM A INNER JOIN B USING (x);

/*
Table A    Table B   Result
+------+   +---+     +---+
| x    | * | x |  =  | x |
+------+   +---+     +---+
| 1    |   | 2 |     | 2 |
| 2    |   | 9 |     | 9 |
| 9    |   | 9 |     | 9 |
| NULL |   | 5 |     +---+
+------+   +---+
*/
```

The following example shows how to use the `USING` clause with
multiple column names in the column name list:

```sql
WITH
  A AS (
    SELECT 1 as x, 15 as y UNION ALL
    SELECT 2, 10 UNION ALL
    SELECT 9, 16 UNION ALL
    SELECT NULL, 12),
  B AS (
    SELECT 2 as x, 10 as y UNION ALL
    SELECT 9, 17 UNION ALL
    SELECT 9, 16 UNION ALL
    SELECT 5, 15)
SELECT * FROM A INNER JOIN B USING (x, y);

/*
Table A         Table B        Result
+-----------+   +---------+     +---------+
| x    | y  | * | x  | y  |  =  | x  | y  |
+-----------+   +---------+     +---------+
| 1    | 15 |   | 2  | 10 |     | 2  | 10 |
| 2    | 10 |   | 9  | 17 |     | 9  | 16 |
| 9    | 16 |   | 9  | 16 |     +---------+
| NULL | 12 |   | 5  | 15 |
+-----------+   +---------+
*/
```

The following examples show additional ways in which to use the `USING` clause
with one column name in the column name list:

```sql
WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 9 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 9 UNION ALL SELECT 9 UNION ALL SELECT 5)
SELECT x, A.x, B.x FROM A INNER JOIN B USING (x)

/*
Table A    Table B   Result
+------+   +---+     +--------------------+
| x    | * | x |  =  | x    | A.x  | B.x  |
+------+   +---+     +--------------------+
| 1    |   | 2 |     | 2    | 2    | 2    |
| 2    |   | 9 |     | 9    | 9    | 9    |
| 9    |   | 9 |     | 9    | 9    | 9    |
| NULL |   | 5 |     +--------------------+
+------+   +---+
*/
```

```sql
WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 9 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 9 UNION ALL SELECT 9 UNION ALL SELECT 5)
SELECT x, A.x, B.x FROM A LEFT OUTER JOIN B USING (x)

/*
Table A    Table B   Result
+------+   +---+     +--------------------+
| x    | * | x |  =  | x    | A.x  | B.x  |
+------+   +---+     +--------------------+
| 1    |   | 2 |     | 1    | 1    | NULL |
| 2    |   | 9 |     | 2    | 2    | 2    |
| 9    |   | 9 |     | 9    | 9    | 9    |
| NULL |   | 5 |     | 9    | 9    | 9    |
+------+   +---+     | NULL | NULL | NULL |
                     +--------------------+
*/
```

```sql
WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 2 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 9 UNION ALL SELECT 9 UNION ALL SELECT 5)
SELECT x, A.x, B.x FROM A RIGHT OUTER JOIN B USING (x)

/*
Table A    Table B   Result
+------+   +---+     +--------------------+
| x    | * | x |  =  | x    | A.x  | B.x  |
+------+   +---+     +--------------------+
| 1    |   | 2 |     | 2    | 2    | 2    |
| 2    |   | 9 |     | 2    | 2    | 2    |
| 2    |   | 9 |     | 9    | NULL | 9    |
| NULL |   | 5 |     | 9    | NULL | 9    |
+------+   +---+     | 5    | NULL | 5    |
                     +--------------------+
*/
```

```sql
WITH
  A AS ( SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 2 UNION ALL SELECT NULL),
  B AS ( SELECT 2 as x UNION ALL SELECT 9 UNION ALL SELECT 9 UNION ALL SELECT 5)
SELECT x, A.x, B.x FROM A FULL OUTER JOIN B USING (x);

/*
Table A    Table B   Result
+------+   +---+     +--------------------+
| x    | * | x |  =  | x    | A.x  | B.x  |
+------+   +---+     +--------------------+
| 1    |   | 2 |     | 1    | 1    | NULL |
| 2    |   | 9 |     | 2    | 2    | 2    |
| 2    |   | 9 |     | 2    | 2    | 2    |
| NULL |   | 5 |     | NULL | NULL | NULL |
+------+   +---+     | 9    | NULL | 9    |
                     | 9    | NULL | 9    |
                     | 5    | NULL | 5    |
                     +--------------------+
*/
```

The following example shows how to use the `USING` clause with
only some column names in the column name list.

```sql
WITH
  A AS (
    SELECT 1 as x, 15 as y UNION ALL
    SELECT 2, 10 UNION ALL
    SELECT 9, 16 UNION ALL
    SELECT NULL, 12),
  B AS (
    SELECT 2 as x, 10 as y UNION ALL
    SELECT 9, 17 UNION ALL
    SELECT 9, 16 UNION ALL
    SELECT 5, 15)
SELECT * FROM A INNER JOIN B USING (x);

/*
Table A         Table B         Result
+-----------+   +---------+     +-----------------+
| x    | y  | * | x  | y  |  =  | x   | A.y | B.y |
+-----------+   +---------+     +-----------------+
| 1    | 15 |   | 2  | 10 |     | 2   | 10  | 10  |
| 2    | 10 |   | 9  | 17 |     | 9   | 16  | 17  |
| 9    | 16 |   | 9  | 16 |     | 9   | 16  | 16  |
| NULL | 12 |   | 5  | 15 |     +-----------------+
+-----------+   +---------+
*/
```

The following query performs an `INNER JOIN` on the
[`Roster`][roster-table] and [`TeamMascot`][teammascot-table] table.
The query returns the rows from `Roster` and `TeamMascot` where
`Roster.SchoolID` is the same as `TeamMascot.SchoolID`. The results include a
single `SchoolID` column.

```sql
SELECT * FROM Roster INNER JOIN TeamMascot USING (SchoolID);

/*----------------------------------------+
 | SchoolID   | LastName   | Mascot       |
 +----------------------------------------+
 | 50         | Adams      | Jaguars      |
 | 52         | Buchanan   | Lakers       |
 | 52         | Coolidge   | Lakers       |
 | 51         | Davis      | Knights      |
 +----------------------------------------*/
```

#### `ON` and `USING` equivalency

The [`ON`][on-clause] and [`USING`][using-clause] join conditions aren't
equivalent, but they share some rules and sometimes they can produce similar
results.

In the following examples, observe what is returned when all rows
are produced for inner and outer joins. Also, look at how
each join condition handles `NULL` values.

```sql
WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4)
SELECT * FROM A INNER JOIN B ON A.x = B.x;

WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4)
SELECT * FROM A INNER JOIN B USING (x);

/*
Table A   Table B   Result ON     Result USING
+---+     +---+     +-------+     +---+
| x |  *  | x |  =  | x | x |     | x |
+---+     +---+     +-------+     +---+
| 1 |     | 2 |     | 2 | 2 |     | 2 |
| 2 |     | 3 |     | 3 | 3 |     | 3 |
| 3 |     | 4 |     +-------+     +---+
+---+     +---+
*/
```

```sql
WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT * FROM A LEFT OUTER JOIN B ON A.x = B.x;

WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT * FROM A LEFT OUTER JOIN B USING (x);

/*
Table A    Table B   Result ON           Result USING
+------+   +---+     +-------------+     +------+
| x    | * | x |  =  | x    | x    |     | x    |
+------+   +---+     +-------------+     +------+
| 1    |   | 2 |     | 1    | NULL |     | 1    |
| 2    |   | 3 |     | 2    | 2    |     | 2    |
| 3    |   | 4 |     | 3    | 3    |     | 3    |
| NULL |   | 5 |     | NULL | NULL |     | NULL |
+------+   +---+     +-------------+     +------+
*/
```

```sql
WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4)
SELECT * FROM A FULL OUTER JOIN B ON A.x = B.x;

WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4)
SELECT * FROM A FULL OUTER JOIN B USING (x);

/*
Table A   Table B   Result ON           Result USING
+---+     +---+     +-------------+     +---+
| x |  *  | x |  =  | x    | x    |     | x |
+---+     +---+     +-------------+     +---+
| 1 |     | 2 |     | 1    | NULL |     | 1 |
| 2 |     | 3 |     | 2    | 2    |     | 2 |
| 3 |     | 4 |     | 3    | 3    |     | 3 |
+---+     +---+     | NULL | 4    |     | 4 |
                    +-------------+     +---+
*/
```

Although `ON` and `USING` aren't equivalent, they can return the same
results in some situations if you specify the columns you want to return.

In the following examples, observe what is returned when a specific row
is produced for inner and outer joins. Also, look at how each
join condition handles `NULL` values.

```sql
WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT A.x FROM A INNER JOIN B ON A.x = B.x;

WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT x FROM A INNER JOIN B USING (x);

/*
Table A    Table B   Result ON     Result USING
+------+   +---+     +---+         +---+
| x    | * | x |  =  | x |         | x |
+------+   +---+     +---+         +---+
| 1    |   | 2 |     | 2 |         | 2 |
| 2    |   | 3 |     | 3 |         | 3 |
| 3    |   | 4 |     +---+         +---+
| NULL |   | 5 |
+------+   +---+
*/
```

```sql
WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT A.x FROM A LEFT OUTER JOIN B ON A.x = B.x;

WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT x FROM A LEFT OUTER JOIN B USING (x);

/*
Table A    Table B   Result ON    Result USING
+------+   +---+     +------+     +------+
| x    | * | x |  =  | x    |     | x    |
+------+   +---+     +------+     +------+
| 1    |   | 2 |     | 1    |     | 1    |
| 2    |   | 3 |     | 2    |     | 2    |
| 3    |   | 4 |     | 3    |     | 3    |
| NULL |   | 5 |     | NULL |     | NULL |
+------+   +---+     +------+     +------+
*/
```

```sql
WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT A.x FROM A FULL OUTER JOIN B ON A.x = B.x;

WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT x FROM A FULL OUTER JOIN B USING (x);

/*
Table A    Table B   Result ON    Result USING
+------+   +---+     +------+     +------+
| x    | * | x |  =  | x    |     | x    |
+------+   +---+     +------+     +------+
| 1    |   | 2 |     | 1    |     | 1    |
| 2    |   | 3 |     | 2    |     | 2    |
| 3    |   | 4 |     | 3    |     | 3    |
| NULL |   | 5 |     | NULL |     | NULL |
+------+   +---+     | NULL |     | 4    |
                     | NULL |     | 5    |
                     +------+     +------+
*/
```

```sql
WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT B.x FROM A FULL OUTER JOIN B ON A.x = B.x;

WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT x FROM A FULL OUTER JOIN B USING (x);

/*
Table A    Table B   Result ON    Result USING
+------+   +---+     +------+     +------+
| x    | * | x |  =  | x    |     | x    |
+------+   +---+     +------+     +------+
| 1    |   | 2 |     | 2    |     | 1    |
| 2    |   | 3 |     | 3    |     | 2    |
| 3    |   | 4 |     | NULL |     | 3    |
| NULL |   | 5 |     | NULL |     | NULL |
+------+   +---+     | 4    |     | 4    |
                     | 5    |     | 5    |
                     +------+     +------+
*/
```

In the following example, observe what is returned when `COALESCE` is used
with the `ON` clause. It provides the same results as a query
with the `USING` clause.

```sql
WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT COALESCE(A.x, B.x) FROM A FULL OUTER JOIN B ON A.x = B.x;

WITH
  A AS (SELECT 1 as x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT NULL),
  B AS (SELECT 2 as x UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5)
SELECT x FROM A FULL OUTER JOIN B USING (x);

/*
Table A    Table B   Result ON    Result USING
+------+   +---+     +------+     +------+
| x    | * | x |  =  | x    |     | x    |
+------+   +---+     +------+     +------+
| 1    |   | 2 |     | 1    |     | 1    |
| 2    |   | 3 |     | 2    |     | 2    |
| 3    |   | 4 |     | 3    |     | 3    |
| NULL |   | 5 |     | NULL |     | NULL |
+------+   +---+     | 4    |     | 4    |
                     | 5    |     | 5    |
                     +------+     +------+
*/
```

### Join operations in a sequence

The `FROM` clause can contain multiple `JOIN` operations in a sequence.
`JOIN`s are bound from left to right. For example:

```sql
FROM A JOIN B USING (x) JOIN C USING (x)

-- A JOIN B USING (x)        = result_1
-- result_1 JOIN C USING (x) = result_2
-- result_2                  = return value
```

You can also insert parentheses to group `JOIN`s:

```sql
FROM ( (A JOIN B USING (x)) JOIN C USING (x) )

-- A JOIN B USING (x)        = result_1
-- result_1 JOIN C USING (x) = result_2
-- result_2                  = return value
```

With parentheses, you can group `JOIN`s so that they are bound in a different
order:

```sql
FROM ( A JOIN (B JOIN C USING (x)) USING (x) )

-- B JOIN C USING (x)       = result_1
-- A JOIN result_1          = result_2
-- result_2                 = return value
```

A `FROM` clause can have multiple joins. Provided there are no comma cross joins
in the `FROM` clause, joins don't require parenthesis, though parenthesis can
help readability:

```sql
FROM A JOIN B JOIN C JOIN D USING (w) ON B.x = C.y ON A.z = B.x
```

If your clause contains comma cross joins, you must use parentheses:

```sql {.bad}
FROM A, B JOIN C JOIN D ON C.x = D.y ON B.z = C.x    // INVALID
```

```sql
FROM A, B JOIN (C JOIN D ON C.x = D.y) ON B.z = C.x  // VALID
```

When comma cross joins are present in a query with a sequence of JOINs, they
group from left to right like other `JOIN` types:

```sql
FROM A JOIN B USING (x) JOIN C USING (x), D

-- A JOIN B USING (x)        = result_1
-- result_1 JOIN C USING (x) = result_2
-- result_2 CROSS JOIN D     = return value
```

There can't be a `RIGHT JOIN` or `FULL JOIN` after a comma cross join unless
it's parenthesized:

```sql {.bad}
FROM A, B RIGHT JOIN C ON TRUE // INVALID
```

```sql {.bad}
FROM A, B FULL JOIN C ON TRUE  // INVALID
```

```sql
FROM A, B JOIN C ON TRUE       // VALID
```

```sql
FROM A, (B RIGHT JOIN C ON TRUE) // VALID
```

```sql
FROM A, (B FULL JOIN C ON TRUE)  // VALID
```

### Correlated join operation

A join operation is _correlated_ when the right `from_item` contains a
reference to at least one range variable or
column name introduced by the left `from_item`.

In a correlated join operation, rows from the right `from_item` are determined
by a row from the left `from_item`. Consequently, `RIGHT OUTER` and `FULL OUTER`
joins can't be correlated because right `from_item` rows can't be determined
in the case when there is no row from the left `from_item`.

All correlated join operations must reference an array in the right `from_item`.

This is a conceptual example of a correlated join operation that includes
a [correlated subquery][correlated-subquery]:

```sql
FROM A JOIN UNNEST(ARRAY(SELECT AS STRUCT * FROM B WHERE A.ID = B.ID)) AS C
```

- Left `from_item`: `A`
- Right `from_item`: `UNNEST(...) AS C`
- A correlated subquery: `(SELECT AS STRUCT * FROM B WHERE A.ID = B.ID)`

This is another conceptual example of a correlated join operation.
`array_of_IDs` is part of the left `from_item` but is referenced in the
right `from_item`.

```sql
FROM A JOIN UNNEST(A.array_of_IDs) AS C
```

The [`UNNEST` operator][unnest-operator] can be explicit or implicit.
These are both allowed:

```sql
FROM A JOIN UNNEST(A.array_of_IDs) AS IDs
```

```sql
FROM A JOIN A.array_of_IDs AS IDs
```

In a correlated join operation, the right `from_item` is re-evaluated
against each distinct row from the left `from_item`. In the following
conceptual example, the correlated join operation first
evaluates `A` and `B`, then `A` and `C`:

```sql
FROM
  A
  JOIN
  UNNEST(ARRAY(SELECT AS STRUCT * FROM B WHERE A.ID = B.ID)) AS C
  ON A.Name = C.Name
```

**Caveats**

- In a correlated `LEFT JOIN`, when the input table on the right side is empty
  for some row from the left side, it's as if no rows from the right side
  satisfied the join condition in a regular `LEFT JOIN`. When there are no
  joining rows, a row with `NULL` values for all columns on the right side is
  generated to join with the row from the left side.
- In a correlated `CROSS JOIN`, when the input table on the right side is
  empty for some row from the left side, it's as if no rows from the right
  side satisfied the join condition in a regular correlated `INNER JOIN`. This
  means that the row is dropped from the results.

**Examples**

This is an example of a correlated join, using the
[Roster][roster-table] and [PlayerStats][playerstats-table] tables:

```sql
SELECT *
FROM
  Roster
JOIN
  UNNEST(
    ARRAY(
      SELECT AS STRUCT *
      FROM PlayerStats
      WHERE PlayerStats.OpponentID = Roster.SchoolID
    )) AS PlayerMatches
  ON PlayerMatches.LastName = 'Buchanan'

/*------------+----------+----------+------------+--------------+
 | LastName   | SchoolID | LastName | OpponentID | PointsScored |
 +------------+----------+----------+------------+--------------+
 | Adams      | 50       | Buchanan | 50         | 13           |
 | Eisenhower | 77       | Buchanan | 77         | 0            |
 +------------+----------+----------+------------+--------------*/
```

A common pattern for a correlated `LEFT JOIN` is to have an `UNNEST` operation
on the right side that references an array from some column introduced by
input on the left side. For rows where that array is empty or `NULL`,
the `UNNEST` operation produces no rows on the right input. In that case, a row
with a `NULL` entry in each column of the right input is created to join with
the row from the left input. For example:

```sql
SELECT A.name, item, ARRAY_LENGTH(A.items) item_count_for_name
FROM
  UNNEST(
    [
      STRUCT(
        'first' AS name,
        [1, 2, 3, 4] AS items),
      STRUCT(
        'second' AS name,
        [] AS items)]) AS A
LEFT JOIN
  A.items AS item;

/*--------+------+---------------------+
 | name   | item | item_count_for_name |
 +--------+------+---------------------+
 | first  | 1    | 4                   |
 | first  | 2    | 4                   |
 | first  | 3    | 4                   |
 | first  | 4    | 4                   |
 | second | NULL | 0                   |
 +--------+------+---------------------*/
```

In the case of a correlated `INNER JOIN` or `CROSS JOIN`, when the input on the
right side is empty for some row from the left side, the final row is dropped
from the results. For example:

```sql
SELECT A.name, item
FROM
  UNNEST(
    [
      STRUCT(
        'first' AS name,
        [1, 2, 3, 4] AS items),
      STRUCT(
        'second' AS name,
        [] AS items)]) AS A
INNER JOIN
  A.items AS item;

/*-------+------+
 | name  | item |
 +-------+------+
 | first | 1    |
 | first | 2    |
 | first | 3    |
 | first | 4    |
 +-------+------*/
```
