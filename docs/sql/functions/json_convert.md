# JSON Conversion Functions

Functions for converting between JSON and SQL types.

## `BOOL`

```sql
BOOL(json_expr)
```

**Description**

Converts a JSON boolean to a SQL `BOOL` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON 'true'
  ```

  If the JSON value isn't a boolean, an error is produced. If the expression
  is SQL `NULL`, the function returns SQL `NULL`.

**Return type**

`BOOL`

**Examples**

```sql
SELECT BOOL(JSON 'true') AS vacancy;

/*---------+
 | vacancy |
 +---------+
 | true    |
 +---------*/
```

```sql
SELECT BOOL(JSON_QUERY(JSON '{"hotel class": "5-star", "vacancy": true}', "$.vacancy")) AS vacancy;

/*---------+
 | vacancy |
 +---------+
 | true    |
 +---------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if JSON isn't of type bool.
SELECT BOOL(JSON '123') AS result; -- Throws an error
SELECT BOOL(JSON 'null') AS result; -- Throws an error
SELECT SAFE.BOOL(JSON '123') AS result; -- Returns a SQL NULL
```

## `BOOL_ARRAY`

```sql
BOOL_ARRAY(json_expr)
```

**Description**

Converts a JSON array of booleans to a SQL `ARRAY<BOOL>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[true]'
  ```

  If the JSON value isn't an array of booleans, an error is produced. If the
  expression is SQL `NULL`, the function returns SQL `NULL`.

**Return type**

`ARRAY<BOOL>`

**Examples**

```sql
SELECT BOOL_ARRAY(JSON '[true, false]') AS vacancies;

/*---------------+
 | vacancies     |
 +---------------+
 | [true, false] |
 +---------------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't an array of booleans.
SELECT BOOL_ARRAY(JSON '[123]') AS result; -- Throws an error
SELECT BOOL_ARRAY(JSON '[null]') AS result; -- Throws an error
SELECT BOOL_ARRAY(JSON 'null') AS result; -- Throws an error
```

## `DOUBLE`

```sql
DOUBLE(
  json_expr
  [, wide_number_mode => { 'exact' | 'round' } ]
)
```

**Description**

Converts a JSON number to a SQL `DOUBLE` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '9.8'
  ```

  If the JSON value isn't a number, an error is produced. If the expression
  is a SQL `NULL`, the function returns SQL `NULL`.

- `wide_number_mode`: A named argument with a `VARCHAR` value.
  Defines what happens with a number that can't be
  represented as a `DOUBLE` without loss of
  precision. This argument accepts one of the two case-sensitive values:
  - `exact`: The function fails if the result can't be represented as a
    `DOUBLE` without loss of precision.
  - `round` (default): The numeric value stored in JSON will be rounded to
    `DOUBLE`. If such rounding isn't possible,
    the function fails.

**Return type**

`DOUBLE`

**Examples**

```sql
SELECT DOUBLE(JSON '9.8') AS velocity;

/*----------+
 | velocity |
 +----------+
 | 9.8      |
 +----------*/
```

```sql
SELECT DOUBLE(JSON_QUERY(JSON '{"vo2_max": 39.1, "age": 18}', "$.vo2_max")) AS vo2_max;

/*---------+
 | vo2_max |
 +---------+
 | 39.1    |
 +---------*/
```

```sql
SELECT DOUBLE(JSON '18446744073709551615', wide_number_mode=>'round') as result;

/*------------------------+
 | result                 |
 +------------------------+
 | 1.8446744073709552e+19 |
 +------------------------*/
```

```sql
SELECT DOUBLE(JSON '18446744073709551615') as result;

/*------------------------+
 | result                 |
 +------------------------+
 | 1.8446744073709552e+19 |
 +------------------------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if JSON isn't of type DOUBLE.
SELECT DOUBLE(JSON '"strawberry"') AS result;
SELECT DOUBLE(JSON 'null') AS result;

-- An error is thrown because `wide_number_mode` is case-sensitive and not "exact" or "round".
SELECT DOUBLE(JSON '123.4', wide_number_mode=>'EXACT') as result;
SELECT DOUBLE(JSON '123.4', wide_number_mode=>'exac') as result;

-- An error is thrown because the number can't be converted to DOUBLE without loss of precision
SELECT DOUBLE(JSON '18446744073709551615', wide_number_mode=>'exact') as result;

-- Returns a SQL NULL
SELECT SAFE.DOUBLE(JSON '"strawberry"') AS result;
```

## `DOUBLE_ARRAY`

```sql
DOUBLE_ARRAY(
  json_expr
  [, wide_number_mode => { 'exact' | 'round' } ]
)
```

**Description**

Converts a JSON array of numbers to a SQL `ARRAY<DOUBLE>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[9.8]'
  ```

  If the JSON value isn't an array of numbers, an error is produced. If the
  expression is a SQL `NULL`, the function returns SQL `NULL`.

- `wide_number_mode`: A named argument that takes a `VARCHAR` value. Defines
  what happens with a number that can't be represented as a
  `DOUBLE` without loss of precision. This argument accepts
  one of the two case-sensitive values:
  - `exact`: The function fails if the result can't be represented as a
    `DOUBLE` without loss of precision.
  - `round` (default): The numeric value stored in JSON will be rounded to
    `DOUBLE`. If such rounding isn't possible, the
    function fails.

**Return type**

`ARRAY<DOUBLE>`

**Examples**

```sql
SELECT DOUBLE_ARRAY(JSON '[9, 9.8]') AS velocities;

/*-------------+
 | velocities  |
 +-------------+
 | [9.0, 9.8]  |
 +-------------*/
```

```sql
SELECT DOUBLE_ARRAY(JSON '[18446744073709551615]', wide_number_mode=>'round') as result;

/*--------------------------+
 | result                   |
 +--------------------------+
 | [1.8446744073709552e+19] |
 +--------------------------*/
```

```sql
SELECT DOUBLE_ARRAY(JSON '[18446744073709551615]') as result;

/*--------------------------+
 | result                   |
 +--------------------------+
 | [1.8446744073709552e+19] |
 +--------------------------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't an array of numbers.
SELECT DOUBLE_ARRAY(JSON '["strawberry"]') AS result;
SELECT DOUBLE_ARRAY(JSON '[null]') AS result;
SELECT DOUBLE_ARRAY(JSON 'null') AS result;

-- An error is thrown because `wide_number_mode` is case-sensitive and not "exact" or "round".
SELECT DOUBLE_ARRAY(JSON '[123.4]', wide_number_mode=>'EXACT') as result;
SELECT DOUBLE_ARRAY(JSON '[123.4]', wide_number_mode=>'exac') as result;

-- An error is thrown because the number can't be converted to DOUBLE without loss of precision
SELECT DOUBLE_ARRAY(JSON '[18446744073709551615]', wide_number_mode=>'exact') as result;
```

## `FLOAT`

```sql
FLOAT(
  json_expr
  [, [ wide_number_mode => ] { 'exact' | 'round' } ]
)
```

**Description**

Converts a JSON number to a SQL `FLOAT` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '9.8'
  ```

  If the JSON value isn't a number, an error is produced. If the expression
  is a SQL `NULL`, the function returns SQL `NULL`.

- `wide_number_mode`: A named argument with a `VARCHAR` value. Defines what
  happens with a number that can't be represented as a
  `FLOAT` without loss of precision. This argument accepts
  one of the two case-sensitive values:
  - `exact`: The function fails if the result can't be represented as a
    `FLOAT` without loss of precision.
  - `round` (default): The numeric value stored in JSON will be rounded to
    `FLOAT`. If such rounding isn't possible, the function
    fails.

**Return type**

`FLOAT`

**Examples**

```sql
SELECT FLOAT(JSON '9.8') AS velocity;

/*----------+
 | velocity |
 +----------+
 | 9.8      |
 +----------*/
```

```sql
SELECT FLOAT(JSON_QUERY(JSON '{"vo2_max": 39.1, "age": 18}', "$.vo2_max")) AS vo2_max;

/*---------+
 | vo2_max |
 +---------+
 | 39.1    |
 +---------*/
```

```sql
SELECT FLOAT(JSON '16777217', wide_number_mode=>'round') as result;

/*------------+
 | result     |
 +------------+
 | 16777216.0 |
 +------------*/
```

```sql
SELECT FLOAT(JSON '16777216') as result;

/*------------+
 | result     |
 +------------+
 | 16777216.0 |
 +------------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if JSON isn't of type FLOAT.
SELECT FLOAT(JSON '"strawberry"') AS result;
SELECT FLOAT(JSON 'null') AS result;

-- An error is thrown because `wide_number_mode` is case-sensitive and not "exact" or "round".
SELECT FLOAT(JSON '123.4', wide_number_mode=>'EXACT') as result;
SELECT FLOAT(JSON '123.4', wide_number_mode=>'exac') as result;

-- An error is thrown because the number can't be converted to FLOAT without loss of precision
SELECT FLOAT(JSON '16777217', wide_number_mode=>'exact') as result;

-- Returns a SQL NULL
SELECT SAFE.FLOAT(JSON '"strawberry"') AS result;
```

## `FLOAT_ARRAY`

```sql
FLOAT_ARRAY(
  json_expr
  [, wide_number_mode => { 'exact' | 'round' } ]
)
```

**Description**

Converts a JSON array of numbers to a SQL `ARRAY<FLOAT>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[9.8]'
  ```

  If the JSON value isn't an array of numbers, an error is produced. If the
  expression is a SQL `NULL`, the function returns SQL `NULL`.

- `wide_number_mode`: A named argument with a `VARCHAR` value. Defines
  what happens with a number that can't be represented as a
  `FLOAT` without loss of precision. This argument accepts
  one of the two case-sensitive values:
  - `exact`: The function fails if the result can't be represented as a
    `FLOAT` without loss of precision.
  - `round` (default): The numeric value stored in JSON will be rounded to
    `FLOAT`. If such rounding isn't possible, the function
    fails.

**Return type**

`ARRAY<FLOAT>`

**Examples**

```sql
SELECT FLOAT_ARRAY(JSON '[9, 9.8]') AS velocities;

/*-------------+
 | velocities  |
 +-------------+
 | [9.0, 9.8]  |
 +-------------*/
