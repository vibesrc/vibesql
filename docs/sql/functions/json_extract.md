# JSON Extraction Functions

Functions for extracting data from JSON values.

## `JSON_EXTRACT`

Note: This function is deprecated. Consider using [JSON_QUERY][json-query].

```sql
JSON_EXTRACT(json_string_expr, json_path)
```

```sql
JSON_EXTRACT(json_expr, json_path)
```

**Description**

Extracts a JSON value and converts it to a
SQL JSON-formatted `VARCHAR` or `JSON` value.
This function uses single quotes and brackets to escape invalid
[JSONPath][JSONPath-format] characters in JSON keys. For example: `['a.b']`.

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ```sql
  '{"class": {"students": [{"name": "Jane"}]}}'
  ```

  Extracts a SQL `NULL` when a JSON-formatted string `null` is encountered.
  For example:

  ````sql
  SELECT JSON_EXTRACT("null", "$") -- Returns a SQL NULL
  ```sql
  ````

- `json_expr`: JSON. For example:

  ```sql
  JSON '{"class": {"students": [{"name": "Jane"}]}}'
  ```

  Extracts a JSON `null` when a JSON `null` is encountered.

  ````sql
  SELECT JSON_EXTRACT(JSON 'null', "$") -- Returns a JSON 'null'
  ```sql
  ````

- `json_path`: The [JSONPath][JSONPath-format]. This identifies the data that
  you want to obtain from the input.

There are differences between the JSON-formatted string and JSON input types.
For details, see [Differences between the JSON and JSON-formatted VARCHAR types][differences-json-and-string].

**Return type**

- `json_string_expr`: A JSON-formatted `VARCHAR`
- `json_expr`: `JSON`

**Examples**

In the following example, JSON data is extracted and returned as JSON.

```sql
SELECT
  JSON_EXTRACT(JSON '{"class": {"students": [{"id": 5}, {"id": 12}]}}', '$.class')
  AS json_data;

/*-----------------------------------+
 | json_data                         |
 +-----------------------------------+
 | {"students":[{"id":5},{"id":12}]} |
 +-----------------------------------*/
```

In the following examples, JSON data is extracted and returned as
JSON-formatted strings.

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "Jane"}]}}',
  '$') AS json_text_string;

/*-----------------------------------------------------------+
 | json_text_string                                          |
 +-----------------------------------------------------------+
 | {"class":{"students":[{"name":"Jane"}]}}                  |
 +-----------------------------------------------------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": []}}',
  '$') AS json_text_string;

/*-----------------------------------------------------------+
 | json_text_string                                          |
 +-----------------------------------------------------------+
 | {"class":{"students":[]}}                                 |
 +-----------------------------------------------------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "John"}, {"name": "Jamie"}]}}',
  '$') AS json_text_string;

/*-----------------------------------------------------------+
 | json_text_string                                          |
 +-----------------------------------------------------------+
 | {"class":{"students":[{"name":"John"},{"name":"Jamie"}]}} |
 +-----------------------------------------------------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "Jane"}]}}',
  '$.class.students[0]') AS first_student;

/*-----------------+
 | first_student   |
 +-----------------+
 | {"name":"Jane"} |
 +-----------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": []}}',
  '$.class.students[0]') AS first_student;

/*-----------------+
 | first_student   |
 +-----------------+
 | NULL            |
 +-----------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "John"}, {"name": "Jamie"}]}}',
  '$.class.students[0]') AS first_student;

/*-----------------+
 | first_student   |
 +-----------------+
 | {"name":"John"} |
 +-----------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "Jane"}]}}',
  '$.class.students[1].name') AS second_student;

/*----------------+
 | second_student |
 +----------------+
 | NULL           |
 +----------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": []}}',
  '$.class.students[1].name') AS second_student;

/*----------------+
 | second_student |
 +----------------+
 | NULL           |
 +----------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "John"}, {"name": null}]}}',
  '$.class.students[1].name') AS second_student;

/*----------------+
 | second_student |
 +----------------+
 | NULL           |
 +----------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "John"}, {"name": "Jamie"}]}}',
  '$.class.students[1].name') AS second_student;

/*----------------+
 | second_student |
 +----------------+
 | "Jamie"        |
 +----------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "Jane"}]}}',
  "$.class['students']") AS student_names;

/*------------------------------------+
 | student_names                      |
 +------------------------------------+
 | [{"name":"Jane"}]                  |
 +------------------------------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": []}}',
  "$.class['students']") AS student_names;

/*------------------------------------+
 | student_names                      |
 +------------------------------------+
 | []                                 |
 +------------------------------------*/
```

```sql
SELECT JSON_EXTRACT(
  '{"class": {"students": [{"name": "John"}, {"name": "Jamie"}]}}',
  "$.class['students']") AS student_names;

/*------------------------------------+
 | student_names                      |
 +------------------------------------+
 | [{"name":"John"},{"name":"Jamie"}] |
 +------------------------------------*/
