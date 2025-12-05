# From

## `FROM` clause

```sql
FROM from_clause[, ...]

from_clause:
  from_item
  [ { pivot_operator | unpivot_operator } ]
  [ tablesample_operator ]

from_item:
  {
    table_name [ as_alias ]
    | { join_operation | ( join_operation ) }
    | ( query_expr ) [ as_alias ]
    | field_path
    | unnest_operator
    | cte_name [ as_alias ]
    | graph_table_operator [ as_alias ]
  }

as_alias:
  [ AS ] alias
```

The `FROM` clause indicates the table or tables from which to retrieve rows,
and specifies how to join those rows together to produce a single stream of
rows for processing in the rest of the query.

#### `pivot_operator`

See [PIVOT operator][pivot-operator].

#### `unpivot_operator`

See [UNPIVOT operator][unpivot-operator].

#### `tablesample_operator`

See [TABLESAMPLE operator][tablesample-operator].

#### `graph_table_operator`

See [GRAPH_TABLE operator][graph-table-operator].

#### `table_name`

The name (optionally qualified) of an existing table.

```sql
SELECT * FROM Roster;
SELECT * FROM db.Roster;
```

#### `join_operation`

See [Join operation][query-joins].

#### `query_expr`

`( query_expr ) [ [ AS ] alias ]` is a [table subquery][table-subquery-concepts].

#### `field_path`

In the `FROM` clause, `field_path` is any path that
resolves to a field within a data type. `field_path` can go
arbitrarily deep into a nested data structure.

Some examples of valid `field_path` values include:

```sql
SELECT * FROM T1 t1, t1.array_column;

SELECT * FROM T1 t1, t1.struct_column.array_field;

SELECT (SELECT ARRAY_AGG(c) FROM t1.array_column c) FROM T1 t1;

SELECT a.struct_field1 FROM T1 t1, t1.array_of_structs a;

SELECT (SELECT STRING_AGG(a.struct_field1) FROM t1.array_of_structs a) FROM T1 t1;
```

Field paths in the `FROM` clause must end in an
array or a repeated field. In
addition, field paths can't contain arrays
or repeated fields before the end of the path. For example, the path
`array_column.some_array.some_array_field` is invalid because it
contains an array before the end of the path.

Note: If a path has only one name, it's interpreted as a table.
To work around this, wrap the path using `UNNEST`, or use the
fully-qualified path.

Note: If a path has more than one name, and it matches a field
name, it's interpreted as a field name. To force the path to be interpreted as
a table name, wrap the path using ```.

#### `unnest_operator`

See [UNNEST operator][unnest-operator].

#### `cte_name`

Common table expressions (CTEs) in a [`WITH` Clause][with-clause] act like
temporary tables that you can reference anywhere in the `FROM` clause.
In the example below, `subQ1` and `subQ2` are CTEs.

Example:

```sql
WITH
  subQ1 AS (SELECT * FROM Roster WHERE SchoolID = 52),
  subQ2 AS (SELECT SchoolID FROM subQ1)
SELECT DISTINCT * FROM subQ2;
```

The `WITH` clause hides any permanent tables with the same name
for the duration of the query, unless you qualify the table name, for example:

`db.Roster`.

## `UNNEST` operator

```sql
unnest_operator:
  {
    UNNEST( array ) [ as_alias ]
    | array_path [ as_alias ]
  }
  [ WITH OFFSET [ as_alias ] ]

array:
  { array_expression | array_path }

as_alias:
  [AS] alias
```

The `UNNEST` operator takes an array and returns a table with one row for each
element in the array. The output of `UNNEST` is one [value table][value-tables] column.
For these `ARRAY` element types, `SELECT *` against the value table column
returns multiple columns:

- `STRUCT`
-

Input values:

- `array_expression`: An expression that produces an array and that's not an
  array path.
- `array_path`: The path to an `ARRAY` or
  non-`ARRAY` type, which may or may not contain a flattening operation, using the
  [array elements field access operation][array-el-field-operator].
  - In an implicit `UNNEST` operation, the path
    must
    start with
    a
    [range variable][range-variables] name.
  - In an explicit `UNNEST` operation, the path can optionally start with a
    [range variable][range-variables] name.

  The `UNNEST` operation with any [correlated][correlated-join] `array_path` must
  be on the right side of a `CROSS JOIN`, `LEFT JOIN`, or
  `INNER JOIN` operation.

- `as_alias`: If specified, defines the explicit name of the value table
  column containing the array element values. It can be used to refer to
  the column elsewhere in the query.
- `WITH OFFSET`: `UNNEST` destroys the order of elements in the input
  array. Use this optional clause to return an additional column with
  the array element indexes, or _offsets_. Offset counting starts at zero for
  each row produced by the `UNNEST` operation. This column has an
  optional alias; If the optional alias isn't used, the default column name is
  `offset`.

  Example:

  ```sql
  SELECT * FROM UNNEST ([10,20,30]) as numbers WITH OFFSET;

  /*---------+--------+
   | numbers | offset |
   +---------+--------+
   | 10      | 0      |
   | 20      | 1      |
   | 30      | 2      |
   +---------+--------*/
  ```

You can also use `UNNEST` outside of the `FROM` clause with the
[`IN` operator][in-operator].

For several ways to use `UNNEST`, including construction, flattening, and
filtering, see [Work with arrays][working-with-arrays].

To learn more about the ways you can use `UNNEST` explicitly and implicitly,
see [Explicit and implicit `UNNEST`][explicit-implicit-unnest].

### `UNNEST` and structs

For an input array of structs, `UNNEST`
returns a row for each struct, with a separate column for each field in the
struct. The alias for each column is the name of the corresponding struct
field.

Example:

```sql
SELECT *
FROM UNNEST(
  ARRAY<
    STRUCT<
      x BIGINT,
      y VARCHAR,
      z STRUCT>>[
        (1, 'foo', (10, 11)),
        (3, 'bar', (20, 21))]);

