# String Functions

Functions for manipulating `VARCHAR` and `VARBINARY` values.

## Categories

| Category                                | Description                                           |
| --------------------------------------- | ----------------------------------------------------- |
| [Manipulation](string_manipulation.md)  | `CONCAT`, `SUBSTR`, `REPLACE`, `TRIM`, `SPLIT`        |
| [Search](string_search.md)              | `INSTR`, `STRPOS`, `STARTS_WITH`, `ENDS_WITH`         |
| [Regular Expressions](string_regexp.md) | `REGEXP_CONTAINS`, `REGEXP_EXTRACT`, `REGEXP_REPLACE` |
| [Formatting](string_format.md)          | `FORMAT`, `TO_BASE64`, `FROM_HEX`, `CODE_POINTS`      |
| [Case & Collation](string_case.md)      | `UPPER`, `LOWER`, `COLLATE`, `NORMALIZE`              |

## Quick Reference

### Building Strings

```sql
CONCAT('Hello', ' ', 'World')          -- 'Hello World'
REPEAT('ab', 3)                        -- 'ababab'
LPAD('42', 5, '0')                     -- '00042'
```

### Extracting Parts

```sql
SUBSTR('Hello', 1, 3)                  -- 'Hel'
LEFT('Hello', 2)                       -- 'He'
SPLIT('a,b,c', ',')                    -- ['a', 'b', 'c']
```

### Searching

```sql
STRPOS('Hello', 'l')                   -- 3
STARTS_WITH('Hello', 'He')             -- TRUE
REGEXP_CONTAINS('abc123', r'\d+')      -- TRUE
```

### Case Conversion

```sql
UPPER('hello')                         -- 'HELLO'
LOWER('HELLO')                         -- 'hello'
INITCAP('hello world')                 -- 'Hello World'
```

## See Also

- [Data Types - VARCHAR](../types/data_types.md#character-types)
- [Operators - LIKE](../syntax/operators.md)