```

```sql
SELECT JSON_EXTRACT('{"a": null}', "$.a"); -- Returns a SQL NULL
SELECT JSON_EXTRACT('{"a": null}', "$.b"); -- Returns a SQL NULL
```

```sql
SELECT JSON_EXTRACT(JSON '{"a": null}', "$.a"); -- Returns a JSON 'null'
SELECT JSON_EXTRACT(JSON '{"a": null}', "$.b"); -- Returns a SQL NULL
```

[json-query]: #json-query
[JSONPath-format]: #JSONPath_format
[differences-json-and-string]: #differences-json_and_string

## `JSON_EXTRACT_ARRAY`

Note: This function is deprecated. Consider using
[JSON_QUERY_ARRAY][json-query-array].

```sql
JSON_EXTRACT_ARRAY(json_string_expr[, json_path])
```

```sql
JSON_EXTRACT_ARRAY(json_expr[, json_path])
```

**Description**

Extracts a JSON array and converts it to
a SQL `ARRAY<JSON-formatted VARCHAR>` or
`ARRAY<JSON>` value.
This function uses single quotes and brackets to escape invalid
[JSONPath][JSONPath-format] characters in JSON keys. For example: `['a.b']`.

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ````sql
  '["a", "b", {"key": "c"}]'
  ```sql
  ````

- `json_expr`: JSON. For example:

  ````sql
  JSON '["a", "b", {"key": "c"}]'
  ```sql
  ````

- `json_path`: The [JSONPath][JSONPath-format]. This identifies the data that
  you want to obtain from the input. If this optional parameter isn't
  provided, then the JSONPath `$` symbol is applied, which means that all of
  the data is analyzed.

There are differences between the JSON-formatted string and JSON input types.
For details, see [Differences between the JSON and JSON-formatted VARCHAR types][differences-json-and-string].

**Return type**

- `json_string_expr`: `ARRAY<JSON-formatted VARCHAR>`
- `json_expr`: `ARRAY<JSON>`

**Examples**

This extracts items in JSON to an array of `JSON` values:

```sql
SELECT JSON_EXTRACT_ARRAY(
  JSON '{"fruits":["apples","oranges","grapes"]}','$.fruits'
  ) AS json_array;

/*---------------------------------+
 | json_array                      |
 +---------------------------------+
 | ["apples", "oranges", "grapes"] |
 +---------------------------------*/
```

This extracts the items in a JSON-formatted string to a string array:

```sql
SELECT JSON_EXTRACT_ARRAY('[1,2,3]') AS string_array;

/*--------------+
 | string_array |
 +--------------+
 | [1, 2, 3]    |
 +--------------*/
```

This extracts a string array and converts it to an integer array:

```sql
SELECT ARRAY(
  SELECT CAST(integer_element AS BIGINT)
  FROM UNNEST(
    JSON_EXTRACT_ARRAY('[1,2,3]','$')
  ) AS integer_element
) AS integer_array;

/*---------------+
 | integer_array |
 +---------------+
 | [1, 2, 3]     |
 +---------------*/
```

This extracts string values in a JSON-formatted string to an array:

```sql
-- Doesn't strip the double quotes
SELECT JSON_EXTRACT_ARRAY('["apples", "oranges", "grapes"]', '$') AS string_array;

/*---------------------------------+
 | string_array                    |
 +---------------------------------+
 | ["apples", "oranges", "grapes"] |
 +---------------------------------*/

-- Strips the double quotes
SELECT ARRAY(
  SELECT JSON_EXTRACT_SCALAR(string_element, '$')
  FROM UNNEST(JSON_EXTRACT_ARRAY('["apples","oranges","grapes"]','$')) AS string_element
) AS string_array;

/*---------------------------+
 | string_array              |
 +---------------------------+
 | [apples, oranges, grapes] |
 +---------------------------*/
```

This extracts only the items in the `fruit` property to an array:

```sql
SELECT JSON_EXTRACT_ARRAY(
  '{"fruit": [{"apples": 5, "oranges": 10}, {"apples": 2, "oranges": 4}], "vegetables": [{"lettuce": 7, "kale": 8}]}',
  '$.fruit'
) AS string_array;

/*-------------------------------------------------------+
 | string_array                                          |
 +-------------------------------------------------------+
 | [{"apples":5,"oranges":10}, {"apples":2,"oranges":4}] |
 +-------------------------------------------------------*/
```

These are equivalent:

```sql
SELECT JSON_EXTRACT_ARRAY('{"fruits": ["apples", "oranges", "grapes"]}', '$[fruits]') AS string_array;

SELECT JSON_EXTRACT_ARRAY('{"fruits": ["apples", "oranges", "grapes"]}', '$.fruits') AS string_array;

-- The queries above produce the following result:
/*---------------------------------+
 | string_array                    |
 +---------------------------------+
 | ["apples", "oranges", "grapes"] |
 +---------------------------------*/
```

In cases where a JSON key uses invalid JSONPath characters, you can escape those
characters using single quotes and brackets, `[' ']`. For example:

```sql
SELECT JSON_EXTRACT_ARRAY('{"a.b": {"c": ["world"]}}', "$['a.b'].c") AS hello;

/*-----------+
 | hello     |
 +-----------+
 | ["world"] |
 +-----------*/