/*---+-----+----------+
 | x | y   | z        |
 +---+-----+----------+
 | 1 | foo | {10, 11} |
 | 3 | bar | {20, 21} |
 +---+-----+----------*/
```

Because the `UNNEST` operator returns a
[value table][value-tables],
you can alias `UNNEST` to define a range variable that you can reference
elsewhere in the query. If you reference the range variable in the `SELECT`
list, the query returns a struct containing all of the fields of the original
struct in the input table.

Example:

```sql
SELECT *, struct_value
FROM UNNEST(
  ARRAY<
    STRUCT<
    x BIGINT,
    y VARCHAR>>[
      (1, 'foo'),
      (3, 'bar')]) AS struct_value;

/*---+-----+--------------+
 | x | y   | struct_value |
 +---+-----+--------------+
 | 3 | bar | {3, bar}     |
 | 1 | foo | {1, foo}     |
 +---+-----+--------------*/
```

### `UNNEST` and

For an input array of , `UNNEST` returns a row for each
, with a separate column for each field in the
. The alias for each column is the name of the corresponding
field.

Example:

```sql
SELECT *
FROM UNNEST(
  ARRAY<>[
    NEW  (
      'The Goldberg Variations' AS album_name,
      ['Aria', 'Variation 1', 'Variation 2'] AS song
    )
  ]
);

/*-------------------------+--------+----------------------------------+
 | album_name              | singer | song                             |
 +-------------------------+--------+----------------------------------+
 | The Goldberg Variations | NULL   | [Aria, Variation 1, Variation 2] |
 +-------------------------+--------+----------------------------------*/
```

As with structs, you can alias `UNNEST` to define a range variable. You
can reference this alias in the `SELECT` list to return a value table where each
row is a element from the array.

```sql
SELECT proto_value
FROM UNNEST(
  ARRAY<>[
    NEW  (
      'The Goldberg Variations' AS album_name,
      ['Aria', 'Var. 1'] AS song
    )
  ]
) AS proto_value;

/*---------------------------------------------------------------------+
 | proto_value                                                         |
 +---------------------------------------------------------------------+
 | {album_name: "The Goldberg Variations" song: "Aria" song: "Var. 1"} |
 +---------------------------------------------------------------------*/
```

### Explicit and implicit `UNNEST`

Array unnesting can be either explicit or implicit. To learn more, see the
following sections.

#### Explicit unnesting

The `UNNEST` keyword is required in explicit unnesting. For example:

```sql
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, UNNEST(Coordinates.position.y) AS results;
```

This example and the following examples use the `array_path` called
`Coordinates.position` to illustrate unnesting.

##### Tables and explicit unnesting

When you use `array_path` with explicit `UNNEST`,
you can optionally prepend `array_path` with a table.

The following queries produce the same results:

```sql
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, UNNEST(position.y) AS results;
```

```sql
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, UNNEST(Coordinates.position.y) AS results;
```

#### Implicit unnesting

The `UNNEST` keyword isn't used in implicit unnesting.

For example:

```sql
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, Coordinates.position.y AS results;
```

When you use `array_path` with `UNNEST`, the
[`FLATTEN` operator][flatten-operator] is used implicitly. These are equivalent:

```sql
-- In UNNEST, FLATTEN used explicitly:
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, UNNEST(FLATTEN(Coordinates.position.y)) AS results;
```

```sql
-- In UNNEST, FLATTEN used implicitly:
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, UNNEST(Coordinates.position.y) AS results;
```

```sql
-- In the FROM clause, UNNEST used implicitly:
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, Coordinates.position.y AS results;
```

##### Tables and implicit unnesting

When you use `array_path` with implicit `UNNEST`, `array_path` must be prepended
with the table. For example:

```sql
WITH Coordinates AS (SELECT [1,2] AS position)
SELECT results FROM Coordinates, Coordinates.position AS results;
```

##### Array subscript operator limitations in implicit unnesting

You can use `UNNEST` with `array_path` implicitly
in the `FROM` clause, but only if the
[array subscript operator][array-subscript-operator] isn't included.

The following query is valid:

```sql
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, UNNEST(Coordinates.position.y[OFFSET(1)]) AS results;
```

The following query is invalid:

```sql {.bad}
-- Invalid
WITH Coordinates AS (SELECT ARRAY>>[(1, [2,3]), (4, [5,6])] AS position)
SELECT results FROM Coordinates, Coordinates.position.y[OFFSET(1)] AS results;
```

### `UNNEST` and `NULL` values

`UNNEST` treats `NULL` values as follows:

- `NULL` and empty arrays produce zero rows.
- An array containing `NULL` values produces rows containing `NULL` values.

## `PIVOT` operator

```sql
FROM from_item[, ...] pivot_operator

