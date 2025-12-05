# JSON Core Functions

Core functions for creating and inspecting JSON values.

## `JSON_ARRAY`

```sql
JSON_ARRAY([value][, ...])
```

**Description**

Creates a JSON array from zero or more SQL values.

Arguments:

- `value`: A [JSON encoding-supported][json-encodings] value to add
  to a JSON array.

**Return type**

`JSON`

**Examples**

The following query creates a JSON array with one value in it:

```sql
SELECT JSON_ARRAY(10) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | [10]      |
 +-----------*/
```

You can create a JSON array with an empty JSON array in it. For example:

```sql
SELECT JSON_ARRAY([]) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | [[]]      |
 +-----------*/
```

```sql
SELECT JSON_ARRAY(10, 'foo', NULL) AS json_data

/*-----------------+
 | json_data       |
 +-----------------+
 | [10,"foo",null] |
 +-----------------*/
```

```sql
SELECT JSON_ARRAY(STRUCT(10 AS a, 'foo' AS b)) AS json_data

/*----------------------+
 | json_data            |
 +----------------------+
 | [{"a":10,"b":"foo"}] |
 +----------------------*/
```

```sql
SELECT JSON_ARRAY(10, ['foo', 'bar'], [20, 30]) AS json_data

/*----------------------------+
 | json_data                  |
 +----------------------------+
 | [10,["foo","bar"],[20,30]] |
 +----------------------------*/
```

```sql
SELECT JSON_ARRAY(10, [JSON '20', JSON '"foo"']) AS json_data

/*-----------------+
 | json_data       |
 +-----------------+
 | [10,[20,"foo"]] |
 +-----------------*/
```

You can create an empty JSON array. For example:

```sql
SELECT JSON_ARRAY() AS json_data

/*-----------+
 | json_data |
 +-----------+
 | []        |
 +-----------*/
```

## `JSON_CONTAINS`

```sql
JSON_CONTAINS(json_expr, json_expr)
```

**Description**

Checks if a JSON document contains another JSON document. This function returns
`true` if the first parameter JSON document contains the second parameter JSON
document; otherwise the function returns `false`. If any input argument is
`NULL`, a `NULL` value is returned.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '{"class": {"students": [{"name": "Jane"}]}}'
  ```

Details:

- The structure and data of the contained document must match a portion of the
  containing document. This function determines if the smaller JSON document
  is part of the larger JSON document.
- JSON scalars: A JSON scalar value (like a string, number, bool, or JSON null
  ) contains only itself.
- JSON objects:
  - An object contains another object if the first object contains all the
    key-value pairs present in the second JSON object.
  - When checking for object containment, extra key-value pairs in the
    containing object don't prevent a match.
  - Any JSON object can contain an empty object.

- JSON arrays:
  - An array contains another array if every element of the second array is
    contained by some element of the first.
  - Duplicate elements in arrays are treated as if they appear only once.
  - The order of elements within JSON arrays isn't significant for
    containment checks.
  - Any array can contain an empty array.
  - As a special case, a top-level array can contain a scalar value.

**Return type**

`BOOL`

**Examples**

In the following example, a JSON scalar value (a string) contains only itself:

```sql
SELECT JSON_CONTAINS(JSON '"a"', JSON '"a"') AS result;

/*----------+
 |  result  |
 +----------+
 |   true   |
 +----------*/
```

The following examples check if a JSON object contains another JSON object:

```sql
SELECT
    JSON_CONTAINS(JSON '{"a": {"b": 1}, "c": 2}', JSON '{"b": 1}') AS result1,
    JSON_CONTAINS(JSON '{"a": {"b": 1}, "c": 2}', JSON '{"a": {"b": 1}}') AS result2,
    JSON_CONTAINS(JSON '{"a": {"b": 1, "d": 3}, "c": 2}', JSON '{"a": {"b": 1}}') AS result3;