```

The following examples explore how invalid requests and empty arrays are
handled:

- If a JSONPath is invalid, an error is thrown.
- If a JSON-formatted string is invalid, the output is NULL.
- It's okay to have empty arrays in the JSON-formatted string.

```sql
-- An error is thrown if you provide an invalid JSONPath.
SELECT JSON_EXTRACT_ARRAY('["foo", "bar", "baz"]', 'INVALID_JSONPath') AS result;

-- If the JSONPath doesn't refer to an array, then NULL is returned.
SELECT JSON_EXTRACT_ARRAY('{"a": "foo"}', '$.a') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a key that doesn't exist is specified, then the result is NULL.
SELECT JSON_EXTRACT_ARRAY('{"a": "foo"}', '$.b') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- Empty arrays in JSON-formatted strings are supported.
SELECT JSON_EXTRACT_ARRAY('{"a": "foo", "b": []}', '$.b') AS result;

/*--------+
 | result |
 +--------+
 | []     |
 +--------*/
```

[json-query-array]: #json-query_array
[JSONPath-format]: #JSONPath_format
[differences-json-and-string]: #differences-json_and_string

## `JSON_EXTRACT_SCALAR`

Note: This function is deprecated. Consider using [JSON_VALUE][json-value].

```sql
JSON_EXTRACT_SCALAR(json_string_expr[, json_path])
```

```sql
JSON_EXTRACT_SCALAR(json_expr[, json_path])
```

**Description**

Extracts a JSON scalar value and converts it to a SQL `VARCHAR` value.
In addition, this function:

- Removes the outermost quotes and unescapes the return values.
- Returns a SQL `NULL` if a non-scalar value is selected.
- Uses single quotes and brackets to escape invalid [JSONPath][JSONPath-format]
  characters in JSON keys. For example: `['a.b']`.

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ````sql
  '{"name": "Jane", "age": "6"}'
  ```sql
  ````

- `json_expr`: JSON. For example:

  ````sql
  JSON '{"name": "Jane", "age": "6"}'
  ```sql
  ````

- `json_path`: The [JSONPath][JSONPath-format]. This identifies the data that
  you want to obtain from the input. If this optional parameter isn't
  provided, then the JSONPath `$` symbol is applied, which means that all of
  the data is analyzed.

  If `json_path` returns a JSON `null` or a non-scalar value (in other words,
  if `json_path` refers to an object or an array), then a SQL `NULL` is
  returned.

There are differences between the JSON-formatted string and JSON input types.
For details, see [Differences between the JSON and JSON-formatted VARCHAR types][differences-json-and-string].

**Return type**

`VARCHAR`

**Examples**

In the following example, `age` is extracted.

```sql
SELECT JSON_EXTRACT_SCALAR(JSON '{"name": "Jakob", "age": "6" }', '$.age') AS scalar_age;

/*------------+
 | scalar_age |
 +------------+
 | 6          |
 +------------*/
```

The following example compares how results are returned for the `JSON_EXTRACT`
and `JSON_EXTRACT_SCALAR` functions.

```sql
SELECT JSON_EXTRACT('{"name": "Jakob", "age": "6" }', '$.name') AS json_name,
  JSON_EXTRACT_SCALAR('{"name": "Jakob", "age": "6" }', '$.name') AS scalar_name,
  JSON_EXTRACT('{"name": "Jakob", "age": "6" }', '$.age') AS json_age,
  JSON_EXTRACT_SCALAR('{"name": "Jakob", "age": "6" }', '$.age') AS scalar_age;

/*-----------+-------------+----------+------------+
 | json_name | scalar_name | json_age | scalar_age |
 +-----------+-------------+----------+------------+
 | "Jakob"   | Jakob       | "6"      | 6          |
 +-----------+-------------+----------+------------*/
```

```sql
SELECT JSON_EXTRACT('{"fruits": ["apple", "banana"]}', '$.fruits') AS json_extract,
  JSON_EXTRACT_SCALAR('{"fruits": ["apple", "banana"]}', '$.fruits') AS json_extract_scalar;

/*--------------------+---------------------+
 | json_extract       | json_extract_scalar |
 +--------------------+---------------------+
 | ["apple","banana"] | NULL                |
 +--------------------+---------------------*/
```

In cases where a JSON key uses invalid JSONPath characters, you can escape those
characters using single quotes and brackets, `[' ']`. For example:

```sql
SELECT JSON_EXTRACT_SCALAR('{"a.b": {"c": "world"}}', "$['a.b'].c") AS hello;

/*-------+
 | hello |
 +-------+
 | world |
 +-------*/
```

[json-value]: #json-value
[JSONPath-format]: #JSONPath_format
[differences-json-and-string]: #differences-json_and_string

## `JSON_EXTRACT_STRING_ARRAY`

Note: This function is deprecated. Consider using
[JSON_VALUE_ARRAY][json-value-array].

```sql
JSON_EXTRACT_STRING_ARRAY(json_string_expr[, json_path])
```

```sql
JSON_EXTRACT_STRING_ARRAY(json_expr[, json_path])
```

**Description**

Extracts a JSON array of scalar values and converts it to a SQL `ARRAY<VARCHAR>`
value. In addition, this function:

- Removes the outermost quotes and unescapes the values.
- Returns a SQL `NULL` if the selected value isn't an array or
  not an array containing only scalar values.
- Uses single quotes and brackets to escape invalid [JSONPath][JSONPath-format]
  characters in JSON keys. For example: `['a.b']`.

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ````sql
  '["apples", "oranges", "grapes"]'
  ```sql
  ````