pivot_operator:
  PIVOT(
    aggregate_function_call [as_alias][, ...]
    FOR input_column
    IN ( pivot_column [as_alias][, ...] )
  ) [AS alias]

as_alias:
  [AS] alias
```

The `PIVOT` operator rotates rows into columns, using aggregation.
`PIVOT` is part of the `FROM` clause.

- `PIVOT` can be used to modify any table expression.
- A `WITH OFFSET` clause immediately preceding the `PIVOT` operator isn't
  allowed.

Conceptual example:

```sql
-- Before PIVOT is used to rotate sales and quarter into Q1, Q2, Q3, Q4 columns:
/*---------+-------+---------+------+
 | product | sales | quarter | year |
 +---------+-------+---------+------|
 | Kale    | 51    | Q1      | 2020 |
 | Kale    | 23    | Q2      | 2020 |
 | Kale    | 45    | Q3      | 2020 |
 | Kale    | 3     | Q4      | 2020 |
 | Kale    | 70    | Q1      | 2021 |
 | Kale    | 85    | Q2      | 2021 |
 | Apple   | 77    | Q1      | 2020 |
 | Apple   | 0     | Q2      | 2020 |
 | Apple   | 1     | Q1      | 2021 |
 +---------+-------+---------+------*/

-- After PIVOT is used to rotate sales and quarter into Q1, Q2, Q3, Q4 columns:
/*---------+------+----+------+------+------+
 | product | year | Q1 | Q2   | Q3   | Q4   |
 +---------+------+----+------+------+------+
 | Apple   | 2020 | 77 | 0    | NULL | NULL |
 | Apple   | 2021 | 1  | NULL | NULL | NULL |
 | Kale    | 2020 | 51 | 23   | 45   | 3    |
 | Kale    | 2021 | 70 | 85   | NULL | NULL |
 +---------+------+----+------+------+------*/
