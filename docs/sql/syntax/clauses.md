# Clauses

## `WHERE` clause

```sql
WHERE bool_expression
```

The `WHERE` clause filters the results of the `FROM` clause.

Only rows whose `bool_expression` evaluates to `TRUE` are included. Rows
whose `bool_expression` evaluates to `NULL` or `FALSE` are
discarded.

The evaluation of a query with a `WHERE` clause is typically completed in this
order:

- `FROM`
- `WHERE`
- `GROUP BY` and aggregation
- `HAVING`
- `WINDOW`
- `QUALIFY`
- `DISTINCT`
- `ORDER BY`
- `LIMIT`

Evaluation order doesn't always match syntax order.

The `WHERE` clause only references columns available via the `FROM` clause;
it can't reference `SELECT` list aliases.

**Examples**

This query returns returns all rows from the [`Roster`][roster-table] table
where the `SchoolID` column has the value `52`:

```sql
SELECT * FROM Roster
WHERE SchoolID = 52;
```

The `bool_expression` can contain multiple sub-conditions:

```sql
SELECT * FROM Roster
WHERE STARTS_WITH(LastName, "Mc") OR STARTS_WITH(LastName, "Mac");
```

Expressions in an `INNER JOIN` have an equivalent expression in the
`WHERE` clause. For example, a query using `INNER` `JOIN` and `ON` has an
equivalent expression using `CROSS JOIN` and `WHERE`. For example,
the following two queries are equivalent:

```sql
SELECT Roster.LastName, TeamMascot.Mascot
FROM Roster INNER JOIN TeamMascot
ON Roster.SchoolID = TeamMascot.SchoolID;
```

```sql
SELECT Roster.LastName, TeamMascot.Mascot
FROM Roster CROSS JOIN TeamMascot
WHERE Roster.SchoolID = TeamMascot.SchoolID;
```

## `GROUP BY` clause

```sql
GROUP BY group_by_specification

group_by_specification:
  {
    groupable_items
    | ALL
    | grouping_sets_specification
    | rollup_specification
    | cube_specification
    | ()
  }
```

**Description**

The `GROUP BY` clause groups together rows in a table that share common values
for certain columns. For a group of rows in the source table with
non-distinct values, the `GROUP BY` clause aggregates them into a single
combined row. This clause is commonly used when aggregate functions are
present in the `SELECT` list, or to eliminate redundancy in the output.

**Definitions**

- `groupable_items`: Group rows in a table that share common values
  for certain columns. To learn more, see
  [Group rows by groupable items][group-by-groupable-item].
- `ALL`: Automatically group rows. To learn more, see
  [Group rows automatically][group-by-all].
- `grouping_sets_specification`: Group rows with the
  `GROUP BY GROUPING SETS` clause. To learn more, see
  [Group rows by `GROUPING SETS`][group-by-grouping-sets].
- `rollup_specification`: Group rows with the `GROUP BY ROLLUP` clause.
  To learn more, see [Group rows by `ROLLUP`][group-by-rollup].
- `cube_specification`: Group rows with the `GROUP BY CUBE` clause.
  To learn more, see [Group rows by `CUBE`][group-by-cube].
- `()`: Group all rows and produce a grand total. Equivalent to no
  `group_by_specification`.

### Group rows by groupable items

```sql
GROUP BY groupable_item[, ...]

groupable_item:
  {
    value
    | value_alias
    | column_ordinal
  }
```

**Description**

The `GROUP BY` clause can include [groupable][data-type-properties] expressions
and their ordinals.

**Definitions**

- `value`: An expression that represents a non-distinct, groupable value.
  To learn more, see [Group rows by values][group-by-values].
- `value_alias`: An alias for `value`.
  To learn more, see [Group rows by values][group-by-values].
- `column_ordinal`: An `BIGINT` value that represents the ordinal assigned to a
  groupable expression in the `SELECT` list.
  To learn more, see [Group rows by column ordinals][group-by-col-ordinals].

#### Group rows by values

The `GROUP BY` clause can group rows in a table with non-distinct
values in the `GROUP BY` clause. For example:

```sql
WITH PlayerStats AS (
  SELECT 'Adams' as LastName, 'Noam' as FirstName, 3 as PointsScored UNION ALL
  SELECT 'Buchanan', 'Jie', 0 UNION ALL
  SELECT 'Coolidge', 'Kiran', 1 UNION ALL
  SELECT 'Adams', 'Noam', 4 UNION ALL
  SELECT 'Buchanan', 'Jie', 13)
SELECT SUM(PointsScored) AS total_points, LastName
FROM PlayerStats
GROUP BY LastName;

/*--------------+----------+
 | total_points | LastName |
 +--------------+----------+
 | 7            | Adams    |
 | 13           | Buchanan |
 | 1            | Coolidge |
 +--------------+----------*/
```

`GROUP BY` clauses may also refer to aliases. If a query contains aliases in
the `SELECT` clause, those aliases override names in the corresponding `FROM`
clause. For example:

```sql
WITH PlayerStats AS (
  SELECT 'Adams' as LastName, 'Noam' as FirstName, 3 as PointsScored UNION ALL
  SELECT 'Buchanan', 'Jie', 0 UNION ALL
  SELECT 'Coolidge', 'Kiran', 1 UNION ALL
  SELECT 'Adams', 'Noam', 4 UNION ALL
  SELECT 'Buchanan', 'Jie', 13)
SELECT SUM(PointsScored) AS total_points, LastName AS last_name
FROM PlayerStats
GROUP BY last_name;

/*--------------+-----------+
 | total_points | last_name |
 +--------------+-----------+
 | 7            | Adams     |
 | 13           | Buchanan  |
 | 1            | Coolidge  |
 +--------------+-----------*/
```

You can use the `GROUP BY` clause with arrays. The following query executes
because the array elements being grouped are the same length and group type:

```sql
WITH PlayerStats AS (
  SELECT ['Coolidge', 'Adams'] as Name, 3 as PointsScored UNION ALL
  SELECT ['Adams', 'Buchanan'], 0 UNION ALL
  SELECT ['Coolidge', 'Adams'], 1 UNION ALL
  SELECT ['Kiran', 'Noam'], 1)
SELECT SUM(PointsScored) AS total_points, name
FROM PlayerStats
GROUP BY Name;

/*--------------+------------------+
 | total_points | name             |
 +--------------+------------------+
 | 4            | [Coolidge,Adams] |
 | 0            | [Adams,Buchanan] |
 | 1            | [Kiran,Noam]     |
 +--------------+------------------*/
```

You can use the `GROUP BY` clause with structs. The following query executes
because the struct fields being grouped have the same group types:

```sql
WITH
  TeamStats AS (
    SELECT
      ARRAY>[
        ('Adams', 'Noam', 20), ('Buchanan', 'Jie', 19)] AS Team,
      3 AS PointsScored
    UNION ALL
    SELECT [('Coolidge', 'Kiran', 21), ('Yang', 'Jason', 22)], 4
    UNION ALL
    SELECT [('Adams', 'Noam', 20), ('Buchanan', 'Jie', 19)], 10
    UNION ALL
    SELECT [('Coolidge', 'Kiran', 21), ('Yang', 'Jason', 22)], 7
  )
SELECT
  SUM(PointsScored) AS total_points,
  Team
FROM TeamStats
GROUP BY Team;

/*--------------+--------------------------+
 | total_points | teams                    |
 +--------------+--------------------------+
 | 13           | [{                       |
 |              |    last_name: "Adams",   |
 |              |    first_name: "Noam",   |
 |              |    age: 20               |
 |              |  },{                     |
 |              |    last_name: "Buchanan",|
 |              |    first_name: "Jie",    |
 |              |    age: 19               |
 |              |  }]                      |
 +-----------------------------------------+
 | 11           | [{                       |
 |              |    last_name: "Coolidge",|
 |              |    first_name: "Kiran",  |
 |              |    age: 21               |
 |              |  },{                     |
 |              |    last_name: "Yang",    |
 |              |    first_name: "Jason",  |
 |              |    age: 22               |
 |              |  }]                      |
 +--------------+--------------------------*/
```

To learn more about the data types that are supported for values in the
`GROUP BY` clause, see [Groupable data types][data-type-properties].

#### Group rows by column ordinals

The `GROUP BY` clause can refer to expression names in the `SELECT` list. The
`GROUP BY` clause also allows ordinal references to expressions in the `SELECT`
list, using integer values. `1` refers to the first value in the
`SELECT` list, `2` the second, and so forth. The value list can combine
ordinals and value names. The following queries are equivalent:

```sql
WITH PlayerStats AS (
  SELECT 'Adams' as LastName, 'Noam' as FirstName, 3 as PointsScored UNION ALL
  SELECT 'Buchanan', 'Jie', 0 UNION ALL
  SELECT 'Coolidge', 'Kiran', 1 UNION ALL
  SELECT 'Adams', 'Noam', 4 UNION ALL
  SELECT 'Buchanan', 'Jie', 13)
SELECT SUM(PointsScored) AS total_points, LastName, FirstName
FROM PlayerStats
GROUP BY LastName, FirstName;

/*--------------+----------+-----------+
 | total_points | LastName | FirstName |
 +--------------+----------+-----------+
 | 7            | Adams    | Noam      |
 | 13           | Buchanan | Jie       |
 | 1            | Coolidge | Kiran     |
 +--------------+----------+-----------*/
```

```sql
WITH PlayerStats AS (
  SELECT 'Adams' as LastName, 'Noam' as FirstName, 3 as PointsScored UNION ALL
  SELECT 'Buchanan', 'Jie', 0 UNION ALL
  SELECT 'Coolidge', 'Kiran', 1 UNION ALL
  SELECT 'Adams', 'Noam', 4 UNION ALL
  SELECT 'Buchanan', 'Jie', 13)
SELECT SUM(PointsScored) AS total_points, LastName, FirstName
FROM PlayerStats
GROUP BY 2, 3;

/*--------------+----------+-----------+
 | total_points | LastName | FirstName |
 +--------------+----------+-----------+
 | 7            | Adams    | Noam      |
 | 13           | Buchanan | Jie       |
 | 1            | Coolidge | Kiran     |
 +--------------+----------+-----------*/
```

