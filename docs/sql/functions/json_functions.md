# JSON Functions

Functions for creating, querying, and manipulating JSON data.

## Categories

| Category                       | Description                                                       |
| ------------------------------ | ----------------------------------------------------------------- |
| [Core](json_core.md)           | Create and inspect JSON: `JSON_ARRAY`, `JSON_OBJECT`, `JSON_TYPE` |
| [Extraction](json_extract.md)  | Extract data: `JSON_QUERY`, `JSON_VALUE`, `JSON_EXTRACT`          |
| [Modification](json_modify.md) | Modify JSON: `JSON_SET`, `JSON_REMOVE`, `JSON_ARRAY_APPEND`       |
| [Conversion](json_convert.md)  | Type conversion: `TO_JSON`, `PARSE_JSON`, type extractors         |

## Quick Reference

### Creating JSON

```sql
JSON_ARRAY(1, 2, 3)                    -- [1, 2, 3]
JSON_OBJECT('a', 1, 'b', 2)            -- {"a": 1, "b": 2}
```

### Extracting Data

```sql
JSON_VALUE(json_col, '$.name')         -- Extract scalar as VARCHAR
JSON_QUERY(json_col, '$.items')        -- Extract JSON subtree
```

### Modifying JSON

```sql
JSON_SET(json_col, '$.name', 'Alice')  -- Set/update value
JSON_REMOVE(json_col, '$.temp')        -- Remove key
```

### Converting Types

```sql
TO_JSON(any_value)                     -- Convert SQL to JSON
PARSE_JSON('{"a": 1}')                 -- Parse string to JSON
```

## See Also

- [Data Types - JSON](../types/data_types.md#json-type)
- [Operators - JSON](../syntax/operators.md)