```

**Definitions**

Top-level definitions:

- `from_item`: The table, subquery, or
  table-valued function (TVF) on which
  to perform a pivot operation. The `from_item` must
  [follow these rules](#rules-for-pivot-from-item).
- `pivot_operator`: The pivot operation to perform on a `from_item`.
- `alias`: An alias to use for an item in the query.

`pivot_operator` definitions:

- `aggregate_function_call`: An aggregate function call that aggregates all
  input rows such that `input_column` matches a particular value in
  `pivot_column`. Each aggregation corresponding to a different `pivot_column`
  value produces a different column in the output.
  [Follow these rules](#rules-for-pivot-agg-function) when creating an
  aggregate function call.
- `input_column`: Takes a column and retrieves the row values for the
  column, [following these rules](#rules-input-column).
- `pivot_column`: A pivot column to create for each aggregate function
  call. If an alias isn't provided, a default alias is created. A pivot column
  value type must match the value type in `input_column` so that the values can
  be compared. It's possible to have a value in `pivot_column` that doesn't
  match a value in `input_column`. Must be a constant and
  [follow these rules](#rules-pivot-column).

**Rules**

Rules for a `from_item` passed to `PIVOT`:

- The `from_item` may consist of any
  table, subquery, or table-valued function
  (TVF) result.
- The `from_item` may not produce a value table.
- The `from_item` may not be a subquery using `SELECT AS STRUCT`.

Rules for `aggregate_function_call`:

- Must be an aggregate function. For example, `SUM`.
- You may reference columns in a table passed to `PIVOT`, as
  well as correlated columns, but may not access columns defined by the `PIVOT`
  clause itself.
- A table passed to `PIVOT` may be accessed through its alias if one is
  provided.

Rules for `input_column`:

- May access columns from the input table, as well as correlated columns,
  not columns defined by the `PIVOT` clause, itself.
- Evaluated against each row in the input table; aggregate and window function
  calls are prohibited.
- Non-determinism is okay.
- The type must be groupable.
- The input table may be accessed through its alias if one is provided.

Rules for `pivot_column`:

- A `pivot_column` must be a constant.
- Named constants, such as variables, aren't supported.
- Query parameters aren't supported.
- If a name is desired for a named constant or query parameter,
  specify it explicitly with an alias.
- Corner cases exist where a distinct `pivot_column`s can end up with the same
  default column names. For example, an input column might contain both a
  `NULL` value and the string literal `"NULL"`. When this happens, multiple
  pivot columns are created with the same name. To avoid this situation,
  use aliases for pivot column names.
- If a `pivot_column` doesn't specify an alias, a column name is constructed as
  follows:

| From                                                        | To                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Example                                                                                                    |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| NULL                                                        | NULL                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Input: NULL Output: "NULL"                                                                                 |
| `INTEGER` `BIGINT` `UINTEGER` `UBIGINT` `NUMERIC` `NUMERIC` | The number in string format with the following rules: - Positive numbers are preceded with `_`. - Negative numbers are preceded with `minus_`. - A decimal point is replaced with `_point_`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Input: 1 Output: \_1 <hr/>Input: -1 Output: minus_1 <hr/>Input: 1.0 Output: \_1_point_0                    |
| BOOL                                                        | `TRUE` or `FALSE`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Input: TRUE Output: TRUE <hr/>Input: FALSE Output: FALSE                                                   |
| VARCHAR                                                     | The string value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Input: "PlayerName" Output: PlayerName                                                                     |
| DATE                                                        | The date in `_YYYY_MM_DD` format.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Input: DATE '2013-11-25' Output: \_2013_11_25                                                              |
| ENUM                                                        | The name of the enumeration constant.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Input: COLOR.RED Output: RED                                                                               |
| STRUCT                                                      | A string formed by computing the `pivot_column` name for each field and joining the results together with an underscore. The following rules apply: - If the field is named: `<field_name>_<pivot_column_name_for_field_name>`. - If the field is unnamed: `<pivot_column_name_for_field_name>`. `<pivot_column_name_for_field_name>` is determined by applying the rules in this table, recursively. If no rule is available for any `STRUCT` field, the entire pivot column is unnamed. Due to implicit type coercion from the `IN` list values to the type of `<value-expression>`, field names must be present in `input_column` to have an effect on the names of the pivot columns. | Input: STRUCT("one", "two") Output: one_two <hr/>Input: STRUCT("one" AS a, "two" AS b) Output: one_a_two_b |
| All other data types                                        | Not supported. You must provide an alias.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |                                                                                                            |

**Examples**

The following examples reference a table called `Produce` that looks like this:

```sql
WITH Produce AS (
  SELECT 'Kale' as product, 51 as sales, 'Q1' as quarter, 2020 as year UNION ALL
  SELECT 'Kale', 23, 'Q2', 2020 UNION ALL
  SELECT 'Kale', 45, 'Q3', 2020 UNION ALL
  SELECT 'Kale', 3, 'Q4', 2020 UNION ALL
  SELECT 'Kale', 70, 'Q1', 2021 UNION ALL
  SELECT 'Kale', 85, 'Q2', 2021 UNION ALL
  SELECT 'Apple', 77, 'Q1', 2020 UNION ALL
  SELECT 'Apple', 0, 'Q2', 2020 UNION ALL
  SELECT 'Apple', 1, 'Q1', 2021)
SELECT * FROM Produce

/*---------+-------+---------+------+
 | product | sales | quarter | year |
 +---------+-------+---------+------|
 | Kale    | 51    | Q1      | 2020 |
 | Kale    | 23    | Q2      | 2020 |
 | Kale    | 45    | Q3      | 2020 |
 | Kale    | 3     | Q4      | 2020 |
 | Kale    | 70    | Q1      | 2021 |
 | Kale    | 85    | Q2      | 2021 |
 | Apple   | 77    | Q1      | 2020 |
 | Apple   | 0     | Q2      | 2020 |
 | Apple   | 1     | Q1      | 2021 |
 +---------+-------+---------+------*/
```

With the `PIVOT` operator, the rows in the `quarter` column are rotated into
these new columns: `Q1`, `Q2`, `Q3`, `Q4`. The aggregate function `SUM` is
implicitly grouped by all unaggregated columns other than the `pivot_column`:
`product` and `year`.

```sql
SELECT * FROM
  Produce
  PIVOT(SUM(sales) FOR quarter IN ('Q1', 'Q2', 'Q3', 'Q4'))

/*---------+------+----+------+------+------+
 | product | year | Q1 | Q2   | Q3   | Q4   |
 +---------+------+----+------+------+------+
 | Apple   | 2020 | 77 | 0    | NULL | NULL |
 | Apple   | 2021 | 1  | NULL | NULL | NULL |
 | Kale    | 2020 | 51 | 23   | 45   | 3    |
 | Kale    | 2021 | 70 | 85   | NULL | NULL |
 +---------+------+----+------+------+------*/
