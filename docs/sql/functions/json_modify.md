# JSON Modification Functions

Functions for modifying JSON values.

## `JSON_ARRAY_APPEND`

```sql
JSON_ARRAY_APPEND(
  json_expr,
  json_path_value_pair[, ...]
  [, append_each_element => { TRUE | FALSE } ]
)

json_path_value_pair:
  json_path, value
```

Appends JSON data to the end of a JSON array.

Arguments:

- `json_expr`: JSON. For example:

  ````sql
  JSON '["a", "b", "c"]'
  ```sql
  ````

- `json_path_value_pair`: A value and the [JSONPath][JSONPath-format] for
  that value. This includes:
  - `json_path`: Append `value` at this [JSONPath][JSONPath-format]
    in `json_expr`.

  - `value`: A [JSON encoding-supported][json-encodings] value to
    append.

- `append_each_element`: A named argument with a `BOOL` value.
  - If `TRUE` (default), and `value` is a SQL array,
    appends each element individually.

  - If `FALSE,` and `value` is a SQL array, appends
    the array as one element.

Details:

- Path value pairs are evaluated left to right. The JSON produced by
  evaluating one pair becomes the JSON against which the next pair
  is evaluated.
- The operation is ignored if the path points to a JSON non-array value that
  isn't a JSON null.
- If `json_path` points to a JSON null, the JSON null is replaced by a
  JSON array that contains `value`.
- If the path exists but has an incompatible type at any given path token,
  the path value pair operation is ignored.
- The function applies all path value pair append operations even if an
  individual path value pair operation is invalid. For invalid operations,
  the operation is ignored and the function continues to process the rest of
  the path value pairs.
- If any `json_path` is an invalid [JSONPath][JSONPath-format], an error is
  produced.
- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- If `append_each_element` is SQL `NULL`, the function returns `json_expr`.
- If `json_path` is SQL `NULL`, the `json_path_value_pair` operation is
  ignored.

**Return type**

`JSON`

**Examples**

In the following example, path `$` is matched and appends `1`.

```sql
SELECT JSON_ARRAY_APPEND(JSON '["a", "b", "c"]', '$', 1) AS json_data

/*-----------------+
 | json_data       |
 +-----------------+
 | ["a","b","c",1] |
 +-----------------*/
```

In the following example, `append_each_element` defaults to `TRUE`, so
`[1, 2]` is appended as individual elements.

```sql
SELECT JSON_ARRAY_APPEND(JSON '["a", "b", "c"]', '$', [1, 2]) AS json_data

/*-------------------+
 | json_data         |
 +-------------------+
 | ["a","b","c",1,2] |
 +-------------------*/
```

In the following example, `append_each_element` is `FALSE`, so
`[1, 2]` is appended as one element.

```sql
SELECT JSON_ARRAY_APPEND(
  JSON '["a", "b", "c"]',
  '$', [1, 2],
  append_each_element=>FALSE) AS json_data

/*---------------------+
 | json_data           |
 +---------------------+
 | ["a","b","c",[1,2]] |
 +---------------------*/
```

In the following example, `append_each_element` is `FALSE`, so
`[1, 2]` and `[3, 4]` are each appended as one element.

```sql
SELECT JSON_ARRAY_APPEND(
  JSON '["a", ["b"], "c"]',
  '$[1]', [1, 2],
  '$[1][1]', [3, 4],
  append_each_element=>FALSE) AS json_data

/*-----------------------------+
 | json_data                   |
 +-----------------------------+
 | ["a",["b",[1,2,[3,4]]],"c"] |
 +-----------------------------*/
```

In the following example, the first path `$[1]` appends `[1, 2]` as single
elements, and then the second path `$[1][1]` isn't a valid path to an array,
so the second operation is ignored.

```sql
SELECT JSON_ARRAY_APPEND(
  JSON '["a", ["b"], "c"]',
  '$[1]', [1, 2],
  '$[1][1]', [3, 4]) AS json_data

/*---------------------+
 | json_data           |
 +---------------------+
 | ["a",["b",1,2],"c"] |
 +---------------------*/
```

In the following example, path `$.a` is matched and appends `2`.

```sql
SELECT JSON_ARRAY_APPEND(JSON '{"a": [1]}', '$.a', 2) AS json_data

/*-------------+
 | json_data   |
 +-------------+
 | {"a":[1,2]} |
 +-------------*/
```

In the following example, a value is appended into a JSON null.

```sql
SELECT JSON_ARRAY_APPEND(JSON '{"a": null}', '$.a', 10)