### Group rows by `ALL`

```sql
GROUP BY ALL
```

**Description**

The `GROUP BY ALL` clause groups rows by inferring grouping keys from the
`SELECT` items.

The following `SELECT` items are excluded from the `GROUP BY ALL` clause:

- Expressions that include [aggregate functions][aggregate-function-calls].
- Expressions that include [window functions][window-function-calls].
- Expressions that don't reference a name from the `FROM` clause.
  This includes:
  - Constants
  - Query parameters
  - Correlated column references
  - Expressions that only reference `GROUP BY` keys inferred from
    other `SELECT` items.

After exclusions are applied, an error is produced if any remaining `SELECT`
item includes a volatile function or has a non-groupable type.

If the set of inferred grouping keys is empty after exclusions are applied, all
input rows are considered a single group for aggregation. This
behavior is equivalent to writing `GROUP BY ()`.

**Examples**

In the following example, the query groups rows by `first_name` and
`last_name`. `total_points` is excluded because it represents an
aggregate function.

```sql
WITH PlayerStats AS (
  SELECT 'Adams' as LastName, 'Noam' as FirstName, 3 as PointsScored UNION ALL
  SELECT 'Buchanan', 'Jie', 0 UNION ALL
  SELECT 'Coolidge', 'Kiran', 1 UNION ALL
  SELECT 'Adams', 'Noam', 4 UNION ALL
  SELECT 'Buchanan', 'Jie', 13)
SELECT
  SUM(PointsScored) AS total_points,
  FirstName AS first_name,
  LastName AS last_name
FROM PlayerStats
GROUP BY ALL;

/*--------------+------------+-----------+
 | total_points | first_name | last_name |
 +--------------+------------+-----------+
 | 7            | Noam       | Adams     |
 | 13           | Jie        | Buchanan  |
 | 1            | Kiran      | Coolidge  |
 +--------------+------------+-----------*/
```

If the select list contains an analytic function, the query groups rows by
`first_name` and `last_name`. `total_people` is excluded because it
contains a window function.

```sql
WITH PlayerStats AS (
  SELECT 'Adams' as LastName, 'Noam' as FirstName, 3 as PointsScored UNION ALL
  SELECT 'Buchanan', 'Jie', 0 UNION ALL
  SELECT 'Coolidge', 'Kiran', 1 UNION ALL
  SELECT 'Adams', 'Noam', 4 UNION ALL
  SELECT 'Buchanan', 'Jie', 13)
SELECT
  COUNT(*) OVER () AS total_people,
  FirstName AS first_name,
  LastName AS last_name
FROM PlayerStats
GROUP BY ALL;

/*--------------+------------+-----------+
 | total_people | first_name | last_name |
 +--------------+------------+-----------+
 | 3            | Noam       | Adams     |
 | 3            | Jie        | Buchanan  |
 | 3            | Kiran      | Coolidge  |
 +--------------+------------+-----------*/
```

If multiple `SELECT` items reference the same `FROM` item, and any of them is
a path expression prefix of another, only the prefix path is used for grouping.
In the following example, `coordinates` is excluded because `x_coordinate` and
`y_coordinate` have already referenced `Values.x` and `Values.y` in the
`FROM` clause, and they are prefixes of the path expression used in
`x_coordinate`:

```sql
WITH Values AS (
  SELECT 1 AS x, 2 AS y
  UNION ALL SELECT 1 AS x, 4 AS y
  UNION ALL SELECT 2 AS x, 5 AS y
)
SELECT
  Values.x AS x_coordinate,
  Values.y AS y_coordinate,
  [Values.x, Values.y] AS coordinates
FROM Values
GROUP BY ALL

/*--------------+--------------+-------------+
 | x_coordinate | y_coordinate | coordinates |
 +--------------+--------------+-------------+
 | 1            | 4            | [1, 4]      |
 | 1            | 2            | [1, 2]      |
 | 2            | 5            | [2, 5]      |
 +--------------+--------------+-------------*/
```

In the following example, the inferred set of grouping keys is empty. The query
returns one row even when the input contains zero rows.

```sql
SELECT COUNT(*) AS num_rows
FROM UNNEST([])
GROUP BY ALL

/*----------+
 | num_rows |
 +----------+
 | 0        |
 +----------*/
```

### Group rows by `GROUPING SETS`

```sql
GROUP BY GROUPING SETS ( grouping_list )

grouping_list:
  {
    rollup_specification
    | cube_specification
    | groupable_item
    | groupable_item_set
  }[, ...]

groupable_item_set:
  ( [ groupable_item[, ...] ] )
```

**Description**

The `GROUP BY GROUPING SETS` clause produces aggregated data for one or more
_grouping sets_. A grouping set is a group of columns by which rows can
be grouped together. This clause is helpful if you want to produce
aggregated data for sets of data without using the `UNION` operation.
For example, `GROUP BY GROUPING SETS(x,y)` is roughly equivalent to
`GROUP BY x UNION ALL GROUP BY y`.