```

If you don't include `year`, then `SUM` is grouped only by `product`.

```sql
SELECT * FROM
  (SELECT product, sales, quarter FROM Produce)
  PIVOT(SUM(sales) FOR quarter IN ('Q1', 'Q2', 'Q3', 'Q4'))

/*---------+-----+-----+------+------+
 | product | Q1  | Q2  | Q3   | Q4   |
 +---------+-----+-----+------+------+
 | Apple   | 78  | 0   | NULL | NULL |
 | Kale    | 121 | 108 | 45   | 3    |
 +---------+-----+-----+------+------*/
```

You can select a subset of values in the `pivot_column`:

```sql
SELECT * FROM
  (SELECT product, sales, quarter FROM Produce)
  PIVOT(SUM(sales) FOR quarter IN ('Q1', 'Q2', 'Q3'))

/*---------+-----+-----+------+
 | product | Q1  | Q2  | Q3   |
 +---------+-----+-----+------+
 | Apple   | 78  | 0   | NULL |
 | Kale    | 121 | 108 | 45   |
 +---------+-----+-----+------*/
```

```sql
SELECT * FROM
  (SELECT sales, quarter FROM Produce)
  PIVOT(SUM(sales) FOR quarter IN ('Q1', 'Q2', 'Q3'))

/*-----+-----+----+
 | Q1  | Q2  | Q3 |
 +-----+-----+----+
 | 199 | 108 | 45 |
 +-----+-----+----*/
```

You can include multiple aggregation functions in the `PIVOT`. In this case, you
must specify an alias for each aggregation. These aliases are used to construct
the column names in the resulting table.

```sql
SELECT * FROM
  (SELECT product, sales, quarter FROM Produce)
  PIVOT(SUM(sales) AS total_sales, COUNT(*) AS num_records FOR quarter IN ('Q1', 'Q2'))

/*--------+----------------+----------------+----------------+----------------+
 |product | total_sales_Q1 | num_records_Q1 | total_sales_Q2 | num_records_Q2 |
 +--------+----------------+----------------+----------------+----------------+
 | Kale   | 121            | 2              | 108            | 2              |
 | Apple  | 78             | 2              | 0              | 1              |
 +--------+----------------+----------------+----------------+----------------*/
```

## `UNPIVOT` operator

```sql
FROM from_item[, ...] unpivot_operator

unpivot_operator:
  UNPIVOT [ { INCLUDE NULLS | EXCLUDE NULLS } ] (
    { single_column_unpivot | multi_column_unpivot }
  ) [unpivot_alias]

single_column_unpivot:
  values_column
  FOR name_column
  IN (columns_to_unpivot)

multi_column_unpivot:
  values_column_set
  FOR name_column
  IN (column_sets_to_unpivot)

values_column_set:
  (values_column[, ...])

columns_to_unpivot:
  unpivot_column [row_value_alias][, ...]

column_sets_to_unpivot:
  (unpivot_column [row_value_alias][, ...])

unpivot_alias and row_value_alias:
  [AS] alias
```

The `UNPIVOT` operator rotates columns into rows. `UNPIVOT` is part of the
`FROM` clause.

- `UNPIVOT` can be used to modify any table
  expression.
- A `WITH OFFSET` clause immediately preceding the `UNPIVOT` operator isn't
  allowed.
- `PIVOT` aggregations can't be reversed with `UNPIVOT`.

Conceptual example:

```sql
-- Before UNPIVOT is used to rotate Q1, Q2, Q3, Q4 into sales and quarter columns:
/*---------+----+----+----+----+
 | product | Q1 | Q2 | Q3 | Q4 |
 +---------+----+----+----+----+
 | Kale    | 51 | 23 | 45 | 3  |
 | Apple   | 77 | 0  | 25 | 2  |
 +---------+----+----+----+----*/

-- After UNPIVOT is used to rotate Q1, Q2, Q3, Q4 into sales and quarter columns:
/*---------+-------+---------+
 | product | sales | quarter |
 +---------+-------+---------+
 | Kale    | 51    | Q1      |
 | Kale    | 23    | Q2      |
 | Kale    | 45    | Q3      |
 | Kale    | 3     | Q4      |
 | Apple   | 77    | Q1      |
 | Apple   | 0     | Q2      |
 | Apple   | 25    | Q3      |
 | Apple   | 2     | Q4      |
 +---------+-------+---------*/