- `json_expr`: JSON. For example:

  ````sql
  JSON '["apples", "oranges", "grapes"]'
  ```sql
  ````

- `json_path`: The [JSONPath][JSONPath-format]. This identifies the data that
  you want to obtain from the input. If this optional parameter isn't
  provided, then the JSONPath `$` symbol is applied, which means that all of
  the data is analyzed.

There are differences between the JSON-formatted string and JSON input types.
For details, see [Differences between the JSON and JSON-formatted VARCHAR types][differences-json-and-string].

Caveats:

- A JSON `null` in the input array produces a SQL `NULL` as the output for that
  JSON `null`.
- If a JSONPath matches an array that contains scalar objects and a JSON `null`,
  then the output is an array of the scalar objects and a SQL `NULL`.

**Return type**

`ARRAY<VARCHAR>`

**Examples**

This extracts items in JSON to a string array:

```sql
SELECT JSON_EXTRACT_STRING_ARRAY(
  JSON '{"fruits": ["apples", "oranges", "grapes"]}', '$.fruits'
  ) AS string_array;

/*---------------------------+
 | string_array              |
 +---------------------------+
 | [apples, oranges, grapes] |
 +---------------------------*/
```

The following example compares how results are returned for the
`JSON_EXTRACT_ARRAY` and `JSON_EXTRACT_STRING_ARRAY` functions.

```sql
SELECT JSON_EXTRACT_ARRAY('["apples", "oranges"]') AS json_array,
JSON_EXTRACT_STRING_ARRAY('["apples", "oranges"]') AS string_array;

/*-----------------------+-------------------+
 | json_array            | string_array      |
 +-----------------------+-------------------+
 | ["apples", "oranges"] | [apples, oranges] |
 +-----------------------+-------------------*/
```

This extracts the items in a JSON-formatted string to a string array:

```sql
-- Strips the double quotes
SELECT JSON_EXTRACT_STRING_ARRAY('["foo", "bar", "baz"]', '$') AS string_array;

/*-----------------+
 | string_array    |
 +-----------------+
 | [foo, bar, baz] |
 +-----------------*/
```

This extracts a string array and converts it to an integer array:

```sql
SELECT ARRAY(
  SELECT CAST(integer_element AS BIGINT)
  FROM UNNEST(
    JSON_EXTRACT_STRING_ARRAY('[1, 2, 3]', '$')
  ) AS integer_element
) AS integer_array;

/*---------------+
 | integer_array |
 +---------------+
 | [1, 2, 3]     |
 +---------------*/
```

These are equivalent:

```sql
SELECT JSON_EXTRACT_STRING_ARRAY('{"fruits": ["apples", "oranges", "grapes"]}', '$[fruits]') AS string_array;

SELECT JSON_EXTRACT_STRING_ARRAY('{"fruits": ["apples", "oranges", "grapes"]}', '$.fruits') AS string_array;

-- The queries above produce the following result:
/*---------------------------+
 | string_array              |
 +---------------------------+
 | [apples, oranges, grapes] |
 +---------------------------*/
```

In cases where a JSON key uses invalid JSONPath characters, you can escape those
characters using single quotes and brackets: `[' ']`. For example:

```sql
SELECT JSON_EXTRACT_STRING_ARRAY('{"a.b": {"c": ["world"]}}', "$['a.b'].c") AS hello;

/*---------+
 | hello   |
 +---------+
 | [world] |
 +---------*/
```

The following examples explore how invalid requests and empty arrays are
handled:

```sql
-- An error is thrown if you provide an invalid JSONPath.
SELECT JSON_EXTRACT_STRING_ARRAY('["foo", "bar", "baz"]', 'INVALID_JSONPath') AS result;

-- If the JSON formatted string is invalid, then NULL is returned.
SELECT JSON_EXTRACT_STRING_ARRAY('}}', '$') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If the JSON document is NULL, then NULL is returned.
SELECT JSON_EXTRACT_STRING_ARRAY(NULL, '$') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath doesn't match anything, then the output is NULL.
SELECT JSON_EXTRACT_STRING_ARRAY('{"a": ["foo", "bar", "baz"]}', '$.b') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath matches an object that isn't an array, then the output is NULL.
SELECT JSON_EXTRACT_STRING_ARRAY('{"a": "foo"}', '$') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath matches an array of non-scalar objects, then the output is NULL.
SELECT JSON_EXTRACT_STRING_ARRAY('{"a": [{"b": "foo", "c": 1}, {"b": "bar", "c":2}], "d": "baz"}', '$.a') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath matches an array of mixed scalar and non-scalar objects, then the output is NULL.
SELECT JSON_EXTRACT_STRING_ARRAY('{"a": [10, {"b": 20}]', '$.a') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath matches an empty JSON array, then the output is an empty array instead of NULL.
SELECT JSON_EXTRACT_STRING_ARRAY('{"a": "foo", "b": []}', '$.b') AS result;