**Definitions**

- `grouping_list`: A list of items that you can add to the
  `GROUPING SETS` clause. Grouping sets are generated based upon what is in
  this list.
- `rollup_specification`: Group rows with the `ROLLUP` clause.
  Don't include `GROUP BY` if you use this inside the `GROUPING SETS` clause.
  To learn more, see [Group rows by `ROLLUP`][group-by-rollup].
- `cube_specification`: Group rows with the `CUBE` clause.
  Don't include `GROUP BY` if you use this inside the `GROUPING SETS` clause.
  To learn more, see [Group rows by `CUBE`][group-by-cube].
- `groupable_item`: Group rows in a table that share common values
  for certain columns. To learn more, see
  [Group rows by groupable items][group-by-groupable-item].
  [Anonymous `STRUCT` values][tuple-struct] aren't allowed.
- `groupable_item_set`: Group rows by a set of
  [groupable items][group-by-groupable-item]. If the set contains no
  groupable items, group all rows and produce a grand total.

**Details**

`GROUP BY GROUPING SETS` works by taking a grouping list, generating
grouping sets from it, and then producing a table as a union of queries
grouped by each grouping set.

For example, `GROUP BY GROUPING SETS (a, b, c)` generates the
following grouping sets from the grouping list, `a, b, c`, and
then produces aggregated rows for each of them:

- `(a)`
- `(b)`
- `(c)`

Here is an example that includes groupable item sets in
`GROUP BY GROUPING SETS (a, (b, c), d)`:

| Conceptual grouping sets | Actual grouping sets |
| ------------------------ | -------------------- |
| `(a)`                    | `(a)`                |
| `((b, c))`               | `(b, c)`             |
| `(d)`                    | `(d)`                |

`GROUP BY GROUPING SETS` can include `ROLLUP` and `CUBE` operations, which
generate grouping sets. If `ROLLUP` is added, it generates rolled up
grouping sets. If `CUBE` is added, it generates grouping set permutations.

The following grouping sets are generated for
`GROUP BY GROUPING SETS (a, ROLLUP(b, c), d)`.

| Conceptual grouping sets | Actual grouping sets |
| ------------------------ | -------------------- |
| `(a)`                    | `(a)`                |
| `((b, c))`               | `(b, c)`             |
| `((b))`                  | `(b)`                |
| `(())`                   | `()`                 |
| `(d)`                    | `(d)`                |

The following grouping sets are generated for
`GROUP BY GROUPING SETS (a, CUBE(b, c), d)`:

| Conceptual grouping sets | Actual grouping sets |
| ------------------------ | -------------------- |
| `(a)`                    | `(a)`                |
| `((b, c))`               | `(b, c)`             |
| `((b))`                  | `(b)`                |
| `((c))`                  | `(c)`                |
| `(())`                   | `()`                 |
| `(d)`                    | `(d)`                |

When evaluating the results for a particular grouping set,
expressions that aren't in the grouping set are aggregated and produce a
`NULL` placeholder.

You can filter results for specific groupable items. To learn more, see the
[`GROUPING` function][grouping-function]

**Examples**

The following queries produce the same results, but
the first one uses `GROUP BY GROUPING SETS` and the second one doesn't:

```sql
-- GROUP BY with GROUPING SETS
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY GROUPING SETS (product_type, product_name)
ORDER BY product_name

/*--------------+--------------+-------------+
 | product_type | product_name | product_sum |
 +--------------+--------------+-------------+
 | shirt        | NULL         | 36          |
 | pants        | NULL         | 6           |
 | NULL         | jeans        | 6           |
 | NULL         | polo         | 25          |
 | NULL         | t-shirt      | 11          |
 +--------------+--------------+-------------*/
```

```sql
-- GROUP BY without GROUPING SETS
-- (produces the same results as GROUPING SETS)
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, NULL, SUM(product_count) AS product_sum
FROM Products
GROUP BY product_type
UNION ALL
SELECT NULL, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY product_name
ORDER BY product_name
```

You can include groupable item sets in a `GROUP BY GROUPING SETS` clause.
In the example below, `(product_type, product_name)` is a groupable item set.

```sql
-- GROUP BY with GROUPING SETS and a groupable item set
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY GROUPING SETS (
  product_type,
  (product_type, product_name))
ORDER BY product_type, product_name;

/*--------------+--------------+-------------+
 | product_type | product_name | product_sum |
 +--------------+--------------+-------------+
 | pants        | NULL         | 6           |
 | pants        | jeans        | 6           |
 | shirt        | NULL         | 36          |
 | shirt        | polo         | 25          |
 | shirt        | t-shirt      | 11          |
 +--------------+--------------+-------------*/
```

```sql
-- GROUP BY with GROUPING SETS but without a groupable item set
-- (produces the same results as GROUPING SETS with a groupable item set)
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, NULL, SUM(product_count) AS product_sum
FROM Products
GROUP BY product_type
UNION ALL
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY product_type, product_name
ORDER BY product_type, product_name;
```