```

```sql
SELECT FLOAT_ARRAY(JSON '[16777217]', wide_number_mode=>'round') as result;

/*--------------+
 | result       |
 +--------------+
 | [16777216.0] |
 +--------------*/
```

```sql
SELECT FLOAT_ARRAY(JSON '[16777216]') as result;

/*--------------+
 | result       |
 +--------------+
 | [16777216.0] |
 +--------------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't an array of numbers in FLOAT domain.
SELECT FLOAT_ARRAY(JSON '["strawberry"]') AS result;
SELECT FLOAT_ARRAY(JSON '[null]') AS result;
SELECT FLOAT_ARRAY(JSON 'null') AS result;

-- An error is thrown because `wide_number_mode` is case-sensitive and not "exact" or "round".
SELECT FLOAT_ARRAY(JSON '[123.4]', wide_number_mode=>'EXACT') as result;
SELECT FLOAT_ARRAY(JSON '[123.4]', wide_number_mode=>'exac') as result;

-- An error is thrown because the number can't be converted to FLOAT without loss of precision
SELECT FLOAT_ARRAY(JSON '[16777217]', wide_number_mode=>'exact') as result;
```

## `INTEGER`

```sql
INTEGER(json_expr)
```

**Description**

Converts a JSON number to a SQL `INTEGER` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '999'
  ```

  If the JSON value isn't a number, or the JSON number isn't in the SQL
  `INTEGER` domain, an error is produced. If the expression is SQL `NULL`, the
  function returns SQL `NULL`.

**Return type**

`INTEGER`

**Examples**

```sql
SELECT INTEGER(JSON '2005') AS flight_number;

/*---------------+
 | flight_number |
 +---------------+
 | 2005          |
 +---------------*/
```

```sql
SELECT INTEGER(JSON_QUERY(JSON '{"gate": "A4", "flight_number": 2005}', "$.flight_number")) AS flight_number;

/*---------------+
 | flight_number |
 +---------------+
 | 2005          |
 +---------------*/
```

```sql
SELECT INTEGER(JSON '10.0') AS score;

/*-------+
 | score |
 +-------+
 | 10    |
 +-------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if JSON isn't a number or can't be converted to a 64-bit integer.
SELECT INTEGER(JSON '10.1') AS result;  -- Throws an error
SELECT INTEGER(JSON '"strawberry"') AS result; -- Throws an error
SELECT INTEGER(JSON 'null') AS result; -- Throws an error
SELECT SAFE.INTEGER(JSON '"strawberry"') AS result;  -- Returns a SQL NULL
```

## `INTEGER_ARRAY`

```sql
INTEGER_ARRAY(json_expr)
```

**Description**

Converts a JSON number to a SQL `INTEGER_ARRAY` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[999]'
  ```

  If the JSON value isn't an array of numbers, or the JSON numbers aren't in
  the SQL `INTEGER` domain, an error is produced. If the expression is SQL
  `NULL`, the function returns SQL `NULL`.

**Return type**

`ARRAY<INTEGER>`

**Examples**

```sql
SELECT INTEGER_ARRAY(JSON '[2005, 2003]') AS flight_numbers;

/*----------------+
 | flight_numbers |
 +----------------+
 | [2005, 2003]   |
 +----------------*/
```

```sql
SELECT INTEGER_ARRAY(JSON '[10.0]') AS scores;

/*--------+
 | scores |
 +--------+
 | [10]   |
 +--------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't an array of numbers in INTEGER domain.
SELECT INTEGER_ARRAY(JSON '[10.1]') AS result;  -- Throws an error
SELECT INTEGER_ARRAY(JSON '["strawberry"]') AS result; -- Throws an error
SELECT INTEGER_ARRAY(JSON '[null]') AS result; -- Throws an error
SELECT INTEGER_ARRAY(JSON 'null') AS result; -- Throws an error
```

## `BIGINT`

```sql
BIGINT(json_expr)
```

**Description**

Converts a JSON number to a SQL `BIGINT` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '999'
  ```

  If the JSON value isn't a number, or the JSON number isn't in the SQL
  `BIGINT` domain, an error is produced. If the expression is SQL `NULL`, the
  function returns SQL `NULL`.

**Return type**

`BIGINT`

**Examples**

```sql
SELECT BIGINT(JSON '2005') AS flight_number;

/*---------------+
 | flight_number |
 +---------------+
 | 2005          |
 +---------------*/
```

```sql
SELECT BIGINT(JSON_QUERY(JSON '{"gate": "A4", "flight_number": 2005}', "$.flight_number")) AS flight_number;

/*---------------+
 | flight_number |
 +---------------+
 | 2005          |
 +---------------*/
```

```sql
SELECT BIGINT(JSON '10.0') AS score;

/*-------+
 | score |
 +-------+
 | 10    |
 +-------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if JSON isn't a number or can't be converted to a 64-bit integer.
SELECT BIGINT(JSON '10.1') AS result;  -- Throws an error
SELECT BIGINT(JSON '"strawberry"') AS result; -- Throws an error
SELECT BIGINT(JSON 'null') AS result; -- Throws an error
SELECT SAFE.BIGINT(JSON '"strawberry"') AS result;  -- Returns a SQL NULL
```

## `BIGINT_ARRAY`

```sql
BIGINT_ARRAY(json_expr)
```

**Description**

Converts a JSON array of numbers to a SQL `BIGINT_ARRAY` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[999]'
  ```

  If the JSON value isn't an array of numbers, or the JSON numbers aren't in
  the SQL `BIGINT` domain, an error is produced. If the expression is SQL
  `NULL`, the function returns SQL `NULL`.

**Return type**

`ARRAY<BIGINT>`

**Examples**

```sql
SELECT BIGINT_ARRAY(JSON '[2005, 2003]') AS flight_numbers;

/*----------------+
 | flight_numbers |
 +----------------+
 | [2005, 2003]   |
 +----------------*/
```

```sql
SELECT BIGINT_ARRAY(JSON '[10.0]') AS scores;

/*--------+
 | scores |
 +--------+
 | [10]   |
 +--------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't an array of numbers in BIGINT domain.
SELECT BIGINT_ARRAY(JSON '[10.1]') AS result;  -- Throws an error
SELECT BIGINT_ARRAY(JSON '["strawberry"]') AS result; -- Throws an error
SELECT BIGINT_ARRAY(JSON '[null]') AS result; -- Throws an error
SELECT BIGINT_ARRAY(JSON 'null') AS result; -- Throws an error
```

## `LAX_BOOL`

```sql
LAX_BOOL(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `BOOL` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON 'true'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `BOOL`|
| boolean | If the JSON boolean is `true`, returns `TRUE`. Otherwise, returns `FALSE`. |
| string | If the JSON string is `'true'`, returns `TRUE`. If the JSON string is `'false'`, returns `FALSE`. If the JSON string is any other value or has whitespace in it, returns `NULL`. This conversion is case-insensitive. |
| number | If the JSON number is a representation of `0`, returns `FALSE`. Otherwise, returns `TRUE`. |
| other type or null |`NULL`|

**Return type**

`BOOL`

**Examples**

Example with input that's a JSON boolean:

```sql
SELECT LAX_BOOL(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | true   |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_BOOL(JSON '"true"') AS result;

/*--------+
 | result |
 +--------+
 | TRUE   |
 +--------*/