/*------------+
 | json_data  |
 +------------+
 | {"a":[10]} |
 +------------*/
```

In the following example, path `$.a` isn't an array, so the operation is
ignored.

```sql
SELECT JSON_ARRAY_APPEND(JSON '{"a": 1}', '$.a', 2) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"a":1}   |
 +-----------*/
```

In the following example, path `$.b` doesn't exist, so the operation is
ignored.

```sql
SELECT JSON_ARRAY_APPEND(JSON '{"a": 1}', '$.b', 2) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"a":1}   |
 +-----------*/
```

## `JSON_ARRAY_INSERT`

```sql
JSON_ARRAY_INSERT(
  json_expr,
  json_path_value_pair[, ...]
  [, insert_each_element => { TRUE | FALSE } ]
)

json_path_value_pair:
  json_path, value
```

Produces a new JSON value that's created by inserting JSON data into
a JSON array.

Arguments:

- `json_expr`: JSON. For example:

  ````sql
  JSON '["a", "b", "c"]'
  ```sql
  ````

- `json_path_value_pair`: A value and the [JSONPath][JSONPath-format] for
  that value. This includes:
  - `json_path`: Insert `value` at this [JSONPath][JSONPath-format]
    in `json_expr`.

  - `value`: A [JSON encoding-supported][json-encodings] value to
    insert.

- `insert_each_element`: A named argument with a `BOOL` value.
  - If `TRUE` (default), and `value` is a SQL array,
    inserts each element individually.

  - If `FALSE,` and `value` is a SQL array, inserts
    the array as one element.

Details:

- Path value pairs are evaluated left to right. The JSON produced by
  evaluating one pair becomes the JSON against which the next pair
  is evaluated.
- The operation is ignored if the path points to a JSON non-array value that
  isn't a JSON null.
- If `json_path` points to a JSON null, the JSON null is replaced by a
  JSON array of the appropriate size and padded on the left with JSON nulls.
- If the path exists but has an incompatible type at any given path token,
  the path value pair operator is ignored.
- The function applies all path value pair append operations even if an
  individual path value pair operation is invalid. For invalid operations,
  the operation is ignored and the function continues to process the rest of
  the path value pairs.
- If the array index in `json_path` is larger than the size of the array, the
  function extends the length of the array to the index, fills in
  the array with JSON nulls, then adds `value` at the index.
- If any `json_path` is an invalid [JSONPath][JSONPath-format], an error is
  produced.
- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- If `insert_each_element` is SQL `NULL`, the function returns `json_expr`.
- If `json_path` is SQL `NULL`, the `json_path_value_pair` operation is
  ignored.

**Return type**

`JSON`

**Examples**

In the following example, path `$[1]` is matched and inserts `1`.

```sql
SELECT JSON_ARRAY_INSERT(JSON '["a", ["b", "c"], "d"]', '$[1]', 1) AS json_data

/*-----------------------+
 | json_data             |
 +-----------------------+
 | ["a",1,["b","c"],"d"] |
 +-----------------------*/
```

In the following example, path `$[1][0]` is matched and inserts `1`.

```sql
SELECT JSON_ARRAY_INSERT(JSON '["a", ["b", "c"], "d"]', '$[1][0]', 1) AS json_data

/*-----------------------+
 | json_data             |
 +-----------------------+
 | ["a",[1,"b","c"],"d"] |
 +-----------------------*/
```

In the following example, `insert_each_element` defaults to `TRUE`, so
`[1, 2]` is inserted as individual elements.

```sql
SELECT JSON_ARRAY_INSERT(JSON '["a", "b", "c"]', '$[1]', [1, 2]) AS json_data

/*-------------------+
 | json_data         |
 +-------------------+
 | ["a",1,2,"b","c"] |
 +-------------------*/
```

In the following example, `insert_each_element` is `FALSE`, so `[1, 2]` is
inserted as one element.

```sql
SELECT JSON_ARRAY_INSERT(
  JSON '["a", "b", "c"]',
  '$[1]', [1, 2],
  insert_each_element=>FALSE) AS json_data

/*---------------------+
 | json_data           |
 +---------------------+
 | ["a",[1,2],"b","c"] |
 +---------------------*/
```

In the following example, path `$[7]` is larger than the length of the
matched array, so the array is extended with JSON nulls and `"e"` is inserted at
the end of the array.