/*----------*----------*----------+
 |  result1 |  result2 |  result3 |
 +----------+----------+----------+
 |   false  |   true   |   true   |
 +----------*----------*----------*/
```

The following examples check if a JSON array contains another JSON array. An
array contains another array if the first JSON array contains all the elements
present in the second array. The order of elements doesn't matter.

Also, if the array is a top-level array, it can contain a scalar value.

```sql
SELECT
    JSON_CONTAINS(JSON '[1, 2, 3]', JSON '[2]') AS result1,
    JSON_CONTAINS(JSON '[1, 2, 3]', JSON '2') AS result2;

/*----------*----------+
 |  result1 |  result2 |
 +----------+----------+
 |   true   |   true   |
 +----------*----------*/
```

```sql
SELECT
    JSON_CONTAINS(JSON '[[1, 2, 3]]', JSON '2') AS result1,
    JSON_CONTAINS(JSON '[[1, 2, 3]]', JSON '[2]') AS result2,
    JSON_CONTAINS(JSON '[[1, 2, 3]]', JSON '[[2]]') AS result3;

/*----------*----------*----------+
 |  result1 |  result2 |  result3 |
 +----------+----------+----------+
 |   false  |   false  |   true   |
 +----------*----------*----------*/
```

The following examples check if a JSON array contains a JSON object:

```sql
SELECT
    JSON_CONTAINS(JSON '[{"a":0}, {"b":1, "c":2}]', JSON '[{"b":1}]') AS result1,
    JSON_CONTAINS(JSON '[{"a":0}, {"b":1, "c":2}]', JSON '{"b":1}') AS results2,
    JSON_CONTAINS(JSON '[{"a":0}, {"b":1, "c":2}]', JSON '[{"a":0, "b":1}]') AS results3;

/*----------*----------*----------+
 |  result1 |  result2 |  result3 |
 +----------+----------+----------+
 |   true   |   false  |   false  |
 +----------*----------*----------*/
```

## `JSON_FLATTEN`

```sql
JSON_FLATTEN(json_expr)
```

**Description**

Produces a new SQL `ARRAY<JSON>` value containing all non-array values that are
either directly in the input JSON value or children of one or more consecutively
nested arrays in the input JSON value.

Arguments:

- `json_expr`: `JSON`. For example:

  ```sql
  JSON '["Jane", ["John", "Jamie"]]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.

**Return type**

`ARRAY<JSON>`

**Examples**

In the following example, there is a single non-array value that is returned.

```sql
SELECT JSON_FLATTEN(JSON '1') AS json_flatten

/*--------------+
 | json_flatten |
 +--------------+
 | [1]          |
 +--------------*/
```

In the following example, an input array of values is flattened.

```sql
SELECT JSON_FLATTEN(JSON '[1, 2, null]') AS json_flatten

/*--------------+
 | json_flatten |
 +--------------+
 | [1, 2, null] |
 +--------------*/
```

In the following example, an input array which includes nested array elements is
flattened.

```sql
SELECT JSON_FLATTEN(JSON '[[[1]], 2, [3]]') AS json_flatten

/*--------------+
 | json_flatten |
 +--------------+
 | [1, 2, 3]    |
 +--------------*/
```

In the following example, the nested-array value in a key-value pair is not
flattened because it is enclosed within a JSON object.

```sql
SELECT JSON_FLATTEN(JSON '{"a": [[1]]}') AS json_flatten

/*---------------+
 | json_flatten  |
 +---------------+
 | [{"a":[[1]]}] |
 +---------------*/
```

In the following example, the output contains both the flattened array elements
from the input and the non-array elements from the input.

```sql
SELECT JSON_FLATTEN(JSON '[[[1, 2], 3], {"a": 4}, true]') AS json_flatten

/*---------------------------+
 | json_flatten              |
 +---------------------------+
 | [1, 2, 3, {"a": 4}, true] |
 +---------------------------*/
```

## `JSON_OBJECT`

