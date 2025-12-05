# With

## `WITH` clause

```sql
WITH [ RECURSIVE ] { non_recursive_cte | recursive_cte }[, ...]
```

A `WITH` clause contains one or more common table expressions (CTEs).
A CTE acts like a temporary table that you can reference within a single
query expression. Each CTE binds the results of a [subquery][subquery-concepts]
to a table name, which can be used elsewhere in the same query expression,
but [rules apply][cte-rules].

CTEs can be [non-recursive][non-recursive-cte] or
[recursive][recursive-cte] and you can include both of these in your
`WITH` clause. A recursive CTE references itself, where a
non-recursive CTE doesn't. If a recursive CTE is included in the `WITH` clause,
the `RECURSIVE` keyword must also be included.

You can include the `RECURSIVE` keyword in a `WITH` clause even if no
recursive CTEs are present. You can learn more about the `RECURSIVE` keyword
[here][recursive-keyword].

### `RECURSIVE` keyword

A `WITH` clause can optionally include the `RECURSIVE` keyword, which does
two things:

- Enables recursion in the `WITH` clause. If this keyword isn't present,
  you can only include non-recursive common table expressions (CTEs).
  If this keyword is present, you can use both [recursive][recursive-cte] and
  [non-recursive][non-recursive-cte] CTEs.
- [Changes the visibility][cte-visibility] of CTEs in the `WITH` clause. If this
  keyword isn't present, a CTE is only visible to CTEs defined after it in the
  `WITH` clause. If this keyword is present, a CTE is visible to all CTEs in the
  `WITH` clause where it was defined.

### Non-recursive CTEs

```sql
non_recursive_cte:
  cte_name AS ( query_expr )
```

A non-recursive common table expression (CTE) contains
a non-recursive [subquery][subquery-concepts]
and a name associated with the CTE.

- A non-recursive CTE can't reference itself.
- A non-recursive CTE can be referenced by the query expression that
  contains the `WITH` clause, but [rules apply][cte-rules].

##### Examples

In this example, a `WITH` clause defines two non-recursive CTEs that
are referenced in the related set operation, where one CTE is referenced by
each of the set operation's input query expressions:

```sql
WITH subQ1 AS (SELECT SchoolID FROM Roster),
     subQ2 AS (SELECT OpponentID FROM PlayerStats)
SELECT * FROM subQ1
UNION ALL
SELECT * FROM subQ2
```

You can break up more complex queries into a `WITH` clause and
`WITH` `SELECT` statement instead of writing nested table subqueries.
For example:

```sql
WITH q1 AS (my_query)
SELECT *
FROM
  (WITH q2 AS (SELECT * FROM q1) SELECT * FROM q2)
```

```sql
WITH q1 AS (my_query)
SELECT *
FROM
  (WITH q2 AS (SELECT * FROM q1),  # q1 resolves to my_query
        q3 AS (SELECT * FROM q1),  # q1 resolves to my_query
        q1 AS (SELECT * FROM q1),  # q1 (in the query) resolves to my_query
        q4 AS (SELECT * FROM q1)   # q1 resolves to the WITH subquery on the previous line.
    SELECT * FROM q1)              # q1 resolves to the third inner WITH subquery.
```

### Recursive CTEs

```sql
recursive_cte:
  cte_name AS ( recursive_union_operation )

recursive_union_operation:
  base_term union_operator recursive_term

base_term:
  query_expr

recursive_term:
  query_expr

union_operator:
  { UNION ALL | UNION DISTINCT }
```

A recursive common table expression (CTE) contains a
recursive [subquery][subquery-concepts] and a name associated with the CTE.

- A recursive CTE references itself.
- A recursive CTE can be referenced in the query expression that contains the
  `WITH` clause, but [rules apply][cte-rules].
- When a recursive CTE is defined in a `WITH` clause, the
  [`RECURSIVE`][recursive-keyword] keyword must be present.

A recursive CTE is defined by a _recursive union operation_. The
recursive union operation defines how input is recursively processed
to produce the final CTE result. The recursive union operation has the
following parts:

- `base_term`: Runs the first iteration of the
  recursive union operation. This term must follow the
  [base term rules][base-term-rules].
- `union_operator`: The `UNION` operator returns the rows that are from
  the union of the base term and recursive term. With `UNION ALL`,
  each row produced in iteration `N` becomes part of the final CTE result and
  input for iteration `N+1`. With
  `UNION DISTINCT`, only distinct rows become part of the final CTE result,
  and only new distinct rows move into iteration `N+1`. Iteration
  stops when an iteration produces no rows to move into the next iteration.