/*--------+
 | result |
 +--------+
 | []     |
 +--------*/

-- In the following query, the JSON null input is returned as a
-- SQL NULL in the output.
SELECT JSON_EXTRACT_STRING_ARRAY('["world", 1, null]') AS result;

/*------------------+
 | result           |
 +------------------+
 | [world, 1, NULL] |
 +------------------*/

```

[json-value-array]: #json-value_array
[JSONPath-format]: #JSONPath_format
[differences-json-and-string]: #differences-json_and_string

## `JSON_QUERY`

```sql
JSON_QUERY(json_string_expr, json_path)
```

```sql
JSON_QUERY(json_expr, json_path)
```

**Description**

Extracts a JSON value and converts it to a SQL
JSON-formatted `VARCHAR` or
`JSON` value.
This function uses double quotes to escape invalid
[JSONPath][JSONPath-format] characters in JSON keys. For example: `"a.b"`.

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ```sql
  '{"class": {"students": [{"name": "Jane"}]}}'
  ```

  Extracts a SQL `NULL` when a JSON-formatted string `null` is encountered.
  For example:

  ````sql
  SELECT JSON_QUERY("null", "$") -- Returns a SQL NULL
  ```sql
  ````

- `json_expr`: JSON. For example:

  ```sql
  JSON '{"class": {"students": [{"name": "Jane"}]}}'
  ```

  Extracts a JSON `null` when a JSON `null` is encountered.

  ````sql
  SELECT JSON_QUERY(JSON 'null', "$") -- Returns a JSON 'null'
  ```sql
  ````

- `json_path`: The [JSONPath][JSONPath-format]. This identifies the data that
  you want to obtain from the input.

There are differences between the JSON-formatted string and JSON input types.
For details, see [Differences between the JSON and JSON-formatted VARCHAR types][differences-json-and-string].

**Return type**

- `json_string_expr`: A JSON-formatted `VARCHAR`
- `json_expr`: `JSON`

**Examples**

In the following example, JSON data is extracted and returned as JSON.

```sql
SELECT
  JSON_QUERY(
    JSON '{"class": {"students": [{"id": 5}, {"id": 12}]}}',
    '$.class') AS json_data;

/*-----------------------------------+
 | json_data                         |
 +-----------------------------------+
 | {"students":[{"id":5},{"id":12}]} |
 +-----------------------------------*/
```

In the following examples, JSON data is extracted and returned as
JSON-formatted strings.

```sql
SELECT
  JSON_QUERY('{"class": {"students": [{"name": "Jane"}]}}', '$') AS json_text_string;

/*-----------------------------------------------------------+
 | json_text_string                                          |
 +-----------------------------------------------------------+
 | {"class":{"students":[{"name":"Jane"}]}}                  |
 +-----------------------------------------------------------*/
```

```sql
SELECT JSON_QUERY('{"class": {"students": []}}', '$') AS json_text_string;

/*-----------------------------------------------------------+
 | json_text_string                                          |
 +-----------------------------------------------------------+
 | {"class":{"students":[]}}                                 |
 +-----------------------------------------------------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": [{"name": "John"},{"name": "Jamie"}]}}',
    '$') AS json_text_string;

/*-----------------------------------------------------------+
 | json_text_string                                          |
 +-----------------------------------------------------------+
 | {"class":{"students":[{"name":"John"},{"name":"Jamie"}]}} |
 +-----------------------------------------------------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": [{"name": "Jane"}]}}',
    '$.class.students[0]') AS first_student;

/*-----------------+
 | first_student   |
 +-----------------+
 | {"name":"Jane"} |
 +-----------------*/
```

```sql
SELECT
  JSON_QUERY('{"class": {"students": []}}', '$.class.students[0]') AS first_student;

/*-----------------+
 | first_student   |
 +-----------------+
 | NULL            |
 +-----------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": [{"name": "John"}, {"name": "Jamie"}]}}',
    '$.class.students[0]') AS first_student;

/*-----------------+
 | first_student   |
 +-----------------+
 | {"name":"John"} |
 +-----------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": [{"name": "Jane"}]}}',
    '$.class.students[1].name') AS second_student;

/*----------------+
 | second_student |
 +----------------+
 | NULL           |
 +----------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": []}}',
    '$.class.students[1].name') AS second_student;

/*----------------+
 | second_student |
 +----------------+
 | NULL           |
 +----------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": [{"name": "John"}, {"name": null}]}}',
    '$.class.students[1].name') AS second_student;

/*----------------+
 | second_student |
 +----------------+
 | NULL           |
 +----------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": [{"name": "John"}, {"name": "Jamie"}]}}',
    '$.class.students[1].name') AS second_student;

/*----------------+
 | second_student |
 +----------------+
 | "Jamie"        |
 +----------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": [{"name": "Jane"}]}}',
    '$.class."students"') AS student_names;

/*------------------------------------+
 | student_names                      |
 +------------------------------------+
 | [{"name":"Jane"}]                  |
 +------------------------------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": []}}',
    '$.class."students"') AS student_names;

/*------------------------------------+
 | student_names                      |
 +------------------------------------+
 | []                                 |
 +------------------------------------*/