```sql
SELECT JSON_ARRAY_INSERT(JSON '["a", "b", "c", "d"]', '$[7]', "e") AS json_data

/*--------------------------------------+
 | json_data                            |
 +--------------------------------------+
 | ["a","b","c","d",null,null,null,"e"] |
 +--------------------------------------*/
```

In the following example, path `$.a` is an object, so the operation is ignored.

```sql
SELECT JSON_ARRAY_INSERT(JSON '{"a": {}}', '$.a[0]', 2) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"a":{}}  |
 +-----------*/
```

In the following example, path `$` doesn't specify a valid array position,
so the operation is ignored.

```sql
SELECT JSON_ARRAY_INSERT(JSON '[1, 2]', '$', 3) AS json_data

/*-----------+
 | json_data |
 +-----------+
 | [1,2]     |
 +-----------*/
```

In the following example, a value is inserted into a JSON null.

```sql
SELECT JSON_ARRAY_INSERT(JSON '{"a": null}', '$.a[2]', 10) AS json_data

/*----------------------+
 | json_data            |
 +----------------------+
 | {"a":[null,null,10]} |
 +----------------------*/
```

In the following example, the operation is ignored because you can't insert
data into a JSON number.

```sql
SELECT JSON_ARRAY_INSERT(JSON '1', '$[0]', 'r1') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | 1         |
 +-----------*/
```

## `JSON_REMOVE`

```sql
JSON_REMOVE(json_expr, json_path[, ...])
```

Produces a new SQL `JSON` value with the specified JSON data removed.

Arguments:

- `json_expr`: JSON. For example:

  ````sql
  JSON '{"class": {"students": [{"name": "Jane"}]}}'
  ```sql
  ````

- `json_path`: Remove data at this [JSONPath][JSONPath-format] in `json_expr`.

Details:

- Paths are evaluated left to right. The JSON produced by evaluating the
  first path is the JSON for the next path.
- The operation ignores non-existent paths and continue processing the rest
  of the paths.
- For each path, the entire matched JSON subtree is deleted.
- If the path matches a JSON object key, this function deletes the
  key-value pair.
- If the path matches an array element, this function deletes the specific
  element from the matched array.
- If removing the path results in an empty JSON object or empty JSON array,
  the empty structure is preserved.
- If `json_path` is `$` or an invalid [JSONPath][JSONPath-format], an error is
  produced.
- If `json_path` is SQL `NULL`, the path operation is ignored.

**Return type**

`JSON`

**Examples**

In the following example, the path `$[1]` is matched and removes
`["b", "c"]`.

```sql
SELECT JSON_REMOVE(JSON '["a", ["b", "c"], "d"]', '$[1]') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | ["a","d"] |
 +-----------*/
```

You can use the field access operator to pass JSON data into this function.
For example:

```sql
WITH T AS (SELECT JSON '{"a": {"b": 10, "c": 20}}' AS data)
SELECT JSON_REMOVE(data.a, '$.b') AS json_data FROM T

/*-----------+
 | json_data |
 +-----------+
 | {"c":20}  |
 +-----------*/
```

In the following example, the first path `$[1]` is matched and removes
`["b", "c"]`. Then, the second path `$[1]` is matched and removes `"d"`.

```sql
SELECT JSON_REMOVE(JSON '["a", ["b", "c"], "d"]', '$[1]', '$[1]') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | ["a"]     |
 +-----------*/
```

The structure of an empty array is preserved when all elements are deleted
from it. For example:

```sql
SELECT JSON_REMOVE(JSON '["a", ["b", "c"], "d"]', '$[1]', '$[1]', '$[0]') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | []        |
 +-----------*/
```

In the following example, the path `$.a.b.c` is matched and removes the
`"c":"d"` key-value pair from the JSON object.

```sql
SELECT JSON_REMOVE(JSON '{"a": {"b": {"c": "d"}}}', '$.a.b.c') AS json_data

/*----------------+
 | json_data      |
 +----------------+
 | {"a":{"b":{}}} |
 +----------------*/
```

In the following example, the path `$.a.b` is matched and removes the
`"b": {"c":"d"}` key-value pair from the JSON object.

```sql
SELECT JSON_REMOVE(JSON '{"a": {"b": {"c": "d"}}}', '$.a.b') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"a":{}}  |
 +-----------*/
```

In the following example, the path `$.b` isn't valid, so the operation makes
no changes.

```sql
SELECT JSON_REMOVE(JSON '{"a": 1}', '$.b') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"a":1}   |
 +-----------*/