- `recursive_term`: Runs the remaining iterations.
  It must include one self-reference (recursive reference) to the recursive CTE.
  Only this term can include a self-reference. This term
  must follow the [recursive term rules][recursive-cte-rules].

A recursive CTE looks like this:

```sql
WITH RECURSIVE
  T1 AS ( (SELECT 1 AS n) UNION ALL (SELECT n + 1 AS n FROM T1 WHERE n < 3) )
SELECT n FROM T1

/*---+
 | n |
 +---+
 | 2 |
 | 1 |
 | 3 |
 +---*/
```

The first iteration of a recursive union operation runs the base term.
Then, each subsequent iteration runs the recursive term and produces
_new rows_ which are unioned with the previous iteration. The recursive
union operation terminates when a recursive term iteration produces no new
rows.

If recursion doesn't terminate, the query will not terminate.

To avoid a non-terminating iteration in a recursive union operation, you can
use the `LIMIT` clause in a query.

A recursive CTE can include nested `WITH` clauses, however, you can't reference
`recursive_term` inside of an inner `WITH` clause. An inner `WITH`
clause can't be recursive unless it includes its own `RECURSIVE` keyword.
The `RECURSIVE` keyword affects only the particular `WITH` clause to which it
belongs.

To learn more about recursive CTEs and troubleshooting iteration limit errors,
see [Work with recursive CTEs][work-with-recursive-ctes].

##### Examples of allowed recursive CTEs

This is a simple recursive CTE:

```sql
WITH RECURSIVE
  T1 AS (
    (SELECT 1 AS n) UNION ALL
    (SELECT n + 2 FROM T1 WHERE n < 4))
SELECT * FROM T1 ORDER BY n

/*---+
 | n |
 +---+
 | 1 |
 | 3 |
 | 5 |
 +---*/
```

Multiple subqueries in the same recursive CTE are okay, as
long as each recursion has a cycle length of 1. It's also okay for recursive
entries to depend on non-recursive entries and vice-versa:

```sql
WITH RECURSIVE
  T0 AS (SELECT 1 AS n),
  T1 AS ((SELECT * FROM T0) UNION ALL (SELECT n + 1 FROM T1 WHERE n < 4)),
  T2 AS ((SELECT 1 AS n) UNION ALL (SELECT n + 1 FROM T2 WHERE n < 4)),
  T3 AS (SELECT * FROM T1 INNER JOIN T2 USING (n))
SELECT * FROM T3 ORDER BY n

/*---+
 | n |
 +---+
 | 1 |
 | 2 |
 | 3 |
 | 4 |
 +---*/
```

Aggregate functions can be invoked in subqueries, as long as they aren't
aggregating on the table being defined:

```sql
WITH RECURSIVE
  T0 AS (SELECT * FROM UNNEST ([60, 20, 30])),
  T1 AS ((SELECT 1 AS n) UNION ALL (SELECT n + (SELECT COUNT(*) FROM T0) FROM T1 WHERE n < 4))
SELECT * FROM T1 ORDER BY n

/*---+
 | n |
 +---+
 | 1 |
 | 4 |
 +---*/
```

`INNER JOIN` can be used inside subqueries:

```sql
WITH RECURSIVE
  T0 AS (SELECT 1 AS n),
  T1 AS ((SELECT 1 AS n) UNION ALL (SELECT n + 1 FROM T1 INNER JOIN T0 USING (n)))
SELECT * FROM T1 ORDER BY n

/*---+
 | n |
 +---+
 | 1 |
 | 2 |
 +---*/
```

`CROSS JOIN` can be used inside subqueries:

```sql
WITH RECURSIVE
  T0 AS (SELECT 2 AS p),
  T1 AS ((SELECT 1 AS n) UNION ALL (SELECT T1.n + T0.p FROM T1 CROSS JOIN T0 WHERE T1.n < 4))
SELECT * FROM T1 CROSS JOIN T0 ORDER BY n

/*---+---+
 | n | p |
 +---+---+
 | 1 | 2 |
 | 3 | 2 |
 | 5 | 2 |
 +---+---*/
```

In the following query, if `UNION DISTINCT` was replaced with `UNION ALL`,
this query would never terminate; it would keep generating rows
`0, 1, 2, 3, 4, 0, 1, 2, 3, 4...`. With `UNION DISTINCT`, however, the only row
produced by iteration `5` is a duplicate, so the query terminates.

```sql
WITH RECURSIVE
  T1 AS ( (SELECT 0 AS n) UNION DISTINCT (SELECT MOD(n + 1, 5) FROM T1) )
SELECT * FROM T1 ORDER BY n

/*---+
 | n |
 +---+
 | 0 |
 | 1 |
 | 2 |
 | 3 |
 | 4 |
 +---*/
```