```

**Definitions**

Top-level definitions:

- `from_item`: The table, subquery, or
  table-valued function (TVF) on which
  to perform a pivot operation. The `from_item` must
  [follow these rules](#rules-for-unpivot-from-item).
- `unpivot_operator`: The pivot operation to perform on a `from_item`.

`unpivot_operator` definitions:

- `INCLUDE NULLS`: Add rows with `NULL` values to the result.
- `EXCLUDE NULLS`: don't add rows with `NULL` values to the result.
  By default, `UNPIVOT` excludes rows with `NULL` values.
- `single_column_unpivot`: Rotates columns into one `values_column`
  and one `name_column`.
- `multi_column_unpivot`: Rotates columns into multiple
  `values_column`s and one `name_column`.
- `unpivot_alias`: An alias for the results of the `UNPIVOT` operation. This
  alias can be referenced elsewhere in the query.

`single_column_unpivot` definitions:

- `values_column`: A column to contain the row values from `columns_to_unpivot`.
  [Follow these rules](#rules-for-values-column) when creating a values column.
- `name_column`: A column to contain the column names from `columns_to_unpivot`.
  [Follow these rules](#rules-for-name-column) when creating a name column.
- `columns_to_unpivot`: The columns from the `from_item` to populate
  `values_column` and `name_column`.
  [Follow these rules](#rules-for-unpivot-column) when creating an unpivot
  column.
  - `row_value_alias`: An optional alias for a column that's displayed for the
    column in `name_column`. If not specified, the string value of the
    column name is used.
    [Follow these rules](#rules-for-row-value-alias) when creating a
    row value alias.

`multi_column_unpivot` definitions:

- `values_column_set`: A set of columns to contain the row values from
  `columns_to_unpivot`. [Follow these rules](#rules-for-values-column) when
  creating a values column.
- `name_column`: A set of columns to contain the column names from
  `columns_to_unpivot`. [Follow these rules](#rules-for-name-column) when
  creating a name column.
- `column_sets_to_unpivot`: The columns from the `from_item` to unpivot.
  [Follow these rules](#rules-for-unpivot-column) when creating an unpivot
  column.
  - `row_value_alias`: An optional alias for a column set that's displayed
    for the column set in `name_column`. If not specified, a string value for
    the column set is used and each column in the string is separated with an
    underscore (`_`). For example, `(col1, col2)` outputs `col1_col2`.
    [Follow these rules](#rules-for-row-value-alias) when creating a
    row value alias.

**Rules**

Rules for a `from_item` passed to `UNPIVOT`:

- The `from_item` may consist of any
  table, subquery, or table-valued function
  (TVF) result.
- The `from_item` may not produce a value table.
- Duplicate columns in a `from_item` can't be referenced in the `UNPIVOT`
  clause.

Rules for `unpivot_operator`:

- Expressions aren't permitted.
- Qualified names aren't permitted. For example, `mytable.mycolumn` isn't
  allowed.
- In the case where the `UNPIVOT` result has duplicate column names:
  - `SELECT *` is allowed.
  - `SELECT values_column` causes ambiguity.

Rules for `values_column`:

- It can't be a name used for a `name_column` or an `unpivot_column`.
- It can be the same name as a column from the `from_item`.

Rules for `name_column`:

- It can't be a name used for a `values_column` or an `unpivot_column`.
- It can be the same name as a column from the `from_item`.

Rules for `unpivot_column`:

- Must be a column name from the `from_item`.
- It can't reference duplicate `from_item` column names.
- All columns in a column set must have equivalent data types.
  - Data types can't be coerced to a common supertype.
  - If the data types are exact matches (for example, a struct with
    different field names), the data type of the first input is
    the data type of the output.
- You can't have the same name in the same column set. For example,
  `(emp1, emp1)` results in an error.
- You can have a the same name in different column sets. For example,
  `(emp1, emp2), (emp1, emp3)` is valid.

Rules for `row_value_alias`:

- This can be a string or an `BIGINT` literal.
- The data type for all `row_value_alias` clauses must be the same.
- If the value is an `BIGINT`, the `row_value_alias` for each `unpivot_column`
  must be specified.

**Examples**

The following examples reference a table called `Produce` that looks like this:

```sql
WITH Produce AS (
  SELECT 'Kale' as product, 51 as Q1, 23 as Q2, 45 as Q3, 3 as Q4 UNION ALL
  SELECT 'Apple', 77, 0, 25, 2)
SELECT * FROM Produce

/*---------+----+----+----+----+
 | product | Q1 | Q2 | Q3 | Q4 |
 +---------+----+----+----+----+
 | Kale    | 51 | 23 | 45 | 3  |
 | Apple   | 77 | 0  | 25 | 2  |
 +---------+----+----+----+----*/
```

With the `UNPIVOT` operator, the columns `Q1`, `Q2`, `Q3`, and `Q4` are
rotated. The values of these columns now populate a new column called `Sales`
and the names of these columns now populate a new column called `Quarter`.
This is a single-column unpivot operation.

```sql
SELECT * FROM Produce
UNPIVOT(sales FOR quarter IN (Q1, Q2, Q3, Q4))

/*---------+-------+---------+
 | product | sales | quarter |
 +---------+-------+---------+
 | Kale    | 51    | Q1      |
 | Kale    | 23    | Q2      |
 | Kale    | 45    | Q3      |
 | Kale    | 3     | Q4      |
 | Apple   | 77    | Q1      |
 | Apple   | 0     | Q2      |
 | Apple   | 25    | Q3      |
 | Apple   | 2     | Q4      |
 +---------+-------+---------*/