```

In the following example, path `$.a.b` and `$.b` don't exist, so those
operations are ignored, but the others are processed.

```sql
SELECT JSON_REMOVE(JSON '{"a": [1, 2, 3]}', '$.a[0]', '$.a.b', '$.b', '$.a[0]') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | {"a":[3]} |
 +-----------*/
```

If you pass in `$` as the path, an error is produced. For example:

```sql
-- Error: The JSONPath can't be '$'
SELECT JSON_REMOVE(JSON '{}', '$') AS json_data
```

In the following example, the operation is ignored because you can't remove
data from a JSON null.

```sql
SELECT JSON_REMOVE(JSON 'null', '$.a.b') AS json_data

/*-----------+
 | json_data |
 +-----------+
 | null      |
 +-----------*/
```

## `JSON_SET`

```sql
JSON_SET(
  json_expr,
  json_path_value_pair[, ...]
  [, create_if_missing => { TRUE | FALSE } ]
)

json_path_value_pair:
  json_path, value
```

Produces a new SQL `JSON` value with the specified JSON data inserted
or replaced.

Arguments:

- `json_expr`: JSON. For example:

  ````sql
  JSON '{"class": {"students": [{"name": "Jane"}]}}'
  ```sql
  ````

- `json_path_value_pair`: A value and the [JSONPath][JSONPath-format] for
  that value. This includes:
  - `json_path`: Insert or replace `value` at this [JSONPath][JSONPath-format]
    in `json_expr`.

  - `value`: A [JSON encoding-supported][json-encodings] value to
    insert.

- `create_if_missing`: A named argument that takes a `BOOL` value.
  - If `TRUE` (default), replaces or inserts data if the path doesn't exist.

  - If `FALSE`, only existing JSONPath values are replaced. If the path
    doesn't exist, the set operation is ignored.

Details:

- Path value pairs are evaluated left to right. The JSON produced by
  evaluating one pair becomes the JSON against which the next pair
  is evaluated.
- If a matched path has an existing value, it overwrites the existing data
  with `value`.
- If `create_if_missing` is `TRUE`:
  - If a path doesn't exist, the remainder of the path is recursively
    created.
  - If the matched path prefix points to a JSON null, the remainder of the
    path is recursively created, and `value` is inserted.
  - If a path token points to a JSON array and the specified index is
    _larger_ than the size of the array, pads the JSON array with JSON
    nulls, recursively creates the remainder of the path at the specified
    index, and inserts the path value pair.

- This function applies all path value pair set operations even if an
  individual path value pair operation is invalid. For invalid operations,
  the operation is ignored and the function continues to process the rest
  of the path value pairs.
- If the path exists but has an incompatible type at any given path
  token, no update happens for that specific path value pair.
- If any `json_path` is an invalid [JSONPath][JSONPath-format], an error is
  produced.
- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- If `json_path` is SQL `NULL`, the `json_path_value_pair` operation is
  ignored.
- If `create_if_missing` is SQL `NULL`, the set operation is ignored.

**Return type**

`JSON`

**Examples**

In the following example, the path `$` matches the entire `JSON` value
and replaces it with `{"b": 2, "c": 3}`.

```sql
SELECT JSON_SET(JSON '{"a": 1}', '$', JSON '{"b": 2, "c": 3}') AS json_data

/*---------------+
 | json_data     |
 +---------------+
 | {"b":2,"c":3} |
 +---------------*/
```

In the following example, `create_if_missing` is `FALSE` and the path `$.b`
doesn't exist, so the set operation is ignored.

```sql
SELECT JSON_SET(
  JSON '{"a": 1}',
  "$.b", 999,
  create_if_missing => false) AS json_data

/*------------+
 | json_data  |
 +------------+
 | '{"a": 1}' |
 +------------*/
```

In the following example, `create_if_missing` is `TRUE` and the path `$.a`
exists, so the value is replaced.

```sql
SELECT JSON_SET(
  JSON '{"a": 1}',
  "$.a", 999,
  create_if_missing => false) AS json_data

/*--------------+
 | json_data    |
 +--------------+
 | '{"a": 999}' |
 +--------------*/
```

In the following example, the path `$.a` is matched, but `$.a.b` doesn't
exist, so the new path and the value are inserted.

```sql
SELECT JSON_SET(JSON '{"a": {}}', '$.a.b', 100) AS json_data

/*-----------------+
 | json_data       |
 +-----------------+
 | {"a":{"b":100}} |
 +-----------------*/
```

