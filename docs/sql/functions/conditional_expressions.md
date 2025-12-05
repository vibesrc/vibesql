# Conditional expressions

SQL supports conditional expressions.
Conditional expressions impose constraints on the evaluation order of their
inputs. In essence, they are evaluated left to right, with short-circuiting, and
only evaluate the output value that was chosen. In contrast, all inputs to
regular functions are evaluated before calling the function. Short-circuiting in
conditional expressions can be exploited for error handling or performance
tuning.

### Expression list

| Name                        | Summary                                                                                                                         |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| [`CASE expr`](#case-expr)   | Compares the given expression to each successive `WHEN` clause and produces the first result where the values are equal.        |
| [`CASE`](#case)             | Evaluates the condition of each successive `WHEN` clause and produces the first result where the condition evaluates to `TRUE`. |
| [`COALESCE`](#coalesce)     | Produces the value of the first non-`NULL` expression, if any, otherwise `NULL`.                                                |
| [`IF`](#if)                 | If an expression evaluates to `TRUE`, produces a specified result, otherwise produces the evaluation for an _else result_.      |
| [`IFNULL`](#ifnull)         | If an expression evaluates to `NULL`, produces a specified result, otherwise produces the expression.                           |
| [`NULLIF`](#nullif)         | Produces `NULL` if the first expression that matches another evaluates to `TRUE`, otherwise returns the first expression.       |
| [`NULLIFZERO`](#nullifzero) | Produces `NULL` if an expression is `0`, otherwise produces the expression.                                                     |
| [`ZEROIFNULL`](#zeroifnull) | Produces `0` if an expression is `NULL`, otherwise produces the expression.                                                     |

### `CASE expr`

```sql
CASE expr
  WHEN expr_to_match THEN result
  [ ... ]
  [ ELSE else_result ]
  END
```

**Description**

Compares `expr` to `expr_to_match` of each successive `WHEN` clause and returns
the first result where this comparison evaluates to `TRUE`. The remaining `WHEN`
clauses and `else_result` aren't evaluated.

If the `expr = expr_to_match` comparison evaluates to `FALSE` or `NULL` for all
`WHEN` clauses, returns the evaluation of `else_result` if present; if
`else_result` isn't present, then returns `NULL`.

Consistent with [equality comparisons][logical-operators] elsewhere, if both
`expr` and `expr_to_match` are `NULL`, then `expr = expr_to_match` evaluates to
`NULL`, which returns `else_result`. If a CASE statement needs to distinguish a
`NULL` value, then the alternate [CASE][case] syntax should be used.

`expr` and `expr_to_match` can be any type. They must be implicitly
coercible to a common [supertype][cond-exp-supertype]; equality comparisons are
done on coerced values. There may be multiple `result` types. `result` and
`else_result` expressions must be coercible to a common supertype.

This expression supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return Data Type**

[Supertype][cond-exp-supertype] of `result`[, ...] and `else_result`.

**Example**

```sql
WITH Numbers AS (
  SELECT 90 as A, 2 as B UNION ALL
  SELECT 50, 8 UNION ALL
  SELECT 60, 6 UNION ALL
  SELECT 50, 10
)
SELECT
  A,
  B,
  CASE A
    WHEN 90 THEN 'red'
    WHEN 50 THEN 'blue'
    ELSE 'green'
    END
    AS result
FROM Numbers

/*------------------+
 | A  | B  | result |
 +------------------+
 | 90 | 2  | red    |
 | 50 | 8  | blue   |
 | 60 | 6  | green  |
 | 50 | 10 | blue   |
 +------------------*/
```

[logical-operators]: ../syntax/operators.md#logical-operators
[case]: #case
[cond-exp-supertype]: ../types/conversion_rules.md#supertypes

### `CASE`

```sql
CASE
  WHEN condition THEN result
  [ ... ]
  [ ELSE else_result ]
  END
```

**Description**

Evaluates the condition of each successive `WHEN` clause and returns the
first result where the condition evaluates to `TRUE`; any remaining `WHEN`
clauses and `else_result` aren't evaluated.

If all conditions evaluate to `FALSE` or `NULL`, returns evaluation of
`else_result` if present; if `else_result` isn't present, then returns `NULL`.

For additional rules on how values are evaluated, see the
three-valued logic table in [Logical operators][logical-operators].

`condition` must be a boolean expression. There may be multiple `result` types.
`result` and `else_result` expressions must be implicitly coercible to a common
[supertype][cond-exp-supertype].

This expression supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return Data Type**

[Supertype][cond-exp-supertype] of `result`[, ...] and `else_result`.

**Example**

```sql
WITH Numbers AS (
  SELECT 90 as A, 2 as B UNION ALL
  SELECT 50, 6 UNION ALL
  SELECT 20, 10
)
SELECT
  A,
  B,
  CASE
    WHEN A > 60 THEN 'red'
    WHEN B = 6 THEN 'blue'
    ELSE 'green'
    END
    AS result
FROM Numbers

/*------------------+
 | A  | B  | result |
 +------------------+
 | 90 | 2  | red    |
 | 50 | 6  | blue   |
 | 20 | 10 | green  |
 +------------------*/
```

[cond-exp-supertype]: ../types/conversion_rules.md#supertypes
[logical-operators]: ../syntax/operators.md#logical-operators

### `COALESCE`

```sql
COALESCE(expr[, ...])
```

**Description**

Returns the value of the first non-`NULL` expression, if any, otherwise
`NULL`. The remaining expressions aren't evaluated. An input expression can be
any type. There may be multiple input expression types.
All input expressions must be implicitly coercible to a common
[supertype][cond-exp-supertype].

**Return Data Type**

[Supertype][cond-exp-supertype] of `expr`[, ...].

**Examples**

```sql
SELECT COALESCE('A', 'B', 'C') as result

/*--------+
 | result |
 +--------+
 | A      |
 +--------*/
```

```sql
SELECT COALESCE(NULL, 'B', 'C') as result

/*--------+
 | result |
 +--------+
 | B      |
 +--------*/
```

[cond-exp-supertype]: ../types/conversion_rules.md#supertypes

### `IF`

```sql
IF(expr, true_result, else_result)
```

**Description**

If `expr` evaluates to `TRUE`, returns `true_result`, else returns the
evaluation for `else_result`. `else_result` isn't evaluated if `expr` evaluates
to `TRUE`. `true_result` isn't evaluated if `expr` evaluates to `FALSE` or
`NULL`.

`expr` must be a boolean expression. `true_result` and `else_result`
must be coercible to a common [supertype][cond-exp-supertype].

**Return Data Type**

[Supertype][cond-exp-supertype] of `true_result` and `else_result`.

**Examples**

```sql
SELECT
  10 AS A,
  20 AS B,
  IF(10 < 20, 'true', 'false') AS result

/*------------------+
 | A  | B  | result |
 +------------------+
 | 10 | 20 | true   |
 +------------------*/
```

```sql
SELECT
  30 AS A,
  20 AS B,
  IF(30 < 20, 'true', 'false') AS result

/*------------------+
 | A  | B  | result |
 +------------------+
 | 30 | 20 | false  |
 +------------------*/
```

[cond-exp-supertype]: ../types/conversion_rules.md#supertypes

### `IFNULL`

```sql
IFNULL(expr, null_result)
```

**Description**

If `expr` evaluates to `NULL`, returns `null_result`. Otherwise, returns
`expr`. If `expr` doesn't evaluate to `NULL`, `null_result` isn't evaluated.

`expr` and `null_result` can be any type and must be implicitly coercible to
a common [supertype][cond-exp-supertype]. Synonym for
`COALESCE(expr, null_result)`.

**Return Data Type**

[Supertype][cond-exp-supertype] of `expr` or `null_result`.

**Examples**

```sql
SELECT IFNULL(NULL, 0) as result

/*--------+
 | result |
 +--------+
 | 0      |
 +--------*/
```

```sql
SELECT IFNULL(10, 0) as result

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

[cond-exp-supertype]: ../types/conversion_rules.md#supertypes

### `NULLIF`

```sql
NULLIF(expr, expr_to_match)
```

**Description**

Returns `NULL` if `expr = expr_to_match` evaluates to `TRUE`, otherwise
returns `expr`.

`expr` and `expr_to_match` must be implicitly coercible to a
common [supertype][cond-exp-supertype], and must be comparable.

This expression supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return Data Type**

[Supertype][cond-exp-supertype] of `expr` and `expr_to_match`.

**Example**

```sql
SELECT NULLIF(0, 0) as result

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT NULLIF(10, 0) as result

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

[cond-exp-supertype]: ../types/conversion_rules.md#supertypes

### `NULLIFZERO`

```sql
NULLIFZERO(expr)
```

**Description**

Returns `NULL` if the value of `expr` is `0`. Otherwise, returns `expr`. `expr`
must be a zeroable type.

**Return Data Type**

Type of `expr`.

**Example**

```sql
SELECT NULLIFZERO(0) AS result

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

### `ZEROIFNULL`

```sql
ZEROIFNULL(expr)
```

**Description**

Returns `0` if the value of `expr` is `NULL`. Otherwise, returns `expr`. `expr`
must be a zeroable type.

**Return Data Type**

Type of `expr`.

**Example**

```sql
SELECT ZEROIFNULL(NULL) AS result

/*--------+
 | result |
 +--------+
 | 0      |
 +--------*/
```