```

```sql
SELECT
  JSON_QUERY(
    '{"class": {"students": [{"name": "John"}, {"name": "Jamie"}]}}',
    '$.class."students"') AS student_names;

/*------------------------------------+
 | student_names                      |
 +------------------------------------+
 | [{"name":"John"},{"name":"Jamie"}] |
 +------------------------------------*/
```

```sql
SELECT JSON_QUERY('{"a": null}', "$.a"); -- Returns a SQL NULL
SELECT JSON_QUERY('{"a": null}', "$.b"); -- Returns a SQL NULL
```

```sql
SELECT JSON_QUERY(JSON '{"a": null}', "$.a"); -- Returns a JSON 'null'
SELECT JSON_QUERY(JSON '{"a": null}', "$.b"); -- Returns a SQL NULL
```

[JSONPath-format]: #JSONPath_format
[differences-json-and-string]: #differences-json_and_string
[JSONPath-mode]: #JSONPath_mode

## `JSON_QUERY_ARRAY`

```sql
JSON_QUERY_ARRAY(json_string_expr[, json_path])
```

```sql
JSON_QUERY_ARRAY(json_expr[, json_path])
```

**Description**

Extracts a JSON array and converts it to
a SQL `ARRAY<JSON-formatted VARCHAR>` or
`ARRAY<JSON>` value.
In addition, this function uses double quotes to escape invalid
[JSONPath][JSONPath-format] characters in JSON keys. For example: `"a.b"`.

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ````sql
  '["a", "b", {"key": "c"}]'
  ```sql
  ````

- `json_expr`: JSON. For example:

  ````sql
  JSON '["a", "b", {"key": "c"}]'
  ```sql
  ````

- `json_path`: The [JSONPath][JSONPath-format]. This identifies the data that
  you want to obtain from the input. If this optional parameter isn't
  provided, then the JSONPath `$` symbol is applied, which means that all of
  the data is analyzed.

There are differences between the JSON-formatted string and JSON input types.
For details, see [Differences between the JSON and JSON-formatted VARCHAR types][differences-json-and-string].

**Return type**

- `json_string_expr`: `ARRAY<JSON-formatted VARCHAR>`
- `json_expr`: `ARRAY<JSON>`

**Examples**

This extracts items in JSON to an array of `JSON` values:

```sql
SELECT JSON_QUERY_ARRAY(
  JSON '{"fruits": ["apples", "oranges", "grapes"]}', '$.fruits'
  ) AS json_array;

/*---------------------------------+
 | json_array                      |
 +---------------------------------+
 | ["apples", "oranges", "grapes"] |
 +---------------------------------*/
```

This extracts the items in a JSON-formatted string to a string array:

```sql
SELECT JSON_QUERY_ARRAY('[1, 2, 3]') AS string_array;

/*--------------+
 | string_array |
 +--------------+
 | [1, 2, 3]    |
 +--------------*/
```

This extracts a string array and converts it to an integer array:

```sql
SELECT ARRAY(
  SELECT CAST(integer_element AS BIGINT)
  FROM UNNEST(
    JSON_QUERY_ARRAY('[1, 2, 3]','$')
  ) AS integer_element
) AS integer_array;

/*---------------+
 | integer_array |
 +---------------+
 | [1, 2, 3]     |
 +---------------*/
```

This extracts string values in a JSON-formatted string to an array:

```sql
-- Doesn't strip the double quotes
SELECT JSON_QUERY_ARRAY('["apples", "oranges", "grapes"]', '$') AS string_array;

/*---------------------------------+
 | string_array                    |
 +---------------------------------+
 | ["apples", "oranges", "grapes"] |
 +---------------------------------*/
```

```sql
-- Strips the double quotes
SELECT ARRAY(
  SELECT JSON_VALUE(string_element, '$')
  FROM UNNEST(JSON_QUERY_ARRAY('["apples", "oranges", "grapes"]', '$')) AS string_element
) AS string_array;

/*---------------------------+
 | string_array              |
 +---------------------------+
 | [apples, oranges, grapes] |
 +---------------------------*/
```

This extracts only the items in the `fruit` property to an array:

```sql
SELECT JSON_QUERY_ARRAY(
  '{"fruit": [{"apples": 5, "oranges": 10}, {"apples": 2, "oranges": 4}], "vegetables": [{"lettuce": 7, "kale": 8}]}',
  '$.fruit'
) AS string_array;

/*-------------------------------------------------------+
 | string_array                                          |
 +-------------------------------------------------------+
 | [{"apples":5,"oranges":10}, {"apples":2,"oranges":4}] |
 +-------------------------------------------------------*/
```

These are equivalent:

```sql
SELECT JSON_QUERY_ARRAY('{"fruits": ["apples", "oranges", "grapes"]}', '$.fruits') AS string_array;

SELECT JSON_QUERY_ARRAY('{"fruits": ["apples", "oranges", "grapes"]}', '$."fruits"') AS string_array;

-- The queries above produce the following result:
/*---------------------------------+
 | string_array                    |
 +---------------------------------+
 | ["apples", "oranges", "grapes"] |
 +---------------------------------*/