In the following example, the path prefix `$` points to a JSON null, so the
remainder of the path is created for the value `100`.

```sql
SELECT JSON_SET(JSON 'null', '$.a.b', 100) AS json_data

/*-----------------+
 | json_data       |
 +-----------------+
 | {"a":{"b":100}} |
 +-----------------*/
```

In the following example, the path `$.a.c` implies that the value at `$.a` is
a JSON object but it's not. This part of the operation is ignored, but the other
parts of the operation are completed successfully.

```sql
SELECT JSON_SET(
  JSON '{"a": 1}',
  '$.b', 2,
  '$.a.c', 100,
  '$.d', 3) AS json_data

/*---------------------+
 | json_data           |
 +---------------------+
 | {"a":1,"b":2,"d":3} |
 +---------------------*/
```

In the following example, the path `$.a[2]` implies that the value for `$.a` is
an array, but it's not, so the operation is ignored for that value.

```sql
SELECT JSON_SET(
  JSON '{"a": 1}',
  '$.a[2]', 100,
  '$.b', 2) AS json_data

/*---------------+
 | json_data     |
 +---------------+
 | {"a":1,"b":2} |
 +---------------*/
```

In the following example, the path `$[1]` is matched and replaces the
array element value with `foo`.

```sql
SELECT JSON_SET(JSON '["a", ["b", "c"], "d"]', '$[1]', "foo") AS json_data

/*-----------------+
 | json_data       |
 +-----------------+
 | ["a","foo","d"] |
 +-----------------*/
```

In the following example, the path `$[1][0]` is matched and replaces the
array element value with `foo`.

```sql
SELECT JSON_SET(JSON '["a", ["b", "c"], "d"]', '$[1][0]', "foo") AS json_data

/*-----------------------+
 | json_data             |
 +-----------------------+
 | ["a",["foo","c"],"d"] |
 +-----------------------*/
```

In the following example, the path prefix `$` points to a JSON null, so the
remainder of the path is created. The resulting array is padded with
JSON nulls and appended with `foo`.

```sql
SELECT JSON_SET(JSON 'null', '$[0][3]', "foo")

/*--------------------------+
 | json_data                |
 +--------------------------+
 | [[null,null,null,"foo"]] |
 +--------------------------*/
```

In the following example, the path `$[1]` is matched, the matched array is
extended since `$[1][4]` is larger than the existing array, and then `foo` is
inserted in the array.

```sql
SELECT JSON_SET(JSON '["a", ["b", "c"], "d"]', '$[1][4]', "foo") AS json_data

/*-------------------------------------+
 | json_data                           |
 +-------------------------------------+
 | ["a",["b","c",null,null,"foo"],"d"] |
 +-------------------------------------*/
```

In the following example, the path `$[1][0][0]` implies that the value of
`$[1][0]` is an array, but it isn't, so the operation is ignored.

```sql
SELECT JSON_SET(JSON '["a", ["b", "c"], "d"]', '$[1][0][0]', "foo") AS json_data

/*---------------------+
 | json_data           |
 +---------------------+
 | ["a",["b","c"],"d"] |
 +---------------------*/
```

In the following example, the path `$[1][2]` is larger than the length of
the matched array. The array length is extended and the remainder of the path
is recursively created. The operation continues to the path `$[1][2][1]`
and inserts `foo`.

```sql
SELECT JSON_SET(JSON '["a", ["b", "c"], "d"]', '$[1][2][1]', "foo") AS json_data

/*----------------------------------+
 | json_data                        |
 +----------------------------------+
 | ["a",["b","c",[null,"foo"]],"d"] |
 +----------------------------------*/
```

In the following example, because the `JSON` object is empty, key `b` is
inserted, and the remainder of the path is recursively created.

```sql
SELECT JSON_SET(JSON '{}', '$.b[2].d', 100) AS json_data

/*-----------------------------+
 | json_data                   |
 +-----------------------------+
 | {"b":[null,null,{"d":100}]} |
 +-----------------------------*/
```

In the following example, multiple values are set.

```sql
SELECT JSON_SET(
  JSON '{"a": 1, "b": {"c":3}, "d": [4]}',
  '$.a', 'v1',
  '$.b.e', 'v2',
  '$.d[2]', 'v3') AS json_data

/*---------------------------------------------------+
 | json_data                                         |
 +---------------------------------------------------+
 | {"a":"v1","b":{"c":3,"e":"v2"},"d":[4,null,"v3"]} |
 +---------------------------------------------------*/
```