```

In this example, we `UNPIVOT` four quarters into two semesters.
This is a multi-column unpivot operation.

```sql
SELECT * FROM Produce
UNPIVOT(
  (first_half_sales, second_half_sales)
  FOR semesters
  IN ((Q1, Q2) AS 'semester_1', (Q3, Q4) AS 'semester_2'))

/*---------+------------------+-------------------+------------+
 | product | first_half_sales | second_half_sales | semesters  |
 +---------+------------------+-------------------+------------+
 | Kale    | 51               | 23                | semester_1 |
 | Kale    | 45               | 3                 | semester_2 |
 | Apple   | 77               | 0                 | semester_1 |
 | Apple   | 25               | 2                 | semester_2 |
 +---------+------------------+-------------------+------------*/
```

## `TABLESAMPLE` operator

```sql
tablesample_clause:
  TABLESAMPLE sample_method (sample_size percent_or_rows [ partition_by ])
  [ REPEATABLE(repeat_argument) ]
  [ WITH WEIGHT [AS alias] ]

sample_method:
  { BERNOULLI | SYSTEM | RESERVOIR }

sample_size:
  numeric_value_expression

percent_or_rows:
  { PERCENT | ROWS }

partition_by:
  PARTITION BY partition_expression [, ...]
```

**Description**

You can use the `TABLESAMPLE` operator to select a random sample of a dataset.
This operator is useful when you're working with tables that have large
amounts of data and you don't need precise answers.

- `sample_method`: When using the `TABLESAMPLE` operator, you must specify the
  sampling algorithm to use:
  - `BERNOULLI`: Each row is independently selected with the probability
    given in the `percent` clause. As a result, you get approximately
    `N * percent/100` rows.
  - `SYSTEM`: Produces a sample using an
    unspecified engine-dependent method, which may be more efficient but less
    probabilistically random than other methods. For example, it could choose
    random disk blocks and return data from those blocks.
  - `RESERVOIR`: Takes as parameter an actual sample size
    K (expressed as a number of rows). If the input is smaller than K, it
    outputs the entire input relation. If the input is larger than K,
    reservoir sampling outputs a sample of size exactly K, where any sample of
    size K is equally likely.
- `sample_size`: The size of the sample.
- `percent_or_rows`: The `TABLESAMPLE` operator requires that you choose either
  `ROWS` or `PERCENT`. If you choose `PERCENT`, the value must be between
  0 and 100. If you choose `ROWS`, the value must be greater than or equal
  to 0.
- `partition_by`: Optional. Perform [stratified sampling][stratified-sampling]
  for each distinct group identified by the `PARTITION BY` clause. That is,
  if the number of rows in a particular group is less than the specified row
  count, all rows in that group are assigned to the sample. Otherwise, it
  randomly selects the specified number of rows for each group, where for a
  particular group, every sample of that size is equally
  likely.
- `REPEATABLE`: Optional. When it's used, repeated
  executions of the sampling operation return a result table with identical
  rows for a given repeat argument, as long as the underlying data doesn't
  change. `repeat_argument` represents a sampling seed
  and must be a positive value of type `BIGINT`.
- `WITH WEIGHT`: Optional. Retrieves [scaling weight][scaling-weight]. If
  specified, the `TABLESAMPLE` operator outputs one extra column of type
  `DOUBLE` that's greater than or equal 1.0 to represent the actual scaling
  weight. If an alias isn't provided, the default name _weight_ is used.
  - In Bernoulli sampling, the weight is `1 / provided sampling probability`.
    For example, `TABLESAMPLE BERNOULLI (1 percent)` will expose the weight
    of `1 / 0.01`.
  - In System sampling, the weight is approximated or computed exactly in
    some engine-defined way, as long as its type and value range is
    specified.
  - In non-stratified fixed row count sampling,
    (RESERVOIR without the PARTITION BY clause), the weight is equal to the
    total number of input rows divided by the count of sampled rows.
  - In stratified sampling,
    (RESERVOIR with the PARTITION BY clause), the weight for rows from a
    particular group is equal to the group cardinality divided by the count
    of sampled rows for that group.

**Examples**

The following examples illustrate the use of the `TABLESAMPLE` operator.

Select from a table using the `RESERVOIR` sampling method:

```sql
SELECT MessageId
FROM Messages TABLESAMPLE RESERVOIR (100 ROWS);
```

Select from a table using the `BERNOULLI` sampling method:

```sql
SELECT MessageId
FROM Messages TABLESAMPLE BERNOULLI (0.1 PERCENT);
```

Use `TABLESAMPLE` with a repeat argument:

```sql
SELECT MessageId
FROM Messages TABLESAMPLE RESERVOIR (100 ROWS) REPEATABLE(10);
```

Use `TABLESAMPLE` with a subquery:

```sql
SELECT Subject FROM
(SELECT MessageId, Subject FROM Messages WHERE ServerId="test")
TABLESAMPLE BERNOULLI(50 PERCENT)
WHERE MessageId > 3;
```

Use a `TABLESAMPLE` operation with a join to another table.

```sql
SELECT S.Subject
FROM
(SELECT MessageId, ThreadId FROM Messages WHERE ServerId="test") AS R
TABLESAMPLE RESERVOIR(5 ROWS),
Threads AS S
WHERE S.ServerId="test" AND R.ThreadId = S.ThreadId;
```

Group results by country, using stratified sampling:

```sql
SELECT country, SUM(click_cost) FROM ClickEvents
 TABLESAMPLE RESERVOIR (100 ROWS PARTITION BY country)
 GROUP BY country;
