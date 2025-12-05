# Comparison Operators

Operators for comparing values: =, <>, <, >, IN, LIKE, IS, EXISTS.

### Graph concatenation operator

```sql
graph_path || graph_path [ || ... ]
```

**Description**

Combines multiple graph paths into one and preserves the original order of the
nodes and edges.

Arguments:

- `graph_path`: A `GRAPH_PATH` value that represents a graph path to
  concatenate.

**Details**

This operator produces an error if the last node in the first path isn't the
same as the first node in the second path.

```sql
-- This successfully produces the concatenated path called `full_path`.
MATCH
  p=(src:Account)-[t1:Transfers]->(mid:Account),
  q=(mid)-[t2:Transfers]->(dst:Account)
LET full_path = p || q
```

```sql
-- This produces an error because the first node of the path to be concatenated
-- (mid2) isn't equal to the last node of the previous path (mid1).
MATCH
  p=(src:Account)-[t1:Transfers]->(mid1:Account),
  q=(mid2:Account)-[t2:Transfers]->(dst:Account)
LET full_path = p || q
```

The first node in each subsequent path is removed from the
concatenated path.

```sql
-- The concatenated path called `full_path` contains these elements:
-- src, t1, mid, t2, dst.
MATCH
  p=(src:Account)-[t1:Transfers]->(mid:Account),
  q=(mid)-[t2:Transfers]->(dst:Account)
LET full_path = p || q
```

If any `graph_path` is `NULL`, produces `NULL`.

**Example**

In the following query, a path called `p` and `q` are concatenated. Notice that
`mid` is used at the end of the first path and at the beginning of the
second path. Also notice that the duplicate `mid` is removed from the
concatenated path called `full_path`:

```sql
GRAPH FinGraph
MATCH
  p=(src:Account)-[t1:Transfers]->(mid:Account),
  q = (mid)-[t2:Transfers]->(dst:Account)
LET full_path = p || q
RETURN
  JSON_QUERY(TO_JSON(full_path)[0], '$.labels') AS element_a,
  JSON_QUERY(TO_JSON(full_path)[1], '$.labels') AS element_b,
  JSON_QUERY(TO_JSON(full_path)[2], '$.labels') AS element_c,
  JSON_QUERY(TO_JSON(full_path)[3], '$.labels') AS element_d,
  JSON_QUERY(TO_JSON(full_path)[4], '$.labels') AS element_e,
  JSON_QUERY(TO_JSON(full_path)[5], '$.labels') AS element_f

/*-------------------------------------------------------------------------------------+
 | element_a   | element_b     | element_c   | element_d     | element_e   | element_f |
 +-------------------------------------------------------------------------------------+
 | ["Account"] | ["Transfers"] | ["Account"] | ["Transfers"] | ["Account"] |           |
 | ...         | ...           | ...         | ...           | ...         | ...       |
 +-------------------------------------------------------------------------------------/*
```

The following query produces an error because the last node for `p` must
be the first node for `q`:

```sql
-- Error: `mid1` and `mid2` aren't equal.
GRAPH FinGraph
MATCH
  p=(src:Account)-[t1:Transfers]->(mid1:Account),
  q=(mid2:Account)-[t2:Transfers]->(dst:Account)
LET full_path = p || q
RETURN TO_JSON(full_path) AS results
```

The following query produces an error because the path called `p` is `NULL`:

```sql
-- Error: a graph path is NULL.
GRAPH FinGraph
MATCH
  p=NULL,
  q=(mid:Account)-[t2:Transfers]->(dst:Account)
LET full_path = p || q
RETURN TO_JSON(full_path) AS results
```

### Graph logical operators

SQL supports the following logical operators in
element pattern label expressions:

| Name  | Syntax  | Description                                                                  |
| ----- | ------- | ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `NOT` | `!X`    | Returns `TRUE` if `X` isn't included, otherwise, returns `FALSE`.            |
| `OR`  | `X      | Y`                                                                           | Returns `TRUE` if either `X` or `Y` is included, otherwise, returns `FALSE`. |
| `AND` | `X & Y` | Returns `TRUE` if both `X` and `Y` are included, otherwise, returns `FALSE`. |

### Graph predicates

SQL supports the following graph-specific predicates in
graph expressions. A predicate can produce `TRUE`, `FALSE`, or `NULL`.

- [`PROPERTY_EXISTS` predicate][property-exists-predicate]
- [`IS SOURCE` predicate][is-source-predicate]
- [`IS DESTINATION` predicate][is-destination-predicate]
- [`IS LABELED` predicate][is-labeled-predicate]
- [`SAME` predicate][same-predicate]

[all-different-predicate]: #all-different_predicate
[property-exists-predicate]: #property-exists_predicate
[is-source-predicate]: #is-source_predicate
[is-destination-predicate]: #is-destination_predicate
[is-labeled-predicate]: #is-labeled_predicate
[same-predicate]: #same-predicate

### `IS DESTINATION` predicate

```sql
node IS [ NOT ] DESTINATION [ OF ] edge
```

**Description**

In a graph, checks to see if a node is or isn't the destination of an edge.
Can produce `TRUE`, `FALSE`, or `NULL`.

Arguments:

- `node`: The graph pattern variable for the node element.
- `edge`: The graph pattern variable for the edge element.

**Examples**

```sql
GRAPH FinGraph
MATCH (a:Account)-[transfer:Transfers]-(b:Account)
WHERE a IS DESTINATION of transfer
RETURN a.id AS a_id, b.id AS b_id

/*-------------+
 | a_id | b_id |
 +-------------+
 | 16   | 7    |
 | 16   | 7    |
 | 20   | 16   |
 | 7    | 20   |
 | 16   | 20   |
 +-------------*/
```

```sql
GRAPH FinGraph
MATCH (a:Account)-[transfer:Transfers]-(b:Account)
WHERE b IS DESTINATION of transfer
RETURN a.id AS a_id, b.id AS b_id

/*-------------+
 | a_id | b_id |
 +-------------+
 | 7    | 16   |
 | 7    | 16   |
 | 16   | 20   |
 | 20   | 7    |
 | 20   | 16   |
 +-------------*/
```

### `IS LABELED` predicate