You can include [`ROLLUP`][group-by-rollup] in a
`GROUP BY GROUPING SETS` clause. For example:

```sql
-- GROUP BY with GROUPING SETS and ROLLUP
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY GROUPING SETS (
  product_type,
  ROLLUP (product_type, product_name))
ORDER BY product_type, product_name;

/*--------------+--------------+-------------+
 | product_type | product_name | product_sum |
 +--------------+--------------+-------------+
 | NULL         | NULL         | 42          |
 | pants        | NULL         | 6           |
 | pants        | NULL         | 6           |
 | pants        | jeans        | 6           |
 | shirt        | NULL         | 36          |
 | shirt        | NULL         | 36          |
 | shirt        | polo         | 25          |
 | shirt        | t-shirt      | 11          |
 +--------------+--------------+-------------*/
```

```sql
-- GROUP BY with GROUPING SETS, but without ROLLUP
-- (produces the same results as GROUPING SETS with ROLLUP)
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY GROUPING SETS(
  product_type,
  (product_type, product_name),
  product_type,
  ())
ORDER BY product_type, product_name;
```

You can include [`CUBE`][group-by-cube] in a `GROUP BY GROUPING SETS` clause.
For example:

```sql
-- GROUP BY with GROUPING SETS and CUBE
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY GROUPING SETS (
  product_type,
  CUBE (product_type, product_name))
ORDER BY product_type, product_name;

/*--------------+--------------+-------------+
 | product_type | product_name | product_sum |
 +--------------+--------------+-------------+
 | NULL         | NULL         | 42          |
 | NULL         | jeans        | 6           |
 | NULL         | polo         | 25          |
 | NULL         | t-shirt      | 11          |
 | pants        | NULL         | 6           |
 | pants        | NULL         | 6           |
 | pants        | jeans        | 6           |
 | shirt        | NULL         | 36          |
 | shirt        | NULL         | 36          |
 | shirt        | polo         | 25          |
 | shirt        | t-shirt      | 11          |
 +--------------+--------------+-------------*/
```

```sql
-- GROUP BY with GROUPING SETS, but without CUBE
-- (produces the same results as GROUPING SETS with CUBE)
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY GROUPING SETS(
  product_type,
  (product_type, product_name),
  product_type,
  product_name,
  ())
ORDER BY product_type, product_name;
```

### Group rows by `ROLLUP`

```sql
GROUP BY ROLLUP ( grouping_list )

grouping_list:
  { groupable_item | groupable_item_set }[, ...]

groupable_item_set:
  ( groupable_item[, ...] )
```

**Description**

The `GROUP BY ROLLUP` clause produces aggregated data for rolled up
_grouping sets_. A grouping set is a group of columns by which rows can
be grouped together. This clause is helpful if you need to roll up totals
in a set of data.

**Definitions**

- `grouping_list`: A list of items that you can add to the
  `GROUPING SETS` clause. This is used to create a generated list
  of grouping sets when the query is run.
- `groupable_item`: Group rows in a table that share common values
  for certain columns. To learn more, see
  [Group rows by groupable items][group-by-groupable-item].[anonymous `STRUCT` values][tuple-struct]
  aren't allowed.
- `groupable_item_set`: Group rows by a subset of
  [groupable items][group-by-groupable-item].

**Details**

`GROUP BY ROLLUP` works by taking a grouping list, generating
grouping sets from the prefixes inside this list, and then producing a
table as a union of queries grouped by each grouping set. The resulting
grouping sets include an empty grouping set. In the empty grouping set, all
rows are aggregated down to a single group.

For example, `GROUP BY ROLLUP (a, b, c)` generates the
following grouping sets from the grouping list, `a, b, c`, and then produces
aggregated rows for each of them:

- `(a, b, c)`
- `(a, b)`
- `(a)`
- `()`

Here is an example that includes groupable item sets in
`GROUP BY ROLLUP (a, (b, c), d)`:

| Conceptual grouping sets | Actual grouping sets |
| ------------------------ | -------------------- |
| `(a, (b, c), d)`         | `(a, b, c, d)`       |
| `(a, (b, c))`            | `(a, b, c)`          |
| `(a)`                    | `(a)`                |
| `()`                     | `()`                 |

When evaluating the results for a particular grouping set,
expressions that aren't in the grouping set are aggregated and produce a
`NULL` placeholder.

You can filter results by specific groupable items. To learn more, see the
[`GROUPING` function][grouping-function]

**Examples**

The following queries produce the same subtotals and a grand total, but
the first one uses `GROUP BY` with `ROLLUP` and the second one doesn't:

```sql
-- GROUP BY with ROLLUP
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY ROLLUP (product_type, product_name)
ORDER BY product_type, product_name;

/*--------------+--------------+-------------+
 | product_type | product_name | product_sum |
 +--------------+--------------+-------------+
 | NULL         | NULL         | 42          |
 | pants        | NULL         | 6           |
 | pants        | jeans        | 6           |
 | shirt        | NULL         | 36          |
 | shirt        | t-shirt      | 11          |
 | shirt        | polo         | 25          |
 +--------------+--------------+-------------*/
```