```

In cases where a JSON key uses invalid JSONPath characters, you can escape those
characters using double quotes: `" "`. For example:

```sql
SELECT JSON_QUERY_ARRAY('{"a.b": {"c": ["world"]}}', '$."a.b".c') AS hello;

/*-----------+
 | hello     |
 +-----------+
 | ["world"] |
 +-----------*/
```

The following examples show how invalid requests and empty arrays are handled:

```sql
-- An error is returned if you provide an invalid JSONPath.
SELECT JSON_QUERY_ARRAY('["foo", "bar", "baz"]', 'INVALID_JSONPath') AS result;

-- If the JSONPath doesn't refer to an array, then NULL is returned.
SELECT JSON_QUERY_ARRAY('{"a": "foo"}', '$.a') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a key that doesn't exist is specified, then the result is NULL.
SELECT JSON_QUERY_ARRAY('{"a": "foo"}', '$.b') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- Empty arrays in JSON-formatted strings are supported.
SELECT JSON_QUERY_ARRAY('{"a": "foo", "b": []}', '$.b') AS result;

/*--------+
 | result |
 +--------+
 | []     |
 +--------*/
```

[JSONPath-format]: #JSONPath_format
[differences-json-and-string]: #differences-json_and_string

## `JSON_VALUE`

```sql
JSON_VALUE(json_string_expr[, json_path])
```

```sql
JSON_VALUE(json_expr[, json_path])
```

**Description**

Extracts a JSON scalar value and converts it to a SQL `VARCHAR` value.
In addition, this function:

- Removes the outermost quotes and unescapes the values.
- Returns a SQL `NULL` if a non-scalar value is selected.
- Uses double quotes to escape invalid [JSONPath][JSONPath-format] characters
  in JSON keys. For example: `"a.b"`.

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ````sql
  '{"name": "Jakob", "age": "6"}'
  ```sql
  ````

- `json_expr`: JSON. For example:

  ````sql
  JSON '{"name": "Jane", "age": "6"}'
  ```sql
  ````

- `json_path`: The [JSONPath][JSONPath-format]. This identifies the data that
  you want to obtain from the input. If this optional parameter isn't
  provided, then the JSONPath `$` symbol is applied, which means that all of
  the data is analyzed.

  If `json_path` returns a JSON `null` or a non-scalar value (in other words,
  if `json_path` refers to an object or an array), then a SQL `NULL` is
  returned.

There are differences between the JSON-formatted string and JSON input types.
For details, see [Differences between the JSON and JSON-formatted VARCHAR types][differences-json-and-string].

**Return type**

`VARCHAR`

**Examples**

In the following example, JSON data is extracted and returned as a scalar value.

```sql
SELECT JSON_VALUE(JSON '{"name": "Jakob", "age": "6" }', '$.age') AS scalar_age;

/*------------+
 | scalar_age |
 +------------+
 | 6          |
 +------------*/
```

The following example compares how results are returned for the `JSON_QUERY`
and `JSON_VALUE` functions.

```sql
SELECT JSON_QUERY('{"name": "Jakob", "age": "6"}', '$.name') AS json_name,
  JSON_VALUE('{"name": "Jakob", "age": "6"}', '$.name') AS scalar_name,
  JSON_QUERY('{"name": "Jakob", "age": "6"}', '$.age') AS json_age,
  JSON_VALUE('{"name": "Jakob", "age": "6"}', '$.age') AS scalar_age;

/*-----------+-------------+----------+------------+
 | json_name | scalar_name | json_age | scalar_age |
 +-----------+-------------+----------+------------+
 | "Jakob"   | Jakob       | "6"      | 6          |
 +-----------+-------------+----------+------------*/
```

```sql
SELECT JSON_QUERY('{"fruits": ["apple", "banana"]}', '$.fruits') AS json_query,
  JSON_VALUE('{"fruits": ["apple", "banana"]}', '$.fruits') AS json_value;

/*--------------------+------------+
 | json_query         | json_value |
 +--------------------+------------+
 | ["apple","banana"] | NULL       |
 +--------------------+------------*/
```

In cases where a JSON key uses invalid JSONPath characters, you can escape those
characters using double quotes. For example:

```sql
SELECT JSON_VALUE('{"a.b": {"c": "world"}}', '$."a.b".c') AS hello;

/*-------+
 | hello |
 +-------+
 | world |
 +-------*/
```

[JSONPath-format]: #JSONPath_format
[differences-json-and-string]: #differences-json_and_string

## `JSON_VALUE_ARRAY`

```sql
JSON_VALUE_ARRAY(json_string_expr[, json_path])
```

```sql
JSON_VALUE_ARRAY(json_expr[, json_path])
```

**Description**

Extracts a JSON array of scalar values and converts it to a SQL
`ARRAY<VARCHAR>` value.
In addition, this function:

- Removes the outermost quotes and unescapes the values.
- Returns a SQL `NULL` if the selected value isn't an array or
  not an array containing only scalar values.
- Uses double quotes to escape invalid [JSONPath][JSONPath-format] characters
  in JSON keys. For example: `"a.b"`.

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ````sql
  '["apples", "oranges", "grapes"]'
  ```sql
  ````