- [Signature 1](#json-object-signature1):
  `JSON_OBJECT([json_key, json_value][, ...])`
- [Signature 2](#json-object-signature2):
  `JSON_OBJECT(json_key_array, json_value_array)`

#### Signature 1

```sql
JSON_OBJECT([json_key, json_value][, ...])
```

**Description**

Creates a JSON object, using key-value pairs.

Arguments:

- `json_key`: A `VARCHAR` value that represents a key.
- `json_value`: A [JSON encoding-supported][json-encodings] value.

Details:

- If two keys are passed in with the same name, only the first key-value pair
  is preserved.
- The order of key-value pairs isn't preserved.
- If `json_key` is `NULL`, an error is produced.

**Return type**

`JSON`

**Examples**

You can create an empty JSON object by passing in no JSON keys and values.
For example:

```sql
SELECT JSON_OBJECT() AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {}        |
 +-----------*/
```

You can create a JSON object by passing in key-value pairs. For example:

```sql
SELECT JSON_OBJECT('foo', 10, 'bar', TRUE) AS json_data

/*-----------------------+
 | json_data             |
 +-----------------------+
 | {"bar":true,"foo":10} |
 +-----------------------*/
```

```sql
SELECT JSON_OBJECT('foo', 10, 'bar', ['a', 'b']) AS json_data

/*----------------------------+
 | json_data                  |
 +----------------------------+
 | {"bar":["a","b"],"foo":10} |
 +----------------------------*/
```

```sql
SELECT JSON_OBJECT('a', NULL, 'b', JSON 'null') AS json_data

/*---------------------+
 | json_data           |
 +---------------------+
 | {"a":null,"b":null} |
 +---------------------*/
```

```sql
SELECT JSON_OBJECT('a', 10, 'a', 'foo') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"a":10}  |
 +-----------*/
```

```sql
WITH Items AS (SELECT 'hello' AS key, 'world' AS value)
SELECT JSON_OBJECT(key, value) AS json_data FROM Items

/*-------------------+
 | json_data         |
 +-------------------+
 | {"hello":"world"} |
 +-------------------*/
```

An error is produced if a SQL `NULL` is passed in for a JSON key.

```sql
-- Error: A key can't be NULL.
SELECT JSON_OBJECT(NULL, 1) AS json_data
```

An error is produced if the number of JSON keys and JSON values don't match:

```sql
-- Error: No matching signature for function JSON_OBJECT for argument types:
-- VARCHAR, BIGINT, VARCHAR
SELECT JSON_OBJECT('a', 1, 'b') AS json_data
```

#### Signature 2

```sql
JSON_OBJECT(json_key_array, json_value_array)
```

Creates a JSON object, using an array of keys and values.

Arguments:

- `json_key_array`: An array of zero or more `VARCHAR` keys.
- `json_value_array`: An array of zero or more
  [JSON encoding-supported][json-encodings] values.

Details:

- If two keys are passed in with the same name, only the first key-value pair
  is preserved.
- The order of key-value pairs isn't preserved.
- The number of keys must match the number of values, otherwise an error is
  produced.
- If any argument is `NULL`, an error is produced.
- If a key in `json_key_array` is `NULL`, an error is produced.

**Return type**

`JSON`

**Examples**

You can create an empty JSON object by passing in an empty array of
keys and values. For example:

```sql
SELECT JSON_OBJECT(CAST([] AS ARRAY), []) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {}        |
 +-----------*/
```

You can create a JSON object by passing in an array of keys and an array of
values. For example:

```sql
SELECT JSON_OBJECT(['a', 'b'], [10, NULL]) AS json_data

/*-------------------+
 | json_data         |
 +-------------------+
 | {"a":10,"b":null} |
 +-------------------*/
```

```sql
SELECT JSON_OBJECT(['a', 'b'], [JSON '10', JSON '"foo"']) AS json_data

/*--------------------+
 | json_data          |
 +--------------------+
 | {"a":10,"b":"foo"} |
 +--------------------*/
```

```sql
SELECT
  JSON_OBJECT(
    ['a', 'b'],
    [STRUCT(10 AS id, 'Red' AS color), STRUCT(20 AS id, 'Blue' AS color)])
    AS json_data

/*------------------------------------------------------------+
 | json_data                                                  |
 +------------------------------------------------------------+
 | {"a":{"color":"Red","id":10},"b":{"color":"Blue","id":20}} |
 +------------------------------------------------------------*/
```

```sql
SELECT
  JSON_OBJECT(
    ['a', 'b'],
    [TO_JSON(10), TO_JSON(['foo', 'bar'])])
    AS json_data

/*----------------------------+
 | json_data                  |
 +----------------------------+
 | {"a":10,"b":["foo","bar"]} |
 +----------------------------*/
```

The following query groups by `id` and then creates an array of keys and
values from the rows with the same `id`:

```sql
WITH
  Fruits AS (
    SELECT 0 AS id, 'color' AS json_key, 'red' AS json_value UNION ALL
    SELECT 0, 'fruit', 'apple' UNION ALL
    SELECT 1, 'fruit', 'banana' UNION ALL
    SELECT 1, 'ripe', 'true'
  )
SELECT JSON_OBJECT(ARRAY_AGG(json_key), ARRAY_AGG(json_value)) AS json_data
FROM Fruits
GROUP BY id

/*----------------------------------+
 | json_data                        |
 +----------------------------------+
 | {"color":"red","fruit":"apple"}  |
 | {"fruit":"banana","ripe":"true"} |
 +----------------------------------*/
```

An error is produced if the size of the JSON keys and values arrays don't
match:

```sql
-- Error: The number of keys and values must match.
SELECT JSON_OBJECT(['a', 'b'], [10]) AS json_data
```

An error is produced if the array of JSON keys or JSON values is a SQL `NULL`.

```sql
-- Error: The keys array can't be NULL.
SELECT JSON_OBJECT(CAST(NULL AS ARRAY), [10, 20]) AS json_data
```

```sql
-- Error: The values array can't be NULL.
SELECT JSON_OBJECT(['a', 'b'], CAST(NULL AS ARRAY)) AS json_data
```

[json-encodings]: #json-encodings

## `JSON_STRIP_NULLS`

```sql
JSON_STRIP_NULLS(
  json_expr
  [, json_path ]
  [, include_arrays => { TRUE | FALSE } ]
  [, remove_empty => { TRUE | FALSE } ]
)
```

Recursively removes JSON nulls from JSON objects and JSON arrays.

Arguments:

- `json_expr`: JSON. For example:

  ````sql
  JSON '{"a": null, "b": "c"}'
  ```sql
  ````

- `json_path`: Remove JSON nulls at this [JSONPath][JSONPath-format] for
  `json_expr`.
- `include_arrays`: A named argument that's either
  `TRUE` (default) or `FALSE`. If `TRUE` or omitted, the function removes
  JSON nulls from JSON arrays. If `FALSE`, doesn't.
- `remove_empty`: A named argument that's either
  `TRUE` or `FALSE` (default). If `TRUE`, the function removes empty
  JSON objects after JSON nulls are removed. If `FALSE` or omitted, doesn't.

  If `remove_empty` is `TRUE` and `include_arrays` is `TRUE` or omitted,
  the function additionally removes empty JSON arrays.

Details:

- If a value is a JSON null, the associated key-value pair is removed.
- If `remove_empty` is set to `TRUE`, the function recursively removes empty
  containers after JSON nulls are removed.
- If the function generates JSON with nothing in it, the function returns a
  JSON null.
- If `json_path` is an invalid [JSONPath][JSONPath-format], an error is
  produced.
- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- If `json_path`, `include_arrays`, or `remove_empty` is SQL `NULL`, the
  function returns `json_expr`.

**Return type**

`JSON`

**Examples**

In the following example, all JSON nulls are removed.

```sql
SELECT JSON_STRIP_NULLS(JSON '{"a": null, "b": "c"}') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"b":"c"} |
 +-----------*/
```

In the following example, all JSON nulls are removed from a JSON array.

```sql
SELECT JSON_STRIP_NULLS(JSON '[1, null, 2, null]') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | [1,2]     |
 +-----------*/
```

In the following example, `include_arrays` is set as `FALSE` so that JSON nulls
aren't removed from JSON arrays.

```sql
SELECT JSON_STRIP_NULLS(JSON '[1, null, 2, null]', include_arrays=>FALSE) AS json_data

/*-----------------+
 | json_data       |
 +-----------------+
 | [1,null,2,null] |
 +-----------------*/
```

In the following example, `remove_empty` is omitted and defaults to
`FALSE`, and the empty structures are retained.

```sql
SELECT JSON_STRIP_NULLS(JSON '[1, null, 2, null, [null]]') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | [1,2,[]]  |
 +-----------*/
```

In the following example, `remove_empty` is set as `TRUE`, and the
empty structures are removed.

```sql
SELECT JSON_STRIP_NULLS(
  JSON '[1, null, 2, null, [null]]',
  remove_empty=>TRUE) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | [1,2]     |
 +-----------*/
```

In the following examples, `remove_empty` is set as `TRUE`, and the
empty structures are removed. Because no JSON data is left the function
returns JSON null.

```sql
SELECT JSON_STRIP_NULLS(JSON '{"a": null}', remove_empty=>TRUE) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | null      |
 +-----------*/
```

```sql
SELECT JSON_STRIP_NULLS(JSON '{"a": [null]}', remove_empty=>TRUE) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | null      |
 +-----------*/
```

In the following example, empty structures are removed for JSON objects,
but not JSON arrays.

```sql
SELECT JSON_STRIP_NULLS(
  JSON '{"a": {"b": {"c": null}}, "d": [null], "e": [], "f": 1}',
  include_arrays=>FALSE,
  remove_empty=>TRUE) AS json_data

/*---------------------------+
 | json_data                 |
 +---------------------------+
 | {"d":[null],"e":[],"f":1} |
 +---------------------------*/
```

In the following example, empty structures are removed for both JSON objects,
and JSON arrays.

```sql
SELECT JSON_STRIP_NULLS(
  JSON '{"a": {"b": {"c": null}}, "d": [null], "e": [], "f": 1}',
  remove_empty=>TRUE) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"f":1}   |
 +-----------*/
