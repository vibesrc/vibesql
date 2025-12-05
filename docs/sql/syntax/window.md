# Window

## `WINDOW` clause

```sql
WINDOW named_window_expression [, ...]

named_window_expression:
  named_window AS { named_window | ( [ window_specification ] ) }
```

A `WINDOW` clause defines a list of named windows.
A named window represents a group of rows in a table upon which to use a
[window function][window-function-calls]. A named window can be defined with
a [window specification][query-window-specification] or reference another
named window. If another named window is referenced, the definition of the
referenced window must precede the referencing window.

**Examples**

These examples reference a table called [`Produce`][produce-table].
They all return the same [result][named-window-example]. Note the different
ways you can combine named windows and use them in a window function's
`OVER` clause.

```sql
SELECT item, purchases, category, LAST_VALUE(item)
  OVER (item_window) AS most_popular
FROM Produce
WINDOW item_window AS (
  PARTITION BY category
  ORDER BY purchases
  ROWS BETWEEN 2 PRECEDING AND 2 FOLLOWING)
```

```sql
SELECT item, purchases, category, LAST_VALUE(item)
  OVER (d) AS most_popular
FROM Produce
WINDOW
  a AS (PARTITION BY category),
  b AS (a ORDER BY purchases),
  c AS (b ROWS BETWEEN 2 PRECEDING AND 2 FOLLOWING),
  d AS (c)
```

```sql
SELECT item, purchases, category, LAST_VALUE(item)
  OVER (c ROWS BETWEEN 2 PRECEDING AND 2 FOLLOWING) AS most_popular
FROM Produce
WINDOW
  a AS (PARTITION BY category),
  b AS (a ORDER BY purchases),
  c AS b
```