```

```sql
SELECT LAX_BOOL(JSON '"true "') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_BOOL(JSON '"foo"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Examples with inputs that are JSON numbers:

```sql
SELECT LAX_BOOL(JSON '10') AS result;

/*--------+
 | result |
 +--------+
 | TRUE   |
 +--------*/
```

```sql
SELECT LAX_BOOL(JSON '0') AS result;

/*--------+
 | result |
 +--------+
 | FALSE  |
 +--------*/
```

```sql
SELECT LAX_BOOL(JSON '0.0') AS result;

/*--------+
 | result |
 +--------+
 | FALSE  |
 +--------*/
```

```sql
SELECT LAX_BOOL(JSON '-1.1') AS result;

/*--------+
 | result |
 +--------+
 | TRUE   |
 +--------*/
```

## `LAX_BOOL_ARRAY`

```sql
LAX_BOOL_ARRAY(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `ARRAY<BOOL>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[true]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `ARRAY<BOOL>`|
| array | Converts every element according to [`LAX_BOOL`] conversion rules. |
| other type or null |`NULL`|

**Return type**

`ARRAY<BOOL>`

**Examples**

Example with input that's a JSON array of booleans:

```sql
SELECT LAX_BOOL_ARRAY(JSON '[true, false]') AS result;

/*---------------+
 | result        |
 +---------------+
 | [true, false] |
 +---------------*/
```

Examples with inputs that are JSON arrays of strings:

```sql
SELECT LAX_BOOL_ARRAY(JSON '["true", "false", "TRue", "FaLse"]') AS result;

/*----------------------------+
 | result                     |
 +----------------------------+
 | [true, false, true, false] |
 +----------------------------*/
```

```sql
SELECT LAX_BOOL_ARRAY(JSON '["true ", "foo", "null", ""]') AS result;

/*-------------------------+
 | result                  |
 +-------------------------+
 | [NULL, NULL, NULL, NULL |
 +-------------------------*/
```

Examples with input that's JSON array of numbers:

```sql
SELECT LAX_BOOL_ARRAY(JSON '[10, 0, 0.0, -1.1]') AS result;

/*--------------------------+
 | result                   |
 +--------------------------+
 | TRUE, FALSE, FALSE, TRUE |
 +--------------------------*/
```

Example with input that's JSON array of other types:

```sql
SELECT LAX_BOOL_ARRAY(JSON '[null, {"foo": 1}, [1]]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Examples with inputs that aren't JSON arrays:

```sql
SELECT LAX_BOOL_ARRAY(NULL) AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_BOOL_ARRAY(JSON 'null') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_BOOL_ARRAY(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_DOUBLE`

```sql
LAX_DOUBLE(json_expr)
```

**Description**

Attempts to convert a JSON value to a
SQL `DOUBLE` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '9.8'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `DOUBLE`|
| boolean |`NULL`|
| string | If the JSON string represents a JSON number, parses it as a `NUMERIC` value, and then safe casts the result as a `DOUBLE` value. If the JSON string can't be converted, returns `NULL`. |
| number | Casts the JSON number as a `DOUBLE` value. Large JSON numbers are rounded. |
| other type or null |`NULL`|

**Return type**

`DOUBLE`

**Examples**

Examples with inputs that are JSON numbers:

```sql
SELECT LAX_DOUBLE(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | 9.8    |
 +--------*/
```

```sql
SELECT LAX_DOUBLE(JSON '9') AS result;

/*--------+
 | result |
 +--------+
 | 9.0    |
 +--------*/
```

```sql
SELECT LAX_DOUBLE(JSON '9007199254740993') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | 9007199254740992.0 |
 +--------------------*/
```

```sql
SELECT LAX_DOUBLE(JSON '1e100') AS result;

/*--------+
 | result |
 +--------+
 | 1e+100 |
 +--------*/
```

Examples with inputs that are JSON booleans:

```sql
SELECT LAX_DOUBLE(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_DOUBLE(JSON 'false') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_DOUBLE(JSON '"10"') AS result;

/*--------+
 | result |
 +--------+
 | 10.0   |
 +--------*/
```

```sql
SELECT LAX_DOUBLE(JSON '"1.1"') AS result;

/*--------+
 | result |
 +--------+
 | 1.1    |
 +--------*/
```

```sql
SELECT LAX_DOUBLE(JSON '"1.1e2"') AS result;

/*--------+
 | result |
 +--------+
 | 110.0  |
 +--------*/
```

```sql
SELECT LAX_DOUBLE(JSON '"9007199254740993"') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | 9007199254740992.0 |
 +--------------------*/
```

```sql
SELECT LAX_DOUBLE(JSON '"+1.5"') AS result;

/*--------+
 | result |
 +--------+
 | 1.5    |
 +--------*/
```

```sql
SELECT LAX_DOUBLE(JSON '"NaN"') AS result;

/*--------+
 | result |
 +--------+
 | NaN    |
 +--------*/
```

```sql
SELECT LAX_DOUBLE(JSON '"Inf"') AS result;

/*----------+
 | result   |
 +----------+
 | Infinity |
 +----------*/
```

```sql
SELECT LAX_DOUBLE(JSON '"-InfiNiTY"') AS result;

/*-----------+
 | result    |
 +-----------+
 | -Infinity |
 +-----------*/
```

```sql
SELECT LAX_DOUBLE(JSON '"foo"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_DOUBLE_ARRAY`

```sql
LAX_DOUBLE_ARRAY(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `ARRAY<DOUBLE>`
value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[9.8]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `ARRAY<DOUBLE>`|
| array | Converts every element according to [`LAX_DOUBLE`] conversion rules. |
| other type or null |`NULL`|

**Return type**

`ARRAY<DOUBLE>`

**Examples**

Examples with inputs that are JSON arrays of numbers:

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '[9.8, 9]') AS result;

/*-------------+
 | result      |
 +-------------+
 | [9.8, 9.0,] |
 +-------------*/
```

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '[9007199254740993, -9007199254740993]') AS result;

/*-------------------------------------------+
 | result                                    |
 +-------------------------------------------+
 | [9007199254740992.0, -9007199254740992.0] |
 +-------------------------------------------*/
```

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '[-1.79769e+308, 2.22507e-308, 1.79769e+308, 1e100]') AS result;

/*-----------------------------------------------------+
 | result                                              |
 +-----------------------------------------------------+
 | [-1.79769e+308, 2.22507e-308, 1.79769e+308, 1e+100] |
 +-----------------------------------------------------*/
```

Example with inputs that's JSON array of booleans:

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '[true, false]') AS result;

/*----------------+
 | result         |
 +----------------+
 | [NULL, NULL]   |
 +----------------*/
```

Examples with inputs that are JSON arrays of strings:

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '["10", "1.1", "1.1e2", "+1.5"]') AS result;

/*-------------------------+
 | result                  |
 +-------------------------+
 | [10.0, 1.1, 110.0, 1.5] |
 +-------------------------*/
```

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '["9007199254740993"]') AS result;

/*----------------------+
 | result               |
 +----------------------+
 | [9007199254740992.0] |
 +----------------------*/
```

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '["NaN", "Inf", "-InfiNiTY"]') AS result;

/*----------------------------+
 | result                     |
 +----------------------------+
 | [NaN, Infinity, -Infinity] |
 +----------------------------*/
```

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '["foo", "null", ""]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Example with input that's JSON array of other types:

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '[null, {"foo": 1}, [1]]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Examples with inputs that aren't JSON arrays:

```sql
SELECT LAX_DOUBLE_ARRAY(NULL) AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_DOUBLE_ARRAY(JSON 'null') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_DOUBLE_ARRAY(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_FLOAT`

```sql
LAX_FLOAT(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `FLOAT` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '9.8'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `FLOAT`|
| boolean |`NULL`|
| string | If the JSON string represents a JSON number, parses it as a `NUMERIC` value, and then safe casts the result as a `FLOAT` value. If the JSON string can't be converted, returns `NULL`. |
| number | Casts the JSON number as a `FLOAT` value. Large JSON numbers are rounded. |
| other type or null |`NULL`|

**Return type**

`FLOAT`

**Examples**

Examples with inputs that are JSON numbers:

```sql
SELECT LAX_FLOAT(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | 9.8    |
 +--------*/
```

```sql
SELECT LAX_FLOAT(JSON '9') AS result;

/*--------+
 | result |
 +--------+
 | 9.0    |
 +--------*/
```

```sql
SELECT LAX_FLOAT(JSON '16777217') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | 16777216.0         |
 +--------------------*/
```

```sql
SELECT LAX_FLOAT(JSON '1e100') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Examples with inputs that are JSON booleans:

```sql
SELECT LAX_FLOAT(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_FLOAT(JSON 'false') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_FLOAT(JSON '"10"') AS result;

/*--------+
 | result |
 +--------+
 | 10.0   |
 +--------*/
```

```sql
SELECT LAX_FLOAT(JSON '"1.1"') AS result;

/*--------+
 | result |
 +--------+
 | 1.1    |
 +--------*/
```

```sql
SELECT LAX_FLOAT(JSON '"1.1e2"') AS result;

/*--------+
 | result |
 +--------+
 | 110.0  |
 +--------*/
```

```sql
SELECT LAX_FLOAT(JSON '"16777217"') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | 16777216.0         |
 +--------------------*/
```

```sql
SELECT LAX_FLOAT(JSON '"+1.5"') AS result;

/*--------+
 | result |
 +--------+
 | 1.5    |
 +--------*/
```

```sql
SELECT LAX_FLOAT(JSON '"NaN"') AS result;

/*--------+
 | result |
 +--------+
 | NaN    |
 +--------*/
```

```sql
SELECT LAX_FLOAT(JSON '"Inf"') AS result;

/*----------+
 | result   |
 +----------+
 | Infinity |
 +----------*/
```

```sql
SELECT LAX_FLOAT(JSON '"-InfiNiTY"') AS result;

/*-----------+
 | result    |
 +-----------+
 | -Infinity |
 +-----------*/
```

```sql
SELECT LAX_FLOAT(JSON '"foo"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_FLOAT_ARRAY`

```sql
LAX_FLOAT_ARRAY(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `ARRAY<FLOAT>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[9.8, 9]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `ARRAY<FLOAT>`|
| array | Converts every element according to [`LAX_FLOAT_ARRAY`] conversion rules. |
| other type or null |`NULL`|

**Return type**

`ARRAY<FLOAT>`

**Examples**

Examples with inputs that are JSON arrays of numbers:

```sql
SELECT LAX_FLOAT_ARRAY(JSON '[9.8, 9]') AS result;

/*------------+
 | result     |
 +------------+
 | [9.8, 9.0] |
 +------------*/
```

```sql
SELECT LAX_FLOAT_ARRAY(JSON '[16777217, -16777217]') AS result;

/*---------------------------+
 | result                    |
 +---------------------------+
 | [16777216.0, -16777216.0] |
 +---------------------------*/
```

```sql
SELECT LAX_FLOAT_ARRAY(JSON '[-3.40282e+38, 1.17549e-38, 3.40282e+38]') AS result;

/*------------------------------------------+
 | result                                   |
 +------------------------------------------+
 | [-3.40282e+38, 1.17549e-38, 3.40282e+38] |
 +------------------------------------------*/
```

```sql
SELECT LAX_FLOAT_ARRAY(JSON '[-1.79769e+308, 2.22507e-308, 1.79769e+308, 1e100]') AS result;

/*-----------------------+
 | result                |
 +-----------------------+
 | [NULL, 0, NULL, NULL] |
 +-----------------------*/
```

Example with inputs that's JSON array of booleans:

```sql
SELECT LAX_FLOAT_ARRAY(JSON '[true, false]') AS result;

/*----------------+
 | result         |
 +----------------+
 | [NULL, NULL]   |
 +----------------*/
```

Examples with inputs that are JSON arrays of strings:

```sql
SELECT LAX_FLOAT_ARRAY(JSON '["10", "1.1", "1.1e2", "+1.5"]') AS result;

/*-------------------------+
 | result                  |
 +-------------------------+
 | [10.0, 1.1, 110.0, 1.5] |
 +------------------------*/
```

```sql
SELECT LAX_FLOAT_ARRAY(JSON '["16777217"]') AS result;

/*--------------+
 | result       |
 +--------------+
 | [16777216.0] |
 +--------------*/
```

```sql
SELECT LAX_FLOAT_ARRAY(JSON '["NaN", "Inf", "-InfiNiTY"]') AS result;

/*----------------------------+
 | result                     |
 +----------------------------+
 | [NaN, Infinity, -Infinity] |
 +----------------------------*/
```

```sql
SELECT LAX_FLOAT_ARRAY(JSON '["foo", "null", ""]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Example with input that's JSON array of other types:

```sql
SELECT LAX_FLOAT_ARRAY(JSON '[null, {"foo": 1}, [1]]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Examples with inputs that aren't JSON arrays:

```sql
SELECT LAX_FLOAT_ARRAY(NULL) AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_FLOAT_ARRAY(JSON 'null') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_FLOAT_ARRAY(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_INTEGER`

```sql
LAX_INTEGER(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `INTEGER` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '999'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `INTEGER`|
| boolean | If the JSON boolean is `true`, returns `1`. If `false`, returns`0`. |
| string | If the JSON string represents a JSON number, parses it as a `NUMERIC` value, and then safe casts the results as an `INTEGER` value. If the JSON string can't be converted, returns `NULL`. |
| number | Casts the JSON number as an `INTEGER` value. If the JSON number can't be converted, returns `NULL`. |
| other type or null |`NULL`|

**Return type**

`INTEGER`

**Examples**

Examples with inputs that are JSON numbers:

```sql
SELECT LAX_INTEGER(JSON '10') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '10.0') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '1.1') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '3.5') AS result;

/*--------+
 | result |
 +--------+
 | 4      |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '1.1e2') AS result;

/*--------+
 | result |
 +--------+
 | 110    |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '1e100') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Examples with inputs that are JSON booleans:

```sql
SELECT LAX_INTEGER(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON 'false') AS result;

/*--------+
 | result |
 +--------+
 | 0      |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_INTEGER(JSON '"10"') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '"1.1"') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '"1.1e2"') AS result;

/*--------+
 | result |
 +--------+
 | 110    |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '"+1.5"') AS result;

/*--------+
 | result |
 +--------+
 | 2      |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '"1e100"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_INTEGER(JSON '"foo"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_INTEGER_ARRAY`

```sql
LAX_INTEGER_ARRAY(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `ARRAY<INTEGER>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[999, 12]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `ARRAY<INTEGER>`|
| array | Converts every element according to [`LAX_INTEGER`] conversion rules. |
| other type or null |`NULL`|

**Return type**

`ARRAY<INTEGER>`

**Examples**

Examples with inputs that are JSON arrays of numbers:

```sql
SELECT LAX_INTEGER_ARRAY(JSON '[10, 10.0, 1.1, 3.5, 1.1e2]') AS result;

/*---------------------+
 | result              |
 +---------------------+
 | [10, 10, 1, 4, 110] |
 +---------------- ----*/
```

```sql
SELECT LAX_INTEGER_ARRAY(JSON '[1e100]') AS result;

/*--------+
 | result |
 +--------+
 | [NULL] |
 +--------*/
```

Example with inputs that's JSON array of booleans:

```sql
SELECT LAX_INTEGER_ARRAY(JSON '[true, false]') AS result;

/*--------+
 | result |
 +--------+
 | [1, 0] |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_INTEGER_ARRAY(JSON '["10", "1.1", "1.1e2", "+1.5"]') AS result;

/*-----------------+
 | result          |
 +-----------------+
 | [10, 1, 110, 2] |
 +-----------------*/
```

```sql
SELECT LAX_INTEGER_ARRAY(JSON '["1e100"]') AS result;

/*--------+
 | result |
 +--------+
 | [NULL] |
 +--------*/
```

```sql
SELECT LAX_INTEGER_ARRAY(JSON '["foo", "null", ""]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Example with input that's JSON array of other types:

```sql
SELECT LAX_INTEGER_ARRAY(JSON '[null, {"foo": 1}, [1]]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Examples with inputs that aren't JSON arrays:

```sql
SELECT LAX_INTEGER_ARRAY(NULL) AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_INTEGER_ARRAY(JSON 'null') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_INTEGER_ARRAY(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_BIGINT`

```sql
LAX_BIGINT(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `BIGINT` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '999'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `BIGINT`|
| boolean | If the JSON boolean is `true`, returns `1`. If `false`, returns`0`. |
| string | If the JSON string represents a JSON number, parses it as a `NUMERIC` value, and then safe casts the results as an `BIGINT` value. If the JSON string can't be converted, returns `NULL`. |
| number | Casts the JSON number as an `BIGINT` value. If the JSON number can't be converted, returns `NULL`. |
| other type or null |`NULL`|

**Return type**

`BIGINT`

**Examples**

Examples with inputs that are JSON numbers:

```sql
SELECT LAX_BIGINT(JSON '10') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '10.0') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '1.1') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '3.5') AS result;

/*--------+
 | result |
 +--------+
 | 4      |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '1.1e2') AS result;

/*--------+
 | result |
 +--------+
 | 110    |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '1e100') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Examples with inputs that are JSON booleans:

```sql
SELECT LAX_BIGINT(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON 'false') AS result;

/*--------+
 | result |
 +--------+
 | 0      |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_BIGINT(JSON '"10"') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '"1.1"') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '"1.1e2"') AS result;

/*--------+
 | result |
 +--------+
 | 110    |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '"+1.5"') AS result;

/*--------+
 | result |
 +--------+
 | 2      |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '"1e100"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_BIGINT(JSON '"foo"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_BIGINT_ARRAY`

```sql
LAX_BIGINT_ARRAY(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `ARRAY<BIGINT>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[999, 12]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `ARRAY<BIGINT>`|
| array | Converts every element according to [`LAX_BIGINT`] conversion rules. |
| other type or null |`NULL`|

**Return type**

`ARRAY<BIGINT>`

**Examples**

Examples with inputs that are JSON arrays of numbers:

```sql
SELECT LAX_BIGINT_ARRAY(JSON '[10, 10.0, 1.1, 3.5, 1.1e2]') AS result;

/*---------------------+
 | result              |
 +---------------------+
 | [10, 10, 1, 4, 110] |
 +---------------------*/
```

```sql
SELECT LAX_BIGINT_ARRAY(JSON '[1e100]') AS result;

/*--------+
 | result |
 +--------+
 | [NULL] |
 +--------*/
```

Example with inputs that's JSON array of booleans:

```sql
SELECT LAX_BIGINT_ARRAY(JSON '[true, false]') AS result;

/*--------+
 | result |
 +--------+
 | [1, 0] |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_BIGINT_ARRAY(JSON '["10", "1.1", "1.1e2", "+1.5"]') AS result;

/*-----------------+
 | result          |
 +-----------------+
 | [10, 1, 110, 2] |
 +-----------------*/
```

```sql
SELECT LAX_BIGINT_ARRAY(JSON '["1e100"]') AS result;

/*--------+
 | result |
 +--------+
 | [NULL] |
 +--------*/
```

```sql
SELECT LAX_BIGINT_ARRAY(JSON '["foo", "null", ""]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Example with input that's JSON array of other types:

```sql
SELECT LAX_BIGINT_ARRAY(JSON '[null, {"foo": 1}, [1]]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Examples with inputs that aren't JSON arrays:

```sql
SELECT LAX_BIGINT_ARRAY(NULL) AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_BIGINT_ARRAY(JSON 'null') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_BIGINT_ARRAY(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_STRING`

```sql
LAX_STRING(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `VARCHAR` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '"name"'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `VARCHAR`|
| boolean | If the JSON boolean is `true`, returns `'true'`. If `false`, returns`'false'`. |
| string | Returns the JSON string as a `VARCHAR` value. |
| number | Returns the JSON number as a `VARCHAR` value. |
| other type or null |`NULL`|

**Return type**

`VARCHAR`

**Examples**

Examples with inputs that are JSON strings:

```sql
SELECT LAX_STRING(JSON '"purple"') AS result;

/*--------+
 | result |
 +--------+
 | purple |
 +--------*/
```

```sql
SELECT LAX_STRING(JSON '"10"') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

Examples with inputs that are JSON booleans:

```sql
SELECT LAX_STRING(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | true   |
 +--------*/
```

```sql
SELECT LAX_STRING(JSON 'false') AS result;

/*--------+
 | result |
 +--------+
 | false  |
 +--------*/
```

Examples with inputs that are JSON numbers:

```sql
SELECT LAX_STRING(JSON '10.0') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_STRING(JSON '10') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_STRING(JSON '1e100') AS result;

/*--------+
 | result |
 +--------+
 | 1e+100 |
 +--------*/
```

## `LAX_STRING_ARRAY`

```sql
LAX_STRING_ARRAY(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `ARRAY<VARCHAR>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '["a", "b"]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `VARCHAR`|
| array | Converts every element according to [`LAX_STRING`] conversion rules. |
| other type or null |`NULL`|

**Return type**

`ARRAY<VARCHAR>`

**Examples**

Example with input that's a JSON array of strings:

```sql
SELECT LAX_STRING_ARRAY(JSON '["purple", "10"]') AS result;

/*--------------+
 | result       |
 +--------------+
 | [purple, 10] |
 +--------------*/
```

Example with input that's a JSON array of booleans:

```sql
SELECT LAX_STRING_ARRAY(JSON '[true, false]') AS result;

/*---------------+
 | result        |
 +---------------+
 | [true, false] |
 +---------------*/
```

Example with input that's a JSON array of numbers:

```sql
SELECT LAX_STRING_ARRAY(JSON '[10.0, 10, 1e100]') AS result;

/*------------------+
 | result           |
 +------------------+
 | [10, 10, 1e+100] |
 +------------------*/
```

Example with input that's a JSON array of other types:

```sql
SELECT LAX_STRING_ARRAY(JSON '[null, {"foo": 1}, [1]]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Examples with inputs that aren't JSON arrays:

```sql
SELECT LAX_STRING_ARRAY(NULL) AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_STRING_ARRAY(JSON 'null') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_STRING_ARRAY(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_UINTEGER`

```sql
LAX_UINTEGER(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `UINTEGER` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '999'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `UINTEGER`|
| boolean | If the JSON boolean is `true`, returns `1`. If `false`, returns`0`. |
| string | If the JSON string represents a JSON number, parses it as a `NUMERIC` value, and then safe casts the results as an `UINTEGER` value. If the JSON string can't be converted, returns `NULL`. |
| number | Casts the JSON number as an `UINTEGER` value. If the JSON number can't be converted, returns `NULL`. |
| other type or null |`NULL`|

**Return type**

`UINTEGER`

**Examples**

Examples with inputs that are JSON numbers:

```sql
SELECT LAX_UINTEGER(JSON '10') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '10.0') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '1.1') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '3.5') AS result;

/*--------+
 | result |
 +--------+
 | 4      |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '1.1e2') AS result;

/*--------+
 | result |
 +--------+
 | 110    |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '-1') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '1e100') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Examples with inputs that are JSON booleans:

```sql
SELECT LAX_UINTEGER(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON 'false') AS result;

/*--------+
 | result |
 +--------+
 | 0      |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_UINTEGER(JSON '"10"') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '"1.1"') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '"1.1e2"') AS result;

/*--------+
 | result |
 +--------+
 | 110    |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '"+1.5"') AS result;

/*--------+
 | result |
 +--------+
 | 2      |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '"1e100"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_UINTEGER(JSON '"foo"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_UINTEGER_ARRAY`

```sql
LAX_UINTEGER_ARRAY(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `ARRAY<UINTEGER>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[999, 12]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `ARRAY<UINTEGER>`|
| array | Converts every element according to [`LAX_UINTEGER`] conversion rules. |
| other type or null |`NULL`|

**Return type**

`ARRAY<UINTEGER>`

**Examples**

Examples with inputs that are JSON arrays of numbers:

```sql
SELECT LAX_UINTEGER_ARRAY(JSON '[10, 10.0, 1.1, 3.5, 1.1e2]') AS result;

/*---------------------+
 | result              |
 +---------------------+
 | [10, 10, 1, 4, 110] |
 +---------------------*/
```

```sql
SELECT LAX_UINTEGER_ARRAY(JSON '[1e100]') AS result;

/*--------+
 | result |
 +--------+
 | [NULL] |
 +--------*/
```

Example with inputs that's a JSON array of booleans:

```sql
SELECT LAX_UINTEGER_ARRAY(JSON '[true, false]') AS result;

/*--------+
 | result |
 +--------+
 | [1, 0] |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_UINTEGER_ARRAY(JSON '["10", "1.1", "1.1e2", "+1.5"]') AS result;

/*-----------------+
 | result          |
 +-----------------+
 | [10, 1, 110, 2] |
 +-----------------*/
```

```sql
SELECT LAX_UINTEGER_ARRAY(JSON '["1e100"]') AS result;

/*--------+
 | result |
 +--------+
 | [NULL] |
 +--------*/
```

```sql
SELECT LAX_UINTEGER_ARRAY(JSON '["foo", "null", ""]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Example with input that's a JSON array of other types:

```sql
SELECT LAX_UINTEGER_ARRAY(JSON '[null, {"foo": 1}, [1]]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Examples with inputs that aren't JSON arrays:

```sql
SELECT LAX_UINTEGER_ARRAY(NULL) AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_UINTEGER_ARRAY(JSON 'null') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_UINTEGER_ARRAY(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_UBIGINT`

```sql
LAX_UBIGINT(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `UBIGINT` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '999'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `UBIGINT`|
| boolean | If the JSON boolean is `true`, returns `1`. If `false`, returns`0`. |
| string | If the JSON string represents a JSON number, parses it as a `NUMERIC` value, and then safe casts the results as an `UBIGINT` value. If the JSON string can't be converted, returns `NULL`. |
| number | Casts the JSON number as an `UBIGINT` value. If the JSON number can't be converted, returns `NULL`. |
| other type or null |`NULL`|

**Return type**

`UBIGINT`

**Examples**

Examples with inputs that are JSON numbers:

```sql
SELECT LAX_UBIGINT(JSON '10') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '10.0') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '1.1') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '3.5') AS result;

/*--------+
 | result |
 +--------+
 | 4      |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '1.1e2') AS result;

/*--------+
 | result |
 +--------+
 | 110    |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '-1') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '1e100') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Examples with inputs that are JSON booleans:

```sql
SELECT LAX_UBIGINT(JSON 'true') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON 'false') AS result;

/*--------+
 | result |
 +--------+
 | 0      |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_UBIGINT(JSON '"10"') AS result;

/*--------+
 | result |
 +--------+
 | 10     |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '"1.1"') AS result;

/*--------+
 | result |
 +--------+
 | 1      |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '"1.1e2"') AS result;

/*--------+
 | result |
 +--------+
 | 110    |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '"+1.5"') AS result;

/*--------+
 | result |
 +--------+
 | 2      |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '"1e100"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_UBIGINT(JSON '"foo"') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `LAX_UBIGINT_ARRAY`

```sql
LAX_UBIGINT_ARRAY(json_expr)
```

**Description**

Attempts to convert a JSON value to a SQL `ARRAY<UBIGINT>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[999, 12]'
  ```

Details:

- If `json_expr` is SQL `NULL`, the function returns SQL `NULL`.
- See the conversion rules in the next section for additional `NULL` handling.

**Conversion rules**

| From JSON type | To SQL `ARRAY<UBIGINT>`|
| array | Converts every element according to [`LAX_UBIGINT`] conversion rules. |
| other type or null |`NULL`|

**Return type**

`ARRAY<UBIGINT>`

**Examples**

Examples with inputs that are JSON arrays of numbers:

```sql
SELECT LAX_UBIGINT_ARRAY(JSON '[10, 10.0, 1.1, 3.5, 1.1e2]') AS result;

/*---------------------+
 | result              |
 +---------------------+
 | [10, 10, 1, 4, 110] |
 +---------------------*/
```

```sql
SELECT LAX_UBIGINT_ARRAY(JSON '[1e100]') AS result;

/*--------+
 | result |
 +--------+
 | [NULL] |
 +--------*/
```

Example with inputs that's a JSON array of booleans:

```sql
SELECT LAX_UBIGINT_ARRAY(JSON '[true, false]') AS result;

/*--------+
 | result |
 +--------+
 | [1, 0] |
 +--------*/
```

Examples with inputs that are JSON strings:

```sql
SELECT LAX_UBIGINT_ARRAY(JSON '["10", "1.1", "1.1e2", "+1.5"]') AS result;

/*-----------------+
 | result          |
 +-----------------+
 | [10, 1, 110, 2] |
 +-----------------*/
```

```sql
SELECT LAX_UBIGINT_ARRAY(JSON '["1e100"]') AS result;

/*--------+
 | result |
 +--------+
 | [NULL] |
 +--------*/
```

```sql
SELECT LAX_UBIGINT_ARRAY(JSON '["foo", "null", ""]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Example with input that's a JSON array of other types:

```sql
SELECT LAX_UBIGINT_ARRAY(JSON '[null, {"foo": 1}, [1]]') AS result;

/*--------------------+
 | result             |
 +--------------------+
 | [NULL, NULL, NULL] |
 +--------------------*/
```

Examples with inputs that aren't JSON arrays:

```sql
SELECT LAX_UBIGINT_ARRAY(NULL) AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_UBIGINT_ARRAY(JSON 'null') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

```sql
SELECT LAX_UBIGINT_ARRAY(JSON '9.8') AS result;

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

## `PARSE_JSON`

```sql
PARSE_JSON(
  json_string_expr
  [, wide_number_mode => { 'exact' | 'round' } ]
)
```

**Description**

Converts a JSON-formatted `VARCHAR` value to a [`JSON` value](https://www.json.org/json-en.html).

Arguments:

- `json_string_expr`: A JSON-formatted string. For example:

  ````sql
  '{"class": {"students": [{"name": "Jane"}]}}'
  ```sql
  ````

- `wide_number_mode`: A named argument with a `VARCHAR` value. Determines
  how to handle numbers that can't be stored in a `JSON` value without the
  loss of precision. If used, `wide_number_mode` must include one of the
  following values:
  - `exact` (default): Only accept numbers that can be stored without loss
    of precision. If a number that can't be stored without loss of
    precision is encountered, the function throws an error.
  - `round`: If a number that can't be stored without loss of precision is
    encountered, attempt to round it to a number that can be stored without
    loss of precision. If the number can't be rounded, the function throws
    an error.

  If a number appears in a JSON object or array, the `wide_number_mode`
  argument is applied to the number in the object or array.

Numbers from the following domains can be stored in JSON without loss of
precision:

- 64-bit signed/unsigned integers, such as `BIGINT`
- `DOUBLE`

**Return type**

`JSON`

**Examples**

In the following example, a JSON-formatted string is converted to `JSON`.

```sql
SELECT PARSE_JSON('{"coordinates": [10, 20], "id": 1}') AS json_data;

/*--------------------------------+
 | json_data                      |
 +--------------------------------+
 | {"coordinates":[10,20],"id":1} |
 +--------------------------------*/
```

The following queries fail because:

- The number that was passed in can't be stored without loss of precision.
- `wide_number_mode=>'exact'` is used implicitly in the first query and
  explicitly in the second query.

```sql
SELECT PARSE_JSON('{"id": 922337203685477580701}') AS json_data; -- fails
SELECT PARSE_JSON('{"id": 922337203685477580701}', wide_number_mode=>'exact') AS json_data; -- fails
```

The following query rounds the number to a number that can be stored in JSON.

```sql
SELECT PARSE_JSON('{"id": 922337203685477580701}', wide_number_mode=>'round') AS json_data;

/*------------------------------+
 | json_data                    |
 +------------------------------+
 | {"id":9.223372036854776e+20} |
 +------------------------------*/
```

You can also use valid JSON-formatted strings that don't represent name/value pairs. For example:

```sql
SELECT PARSE_JSON('6') AS json_data;

/*------------------------------+
 | json_data                    |
 +------------------------------+
 | 6                            |
 +------------------------------*/
```

```sql
SELECT PARSE_JSON('"red"') AS json_data;

/*------------------------------+
 | json_data                    |
 +------------------------------+
 | "red"                        |
 +------------------------------*/
```

## `SAFE_TO_JSON`

```sql
SAFE_TO_JSON(sql_value)
```

**Description**

Similar to the `TO_JSON` function, but for each unsupported field in the
input argument, produces a JSON null instead of an error.

Arguments:

- `sql_value`: The SQL value to convert to a JSON value. You can review the
  SQL data types that this function supports and their
  [JSON encodings][json-encodings].

**Return type**

`JSON`

**Example**

The following queries are functionally the same, except that `SAFE_TO_JSON`
produces a JSON null instead of an error when a hypothetical unsupported
data type is encountered:

```sql
-- Produces a JSON null.
SELECT SAFE_TO_JSON(CAST(b'' AS UNSUPPORTED_TYPE)) as result;
```

```sql
-- Produces an error.
SELECT TO_JSON(CAST(b'' AS UNSUPPORTED_TYPE), stringify_wide_numbers=>TRUE) as result;
```

In the following query, the value for `ut` is ignored because the value is an
unsupported type:

```sql
SELECT SAFE_TO_JSON(STRUCT(CAST(b'' AS UNSUPPORTED_TYPE) AS ut) AS result;

/*--------------+
 | result       |
 +--------------+
 | {"ut": null} |
 +--------------*/
```

The following array produces a JSON null instead of an error because the data
type for the array isn't supported.

```sql
SELECT SAFE_TO_JSON([
        CAST(b'' AS UNSUPPORTED_TYPE),
        CAST(b'' AS UNSUPPORTED_TYPE),
        CAST(b'' AS UNSUPPORTED_TYPE),
    ]) AS result;

/*------------+
 | result     |
 +------------+
 | null       |
 +------------*/
```

[json-encodings]: #json-encodings

## `VARCHAR`

```sql
VARCHAR(json_expr)
```

**Description**

Converts a JSON string to a SQL `VARCHAR` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '"purple"'
  ```

  If the JSON value isn't a string, an error is produced. If the expression
  is SQL `NULL`, the function returns SQL `NULL`.

**Return type**

`VARCHAR`

**Examples**

```sql
SELECT VARCHAR(JSON '"purple"') AS color;

/*--------+
 | color  |
 +--------+
 | purple |
 +--------*/
```

```sql
SELECT VARCHAR(JSON_QUERY(JSON '{"name": "sky", "color": "blue"}', "$.color")) AS color;

/*-------+
 | color |
 +-------+
 | blue  |
 +-------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't of type string.
SELECT VARCHAR(JSON '123') AS result; -- Throws an error
SELECT VARCHAR(JSON 'null') AS result; -- Throws an error
SELECT SAFE.VARCHAR(JSON '123') AS result; -- Returns a SQL NULL
```

## `STRING_ARRAY`

```sql
STRING_ARRAY(json_expr)
```

**Description**

Converts a JSON array of strings to a SQL `ARRAY<VARCHAR>` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '["purple", "blue"]'
  ```

  If the JSON value isn't an array of strings, an error is produced. If the
  expression is SQL `NULL`, the function returns SQL `NULL`.

**Return type**

`ARRAY<VARCHAR>`

**Examples**

```sql
SELECT STRING_ARRAY(JSON '["purple", "blue"]') AS colors;

/*----------------+
 | colors         |
 +----------------+
 | [purple, blue] |
 +----------------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't an array of strings.
SELECT STRING_ARRAY(JSON '[123]') AS result; -- Throws an error
SELECT STRING_ARRAY(JSON '[null]') AS result; -- Throws an error
SELECT STRING_ARRAY(JSON 'null') AS result; -- Throws an error
```

## `TO_JSON`

```sql
TO_JSON(
  sql_value
  [, stringify_wide_numbers => { TRUE | FALSE } ]
)
```

**Description**

Converts a SQL value to a JSON value.

Arguments:

- `sql_value`: The SQL value to convert to a JSON value. You can review the
  SQL data types that this function supports and their
  JSON encodings [here][json-encodings].
- `stringify_wide_numbers`: A named argument that's either
  `TRUE` or `FALSE` (default).
  - If `TRUE`, numeric values outside of the
    `DOUBLE` type domain are encoded as strings.
  - If `FALSE` (default), numeric values outside of the
    `DOUBLE` type domain aren't encoded as strings,
    but are stored as JSON numbers. If a numerical value can't be stored in
    JSON without loss of precision, an error is thrown.

  The following numerical data types are affected by the
  `stringify_wide_numbers` argument:

- `BIGINT`
- `UBIGINT`
- `NUMERIC`
- `NUMERIC`

  If one of these numerical data types appears in a container data type
  such as an `ARRAY` or `STRUCT`, the `stringify_wide_numbers` argument is
  applied to the numerical data types in the container data type.

**Return type**

`JSON`

**Examples**

In the following example, the query converts rows in a table to JSON values.

```sql
With CoordinatesTable AS (
    (SELECT 1 AS id, [10, 20] AS coordinates) UNION ALL
    (SELECT 2 AS id, [30, 40] AS coordinates) UNION ALL
    (SELECT 3 AS id, [50, 60] AS coordinates))
SELECT TO_JSON(t) AS json_objects
FROM CoordinatesTable AS t;

/*--------------------------------+
 | json_objects                   |
 +--------------------------------+
 | {"coordinates":[10,20],"id":1} |
 | {"coordinates":[30,40],"id":2} |
 | {"coordinates":[50,60],"id":3} |
 +--------------------------------*/
```

In the following example, the query returns a large numerical value as a
JSON string.

```sql
SELECT TO_JSON(9007199254740993, stringify_wide_numbers=>TRUE) as stringify_on;

/*--------------------+
 | stringify_on       |
 +--------------------+
 | "9007199254740993" |
 +--------------------*/
```

In the following example, both queries return a large numerical value as a
JSON number.

```sql
SELECT TO_JSON(9007199254740993, stringify_wide_numbers=>FALSE) as stringify_off;
SELECT TO_JSON(9007199254740993) as stringify_off;

/*------------------+
 | stringify_off    |
 +------------------+
 | 9007199254740993 |
 +------------------*/
```

In the following example, only large numeric values are converted to
JSON strings.

```sql
With T1 AS (
  (SELECT 9007199254740993 AS id) UNION ALL
  (SELECT 2 AS id))
SELECT TO_JSON(t, stringify_wide_numbers=>TRUE) AS json_objects
FROM T1 AS t;

/*---------------------------+
 | json_objects              |
 +---------------------------+
 | {"id":"9007199254740993"} |
 | {"id":2}                  |
 +---------------------------*/
```

In this example, the values `9007199254740993` (`BIGINT`)
and `2.1` (`DOUBLE`) are converted
to the common supertype `DOUBLE`, which isn't
affected by the `stringify_wide_numbers` argument.

```sql
With T1 AS (
  (SELECT 9007199254740993 AS id) UNION ALL
  (SELECT 2.1 AS id))
SELECT TO_JSON(t, stringify_wide_numbers=>TRUE) AS json_objects
FROM T1 AS t;

/*------------------------------+
 | json_objects                 |
 +------------------------------+
 | {"id":9.007199254740992e+15} |
 | {"id":2.1}                   |
 +------------------------------*/
```

[json-encodings]: #json-encodings

## `TO_JSON_STRING`

```sql
TO_JSON_STRING(value[, pretty_print])
```

**Description**

Converts a SQL value to a JSON-formatted `VARCHAR` value.

Arguments:

- `value`: A SQL value. You can review the SQL data types that
  this function supports and their JSON encodings [here][json-encodings].
- `pretty_print`: Optional boolean parameter. If `pretty_print` is `true`, the
  returned value is formatted for easy readability.

**Return type**

A JSON-formatted `VARCHAR`

**Examples**

The following query converts a `STRUCT` value to a JSON-formatted string:

```sql
SELECT TO_JSON_STRING(STRUCT(1 AS id, [10,20] AS coordinates)) AS json_data

/*--------------------------------+
 | json_data                      |
 +--------------------------------+
 | {"id":1,"coordinates":[10,20]} |
 +--------------------------------*/
```

The following query converts a `STRUCT` value to a JSON-formatted string that is
easy to read:

```sql
SELECT TO_JSON_STRING(STRUCT(1 AS id, [10,20] AS coordinates), true) AS json_data

/*--------------------+
 | json_data          |
 +--------------------+
 | {                  |
 |   "id": 1,         |
 |   "coordinates": [ |
 |     10,            |
 |     20             |
 |   ]                |
 | }                  |
 +--------------------*/
```

[json-encodings]: #json-encodings

## `UINTEGER`

```sql
UINTEGER(json_expr)
```

**Description**

Converts a JSON number to a SQL `UINTEGER` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '999'
  ```

  If the JSON value isn't a number, or the JSON number isn't in the SQL
  `UINTEGER` domain, an error is produced. If the expression is SQL `NULL`, the
  function returns SQL `NULL`.

**Return type**

`UINTEGER`

**Examples**

```sql
SELECT UINTEGER(JSON '2005') AS flight_number;

/*---------------+
 | flight_number |
 +---------------+
 | 2005          |
 +---------------*/
```

```sql
SELECT UINTEGER(JSON_QUERY(JSON '{"gate": "A4", "flight_number": 2005}', "$.flight_number")) AS flight_number;

/*---------------+
 | flight_number |
 +---------------+
 | 2005          |
 +---------------*/
```

```sql
SELECT UINTEGER(JSON '10.0') AS score;

/*-------+
 | score |
 +-------+
 | 10    |
 +-------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if JSON isn't a number or can't be converted to a 64-bit integer.
SELECT UINTEGER(JSON '10.1') AS result;  -- Throws an error
SELECT UINTEGER(JSON '-1') AS result;  -- Throws an error
SELECT UINTEGER(JSON '"strawberry"') AS result; -- Throws an error
SELECT UINTEGER(JSON 'null') AS result; -- Throws an error
SELECT SAFE.UINTEGER(JSON '"strawberry"') AS result;  -- Returns a SQL NULL
```

## `UINTEGER_ARRAY`

```sql
UINTEGER_ARRAY(json_expr)
```

**Description**

Converts a JSON number to a SQL `UINTEGER_ARRAY` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[999, 12]'
  ```

  If the JSON value isn't an array of numbers, or the JSON numbers aren't in
  the SQL `UINTEGER` domain, an error is produced. If the expression is SQL
  `NULL`, the function returns SQL `NULL`.

**Return type**

`ARRAY<UINTEGER>`

**Examples**

```sql
SELECT UINTEGER_ARRAY(JSON '[2005, 2003]') AS flight_numbers;

/*----------------+
 | flight_numbers |
 +----------------+
 | [2005, 2003]   |
 +----------------*/
```

```sql
SELECT UINTEGER_ARRAY(JSON '[10.0]') AS scores;

/*--------+
 | scores |
 +--------+
 | [10]   |
 +--------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't an array of numbers in UINTEGER domain.
SELECT UINTEGER_ARRAY(JSON '[10.1]') AS result;  -- Throws an error
SELECT UINTEGER_ARRAY(JSON '[-1]') AS result;  -- Throws an error
SELECT UINTEGER_ARRAY(JSON '["strawberry"]') AS result; -- Throws an error
SELECT UINTEGER_ARRAY(JSON '[null]') AS result; -- Throws an error
SELECT UINTEGER_ARRAY(JSON 'null') AS result; -- Throws an error
```

## `UBIGINT`

```sql
UBIGINT(json_expr)
```

**Description**

Converts a JSON number to a SQL `UBIGINT` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '999'
  ```

  If the JSON value isn't a number, or the JSON number isn't in the SQL
  `UBIGINT` domain, an error is produced. If the expression is SQL `NULL`, the
  function returns SQL `NULL`.

**Return type**

`UBIGINT`

**Examples**

```sql
SELECT UBIGINT(JSON '2005') AS flight_number;

/*---------------+
 | flight_number |
 +---------------+
 | 2005          |
 +---------------*/
```

```sql
SELECT UBIGINT(JSON_QUERY(JSON '{"gate": "A4", "flight_number": 2005}', "$.flight_number")) AS flight_number;

/*---------------+
 | flight_number |
 +---------------+
 | 2005          |
 +---------------*/
```

```sql
SELECT UBIGINT(JSON '10.0') AS score;

/*-------+
 | score |
 +-------+
 | 10    |
 +-------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if JSON isn't a number or can't be converted to a 64-bit integer.
SELECT UBIGINT(JSON '10.1') AS result;  -- Throws an error
SELECT UBIGINT(JSON '-1') AS result;  -- Throws an error
SELECT UBIGINT(JSON '"strawberry"') AS result; -- Throws an error
SELECT UBIGINT(JSON 'null') AS result; -- Throws an error
SELECT SAFE.UBIGINT(JSON '"strawberry"') AS result;  -- Returns a SQL NULL
```

## `UBIGINT_ARRAY`

```sql
UBIGINT_ARRAY(json_expr)
```

**Description**

Converts a JSON number to a SQL `UBIGINT_ARRAY` value.

Arguments:

- `json_expr`: JSON. For example:

  ```sql
  JSON '[999, 12]'
  ```

  If the JSON value isn't an array of numbers, or the JSON numbers aren't in
  the SQL `UBIGINT` domain, an error is produced. If the expression is SQL
  `NULL`, the function returns SQL `NULL`.

**Return type**

`ARRAY<UBIGINT>`

**Examples**

```sql
SELECT UBIGINT_ARRAY(JSON '[2005, 2003]') AS flight_numbers;

/*----------------+
 | flight_numbers |
 +----------------+
 | [2005, 2003]   |
 +----------------*/
```

```sql
SELECT UBIGINT_ARRAY(JSON '[10.0]') AS scores;

/*--------+
 | scores |
 +--------+
 | [10]   |
 +--------*/
```

The following examples show how invalid requests are handled:

```sql
-- An error is thrown if the JSON isn't an array of numbers in UBIGINT domain.
SELECT UBIGINT_ARRAY(JSON '[10.1]') AS result;  -- Throws an error
SELECT UBIGINT_ARRAY(JSON '[-1]') AS result;  -- Throws an error
SELECT UBIGINT_ARRAY(JSON '["strawberry"]') AS result; -- Throws an error
SELECT UBIGINT_ARRAY(JSON '[null]') AS result; -- Throws an error
SELECT UBIGINT_ARRAY(JSON 'null') AS result; -- Throws an error
```

## Supplemental materials

### Differences between the JSON and JSON-formatted VARCHAR types

Many JSON functions accept two input types:

- [`JSON`][JSON-type] type
- `VARCHAR` type

The `VARCHAR` version of the extraction functions behaves differently than the
`JSON` version, mainly because `JSON` type values are always validated whereas
JSON-formatted `VARCHAR` type values aren't.

#### Non-validation of `VARCHAR` inputs

The following `VARCHAR` is invalid JSON because it's missing a trailing `}`:

```sql
{"hello": "world"
```

The JSON function reads the input from the beginning and stops as soon as the
field to extract is found, without reading the remainder of the input. A parsing
error isn't produced.

With the `JSON` type, however, `JSON '{"hello": "world"'` returns a parsing
error.

For example:

```sql
SELECT JSON_VALUE('{"hello": "world"', "$.hello") AS hello;

/*-------+
 | hello |
 +-------+
 | world |
 +-------*/
```

```sql
SELECT JSON_VALUE(JSON '{"hello": "world"', "$.hello") AS hello;
-- An error is returned: Invalid JSON literal: syntax error while parsing
-- object - unexpected end of input; expected '}'
```

#### No strict validation of extracted values

In the following examples, duplicated keys aren't removed when using a
JSON-formatted string. Similarly, keys order is preserved. For the `JSON`
type, `JSON '{"key": 1, "key": 2}'` will result in `JSON '{"key":1}'` during
parsing.

```sql
SELECT JSON_QUERY('{"key": 1, "key": 2}', "$") AS string;

/*-------------------+
 | string            |
 +-------------------+
 | {"key":1,"key":2} |
 +-------------------*/
```

```sql
SELECT JSON_QUERY(JSON '{"key": 1, "key": 2}', "$") AS json;

/*-----------+
 | json      |
 +-----------+
 | {"key":1} |
 +-----------*/
```

#### JSON `null`

When using a JSON-formatted `VARCHAR` type in a JSON function, a JSON `null`
value is extracted as a SQL `NULL` value.

When using a JSON type in a JSON function, a JSON `null` value returns a JSON
`null` value.

```sql
WITH t AS (
  SELECT '{"name": null}' AS json_string, JSON '{"name": null}' AS json)
SELECT JSON_QUERY(json_string, "$.name") AS name_string,
  JSON_QUERY(json_string, "$.name") IS NULL AS name_string_is_null,
  JSON_QUERY(json, "$.name") AS name_json,
  JSON_QUERY(json, "$.name") IS NULL AS name_json_is_null
FROM t;

/*-------------+---------------------+-----------+-------------------+
 | name_string | name_string_is_null | name_json | name_json_is_null |
 +-------------+---------------------+-----------+-------------------+
 | NULL        | true                | null      | false             |
 +-------------+---------------------+-----------+-------------------*/
```

[JSON-type]: ../types/data_types.md#json-type

### JSON encodings

You can encode a SQL value as a JSON value with the following functions:

- `TO_JSON_STRING`
- `TO_JSON`
- `JSON_SET` (uses `TO_JSON` encoding)
- `JSON_ARRAY` (uses `TO_JSON` encoding)
- `JSON_ARRAY_APPEND` (uses `TO_JSON` encoding)
- `JSON_ARRAY_INSERT` (uses `TO_JSON` encoding)
- `JSON_OBJECT` (uses `TO_JSON` encoding)

The following SQL to JSON encodings are supported:

| From SQL         | To JSON                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Examples                                                                                                                                                                                                                                                                                                                                    |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| NULL             | null                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | SQL input:`NULL` JSON output:` null`                                                                                                                                                                                                                                                                                                        |
| BOOL             | boolean                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | SQL input:`TRUE` JSON output:`true` SQL input:`FALSE` JSON output:` false`                                                                                                                                                                                                                                                                  |
| INTEGER UINTEGER | integer                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | SQL input:`-1` JSON output:`-1` SQL input:`0` JSON output:`0` SQL input:`12345678901` JSON output:`12345678901`                                                                                                                                                                                                                             |
| BIGINT UBIGINT   | (TO_JSON_STRING only)number or stringEncoded as a number when the value is in the range of [-253, 253], which is the range of integers that can be represented losslessly as IEEE 754 double-precision floating point numbers. A value outside of this range is encoded as a string.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | SQL input:`9007199254740992` JSON output:`9007199254740992` SQL input:`9007199254740993` JSON output:`"9007199254740993"`                                                                                                                                                                                                                   |
| BIGINT UBIGINT   | (TO_JSON only)number or stringIf the `stringify_wide_numbers ` argument is `TRUE` and the value is outside of the DOUBLE type domain, the value is encoded as a string. If the value can't be stored in JSON without loss of precision, the function fails. Otherwise, the value is encoded as a number.If the `stringify_wide_numbers` isn't used or is `FALSE`, numeric values outside of the `DOUBLE` type domain aren't encoded as strings, but are stored as JSON numbers. If a numerical value can't be stored in JSON without loss of precision, an error is thrown.                                                                                                                                                                                             | SQL input:`9007199254740992` JSON output:`9007199254740992` SQL input:`9007199254740993` JSON output:`9007199254740993` SQL input with stringify_wide_numbers=>TRUE:`9007199254740992` JSON output:`9007199254740992` SQL input with stringify_wide_numbers=>TRUE:`9007199254740993` JSON output:`"9007199254740993"`                       |
| INTERVAL         | string                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | SQL input:`INTERVAL '10:20:30.52' HOUR TO SECOND` JSON output:`"PT10H20M30.52S"` SQL input:` INTERVAL 1 SECOND` JSON output:`"PT1S"` ` INTERVAL -25 MONTH` JSON output:`"P-2Y-1M"` ` INTERVAL '1 5:30' DAY TO MINUTE` JSON output:`"P1DT5H30M"`                                                                                             |
| NUMERIC NUMERIC  | (TO_JSON_STRING only)number or stringEncoded as a number when the value is in the range of [-253, 253] and has no fractional part. A value outside of this range is encoded as a string.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | SQL input:`-1` JSON output:`-1` SQL input:`0` JSON output:`0` SQL input:`9007199254740993` JSON output:`"9007199254740993"` SQL input:`123.56` JSON output:`"123.56"`                                                                                                                                                                       |
| NUMERIC NUMERIC  | (TO_JSON only)number or stringIf the `stringify_wide_numbers ` argument is `TRUE` and the value is outside of the DOUBLE type domain, it's encoded as a string. Otherwise, it's encoded as a number.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | SQL input:`-1` JSON output:`-1` SQL input:`0` JSON output:`0` SQL input:`9007199254740993` JSON output:`9007199254740993` SQL input:`123.56` JSON output:`123.56` SQL input with stringify_wide_numbers=>TRUE:`9007199254740993` JSON output:`"9007199254740993"` SQL input with stringify_wide_numbers=>TRUE:`123.56` JSON output:`123.56` |
| FLOAT DOUBLE     | number or string `+/-inf ` and `NaN` are encoded as `Infinity`,`-Infinity `, and ` NaN`. Otherwise, this value is encoded as a number.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | SQL input:`1.0` JSON output:`1` SQL input:`9007199254740993` JSON output:`9007199254740993` SQL input:`"+inf"` JSON output:`"Infinity"` SQL input:`"-inf"` JSON output:`"-Infinity"` SQL input:`"NaN"` JSON output:`"NaN"`                                                                                                                  |
| VARCHAR          | stringEncoded as a string, escaped according to the JSON standard. Specifically,`"`,`\,` and the control characters from `U+0000` to `U+001F` are escaped.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | SQL input:`"abc"` JSON output:`"abc"` SQL input:`"\"abc\""` JSON output:`"\"abc\""`                                                                                                                                                                                                                                                         |
| VARBINARY        | stringUses RFC 4648 Base64 data encoding.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | SQL input:`b"Google"` JSON output:`"R29vZ2xl"`                                                                                                                                                                                                                                                                                              |
| ENUM             | stringInvalid enum values are encoded as their number, such as 0 or 42.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | SQL input:`Color.Red ` JSON output:`"Red"`                                                                                                                                                                                                                                                                                                  |
| DATE             | string                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | SQL input:`DATE '2017-03-06'` JSON output:`"2017-03-06"`                                                                                                                                                                                                                                                                                    |
| TIMESTAMP        | stringEncoded as ISO 8601 date and time, where T separates the date and time and Z (Zulu/UTC) represents the time zone.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | SQL input:`TIMESTAMP '2017-03-06 12:34:56.789012'` JSON output:`"2017-03-06T12:34:56.789012Z"`                                                                                                                                                                                                                                              |
| DATETIME         | stringEncoded as ISO 8601 date and time, where T separates the date and time.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | SQL input:`DATETIME '2017-03-06 12:34:56.789012'` JSON output:`"2017-03-06T12:34:56.789012"`                                                                                                                                                                                                                                                |
| TIME             | stringEncoded as ISO 8601 time.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | SQL input:`TIME '12:34:56.789012'` JSON output:`"12:34:56.789012"`                                                                                                                                                                                                                                                                          |
| UUID             | stringEncoded as lowercase hexadecimal format as specified in [RFC 9562].                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | SQL input:`CAST('f81d4fae-7dec-11d0-a765-00a0c91e6bf6' AS UUID)` JSON output:`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`                                                                                                                                                                                                                       |
| JSON             | data of the input JSON                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | SQL input:`JSON '{"item": "pen", "price": 10}'` JSON output:`{"item":"pen", "price":10}` SQL input:`[1, 2, 3]` JSON output:`[1, 2, 3]`                                                                                                                                                                                                      |
| ARRAY            | arrayCan contain zero or more elements.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | SQL input:`["red", "blue", "green"]` JSON output:`["red","blue","green"]` SQL input:`[1, 2, 3]` JSON output:`[1,2,3]`                                                                                                                                                                                                                       |
| STRUCT           | objectThe object can contain zero or more key-value pairs. Each value is formatted according to its type.For `TO_JSON`, a field is included in the output string and any duplicates of this field are omitted. For `TO_JSON_STRING`, a field and any duplicates of this field are included in the output string.Anonymous fields are represented with `""`.Invalid UTF-8 field names might result in unparseable JSON. String values are escaped according to the JSON standard. Specifically,`"`,`\,` and the control characters from `U+0000` to `U+001F` are escaped.                                                                                                                                                                                                | SQL input:`STRUCT(12 AS purchases, TRUE AS inStock)` JSON output:`{"inStock": true,"purchases":12}`                                                                                                                                                                                                                                         |
|                  | objectThe object can contain zero or more key-value pairs. Each value is formatted according to its type.Field names with underscores are converted to camel case in accordance with [protobuf json conversion]. Field values are formatted according to [protobuf json conversion]. If a `field_value ` is a non-empty repeated field or submessage, the elements and fields are indented to the appropriate level.Field names that aren't valid UTF-8 might result in unparseable JSON.Field annotations are ignored.Repeated fields are represented as arrays.Submessages are formatted as values of type.Extension fields are included in the output, where the extension field name is enclosed in brackets and prefixed with the full name of the extension type. | SQL input:`NEW Item(12 AS purchases,TRUE AS in_Stock)` JSON output:`{"purchases":12,"inStock": true}`                                                                                                                                                                                                                                       |
| GRAPH_ELEMENT    | objectThe object can contain zero or more key-value pairs. Each value is formatted according to its type.For `TO_JSON`, graph element (node or edge) objects are supported.The graph element identifier is only valid within the scope of the same query response and can't be used to correlate entities across different queries.Field names that aren't valid UTF-8 might result in unparseable JSON.The result may include internal key-value pairs that aren't defined by the users.The conversion can fail if the object contains values of unsupported types.                                                                                                                                                                                                    | SQL: GRAPH FinGraph MATCH (p:Person WHERE p.name = 'Dana') RETURN TO_JSON(p) AS dana_json; JSON output (truncated): {"identifier":"ZGFuYQ==","kind":"node","labels":["Person"],"properties":{"id":2,"name":"Dana"}}                                                                                                                         |
| RANGE            | rangeEncoded as an object with a `start ` and `end` value. Any unbounded part of the range is represented as ` null`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | SQL input:` RANGE<DATE> '[2024-07-24, 2024-07-25)'` JSON output:`{"start":"2024-07-24","end":"2024-07-25"}` SQL input:` RANGE<DATETIME> '[2024-07-24 10:00:00, UNBOUNDED)'` JSON output:`{"start":"2024-07-24T10:00:00","end":null}`                                                                                                        |

### JSONPath format

With the JSONPath format, you can identify the values you want to
obtain from a JSON-formatted string.

If a key in a JSON functions contains a JSON format operator, refer to each
JSON function for how to escape them.

A JSON function returns `NULL` if the JSONPath format doesn't match a value in
a JSON-formatted string. If the selected value for a scalar function isn't
scalar, such as an object or an array, the function returns `NULL`. If the
JSONPath format is invalid, an error is produced.

#### Operators for JSONPath

The JSONPath format supports these operators:

| Operator           | Description                                                                                                                                                    | Examples                                                                                                                                           |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$`                | Root object or element. The JSONPath format must start with this operator, which refers to the outermost level of the JSON-formatted string.                   | JSON-formatted string: `'{"class" : {"students" : [{"name" : "Jane"}]}}'` JSON path: `"$"` JSON result: `{"class":{"students":[{"name":"Jane"}]}}` |
| `.`                | Child operator. You can identify child values using dot-notation.                                                                                              | JSON-formatted string: `'{"class" : {"students" : [{"name" : "Jane"}]}}'` JSON path: `"$.class.students"` JSON result: `[{"name":"Jane"}]`         |
| `[]`               | Subscript operator. If the object is a JSON array, you can use brackets to specify the array index.                                                            | JSON-formatted string: `'{"class" : {"students" : [{"name" : "Jane"}]}}'` JSON path: `"$.class.students [0]"` JSON result: `{"name":"Jane"}`       |
| `[][]` `[][][]...` | Child subscript operator. If the object is a JSON array within an array, you can use as many additional brackets as you need to specify the child array index. | JSON-formatted string: `'{"a": [["b", "c"], "d"], "e":"f"}'` JSON path: `"$.a [0][1]"` JSON result: `"c"`                                          |