```

In the following example, because no JSON data is left, the function returns a
JSON null.

```sql
SELECT JSON_STRIP_NULLS(JSON 'null') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | null      |
 +-----------*/
```

## `JSON_TYPE`

```sql
JSON_TYPE(json_expr)
```

**Description**

Gets the JSON type of the outermost JSON value and converts the name of
this type to a SQL `VARCHAR` value. The names of these JSON types can be
returned: `object`, `array`, `string`, `number`, `boolean`, `null`

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '{"name": "sky", "color": "blue"}'
  ```

  If this expression is SQL `NULL`, the function returns SQL `NULL`. If the
  extracted JSON value isn't a valid JSON type, an error is produced.

**Return type**

`VARCHAR`

**Examples**

```sql
SELECT json_val, JSON_TYPE(json_val) AS type
FROM
  UNNEST(
    [
      JSON '"apple"',
      JSON '10',
      JSON '3.14',
      JSON 'null',
      JSON '{"city": "New York", "State": "NY"}',
      JSON '["apple", "banana"]',
      JSON 'false'
    ]
  ) AS json_val;

/*----------------------------------+---------+
 | json_val                         | type    |
 +----------------------------------+---------+
 | "apple"                          | string  |
 | 10                               | number  |
 | 3.14                             | number  |
 | null                             | null    |
 | {"State":"NY","city":"New York"} | object  |
 | ["apple","banana"]               | array   |
 | false                            | boolean |
 +----------------------------------+---------*/
```