```

Add scaling weight to stratified sampling:

```sql
SELECT country, SUM(click_cost * sampling_weight) FROM ClickEvents
 TABLESAMPLE RESERVOIR (100 ROWS PARTITION BY country)
 WITH WEIGHT AS sampling_weight
 GROUP BY country;
```

This is equivalent to the previous example. Note that you don't have to use
an alias after `WITH WEIGHT`. If you don't, the default alias `weight` is used.

```sql
SELECT country, SUM(click_cost * weight) FROM ClickEvents
 TABLESAMPLE RESERVOIR (100 ROWS PARTITION BY country)
 WITH WEIGHT
 GROUP BY country;
```

### Stratified sampling

If you want better quality generated samples for under-represented groups,
you can use stratified sampling. Stratified sampling helps you
avoid samples with missing groups. To allow stratified sampling per
distinct group, use `PARTITION BY` with `RESERVOIR` in the `TABLESAMPLE` clause.

Stratified sampling performs `RESERVOIR` sampling for each distinct group
identified by the `PARTITION BY` clause. If the number of rows in a particular
group is less than the specified row count, all rows in that group are assigned
to the sample. Otherwise, it randomly selects the specified number of rows for
each group, where for a particular group, every sample of that size is equally
likely.

**Example**

Let’s consider a table named `ClickEvents` representing a stream of
click events, each of which has two fields: `country` and `click_cost`.
`country` represents the country from which the click was originated
and `click_cost` represents how much the click costs. In this example,
100 rows are randomly selected for each country.

```sql
SELECT click_cost, country FROM ClickEvents
TABLESAMPLE RESERVOIR (100 ROWS PARTITION BY country)
```

### Scaling weight

With scaling weight, you can perform fast and reasonable population estimates
from generated samples or estimate the aggregate results from samples. You can
capture scaling weight for a tablesample with the `WITH WEIGHT` clause.

Scaling weight represents the reciprocal of the actual, observed sampling
rate for a tablesample, making it easier to estimate aggregate results for
samples. The exposition of scaling weight generally applies to all variations
of `TABLESAMPLE`, including stratified Reservoir, non-stratified Reservoir,
Bernoulli, and System.

Let’s consider a table named `ClickEvents` representing a stream of
click events, each of which has two fields: `country` and `click_cost`.
`country` represents the country from which the click was originated
and `click_cost` represents how much the click costs. To calculate the
total click cost per country, you can use the following query:

```sql
SELECT country, SUM(click_cost)
FROM ClickEvents
GROUP BY country;
```

You can leverage the existing uniform sampling with fixed probability, using
Bernoulli sampling and run this query to estimate the result of the previous
query:

```sql
SELECT country, SUM(click_cost * weight)
FROM ClickEvents TABLESAMPLE BERNOULLI (1 PERCENT)
WITH WEIGHT
GROUP BY country;
```

You can break the second query into two steps:

1. Materialize a sample for reuse.
1. Perform aggregate estimates of the materialized sample.

Instead of aggregating the entire table, you use a 1% uniform sample to
aggregate a fraction of the original table and to compute the total click cost.
Because only 1% of the rows flow into the aggregation operator, you need to
scale the aggregate with a certain weight. Specifically, we multiply the
aggregate with 100, the reciprocal of the provided sampling probability, for
each group. And because we use uniform sampling, the scaling weight for each
group is effectively equal to the scaling weight for each row of the table,
which is 100.

Even though this sample provides a statistically accurate representation
of the original table, it might miss an entire group of rows, such as countries
in the running example, with small cardinality. For example, suppose that
the `ClickEvents` table contains 10000 rows, with 9990 rows of value `US`
and 10 rows of value `VN`. The number of distinct countries in this example
is two. With 1% uniform sampling, it's statistically probable that all the
sampled rows are from the `US` and none of them are from the `VN` partition.
As a result, the output of the second query doesn't contain the `SUM` estimate
for the group `VN`. We refer to this as the _missing-group problem_, which
can be solved with [stratified sampling][stratified-sampling].

## `GRAPH_TABLE` operator

To learn more about this operator, see
[`GRAPH_TABLE` operator][graph-table-operator] in the
Graph Query Language (GQL) reference guide.