##### Examples of disallowed recursive CTEs

The following recursive CTE is disallowed because the
self-reference doesn't include a set operator, base term, and
recursive term.

```sql {.bad}
WITH RECURSIVE
  T1 AS (SELECT * FROM T1)
SELECT * FROM T1

-- Error
```

The following recursive CTE is disallowed because the self-reference to `T1`
is in the base term. The self reference is only allowed in the recursive term.

```sql {.bad}
WITH RECURSIVE
  T1 AS ((SELECT * FROM T1) UNION ALL (SELECT 1))
SELECT * FROM T1

-- Error
```

The following recursive CTE is disallowed because there are multiple
self-references in the recursive term when there must only be one.

```sql {.bad}
WITH RECURSIVE
  T1 AS ((SELECT 1 AS n) UNION ALL ((SELECT * FROM T1) UNION ALL (SELECT * FROM T1)))
SELECT * FROM T1

-- Error
```

The following recursive CTE is disallowed because the self-reference is
inside an [expression subquery][expression-subquery-concepts]

```sql {.bad}
WITH RECURSIVE
  T1 AS ((SELECT 1 AS n) UNION ALL (SELECT (SELECT n FROM T1)))
SELECT * FROM T1

-- Error
```

The following recursive CTE is disallowed because there is a
self-reference as an argument to a table-valued function (TVF).

```sql {.bad}
WITH RECURSIVE
  T1 AS (
    (SELECT 1 AS n) UNION ALL
    (SELECT * FROM MY_TVF(T1)))
SELECT * FROM T1;

-- Error
```

The following recursive CTE is disallowed because there is a
self-reference as input to an outer join.

```sql {.bad}
WITH RECURSIVE
  T0 AS (SELECT 1 AS n),
  T1 AS ((SELECT 1 AS n) UNION ALL (SELECT * FROM T1 FULL OUTER JOIN T0 USING (n)))
SELECT * FROM T1;

-- Error
```

The following recursive CTE is disallowed because you can't use aggregation
with a self-reference.

```sql {.bad}
WITH RECURSIVE
  T1 AS (
    (SELECT 1 AS n) UNION ALL
    (SELECT COUNT(*) FROM T1))
SELECT * FROM T1;

-- Error
```

The following recursive CTE is disallowed because you can't use the
window function `OVER` clause with a self-reference.

```sql {.bad}
WITH RECURSIVE
  T1 AS (
    (SELECT 1.0 AS n) UNION ALL
    SELECT 1 + AVG(n) OVER(ROWS between 2 PRECEDING and 0 FOLLOWING) FROM T1 WHERE n < 10)
SELECT n FROM T1;

-- Error
```

The following recursive CTE is disallowed because you can't use a
`LIMIT` clause with a self-reference.

```sql {.bad}
WITH RECURSIVE
  T1 AS ((SELECT 1 AS n) UNION ALL (SELECT n FROM T1 LIMIT 3))
SELECT * FROM T1;

-- Error
```

The following recursive CTEs are disallowed because you can't use an
`ORDER BY` clause with a self-reference.

```sql {.bad}
WITH RECURSIVE
  T1 AS ((SELECT 1 AS n) UNION ALL (SELECT n + 1 FROM T1 ORDER BY n))
SELECT * FROM T1;

-- Error
```

The following recursive CTE is disallowed because table `T1` can't be
recursively referenced from inside an inner `WITH` clause

```sql {.bad}
WITH RECURSIVE
  T1 AS ((SELECT 1 AS n) UNION ALL (WITH t AS (SELECT n FROM T1) SELECT * FROM t))
SELECT * FROM T1

-- Error
```

### CTE rules and constraints

Common table expressions (CTEs) can be referenced inside the query expression
that contains the `WITH` clause.

##### General rules

Here are some general rules and constraints to consider when working with CTEs:

- Each CTE in the same `WITH` clause must have a unique name.
- You must include the [`RECURSIVE` keyword][recursive-keyword] keyword if the
  `WITH` clause contains a recursive CTE.
- The [`RECURSIVE` keyword][recursive-keyword] in
  the `WITH` clause changes the visibility of CTEs to other CTEs in the
  same `WITH` clause. You can learn more [here][recursive-keyword].
- A local CTE overrides an outer CTE or table with the same name.
- A CTE on a subquery may not reference correlated columns from the outer query.

##### Base term rules

The following rules apply to the base term in a recursive CTE:

- The base term is required to be non-recursive.
- The base term determines the names and types of all of the
  table columns.

##### Recursive term rules

The following rules apply to the recursive term in a recursive CTE:

- The recursive term must include exactly one reference to the
  recursively-defined table in the base term.
- The recursive term must contain the same number of columns as the
  base term, and the type of each column must be implicitly coercible to
  the type of the corresponding column in the base term.
- A recursive table reference can't be used as an operand to a `FULL JOIN`,
  a right operand to a `LEFT JOIN`, or a left operand to a `RIGHT JOIN`.
- A recursive table reference can't be used with the `TABLESAMPLE` operator.
- A recursive table reference can't be used as an operand to a
  table-valued function (TVF).

The following rules apply to a subquery inside a recursive term:

- A subquery with a recursive table reference must be a `SELECT` expression,
  not a set operation, such as `UNION ALL`.
- A subquery can't contain, directly or indirectly, a
  recursive table reference anywhere outside of its `FROM` clause.
- A subquery with a recursive table reference can't contain an `ORDER BY` or
  `LIMIT` clause.
- A subquery with a recursive table reference can't invoke aggregate functions.
- A subquery with a recursive table reference can't invoke window functions.
- A subquery with a recursive table reference can't contain the
  `DISTINCT` keyword or
  `GROUP BY` clause. To filter
  duplicates, use `UNION DISTINCT` in the top-level set operation,
  instead.

### CTE visibility

The visibility of a common table expression (CTE) within a query expression
is determined by whether or not you add the `RECURSIVE` keyword to the
`WITH` clause where the CTE was defined. You can learn more about these
differences in the following sections.

#### Visibility of CTEs in a `WITH` clause with the `RECURSIVE` keyword

When you include the `RECURSIVE` keyword, references between CTEs in the `WITH`
clause can go backwards and forwards. Cycles aren't allowed.

This is what happens when you have two CTEs that reference
themselves or each other in a `WITH` clause with the `RECURSIVE`
keyword. Assume that `A` is the first CTE and `B` is the second
CTE in the clause:

- A references A = Valid
- A references B = Valid
- B references A = Valid
- A references B references A = Invalid (cycles aren't allowed)

`A` can reference itself because self-references are supported:

```sql
WITH RECURSIVE
  A AS (SELECT 1 AS n UNION ALL (SELECT n + 1 FROM A WHERE n < 3))
SELECT * FROM A

/*---+
 | n |
 +---+
 | 1 |
 | 2 |
 | 3 |
 +---*/
```

`A` can reference `B` because references between CTEs can go forwards:

```sql
WITH RECURSIVE
  A AS (SELECT * FROM B),
  B AS (SELECT 1 AS n)
SELECT * FROM B

/*---+
 | n |
 +---+
 | 1 |
 +---*/
```

`B` can reference `A` because references between CTEs can go backwards:

```sql
WITH RECURSIVE
  A AS (SELECT 1 AS n),
  B AS (SELECT * FROM A)
SELECT * FROM B

/*---+
 | n |
 +---+
 | 1 |
 +---*/
```

This produces an error. `A` and `B` reference each other, which creates a cycle:

```sql
WITH RECURSIVE
  A AS (SELECT * FROM B),
  B AS (SELECT * FROM A)
SELECT * FROM B

-- Error
```

#### Visibility of CTEs in a `WITH` clause without the `RECURSIVE` keyword

When you don't include the `RECURSIVE` keyword in the `WITH` clause,
references between CTEs in the clause can go backward but not forward.

This is what happens when you have two CTEs that reference
themselves or each other in a `WITH` clause without
the `RECURSIVE` keyword. Assume that `A` is the first CTE and `B`
is the second CTE in the clause:

- A references A = Invalid
- A references B = Invalid
- B references A = Valid
- A references B references A = Invalid (cycles aren't allowed)

This produces an error. `A` can't reference itself because self-references
aren't supported:

```sql
WITH
  A AS (SELECT 1 AS n UNION ALL (SELECT n + 1 FROM A WHERE n < 3))
SELECT * FROM A

-- Error
```

This produces an error. `A` can't reference `B` because references between
CTEs can go backwards but not forwards:

```sql
WITH
  A AS (SELECT * FROM B),
  B AS (SELECT 1 AS n)
SELECT * FROM B

-- Error
```

`B` can reference `A` because references between CTEs can go backwards:

```sql
WITH
  A AS (SELECT 1 AS n),
  B AS (SELECT * FROM A)
SELECT * FROM B

/*---+
 | n |
 +---+
 | 1 |
 +---*/
```

This produces an error. `A` and `B` reference each other, which creates a
cycle:

```sql
WITH
  A AS (SELECT * FROM B),
  B AS (SELECT * FROM A)
SELECT * FROM B

-- Error
```
