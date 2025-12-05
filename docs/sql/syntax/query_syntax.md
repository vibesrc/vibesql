# Query Syntax

Query statements scan one or more tables or expressions and return the computed result rows.

## SQL Syntax Overview

```sql
query_statement:
  query_expr

query_expr:
  [ WITH [ RECURSIVE ] { non_recursive_cte | recursive_cte }[, ...] ]
  { select | ( query_expr ) | set_operation }
  [ ORDER BY expression [{ ASC | DESC }] [, ...] ]
  [ LIMIT count [ OFFSET skip_rows ] ]

select:
  SELECT
    [ { ALL | DISTINCT } ]
    [ AS { typename | STRUCT | VALUE } ]
    select_list
  [ FROM from_clause[, ...] ]
  [ WHERE bool_expression ]
  [ GROUP BY group_by_specification ]
  [ HAVING bool_expression ]
  [ QUALIFY bool_expression ]
  [ WINDOW window_clause ]
```

## Query Components

| Topic                               | Description                                      |
| ----------------------------------- | ------------------------------------------------ |
| [SELECT](select.md)                 | SELECT statement, projections, aliases, DISTINCT |
| [FROM](from.md)                     | FROM clause, UNNEST, PIVOT, UNPIVOT, TABLESAMPLE |
| [Joins](joins.md)                   | INNER, LEFT, RIGHT, FULL, CROSS joins            |
| [Clauses](clauses.md)               | WHERE, GROUP BY, HAVING, ORDER BY, QUALIFY       |
| [Window](window.md)                 | WINDOW clause for named window specifications    |
| [Set Operations](set_operations.md) | UNION, INTERSECT, EXCEPT, LIMIT, OFFSET          |
| [WITH (CTEs)](with.md)              | Common Table Expressions                         |

## Syntax Notation

| Notation | Description                |
| -------- | -------------------------- | --------------- |
| `[ ]`    | Optional clause            |
| `{ }`    | Choose one option from set |
| `        | `                          | OR - choose one |
| `...`    | Preceding item can repeat  |
| `, ...`  | Comma-separated list       |

## See Also

- [Operators](operators.md) - Operator precedence and behavior
- [Subqueries](subqueries.md) - Scalar, array, and table subqueries
- [Recursive CTEs](recursive_ctes.md) - WITH RECURSIVE