```sql
-- GROUP BY without ROLLUP (produces the same results as ROLLUP)
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY product_type, product_name
UNION ALL
SELECT product_type, NULL, SUM(product_count)
FROM Products
GROUP BY product_type
UNION ALL
SELECT NULL, NULL, SUM(product_count) FROM Products
ORDER BY product_type, product_name;
```

You can include groupable item sets in a `GROUP BY ROLLUP` clause.
In the following example, `(product_type, product_name)` is a
groupable item set.

```sql
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY ROLLUP (product_type, (product_type, product_name))
ORDER BY product_type, product_name;

/*--------------+--------------+-------------+
 | product_type | product_name | product_sum |
 +--------------+--------------+-------------+
 | NULL         | NULL         | 42          |
 | pants        | NULL         | 6           |
 | pants        | jeans        | 6           |
 | shirt        | NULL         | 36          |
 | shirt        | polo         | 25          |
 | shirt        | t-shirt      | 11          |
 +--------------+--------------+-------------*/
```

### Group rows by `CUBE`

```sql
GROUP BY CUBE ( grouping_list )

grouping_list:
  { groupable_item | groupable_item_set }[, ...]

groupable_item_set:
  ( groupable_item[, ...] )
```

**Description**

The `GROUP BY CUBE` clause produces aggregated data for all _grouping set_
permutations. A grouping set is a group of columns by which rows
can be grouped together. This clause is helpful if you need to create a
[contingency table][contingency-table]{: .external} to find interrelationships
between items in a set of data.

**Definitions**

- `grouping_list`: A list of items that you can add to the
  `GROUPING SETS` clause. This is used to create a generated list
  of grouping sets when the query is run.
- `groupable_item`: Group rows in a table that share common values
  for certain columns. To learn more, see
  [Group rows by groupable items][group-by-groupable-item].
  [Anonymous `STRUCT` values][tuple-struct] aren't allowed.
- `groupable_item_set`: Group rows by a set of
  [groupable items][group-by-groupable-item].

**Details**

`GROUP BY CUBE` is similar to `GROUP BY ROLLUP`, except that it takes a
grouping list and generates grouping sets from all permutations in this
list, including an empty grouping set. In the empty grouping set, all rows
are aggregated down to a single group.

For example, `GROUP BY CUBE (a, b, c)` generates the following
grouping sets from the grouping list, `a, b, c`, and then produces
aggregated rows for each of them:

- `(a, b, c)`
- `(a, b)`
- `(a, c)`
- `(a)`
- `(b, c)`
- `(b)`
- `(c)`
- `()`

Here is an example that includes groupable item sets in
`GROUP BY CUBE (a, (b, c), d)`:

| Conceptual grouping sets | Actual grouping sets |
| ------------------------ | -------------------- |
| `(a, (b, c), d)`         | `(a, b, c, d)`       |
| `(a, (b, c))`            | `(a, b, c)`          |
| `(a, d)`                 | `(a, d)`             |
| `(a)`                    | `(a)`                |
| `((b, c), d)`            | `(b, c, d)`          |
| `((b, c))`               | `(b, c)`             |
| `(d)`                    | `(d)`                |
| `()`                     | `()`                 |

When evaluating the results for a particular grouping set,
expressions that aren't in the grouping set are aggregated and produce a
`NULL` placeholder.

You can filter results by specific groupable items. To learn more, see the
[`GROUPING` function][grouping-function]

**Examples**

The following query groups rows by all combinations of `product_type` and
`product_name` to produce a contingency table:

```sql
-- GROUP BY with CUBE
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY CUBE (product_type, product_name)
ORDER BY product_type, product_name;

/*--------------+--------------+-------------+
 | product_type | product_name | product_sum |
 +--------------+--------------+-------------+
 | NULL         | NULL         | 42          |
 | NULL         | jeans        | 6           |
 | NULL         | polo         | 25          |
 | NULL         | t-shirt      | 11          |
 | pants        | NULL         | 6           |
 | pants        | jeans        | 6           |
 | shirt        | NULL         | 36          |
 | shirt        | polo         | 25          |
 | shirt        | t-shirt      | 11          |
 +--------------+--------------+-------------*/
```

You can include groupable item sets in a `GROUP BY CUBE` clause.
In the following example, `(product_type, product_name)` is a
groupable item set.

```sql
WITH
  Products AS (
    SELECT 'shirt' AS product_type, 't-shirt' AS product_name, 3 AS product_count UNION ALL
    SELECT 'shirt', 't-shirt', 8 UNION ALL
    SELECT 'shirt', 'polo', 25 UNION ALL
    SELECT 'pants', 'jeans', 6
  )
SELECT product_type, product_name, SUM(product_count) AS product_sum
FROM Products
GROUP BY CUBE (product_type, (product_type, product_name))
ORDER BY product_type, product_name;

/*--------------+--------------+-------------+
 | product_type | product_name | product_sum |
 +--------------+--------------+-------------+
 | NULL         | NULL         | 42          |
 | pants        | NULL         | 6           |
 | pants        | jeans        | 6           |
 | pants        | jeans        | 6           |
 | shirt        | NULL         | 36          |
 | shirt        | polo         | 25          |
 | shirt        | polo         | 25          |
 | shirt        | t-shirt      | 11          |
 | shirt        | t-shirt      | 11          |
 +--------------+--------------+-------------*/
```