- `json_expr`: JSON. For example:

  ````sql
  JSON '["apples", "oranges", "grapes"]'
  ```sql
  ````

- `json_path`: The [JSONPath][JSONPath-format]. This identifies the data that
  you want to obtain from the input. If this optional parameter isn't
  provided, then the JSONPath `$` symbol is applied, which means that all of
  the data is analyzed.

There are differences between the JSON-formatted string and JSON input types.
For details, see [Differences between the JSON and JSON-formatted VARCHAR types][differences-json-and-string].

Caveats:

- A JSON `null` in the input array produces a SQL `NULL` as the output for that
  JSON `null`.
- If a JSONPath matches an array that contains scalar objects and a JSON `null`,
  then the output is an array of the scalar objects and a SQL `NULL`.

**Return type**

`ARRAY<VARCHAR>`

**Examples**

This extracts items in JSON to a string array:

```sql
SELECT JSON_VALUE_ARRAY(
  JSON '{"fruits": ["apples", "oranges", "grapes"]}', '$.fruits'
  ) AS string_array;

/*---------------------------+
 | string_array              |
 +---------------------------+
 | [apples, oranges, grapes] |
 +---------------------------*/
```

The following example compares how results are returned for the
`JSON_QUERY_ARRAY` and `JSON_VALUE_ARRAY` functions.

```sql
SELECT JSON_QUERY_ARRAY('["apples", "oranges"]') AS json_array,
       JSON_VALUE_ARRAY('["apples", "oranges"]') AS string_array;

/*-----------------------+-------------------+
 | json_array            | string_array      |
 +-----------------------+-------------------+
 | ["apples", "oranges"] | [apples, oranges] |
 +-----------------------+-------------------*/
```

This extracts the items in a JSON-formatted string to a string array:

```sql
-- Strips the double quotes
SELECT JSON_VALUE_ARRAY('["foo", "bar", "baz"]', '$') AS string_array;

/*-----------------+
 | string_array    |
 +-----------------+
 | [foo, bar, baz] |
 +-----------------*/
```

This extracts a string array and converts it to an integer array:

```sql
SELECT ARRAY(
  SELECT CAST(integer_element AS BIGINT)
  FROM UNNEST(
    JSON_VALUE_ARRAY('[1, 2, 3]', '$')
  ) AS integer_element
) AS integer_array;

/*---------------+
 | integer_array |
 +---------------+
 | [1, 2, 3]     |
 +---------------*/
```

These are equivalent:

```sql
SELECT JSON_VALUE_ARRAY('{"fruits": ["apples", "oranges", "grapes"]}', '$.fruits') AS string_array;
SELECT JSON_VALUE_ARRAY('{"fruits": ["apples", "oranges", "grapes"]}', '$."fruits"') AS string_array;

-- The queries above produce the following result:
/*---------------------------+
 | string_array              |
 +---------------------------+
 | [apples, oranges, grapes] |
 +---------------------------*/
```

In cases where a JSON key uses invalid JSONPath characters, you can escape those
characters using double quotes: `" "`. For example:

```sql
SELECT JSON_VALUE_ARRAY('{"a.b": {"c": ["world"]}}', '$."a.b".c') AS hello;

/*---------+
 | hello   |
 +---------+
 | [world] |
 +---------*/
```

The following examples explore how invalid requests and empty arrays are
handled:

```sql
-- An error is thrown if you provide an invalid JSONPath.
SELECT JSON_VALUE_ARRAY('["foo", "bar", "baz"]', 'INVALID_JSONPath') AS result;

-- If the JSON-formatted string is invalid, then NULL is returned.
SELECT JSON_VALUE_ARRAY('}}', '$') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If the JSON document is NULL, then NULL is returned.
SELECT JSON_VALUE_ARRAY(NULL, '$') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath doesn't match anything, then the output is NULL.
SELECT JSON_VALUE_ARRAY('{"a": ["foo", "bar", "baz"]}', '$.b') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath matches an object that isn't an array, then the output is NULL.
SELECT JSON_VALUE_ARRAY('{"a": "foo"}', '$') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath matches an array of non-scalar objects, then the output is NULL.
SELECT JSON_VALUE_ARRAY('{"a": [{"b": "foo", "c": 1}, {"b": "bar", "c": 2}], "d": "baz"}', '$.a') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath matches an array of mixed scalar and non-scalar objects,
-- then the output is NULL.
SELECT JSON_VALUE_ARRAY('{"a": [10, {"b": 20}]', '$.a') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/

-- If a JSONPath matches an empty JSON array, then the output is an empty array instead of NULL.
SELECT JSON_VALUE_ARRAY('{"a": "foo", "b": []}', '$.b') AS result;

/*--------+
 | result |
 +--------+
 | []     |
 +--------*/

-- In the following query, the JSON null input is returned as a
-- SQL NULL in the output.
SELECT JSON_VALUE_ARRAY('["world", null, 1]') AS result;

/*------------------+
 | result           |
 +------------------+
 | [world, NULL, 1] |
 +------------------*/

```

[JSONPath-format]: #JSONPath_format
[differences-json-and-string]: #differences-json_and_string
