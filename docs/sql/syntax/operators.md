# Operators

Operators are special characters or keywords that manipulate operands and return results.

## Conventions

- All operators return `NULL` when any operand is `NULL`
- Overflow causes an error
- `+/-inf` and `NaN` only returned if an operand is `+/-inf` or `NaN`

## Operator Categories

| Category                                     | Description                                            |
| -------------------------------------------- | ------------------------------------------------------ |
| [Subscript & Access](operators_subscript.md) | Array `[]`, struct `.field`, JSON access               |
| [Arithmetic](operators_arithmetic.md)        | `+`, `-`, `*`, `/`, date/interval math                 |
| [Logical & Bitwise](operators_logical.md)    | `AND`, `OR`, `NOT`, `&`, `\|`, `^`, `~`                |
| [Comparison](operators_comparison.md)        | `=`, `<>`, `<`, `>`, `IN`, `LIKE`, `IS`, `BETWEEN`     |

## Operator Precedence

| Precedence  | Operators                                             |
| ----------- | ----------------------------------------------------- |
| 1 (highest) | `.` `[]` (field/subscript access)                     |
| 2           | `+` `-` `~` (unary)                                   |
| 3           | `*` `/` `\|`                                          |
| 4           | `+` `-` (binary)                                      |
| 5           | `<<` `>>`                                             |
| 6           | `&`                                                   |
| 7           | `^`                                                   |
| 8           | `\|`                                                  |
| 9           | `=` `<>` `<` `>` `<=` `>=` `LIKE` `BETWEEN` `IN` `IS` |
| 10          | `NOT`                                                 |
| 11          | `AND`                                                 |
| 12 (lowest) | `OR`                                                  |

Operators with the same precedence are left-associative:

```sql
x AND y AND z  -- Evaluated as: ((x AND y) AND z)
x * y / z      -- Evaluated as: ((x * y) / z)
```

## Quick Reference

### Arithmetic

```sql
5 + 3          -- 8
10 / 3         -- 3 (integer division)
10.0 / 3       -- 3.333...
-x             -- Negation
```

### Comparison

```sql
a = b          -- Equality
a <> b         -- Inequality (also !=)
a < b          -- Less than
x BETWEEN 1 AND 10
x IN (1, 2, 3)
x IS NULL
x IS NOT NULL
```

### Logical

```sql
a AND b
a OR b
NOT a
```

### String/Array

```sql
'Hello' || ' World'    -- Concatenation
ARRAY[1,2] || ARRAY[3] -- [1, 2, 3]
```

## See Also

- [Data Types](../types/data_types.md) - Type comparison rules
- [Conditional Expressions](../functions/conditional_expressions.md) - CASE, COALESCE, IF
