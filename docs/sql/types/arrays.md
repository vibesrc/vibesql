# Working with Arrays

An array is an ordered list of zero or more values of the same data type. Arrays can contain simple types like `INTEGER` or complex types like `STRUCT`. Nested arrays (arrays of arrays) are not directly supported—use `STRUCT` wrappers instead.

## Constructing Arrays

### Array Literals

```sql
-- Typed array literal
ARRAY[1, 2, 3]
ARRAY['a', 'b', 'c']

-- Empty array
ARRAY[]::INTEGER[]

-- With explicit type
ARRAY['hello', 'world']
```

### From Subqueries

```sql
SELECT ARRAY(SELECT id FROM users WHERE active)
```

### Using ARRAY_AGG

```sql
SELECT department, ARRAY_AGG(name) AS employees
FROM staff
GROUP BY department
```

## Accessing Elements

Arrays support both 0-based and 1-based indexing:

| Syntax            | Description                      |
| ----------------- | -------------------------------- |
| `arr[0]`          | Zero-based index (first element) |
| `arr[OFFSET(n)]`  | Zero-based, explicit             |
| `arr[ORDINAL(n)]` | One-based (SQL standard style)   |

```sql
WITH data AS (
  SELECT ARRAY[10, 20, 30, 40] AS numbers
)
SELECT
  numbers[0] AS first_zero,      -- 10
  numbers[OFFSET(1)] AS second,  -- 20
  numbers[ORDINAL(1)] AS first   -- 10
FROM data
```

### Out-of-Bounds Access

Accessing an index outside the array bounds raises an error. Use bounds checking or `COALESCE` for safety:

```sql
SELECT COALESCE(numbers[10], 0) AS safe_access
FROM data
```

## Array Length

```sql
SELECT ARRAY_LENGTH(ARRAY[1, 2, 3, 4, 5]) AS len  -- 5
```

## UNNEST: Converting Arrays to Rows

The `UNNEST` operator expands an array into a set of rows:

```sql
SELECT *
FROM UNNEST(ARRAY['foo', 'bar', 'baz']) AS element

/*---------+
 | element |
 +---------+
 | foo     |
 | bar     |
 | baz     |
 +---------*/
```

### Preserving Order with OFFSET

```sql
SELECT element, offset
FROM UNNEST(ARRAY['a', 'b', 'c']) WITH OFFSET AS offset
ORDER BY offset

/*---------+--------+
 | element | offset |
 +---------+--------+
 | a       | 0      |
 | b       | 1      |
 | c       | 2      |
 +---------+--------*/
```

### Correlated UNNEST

To unnest an array column while preserving other columns:

```sql
WITH sequences AS (
  SELECT 1 AS id, ARRAY[10, 20, 30] AS numbers
  UNION ALL
  SELECT 2 AS id, ARRAY[5, 15] AS numbers
)
SELECT id, num
FROM sequences
CROSS JOIN UNNEST(numbers) AS num

/*----+-----+
 | id | num |
 +----+-----+
 |  1 |  10 |
 |  1 |  20 |
 |  1 |  30 |
 |  2 |   5 |
 |  2 |  15 |
 +----+-----*/
```

## Arrays of STRUCTs

```sql
WITH races AS (
  SELECT ARRAY[
    STRUCT('Alice' AS name, 102.5 AS time),
    STRUCT('Bob' AS name, 98.3 AS time),
    STRUCT('Carol' AS name, 105.1 AS time)
  ] AS runners
)
SELECT runner.name, runner.time
FROM races
CROSS JOIN UNNEST(runners) AS runner
ORDER BY runner.time

/*-------+-------+
 | name  | time  |
 +-------+-------+
 | Bob   | 98.3  |
 | Alice | 102.5 |
 | Carol | 105.1 |
 +-------+-------*/
```

## Filtering Arrays

### Filter Elements

```sql
WITH data AS (
  SELECT ARRAY[1, 2, 3, 4, 5, 6] AS numbers
)
SELECT ARRAY(
  SELECT x FROM UNNEST(numbers) AS x WHERE x > 3
) AS filtered
FROM data

/*-----------+
 | filtered  |
 +-----------+
 | [4, 5, 6] |
 +-----------*/
```

### Distinct Elements

```sql
SELECT ARRAY(SELECT DISTINCT x FROM UNNEST(ARRAY[1, 1, 2, 2, 3]) AS x)
-- [1, 2, 3]
```

## Searching Arrays

### Check for Value

```sql
SELECT 2 IN UNNEST(ARRAY[1, 2, 3]) AS contains_two  -- true
```

### Filter Rows by Array Contents

```sql
WITH data AS (
  SELECT 1 AS id, ARRAY[1, 2, 3] AS nums
  UNION ALL
  SELECT 2 AS id, ARRAY[4, 5, 6] AS nums
)
SELECT id
FROM data
WHERE 2 IN UNNEST(nums)  -- Returns id=1
```

### Check for Condition

```sql
SELECT id
FROM data
WHERE EXISTS(SELECT 1 FROM UNNEST(nums) AS x WHERE x > 5)
```

## Aggregating into Arrays

### ARRAY_AGG

```sql
SELECT ARRAY_AGG(name ORDER BY name) AS sorted_names
FROM employees

/*-----------------------+
 | sorted_names          |
 +-----------------------+
 | [Alice, Bob, Carol]   |
 +-----------------------*/
```

### ARRAY_CONCAT_AGG

Concatenates arrays across rows:

```sql
WITH data AS (
  SELECT ARRAY[1, 2] AS nums
  UNION ALL
  SELECT ARRAY[3, 4] AS nums
)
SELECT ARRAY_CONCAT_AGG(nums) AS all_nums
FROM data

/*--------------+
 | all_nums     |
 +--------------+
 | [1, 2, 3, 4] |
 +--------------*/
```

## Array Functions

### ARRAY_CONCAT

```sql
SELECT ARRAY_CONCAT(ARRAY[1, 2], ARRAY[3, 4], ARRAY[5])
-- [1, 2, 3, 4, 5]
```

### ARRAY_TO_STRING

```sql
SELECT ARRAY_TO_STRING(ARRAY['Hello', 'World'], ' ')
-- 'Hello World'

-- With NULL handling
SELECT ARRAY_TO_STRING(ARRAY['a', NULL, 'b'], '-', 'X')
-- 'a-X-b'
```

### ARRAY_LENGTH

```sql
SELECT ARRAY_LENGTH(ARRAY[1, 2, 3])  -- 3
SELECT ARRAY_LENGTH(ARRAY[])        -- 0
SELECT ARRAY_LENGTH(NULL)           -- NULL
```

## Casting Arrays

```sql
SELECT CAST(ARRAY[1, 2, 3] AS ARRAY) AS doubles
-- [1.0, 2.0, 3.0]
```

Element types must be castable to the target element type.

## Nested Arrays via STRUCT

Since direct array-of-arrays isn't supported, wrap in STRUCT:

```sql
SELECT ARRAY[
  STRUCT(ARRAY[1, 2] AS point),
  STRUCT(ARRAY[3, 4] AS point),
  STRUCT(ARRAY[5, 6] AS point)
] AS points

/*---------------------------+
 | points                    |
 +---------------------------+
 | [{[1,2]}, {[3,4]}, {[5,6]}]|
 +---------------------------*/
```

## See Also

- [Array Functions](../functions/array_functions.md)
- [Data Types](data_types.md#array-type)
- [Query Syntax - UNNEST](../syntax/query_syntax.md#unnest-operator)