## `HAVING` clause

```sql
HAVING bool_expression
```

The `HAVING` clause filters the results produced by `GROUP BY` or
aggregation. `GROUP BY` or aggregation must be present in the query. If
aggregation is present, the `HAVING` clause is evaluated once for every
aggregated row in the result set.

Only rows whose `bool_expression` evaluates to `TRUE` are included. Rows
whose `bool_expression` evaluates to `NULL` or `FALSE` are
discarded.

The evaluation of a query with a `HAVING` clause is typically completed in this
order:

- `FROM`
- `WHERE`
- `GROUP BY` and aggregation
- `HAVING`
- `WINDOW`
- `QUALIFY`
- `DISTINCT`
- `ORDER BY`
- `LIMIT`

Evaluation order doesn't always match syntax order.

The `HAVING` clause references columns available via the `FROM` clause, as
well as `SELECT` list aliases. Expressions referenced in the `HAVING` clause
must either appear in the `GROUP BY` clause or they must be the result of an
aggregate function:

```sql
SELECT LastName
FROM Roster
GROUP BY LastName
HAVING SUM(PointsScored) > 15;
```

If a query contains aliases in the `SELECT` clause, those aliases override names
in a `FROM` clause.

```sql
SELECT LastName, SUM(PointsScored) AS ps
FROM Roster
GROUP BY LastName
HAVING ps > 0;
```

### Mandatory aggregation

Aggregation doesn't have to be present in the `HAVING` clause itself, but
aggregation must be present in at least one of the following forms:

#### Aggregation function in the `SELECT` list.

```sql
SELECT LastName, SUM(PointsScored) AS total
FROM PlayerStats
GROUP BY LastName
HAVING total > 15;
```

#### Aggregation function in the `HAVING` clause.

```sql
SELECT LastName
FROM PlayerStats
GROUP BY LastName
HAVING SUM(PointsScored) > 15;
```

#### Aggregation in both the `SELECT` list and `HAVING` clause.

When aggregation functions are present in both the `SELECT` list and `HAVING`
clause, the aggregation functions and the columns they reference don't need
to be the same. In the example below, the two aggregation functions,
`COUNT()` and `SUM()`, are different and also use different columns.

```sql
SELECT LastName, COUNT(*)
FROM PlayerStats
GROUP BY LastName
HAVING SUM(PointsScored) > 15;
```

## `ORDER BY` clause

```sql
ORDER BY expression
  [COLLATE collation_specification]
  [{ ASC | DESC }]
  [{ NULLS FIRST | NULLS LAST }]
  [, ...]

collation_specification:
  language_tag[:collation_attribute]
```

The `ORDER BY` clause specifies a column or expression as the sort criterion for
the result set. If an `ORDER BY` clause isn't present, the order of the results
of a query isn't defined. Column aliases from a `FROM` clause or `SELECT` list
are allowed. If a query contains aliases in the `SELECT` clause, those aliases
override names in the corresponding `FROM` clause. The data type of
`expression` must be [orderable][orderable-data-types].

**Optional Clauses**

- `COLLATE`: You can use the `COLLATE` clause to refine how data is ordered
  by an `ORDER BY` clause. _Collation_ refers to a set of rules that determine
  how strings are compared according to the conventions and
  standards of a particular written language, region, or country. These rules
  might define the correct character sequence, with options for specifying
  case-insensitivity. You can use `COLLATE` only on columns of type `VARCHAR`.

  `collation_specification` represents the collation specification for the
  `COLLATE` clause. The collation specification can be a string literal or
  a query parameter. To learn more see
  [collation specification details][collation-spec].

- `NULLS FIRST | NULLS LAST`:
  - `NULLS FIRST`: Sort null values before non-null values.
  - `NULLS LAST`. Sort null values after non-null values.
- `ASC | DESC`: Sort the results in ascending or descending
  order of `expression` values. `ASC` is the
  default value. If null ordering isn't specified
  with `NULLS FIRST` or `NULLS LAST`:
  - `NULLS FIRST` is applied by default if the sort order is ascending.
  - `NULLS LAST` is applied by default if the sort order is
    descending.

**Examples**

Use the default sort order (ascending).

```sql
SELECT x, y
FROM (SELECT 1 AS x, true AS y UNION ALL
      SELECT 9, true UNION ALL
      SELECT NULL, false)
ORDER BY x;

/*------+-------+
 | x    | y     |
 +------+-------+
 | NULL | false |
 | 1    | true  |
 | 9    | true  |
 +------+-------*/
```

Use the default sort order (ascending), but return null values last.