```sql
element IS [ NOT ] LABELED label_expression
```

**Description**

In a graph, checks to see if a node or edge label satisfies a label
expression. Can produce `TRUE`, `FALSE`, or `NULL` if `element` is `NULL`.

Arguments:

- `element`: The graph pattern variable for a graph node or edge element.
- `label_expression`: The label expression to verify.

**Examples**

```sql
GRAPH FinGraph
MATCH (a)
WHERE a IS LABELED Account | Person
RETURN a.id AS a_id, LABELS(a) AS labels

/*----------------+
 | a_id | labels  |
 +----------------+
 | 1    | Person  |
 | 2    | Person  |
 | 3    | Person  |
 | 7    | Account |
 | 16   | Account |
 | 20   | Account |
 +----------------*/
```

```sql
GRAPH FinGraph
MATCH (a)-[e]-(b:Account)
WHERE e IS LABELED Transfers | Owns
RETURN a.Id as a_id, Labels(e) AS labels, b.Id as b_id
ORDER BY a_id, b_id

/*------+-----------------------+------+
 | a_id | labels                | b_id |
 +------+-----------------------+------+
 |    1 | [owns]                |    7 |
 |    2 | [owns]                |   20 |
 |    3 | [owns]                |   16 |
 |    7 | [transfers]           |   16 |
 |    7 | [transfers]           |   16 |
 |    7 | [transfers]           |   20 |
 |   16 | [transfers]           |    7 |
 |   16 | [transfers]           |    7 |
 |   16 | [transfers]           |   20 |
 |   16 | [transfers]           |   20 |
 |   20 | [transfers]           |    7 |
 |   20 | [transfers]           |   16 |
 |   20 | [transfers]           |   16 |
 +------+-----------------------+------*/
```

```sql
GRAPH FinGraph
MATCH (a:Account {Id: 7})
OPTIONAL MATCH (a)-[:OWNS]->(b)
RETURN a.Id AS a_id, b.Id AS b_id, b IS LABELED Account AS b_is_account

/*------+-----------------------+
 | a_id | b_id   | b_is_account |
 +------+-----------------------+
 | 7    | NULL   | NULL         |
 +------+-----------------------+*/
```

### `IS SOURCE` predicate

```sql
node IS [ NOT ] SOURCE [ OF ] edge
```

**Description**

In a graph, checks to see if a node is or isn't the source of an edge.
Can produce `TRUE`, `FALSE`, or `NULL`.

Arguments:

- `node`: The graph pattern variable for the node element.
- `edge`: The graph pattern variable for the edge element.

**Examples**

```sql
GRAPH FinGraph
MATCH (a:Account)-[transfer:Transfers]-(b:Account)
WHERE a IS SOURCE of transfer
RETURN a.id AS a_id, b.id AS b_id

/*-------------+
 | a_id | b_id |
 +-------------+
 | 20   | 7    |
 | 7    | 16   |
 | 7    | 16   |
 | 20   | 16   |
 | 16   | 20   |
 +-------------*/
```

```sql
GRAPH FinGraph
MATCH (a:Account)-[transfer:Transfers]-(b:Account)
WHERE b IS SOURCE of transfer
RETURN a.id AS a_id, b.id AS b_id

/*-------------+
 | a_id | b_id |
 +-------------+
 | 7    | 20   |
 | 16   | 7    |
 | 16   | 7    |
 | 16   | 20   |
 | 20   | 16   |
 +-------------*/
```

### `PROPERTY_EXISTS` predicate

```sql
PROPERTY_EXISTS(element, element_property)
```

**Description**

In a graph, checks to see if a property exists for an element.
Can produce `TRUE`, `FALSE`, or `NULL`.

Arguments:

- `element`: The graph pattern variable for a node or edge element.
- `element_property`: The name of the property to look for in `element`.
  The property name must refer to a property in the graph. If the property
  doesn't exist in the graph, an error is produced. The property name is
  resolved in a case-insensitive manner.

**Example**

```sql
GRAPH FinGraph
MATCH (n:Person|Account WHERE PROPERTY_EXISTS(n, name))
RETURN n.name

/*------+
 | name |
 +------+
 | Alex |
 | Dana |
 | Lee  |
 +------*/
```

### `SAME` predicate

```sql
SAME (element, element[, ...])
```

**Description**

In a graph, checks if all graph elements in a list bind to the same node or
edge. Returns `TRUE` if the elements bind to the same node or edge, otherwise
`FALSE`.

Arguments:

- `element`: The graph pattern variable for a node or edge element.

**Details**

Produces an error if `element` is `NULL`.

**Example**

The following query checks to see if `a` and `b` aren't the same person.

```sql
GRAPH FinGraph
MATCH (src:Account)<-[transfer:Transfers]-(dest:Account)
WHERE NOT SAME(src, dest)
RETURN src.id AS source_id, dest.id AS destination_id

/*----------------------------+
 | source_id | destination_id |
 +----------------------------+
 | 7         | 20             |
 | 16        | 7              |
 | 16        | 7              |
 | 16        | 20             |
 | 20        | 16             |
 +----------------------------*/
```

### Comparison operators

Compares operands and produces the results of the comparison as a `BOOL`
value. These comparison operators are available:

| Name                     | Syntax                    | Description                                                                                                                                                                                                                                                    |
| ------------------------ | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Less Than                | `X < Y`                   | Returns `TRUE` if `X` is less than `Y`. This operator supports specifying [collation](../types/collation_concepts.md).                                                                                                                                         |
| Less Than or Equal To    | `X <= Y`                  | Returns `TRUE` if `X` is less than or equal to `Y`. This operator supports specifying [collation](../types/collation_concepts.md).                                                                                                                             |
| Greater Than             | `X > Y`                   | Returns `TRUE` if `X` is greater than `Y`. This operator supports specifying [collation](../types/collation_concepts.md).                                                                                                                                      |
| Greater Than or Equal To | `X >= Y`                  | Returns `TRUE` if `X` is greater than or equal to `Y`. This operator supports specifying [collation](../types/collation_concepts.md).                                                                                                                          |
| Equal                    | `X = Y`                   | Returns `TRUE` if `X` is equal to `Y`. This operator supports specifying [collation](../types/collation_concepts.md).                                                                                                                                          |
| Not Equal                | `X != Y` `X <> Y`         | Returns `TRUE` if `X` isn't equal to `Y`. This operator supports specifying [collation](../types/collation_concepts.md).                                                                                                                                       |
| `BETWEEN`                | `X [NOT] BETWEEN Y AND Z` | Returns `TRUE` if `X` is [not] within the range specified. The result of `X BETWEEN Y AND Z` is equivalent to `Y <= X AND X <= Z` but `X` is evaluated only once in the former. This operator supports specifying [collation](../types/collation_concepts.md). |
| `LIKE`                   | `X [NOT] LIKE Y`          | See the [`LIKE` operator](#like-operator) for details.                                                                                                                                                                                                         |
| `IN`                     | Multiple                  | See the [`IN` operator](#in-operator) for details.                                                                                                                                                                                                             |

The following rules apply to operands in a comparison operator:

- The operands must be [comparable][data-type-comparable].
- A comparison operator generally requires both operands to be of the
  same type.
- If the operands are of different types, and the values of those types can be
  converted to a common type without loss of precision,
  they are generally coerced to that common type for the comparison.
- A literal operand is generally coerced to the same data type of a
  non-literal operand that's part of the comparison.
- Comparisons between operands that are signed and unsigned integers is
  allowed.
- Struct operands support only these comparison operators: equal
  (`=`), not equal (`!=` and `<>`), and `IN`.

The following rules apply when comparing these data types:

- Floating point:
  All comparisons with `NaN` return `FALSE`,
  except for `!=` and `<>`, which return `TRUE`.
- `BOOL`: `FALSE` is less than `TRUE`.
- `VARCHAR`: Strings are compared codepoint-by-codepoint, which means that
  canonically equivalent strings are only guaranteed to compare as equal if
  they have been normalized first.
- `JSON`: You can't compare JSON, but you can compare
  the values inside of JSON if you convert the values to
  SQL values first. For more information, see
  [`JSON` functions][json-functions].
- `NULL`: Any operation with a `NULL` input returns `NULL`.
- `STRUCT`: When testing a struct for equality, it's possible that one or more
  fields are `NULL`. In such cases:
  - If all non-`NULL` field values are equal, the comparison returns `NULL`.
  - If any non-`NULL` field values aren't equal, the comparison returns
    `FALSE`.

  The following table demonstrates how `STRUCT` data types are compared when
  they have fields that are `NULL` valued.

| Struct1           | Struct2           | Struct1 = Struct2 |
| ----------------- | ----------------- | ----------------- |
| `STRUCT(1, NULL)` | `STRUCT(1, NULL)` | `NULL`            |
| `STRUCT(1, NULL)` | `STRUCT(2, NULL)` | `FALSE`           |
| `STRUCT(1,2)`     | `STRUCT(1, NULL)` | `NULL`            |

[data-type-comparable]: ../types/data_types.md#comparable-data_types
[json-functions]: ../functions/json_functions.md

### `EXISTS` operator

```sql
EXISTS( subquery )
```

**Description**

Returns `TRUE` if the subquery produces one or more rows. Returns `FALSE` if
the subquery produces zero rows. Never returns `NULL`. To learn more about
how you can use a subquery with `EXISTS`,
see [`EXISTS` subqueries][exists-subqueries].

**Examples**

In this example, the `EXISTS` operator returns `FALSE` because there are no
rows in `Words` where the direction is `south`:

```sql
WITH Words AS (
  SELECT 'Intend' as value, 'east' as direction UNION ALL
  SELECT 'Secure', 'north' UNION ALL
  SELECT 'Clarity', 'west'
 )
SELECT EXISTS( SELECT value FROM Words WHERE direction = 'south' ) as result;

/*--------+
 | result |
 +--------+
 | FALSE  |
 +--------*/
```

[exists-subqueries]: subqueries.md#exists-subquery_concepts

### `IN` operator

The `IN` operator supports the following syntax:

```sql
search_value [NOT] IN value_set

value_set:
  {
    (expression[, ...])
    | (subquery)
    | UNNEST(array_expression)
  }
```

**Description**

Checks for an equal value in a set of values.
[Semantic rules][semantic-rules-in] apply, but in general, `IN` returns `TRUE`
if an equal value is found, `FALSE` if an equal value is excluded, otherwise
`NULL`. `NOT IN` returns `FALSE` if an equal value is found, `TRUE` if an
equal value is excluded, otherwise `NULL`.

- `search_value`: The expression that's compared to a set of values.
- `value_set`: One or more values to compare to a search value.
  - `(expression[, ...])`: A list of expressions.
  - `(subquery)`: A [subquery][operators-subqueries] that returns
    a single column. The values in that column are the set of values.
    If no rows are produced, the set of values is empty.
  - `UNNEST(array_expression)`: An [UNNEST operator][operators-link-to-unnest]
    that returns a column of values from an array expression. This is
    equivalent to:

    ```sql
    IN (SELECT element FROM UNNEST(array_expression) AS element)
    ```

This operator supports [collation][collation], but these limitations apply:

- `[NOT] IN UNNEST` doesn't support collation.
- If collation is used with a list of expressions, there must be at least one
  item in the list.

**Semantic rules**

When using the `IN` operator, the following semantics apply in this order:

- Returns `FALSE` if `value_set` is empty.
- Returns `NULL` if `search_value` is `NULL`.
- Returns `TRUE` if `value_set` contains a value equal to `search_value`.
- Returns `NULL` if `value_set` contains a `NULL`.
- Returns `FALSE`.

When using the `NOT IN` operator, the following semantics apply in this order:

- Returns `TRUE` if `value_set` is empty.
- Returns `NULL` if `search_value` is `NULL`.
- Returns `FALSE` if `value_set` contains a value equal to `search_value`.
- Returns `NULL` if `value_set` contains a `NULL`.
- Returns `TRUE`.

The semantics of:

```sql
x IN (y, z, ...)
```

are defined as equivalent to:

```sql
(x = y) OR (x = z) OR ...
```

and the subquery and array forms are defined similarly.

```sql
x NOT IN ...
```

is equivalent to:

```sql
NOT(x IN ...)
```

The `UNNEST` form treats an array scan like `UNNEST` in the
[`FROM`][operators-link-to-from-clause] clause:

```sql
x [NOT] IN UNNEST()
```

This form is often used with array parameters. For example:

```sql
x IN UNNEST(@array_parameter)
```

See the [Arrays][operators-link-to-filtering-arrays] topic for more information
on how to use this syntax.

`IN` can be used with multi-part keys by using the struct constructor syntax.
For example:

```sql
(Key1, Key2) IN ( (12,34), (56,78) )
(Key1, Key2) IN ( SELECT (table.a, table.b) FROM table )
```

See the [Struct Type][operators-link-to-struct-type] topic for more information.

**Return Data Type**

`BOOL`

**Examples**

You can use these `WITH` clauses to emulate temporary tables for
`Words` and `Items` in the following examples:

```sql
WITH Words AS (
  SELECT 'Intend' as value UNION ALL
  SELECT 'Secure' UNION ALL
  SELECT 'Clarity' UNION ALL
  SELECT 'Peace' UNION ALL
  SELECT 'Intend'
 )
SELECT * FROM Words;

/*----------+
 | value    |
 +----------+
 | Intend   |
 | Secure   |
 | Clarity  |
 | Peace    |
 | Intend   |
 +----------*/
```

```sql
WITH
  Items AS (
    SELECT STRUCT('blue' AS color, 'round' AS shape) AS info UNION ALL
    SELECT STRUCT('blue', 'square') UNION ALL
    SELECT STRUCT('red', 'round')
  )
SELECT * FROM Items;

/*----------------------------+
 | info                       |
 +----------------------------+
 | {blue color, round shape}  |
 | {blue color, square shape} |
 | {red color, round shape}   |
 +----------------------------*/
```

Example with `IN` and an expression:

```sql
SELECT * FROM Words WHERE value IN ('Intend', 'Secure');

/*----------+
 | value    |
 +----------+
 | Intend   |
 | Secure   |
 | Intend   |
 +----------*/
```

Example with `NOT IN` and an expression:

```sql
SELECT * FROM Words WHERE value NOT IN ('Intend');

/*----------+
 | value    |
 +----------+
 | Secure   |
 | Clarity  |
 | Peace    |
 +----------*/
```

Example with `IN`, a scalar subquery, and an expression:

```sql
SELECT * FROM Words WHERE value IN ((SELECT 'Intend'), 'Clarity');

/*----------+
 | value    |
 +----------+
 | Intend   |
 | Clarity  |
 | Intend   |
 +----------*/
```

Example with `IN` and an `UNNEST` operation:

```sql
SELECT * FROM Words WHERE value IN UNNEST(['Secure', 'Clarity']);

/*----------+
 | value    |
 +----------+
 | Secure   |
 | Clarity  |
 +----------*/
```

Example with `IN` and a struct:

```sql
SELECT
  (SELECT AS STRUCT Items.info) as item
FROM
  Items
WHERE (info.shape, info.color) IN (('round', 'blue'));

/*------------------------------------+
 | item                               |
 +------------------------------------+
 | { {blue color, round shape} info } |
 +------------------------------------*/
```

[semantic-rules-in]: #semantic-rules_in
[operators-subqueries]: subqueries.md#about-subqueries
[operators-link-to-unnest]: query_syntax.md#unnest-operator
[collation]: ../types/collation_concepts.md#collate-funcs
[operators-link-to-from-clause]: query_syntax.md#from-clause
[operators-link-to-filtering-arrays]: ../types/arrays.md#filtering-arrays
[operators-link-to-struct-type]: ../types/data_types.md#struct-type

### `IS` operators

IS operators return TRUE or FALSE for the condition they are testing. They never
return `NULL`, even for `NULL` inputs, unlike the `IS_INF` and `IS_NAN`
functions defined in [Mathematical Functions][operators-link-to-math-functions].
If `NOT` is present, the output `BOOL` value is
inverted.

| Function Syntax    | Input Data Type | Result Data Type | Description                                                                       |
| ------------------ | --------------- | ---------------- | --------------------------------------------------------------------------------- |
| `X IS TRUE`        | `BOOL`          | `BOOL`           | Evaluates to `TRUE` if `X` evaluates to `TRUE`. Otherwise, evaluates to `FALSE`.  |
| `X IS NOT TRUE`    | `BOOL`          | `BOOL`           | Evaluates to `FALSE` if `X` evaluates to `TRUE`. Otherwise, evaluates to `TRUE`.  |
| `X IS FALSE`       | `BOOL`          | `BOOL`           | Evaluates to `TRUE` if `X` evaluates to `FALSE`. Otherwise, evaluates to `FALSE`. |
| `X IS NOT FALSE`   | `BOOL`          | `BOOL`           | Evaluates to `FALSE` if `X` evaluates to `FALSE`. Otherwise, evaluates to `TRUE`. |
| `X IS NULL`        | Any value type  | `BOOL`           | Evaluates to `TRUE` if `X` evaluates to `NULL`. Otherwise evaluates to `FALSE`.   |
| `X IS NOT NULL`    | Any value type  | `BOOL`           | Evaluates to `FALSE` if `X` evaluates to `NULL`. Otherwise evaluates to `TRUE`.   |
| `X IS UNKNOWN`     | `BOOL`          | `BOOL`           | Evaluates to `TRUE` if `X` evaluates to `NULL`. Otherwise evaluates to `FALSE`.   |
| `X IS NOT UNKNOWN` | `BOOL`          | `BOOL`           | Evaluates to `FALSE` if `X` evaluates to `NULL`. Otherwise, evaluates to `TRUE`.  |

[operators-link-to-math-functions]: ../functions/mathematical_functions.md

### `IS DISTINCT FROM` operator

```sql
expression_1 IS [NOT] DISTINCT FROM expression_2
```

**Description**

`IS DISTINCT FROM` returns `TRUE` if the input values are considered to be
distinct from each other by the [`DISTINCT`][operators-distinct] and
[`GROUP BY`][operators-group-by] clauses. Otherwise, returns `FALSE`.

`a IS DISTINCT FROM b` being `TRUE` is equivalent to:

- `SELECT COUNT(DISTINCT x) FROM UNNEST([a,b]) x` returning `2`.
- `SELECT * FROM UNNEST([a,b]) x GROUP BY x` returning 2 rows.

`a IS DISTINCT FROM b` is equivalent to `NOT (a = b)`, except for the
following cases:

- This operator never returns `NULL` so `NULL` values are considered to be
  distinct from non-`NULL` values, not other `NULL` values.
- `NaN` values are considered to be distinct from non-`NaN` values, but not
  other `NaN` values.

Input values:

- `expression_1`: The first value to compare. This can be a groupable data type,
  `NULL` or `NaN`.
- `expression_2`: The second value to compare. This can be a groupable
  data type, `NULL` or `NaN`.
- `NOT`: If present, the output `BOOL` value is inverted.

**Return type**

`BOOL`

**Examples**

These return `TRUE`:

```sql
SELECT 1 IS DISTINCT FROM 2
```

```sql
SELECT 1 IS DISTINCT FROM NULL
```

```sql
SELECT 1 IS NOT DISTINCT FROM 1
```

```sql
SELECT NULL IS NOT DISTINCT FROM NULL
```

These return `FALSE`:

```sql
SELECT NULL IS DISTINCT FROM NULL
```

```sql
SELECT 1 IS DISTINCT FROM 1
```

```sql
SELECT 1 IS NOT DISTINCT FROM 2
```

```sql
SELECT 1 IS NOT DISTINCT FROM NULL
```

[operators-distinct]: query_syntax.md#select-distinct
[operators-group-by]: query_syntax.md#group-by_clause

### `LIKE` operator

```sql
expression_1 [NOT] LIKE expression_2
```

**Description**

`LIKE` returns `TRUE` if the string in the first operand `expression_1`
matches a pattern specified by the second operand `expression_2`,
otherwise returns `FALSE`.

`NOT LIKE` returns `TRUE` if the string in the first operand `expression_1`
doesn't match a pattern specified by the second operand `expression_2`,
otherwise returns `FALSE`.

Expressions can contain these characters:

- A percent sign (`%`) matches any number of characters or bytes.
- An underscore (`_`) matches a single character or byte.
- You can escape `\ `, `_`, or `%` using two backslashes. For example,
  `\\% `. If you are using raw strings, only a single backslash is
  required. For example, `r'\%'`.

This operator supports [collation][collation], but caveats apply:

- Each `%` character in `expression_2` represents an
  _arbitrary string specifier_. An arbitrary string specifier can represent
  any sequence of `0` or more characters.
- A character in the expression represents itself and is considered a
  _single character specifier_ unless:
  - The character is a percent sign (`%`).

  - The character is an underscore (`_`) and the collator isn't `und:ci`.

- These additional rules apply to the underscore (`_`) character:
  - If the collator isn't `und:ci`, an error is produced when an underscore
    isn't escaped in `expression_2`.

  - If the collator isn't `und:ci`, the underscore isn't allowed when the
    operands have collation specified.

  - Some _compatibility composites_, such as the fi-ligature (`ﬁ`) and the
    telephone sign (`℡`), will produce a match if they are compared to an
    underscore.

  - A single underscore matches the idea of what a character is, based on
    an approximation known as a [_grapheme cluster_][grapheme-cluster].

- For a contiguous sequence of single character specifiers, equality
  depends on the collator and its language tags and tailoring.
  - By default, the `und:ci` collator doesn't fully normalize a string.
    Some canonically equivalent strings are considered unequal for
    both the `=` and `LIKE` operators.

  - The `LIKE` operator with collation has the same behavior as the `=`
    operator when there are no wildcards in the strings.

  - Character sequences with secondary or higher-weighted differences are
    considered unequal. This includes accent differences and some
    special cases.

    For example there are three ways to produce German sharp `ß`:
    - `\u1E9E`
    - `\U00DF`
    - `ss`

    `\u1E9E` and `\U00DF` are considered equal but differ in tertiary.
    They are considered equal with `und:ci` collation but different from
    `ss`, which has secondary differences.

  - Character sequences with tertiary or lower-weighted differences are
    considered equal. This includes case differences and
    kana subtype differences, which are considered equal.

- There are [ignorable characters][ignorable-chars] defined in Unicode.
  Ignorable characters are ignored in the pattern matching.

**Return type**

`BOOL`

**Examples**

The following examples illustrate how you can check to see if the string in the
first operand matches a pattern specified by the second operand.

```sql
-- Returns TRUE
SELECT 'apple' LIKE 'a%';
```

```sql
-- Returns FALSE
SELECT '%a' LIKE 'apple';
```

```sql
-- Returns FALSE
SELECT 'apple' NOT LIKE 'a%';
```

```sql
-- Returns TRUE
SELECT '%a' NOT LIKE 'apple';
```

```sql
-- Produces an error
SELECT NULL LIKE 'a%';
```

```sql
-- Produces an error
SELECT 'apple' LIKE NULL;
```

The following example illustrates how to search multiple patterns in an array
to find a match with the `LIKE` operator:

```sql
WITH Words AS
 (SELECT 'Intend with clarity.' as value UNION ALL
  SELECT 'Secure with intention.' UNION ALL
  SELECT 'Clarity and security.')
SELECT value
FROM Words
WHERE ARRAY_INCLUDES(['%ity%', '%and%'], pattern->(Words.value LIKE pattern));

/*------------------------+
 | value                  |
 +------------------------+
 | Intend with clarity.   |
 | Clarity and security.  |
 +------------------------*/
```

The following examples illustrate how collation can be used with the `LIKE`
operator.

```sql
-- Returns FALSE
'Foo' LIKE '%foo%'
```

```sql
-- Returns TRUE
COLLATE('Foo', 'und:ci') LIKE COLLATE('%foo%', 'und:ci');
```

```sql
-- Returns TRUE
COLLATE('Foo', 'und:ci') = COLLATE('foo', 'und:ci');
```

```sql
-- Produces an error
COLLATE('Foo', 'und:ci') LIKE COLLATE('%foo%', 'binary');
```

```sql
-- Produces an error
COLLATE('Foo', 'und:ci') LIKE COLLATE('%f_o%', 'und:ci');
```

```sql
-- Returns TRUE
COLLATE('Foo_', 'und:ci') LIKE COLLATE('%foo\\_%', 'und:ci');
```

There are two capital forms of `ß`. We can use either `SS` or `ẞ` as upper
case. While the difference between `ß` and `ẞ` is case difference (tertiary
difference), the difference between sharp `s` and `ss` is secondary and
considered not equal using the `und:ci` collator. For example:

```sql
-- Returns FALSE
'MASSE' LIKE 'Maße';
```

```sql
-- Returns FALSE
COLLATE('MASSE', 'und:ci') LIKE '%Maße%';
```

```sql
-- Returns FALSE
COLLATE('MASSE', 'und:ci') = COLLATE('Maße', 'und:ci');
```

The kana differences in Japanese are considered as tertiary or quaternary
differences, and should be considered as equal in the `und:ci` collator with
secondary strength.

- `'\u3042'` is `'あ'` (hiragana)
- `'\u30A2'` is `'ア'` (katakana)

For example:

```sql
-- Returns FALSE
'\u3042' LIKE '%\u30A2%';
```

```sql
-- Returns TRUE
COLLATE('\u3042', 'und:ci') LIKE COLLATE('%\u30A2%', 'und:ci');
```

```sql
-- Returns TRUE
COLLATE('\u3042', 'und:ci') = COLLATE('\u30A2', 'und:ci');
```

When comparing two strings, the `und:ci` collator compares the collation units
based on the specification of the collation. Even though the number of
code points is different, the two strings are considered equal when the
collation units are considered the same.

- `'\u0041\u030A'` is `'Å'` (two code points)
- `'\u0061\u030A'` is `'å'` (two code points)
- `'\u00C5'` is `'Å'` (one code point)

In the following examples, the difference between `'\u0061\u030A'` and
`'\u00C5'` is tertiary.

```sql
-- Returns FALSE
'\u0061\u030A' LIKE '%\u00C5%';
```

```sql
-- Returns TRUE
COLLATE('\u0061\u030A', 'und:ci') LIKE '%\u00C5%';
```

```sql
-- Returns TRUE
COLLATE('\u0061\u030A', 'und:ci') = COLLATE('\u00C5', 'und:ci');
```

In the following example, `'\u0083'` is a `NO BREAK HERE` character and
is ignored.

```sql
-- Returns FALSE
'\u0083' LIKE '';
```

```sql
-- Returns TRUE
COLLATE('\u0083', 'und:ci') LIKE '';
```

[ignorable-chars]: https://www.unicode.org/charts/collation/chart_Ignored.html
[collation]: ../types/collation_concepts.md#collate-funcs
[grapheme-cluster]: https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries

### Quantified `LIKE` operator

The quantified `LIKE` operator supports the following syntax:

```sql
search_value [NOT] LIKE quantifier patterns

quantifier:
 { ANY | SOME | ALL }

patterns:
  {
    pattern_expression_list
    | pattern_subquery
    | pattern_array
  }

pattern_expression_list:
  (expression[, ...])

pattern_subquery:
  (subquery)

pattern_array:
  UNNEST(array_expression)
```

**Description**

Checks `search_value` for matches against several patterns. Each comparison is
case-sensitive. Wildcard searches are supported.
[Semantic rules][semantic-rules-quant-like] apply, but in general, `LIKE`
returns `TRUE` if a matching pattern is found, `FALSE` if a matching pattern
isn't found, or otherwise `NULL`. `NOT LIKE` returns `FALSE` if a
matching pattern is found, `TRUE` if a matching pattern isn't found, or
otherwise `NULL`.

- `search_value`: The value to search for matching patterns. This value can be a
  `VARCHAR` or `VARBINARY` type.
- `patterns`: The patterns to look for in the search value. Each pattern must
  resolve to the same type as `search_value`.
  - `pattern_expression_list`: A list of one or more patterns that match the
    `search_value` type.

  - `pattern_subquery`: A [subquery][operators-subqueries] that returns
    a single column with the same type as `search_value`.

  - `pattern_array`: An [`UNNEST`][operators-link-to-unnest]
    operation that returns a column of values with
    the same type as `search_value` from an array expression.

  The regular expressions that are supported by the
  [`LIKE` operator][like-operator] are also supported by `patterns` in the
  [quantified `LIKE` operator][like-operator].

- `quantifier`: Condition for pattern matching.
  - `ANY`: Checks if the set of patterns contains at least one pattern that
    matches the search value.

  - `SOME`: Synonym for `ANY`.

  - `ALL`: Checks if every pattern in the set of patterns matches the
    search value.

**Collation caveats**

[Collation][collation] is supported, but with the following caveats:

- The collation caveats that apply to the [`LIKE` operator][like-operator] also
  apply to the quantified `LIKE` operator.
- If a collation-supported input contains no collation specification or an
  empty collation specification and another input contains an explicitly defined
  collation, the explicitly defined collation is used for all of the inputs.
- All inputs with a non-empty, explicitly defined collation specification must
  have the same type of collation specification, otherwise an error is thrown.

**Semantics rules**

When using the quantified `LIKE` operator with `ANY` or `SOME`, the
following semantics apply in this order:

- Returns `FALSE` if `patterns` is empty.
- Returns `NULL` if `search_value` is `NULL`.
- Returns `TRUE` if `search_value` matches at least one value in `patterns`.
- Returns `NULL` if a pattern in `patterns` is `NULL`.
- Returns `FALSE`.

When using the quantified `LIKE` operator with `ALL`, the following semantics
apply in this order:

- Returns `TRUE` if `patterns` is empty.
- Returns `NULL` if `search_value` is `NULL`.
- Returns `FALSE` if `search_value LIKE pattern` is `FALSE` for at least one value in `patterns`.
- Returns `NULL` if a pattern in `patterns` is `NULL`.
- Returns `TRUE`.

When using the quantified `NOT LIKE` operator with `ANY` or `SOME`, the
following semantics apply in this order:

- Returns `FALSE` if `patterns` is empty.
- Returns `NULL` if `search_value` is `NULL`.
- Returns `TRUE` if `search_value LIKE pattern` is `FALSE` for at least one value in `patterns`.
- Returns `NULL` if a pattern in `patterns` is `NULL`.
- Returns `FALSE`.

When using the quantified `NOT LIKE` operator with `ALL`, the following
semantics apply in this order:

- Returns `TRUE` if `patterns` is empty.
- For `pattern_array`, returns `TRUE` if `patterns` is empty.
- Returns `NULL` if `search_value` is `NULL`.
- Returns `FALSE` if `search_value` matches at least one value in `patterns`.
- Returns `NULL` if a pattern in `patterns` is `NULL`.
- Returns `TRUE`.

**Return Data Type**

`BOOL`

**Examples**

The following example checks to see if the `Intend%` or `%intention%`
pattern exists in a value and produces that value if either pattern is found:

```sql
WITH Words AS
 (SELECT 'Intend with clarity.' as value UNION ALL
  SELECT 'Secure with intention.' UNION ALL
  SELECT 'Clarity and security.')
SELECT * FROM Words WHERE value LIKE ANY ('Intend%', '%intention%');

/*------------------------+
 | value                  |
 +------------------------+
 | Intend with clarity.   |
 | Secure with intention. |
 +------------------------*/
```

The following example checks to see if the `%ity%`
pattern exists in a value and produces that value if the pattern is found.

Example with `LIKE ALL`:

```sql
WITH Words AS
 (SELECT 'Intend with clarity.' as value UNION ALL
  SELECT 'Secure with intention.' UNION ALL
  SELECT 'Clarity and security.')
SELECT * FROM Words WHERE value LIKE ALL ('%ity%');

/*-----------------------+
 | value                 |
 +-----------------------+
 | Intend with clarity.  |
 | Clarity and security. |
 +-----------------------*/
```

The following example checks to see if the `%ity%`
pattern exists in a value produces that value if the pattern
isn't found:

```sql
WITH Words AS
 (SELECT 'Intend with clarity.' as value UNION ALL
  SELECT 'Secure with intention.' UNION ALL
  SELECT 'Clarity and security.')
SELECT * FROM Words WHERE value NOT LIKE ('%ity%');

/*------------------------+
 | value                  |
 +------------------------+
 | Secure with intention. |
 +------------------------*/
```

You can use a subquery as an expression in `patterns`. For example:

```sql
WITH Words AS
 (SELECT 'Intend with clarity.' as value UNION ALL
  SELECT 'Secure with intention.' UNION ALL
  SELECT 'Clarity and security.')
SELECT * FROM Words WHERE value LIKE ANY ((SELECT '%ion%'), '%and%');

/*------------------------+
 | value                  |
 +------------------------+
 | Secure with intention. |
 | Clarity and security.  |
 +------------------------*/
```

You can pass in a subquery for `patterns`. For example:

```sql
WITH Words AS
 (SELECT 'Intend with clarity.' as value UNION ALL
  SELECT 'Secure with intention.' UNION ALL
  SELECT 'Clarity and security.')
SELECT * FROM Words WHERE value LIKE ANY (SELECT '%with%');

/*------------------------+
 | value                  |
 +------------------------+
 | Intend with clarity.   |
 | Secure with intention. |
 +------------------------*/
```

You can pass in an array for `patterns`. For example:

```sql
WITH Words AS
 (SELECT 'Intend with clarity.' as value UNION ALL
  SELECT 'Secure with intention.' UNION ALL
  SELECT 'Clarity and security.')
SELECT * FROM Words WHERE value LIKE ANY UNNEST(['%ion%', '%and%']);

/*------------------------+
 | value                  |
 +------------------------+
 | Secure with intention. |
 | Clarity and security.  |
 +------------------------*/
```

You can pass in an array and subquery for `patterns`. For example:

```sql
WITH Words AS
 (SELECT 'Intend with clarity.' as value UNION ALL
  SELECT 'Secure with intention.' UNION ALL
  SELECT 'Clarity and security.')
SELECT *
FROM Words
WHERE
  value LIKE ANY UNNEST(ARRAY(SELECT e FROM UNNEST(['%ion%', '%and%']) AS e));

/*------------------------+
 | value                  |
 +------------------------+
 | Secure with intention. |
 | Clarity and security.  |
 +------------------------*/
```

The following queries illustrate some of the semantic rules for the
quantified `LIKE` operator:

```sql
SELECT
  NULL LIKE ANY ('a', 'b'), -- NULL
  'a' LIKE ANY ('a', 'c'), -- TRUE
  'a' LIKE ANY ('b', 'c'), -- FALSE
  'a' LIKE ANY ('a', NULL), -- TRUE
  'a' LIKE ANY ('b', NULL), -- NULL
  NULL NOT LIKE ANY ('a', 'b'), -- NULL
  'a' NOT LIKE ANY ('a', 'b'), -- TRUE
  'a' NOT LIKE ANY ('a', '%a%'), -- FALSE
  'a' NOT LIKE ANY ('a', NULL), -- NULL
  'a' NOT LIKE ANY ('b', NULL); -- TRUE
```

```sql
SELECT
  NULL LIKE SOME ('a', 'b'), -- NULL
  'a' LIKE SOME ('a', 'c'), -- TRUE
  'a' LIKE SOME ('b', 'c'), -- FALSE
  'a' LIKE SOME ('a', NULL), -- TRUE
  'a' LIKE SOME ('b', NULL), -- NULL
  NULL NOT LIKE SOME ('a', 'b'), -- NULL
  'a' NOT LIKE SOME ('a', 'b'), -- TRUE
  'a' NOT LIKE SOME ('a', '%a%'), -- FALSE
  'a' NOT LIKE SOME ('a', NULL), -- NULL
  'a' NOT LIKE SOME ('b', NULL); -- TRUE
```

```sql
SELECT
  NULL LIKE ALL ('a', 'b'), -- NULL
  'a' LIKE ALL ('a', '%a%'), -- TRUE
  'a' LIKE ALL ('a', 'c'), -- FALSE
  'a' LIKE ALL ('a', NULL), -- NULL
  'a' LIKE ALL ('b', NULL), -- FALSE
  NULL NOT LIKE ALL ('a', 'b'), -- NULL
  'a' NOT LIKE ALL ('b', 'c'), -- TRUE
  'a' NOT LIKE ALL ('a', 'c'), -- FALSE
  'a' NOT LIKE ALL ('a', NULL), -- FALSE
  'a' NOT LIKE ALL ('b', NULL); -- NULL
```

The following queries illustrate some of the semantic rules for the
quantified `LIKE` operator and collation:

```sql
SELECT
  COLLATE('a', 'und:ci') LIKE ALL ('a', 'A'), -- TRUE
  'a' LIKE ALL (COLLATE('a', 'und:ci'), 'A'), -- TRUE
  'a' LIKE ALL ('%A%', COLLATE('a', 'und:ci')); -- TRUE
```

```sql
-- ERROR: VARBINARY and VARCHAR values can't be used together.
SELECT b'a' LIKE ALL (COLLATE('a', 'und:ci'), 'A');
```

[like-operator]: #like-operator
[semantic-rules-quant-like]: #semantic-rules_quant_like
[reg-expressions-quant-like]: #reg-expressions_quant_like
[operators-subqueries]: subqueries.md#about-subqueries
[operators-link-to-unnest]: query_syntax.md#unnest-operator
[collation]: ../types/collation_concepts.md#collate-funcs

### `NEW` operator

The `NEW` operator only supports and uses the following syntax:

- `NEW protocol_buffer {...}`: Creates a
  using a map constructor.

````sql
NEW protocol_buffer {
  field_name: literal_or_expression
  field_name { ... }
  repeated_field_name: [literal_or_expression, ... ]
}
```sql
+   `NEW protocol_buffer (...)`: Creates a  using a parenthesized
  list of arguments.

  ```sql
  NEW protocol_buffer(field [AS alias], ...field [AS alias])
  ```

**Examples**

The following example uses the `NEW` operator with a map constructor:

```sql
NEW Universe {
name: "Sol"
closest_planets: ["Mercury", "Venus", "Earth" ]
star {
  radius_miles: 432,690
  age: 4,603,000,000
}
constellations: [{
  name: "Libra"
  index: 0
}, {
  name: "Scorpio"
  index: 1
}]
all_planets: (SELECT planets FROM SolTable)
}
````

The following example uses the `NEW` operator with a parenthesized list of
arguments:

```sql
SELECT
  key,
  name,
  NEW (key AS rank, name AS chart_name)
FROM
  (SELECT 1 AS key, "2" AS name);
```

### Concatenation operator

The concatenation operator combines multiple values into one.

| Function Syntax                               | Input Data Type | Result Data Type |
| --------------------------------------------- | --------------- | ---------------- |
| `VARCHAR \|\| VARCHAR [\|\| ... ]`            | `VARCHAR`       | `VARCHAR`        |
| `VARBINARY \|\| VARBINARY [\|\| ... ]`        | `VARBINARY`     | `VARBINARY`      |
| `ARRAY<T> \|\| ARRAY<T> [\|\| ... ]`          | `ARRAY<T>`      | `ARRAY<T>`       |

Note: The concatenation operator is translated into a nested
[`CONCAT`][concat] function call. For example, `'A' || 'B' || 'C'` becomes
`CONCAT('A', CONCAT('B', 'C'))`.

[concat]: ../functions/string_functions.md#concat

### `WITH` expression

```sql
WITH(variable_assignment[, ...], result_expression)

variable_assignment:
  variable_name AS expression
```

**Description**

Creates one or more variables. Each variable can be used in subsequent
expressions within the `WITH` expression. Returns the value of
`result_expression`.

- `variable_assignment`: Introduces a variable. The variable name must be
  unique within a given `WITH` expression. Each expression can reference the
  variables that come before it. For example, if you create variable `a`,
  then follow it with variable `b`, then you can reference `a` inside of the
  expression for `b`.
  - `variable_name`: The name of the variable.

  - `expression`: The value to assign to the variable.

- `result_expression`: An expression that can use all of the variables defined
  before it. The value of `result_expression` is returned by the `WITH`
  expression.

**Return Type**

- The type of the `result_expression`.

**Requirements and Caveats**

- A variable can only be assigned once within a `WITH` expression.
- Variables created during `WITH` may not be used
  in analytic or aggregate
  function arguments. For example,
  `WITH(a AS ..., SUM(a))` produces an error.
- Each variable's expression is evaluated only once.

**Examples**

The following example first concatenates variable `a` with `b`, then variable
`b` with `c`:

```sql
SELECT WITH(a AS '123',               -- a is '123'
            b AS CONCAT(a, '456'),    -- b is '123456'
            c AS '789',               -- c is '789'
            CONCAT(b, c)) AS result;  -- b + c is '123456789'

/*-------------+
 | result      |
 +-------------+
 | '123456789' |
 +-------------*/
```

In the following example, the volatile expression `RAND()` is evaluated once.
The value of the result expression is always `0.0`:

```sql
SELECT WITH(a AS RAND(), a - a);

/*---------+
 | result  |
 +---------+
 | 0.0     |
 +---------*/
```

Aggregate or analytic function
results can be stored in variables.

```sql
SELECT WITH(s AS SUM(input), c AS COUNT(input), s/c)
FROM UNNEST([1.0, 2.0, 3.0]) AS input;

/*---------+
 | result  |
 +---------+
 | 2.0     |
 +---------*/
```

Variables can't be used in aggregate or
analytic function call arguments.

```sql
SELECT WITH(diff AS a - b, AVG(diff))
FROM UNNEST([
              STRUCT(1 AS a, 2 AS b),
              STRUCT(3 AS a, 4 AS b),
              STRUCT(5 AS a, 6 AS b)
            ]);

-- ERROR: WITH variables like 'diff' can't be used in aggregate or analytic
-- function arguments.
```

A `WITH` expression is different from a `WITH` clause. The following example
shows a query that uses both:

```sql
WITH my_table AS (
  SELECT 1 AS x, 2 AS y
  UNION ALL
  SELECT 3 AS x, 4 AS y
  UNION ALL
  SELECT 5 AS x, 6 AS y
)
SELECT WITH(a AS SUM(x), b AS COUNT(x), a/b) AS avg_x, AVG(y) AS avg_y
FROM my_table
WHERE x > 1;

/*-------+-------+
 | avg_x | avg_y |
 +-------+-------+
 | 4     | 5     |
 +-------+-------*/
```