```sql
SELECT x, y
FROM (SELECT 1 AS x, true AS y UNION ALL
      SELECT 9, true UNION ALL
      SELECT NULL, false)
ORDER BY x NULLS LAST;

/*------+-------+
 | x    | y     |
 +------+-------+
 | 1    | true  |
 | 9    | true  |
 | NULL | false |
 +------+-------*/
```

Use descending sort order.

```sql
SELECT x, y
FROM (SELECT 1 AS x, true AS y UNION ALL
      SELECT 9, true UNION ALL
      SELECT NULL, false)
ORDER BY x DESC;

/*------+-------+
 | x    | y     |
 +------+-------+
 | 9    | true  |
 | 1    | true  |
 | NULL | false |
 +------+-------*/
```

Use descending sort order, but return null values first.

```sql
SELECT x, y
FROM (SELECT 1 AS x, true AS y UNION ALL
      SELECT 9, true UNION ALL
      SELECT NULL, false)
ORDER BY x DESC NULLS FIRST;

/*------+-------+
 | x    | y     |
 +------+-------+
 | NULL | false |
 | 9    | true  |
 | 1    | true  |
 +------+-------*/
```

It's possible to order by multiple columns. In the example below, the result
set is ordered first by `SchoolID` and then by `LastName`:

```sql
SELECT LastName, PointsScored, OpponentID
FROM PlayerStats
ORDER BY SchoolID, LastName;
```

When used in conjunction with
[set operators][set-operators],
the `ORDER BY` clause applies to the result set of the entire query; it doesn't
apply only to the closest `SELECT` statement. For this reason, it can be helpful
(though it isn't required) to use parentheses to show the scope of the `ORDER
BY`.

This query without parentheses:

```sql
SELECT * FROM Roster
UNION ALL
SELECT * FROM TeamMascot
ORDER BY SchoolID;
```

is equivalent to this query with parentheses:

```sql
( SELECT * FROM Roster
  UNION ALL
  SELECT * FROM TeamMascot )
ORDER BY SchoolID;
```

but isn't equivalent to this query, where the `ORDER BY` clause applies only to
the second `SELECT` statement:

```sql
SELECT * FROM Roster
UNION ALL
( SELECT * FROM TeamMascot
  ORDER BY SchoolID );
```

You can also use integer literals as column references in `ORDER BY` clauses. An
integer literal becomes an ordinal (for example, counting starts at 1) into
the `SELECT` list.

Example - the following two queries are equivalent:

```sql
SELECT SUM(PointsScored), LastName
FROM PlayerStats
GROUP BY LastName
ORDER BY LastName;
```

```sql
SELECT SUM(PointsScored), LastName
FROM PlayerStats
GROUP BY 2
ORDER BY 2;
```

Collate results using English - Canada:

```sql
SELECT Place
FROM Locations
ORDER BY Place COLLATE "en_CA"
```

Collate results using a parameter:

```sql
#@collate_param = "arg_EG"
SELECT Place
FROM Locations
ORDER BY Place COLLATE @collate_param
```

Using multiple `COLLATE` clauses in a statement:

```sql
SELECT APlace, BPlace, CPlace
FROM Locations
ORDER BY APlace COLLATE "en_US" ASC,
         BPlace COLLATE "ar_EG" DESC,
         CPlace COLLATE "en" DESC
```

Case insensitive collation:

```sql
SELECT Place
FROM Locations
ORDER BY Place COLLATE "en_US:ci"
```

Default Unicode case-insensitive collation:

```sql
SELECT Place
FROM Locations
ORDER BY Place COLLATE "und:ci"
```

## `QUALIFY` clause

```sql
QUALIFY bool_expression
```

The `QUALIFY` clause filters the results of window functions.
A window function is required to be present in the `QUALIFY` clause or the
`SELECT` list.

Only rows whose `bool_expression` evaluates to `TRUE` are included. Rows
whose `bool_expression` evaluates to `NULL` or `FALSE` are
discarded.

The evaluation of a query with a `QUALIFY` clause is typically completed in this
order:

- `FROM`
- `WHERE`
- `GROUP BY` and aggregation
- `HAVING`
- `WINDOW`
- `QUALIFY`
- `DISTINCT`
- `ORDER BY`
- `LIMIT`

Evaluation order doesn't always match syntax order.

**Examples**

The following query returns the most popular vegetables in the
[`Produce`][produce-table] table and their rank.

```sql
SELECT
  item,
  RANK() OVER (PARTITION BY category ORDER BY purchases DESC) as rank
FROM Produce
WHERE Produce.category = 'vegetable'
QUALIFY rank <= 3

/*---------+------+
 | item    | rank |
 +---------+------+
 | kale    | 1    |
 | lettuce | 2    |
 | cabbage | 3    |
 +---------+------*/
```

You don't have to include a window function in the `SELECT` list to use
`QUALIFY`. The following query returns the most popular vegetables in the
[`Produce`][produce-table] table.

```sql
SELECT item
FROM Produce
WHERE Produce.category = 'vegetable'
QUALIFY RANK() OVER (PARTITION BY category ORDER BY purchases DESC) <= 3

/*---------+
 | item    |
 +---------+
 | kale    |
 | lettuce |
 | cabbage |
 +---------*/
```
