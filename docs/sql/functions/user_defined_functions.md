# User-defined functions

SQL supports user-defined functions (UDFs).

A UDF lets you create a function using another SQL expression or another
programming language, such as JavaScript or Lua. These functions accept columns
of input and perform actions, returning the result of those actions as a value.

## SQL UDFs

A SQL user-defined function (UDF) operates on one row at a time and returns the
result of that calculation as a single value.

All of the arguments are expressions that are computed in the context of a
single row.

### Create a SQL UDF

You can create a SQL UDF using the following syntax:

```sql
CREATE
  [ OR REPLACE ]
  [ { TEMPORARY | TEMP } ] FUNCTION
  [ IF NOT EXISTS ]
  function_name ( [ function_parameter [, ...] ] )
  [ RETURNS data_type ]
  AS ( function_body )

function_parameter:
  parameter_name
  { data_type | ANY TYPE }
  [ DEFAULT default_value ]
```

This syntax consists of the following components:

- `CREATE ... FUNCTION`: Creates a new function. A function
  can have zero or more function parameters.
  - `TEMPORARY` or `TEMP`: Indicates that the function is temporary, meaning
    that it exists for the lifetime of the session. A temporary function can
    have the same name as a built-in function. If this happens, the
    temporary function hides the built-in function for the duration of the
    temporary function's lifetime.

- `OR REPLACE`: Replaces any function with the same name if it exists.
  Can't appear with `IF NOT EXISTS`.
- `IF NOT EXISTS`: If any function exists with the same name, the `CREATE`
  statement has no effect. Can't appear with `OR REPLACE`.
- `function_name`: The name of the function.
- `function_parameter`: A parameter for the function.
  - `parameter_name`: The name of the function parameter.

  - `data_type`: A SQL [data type][data-types].

  - `ANY TYPE`: The function will accept an argument of any type for this
    function parameter. If more than one parameter includes `ANY TYPE`,
    a relationship isn't enforced between these parameters when the function
    is defined. However, if the type of argument passed into the function at
    call time is incompatible with the function definition, this will
    result in an error.

    `ANY TYPE` is a [_templated function parameter_][templated-parameters].

  - `DEFAULT default_value`: If an argument isn't provided for a function
    parameter, `default_value` is used. `default_value` must be a literal
    or `NULL` value. All function parameters following this one
    must also have default values.

- `RETURNS data_type`: Optional clause that specifies the data type
  that the function returns. SQL infers the result type
  of the function from the SQL function body when the `RETURN` clause is
  omitted.
- `function_body`: The SQL expression that defines the function body.

[quoted-literals]: ../syntax/lexical.md#quoted-literals

### Call a SQL UDF

You can call a SQL UDF in the same way that you call a built-in function.
For details, see [Function calls][function-calls].

### SQL UDF examples

The following example shows a UDF that employs a SQL function.

```sql
CREATE TEMP FUNCTION AddFourAndDivide(x BIGINT, y BIGINT)
RETURNS DOUBLE
AS (
  (x + 4) / y
);

WITH
  numbers AS (
    SELECT 1 AS val UNION ALL
    SELECT 3 AS val UNION ALL
    SELECT 4 AS val UNION ALL
    SELECT 5 AS val
  )
SELECT val, AddFourAndDivide(val, 2) AS result
FROM numbers;

/*-----+--------+
 | val | result |
 +-----+--------+
 | 1   | 2.5    |
 | 3   | 3.5    |
 | 4   | 4      |
 | 5   | 4.5    |
 +-----+--------*/
```

The following example shows a SQL UDF that uses the
templated function parameter, `ANY TYPE`. The resulting function accepts
arguments of various types.

```sql
CREATE TEMP FUNCTION AddFourAndDivideAny(x ANY TYPE, y ANY TYPE)
AS (
  (x + 4) / y
);

SELECT
  AddFourAndDivideAny(3, 4) AS integer_input,
  AddFourAndDivideAny(1.59, 3.14) AS floating_point_input;

/*----------------+-----------------------+
 | integer_input  | floating_point_input  |
 +----------------+-----------------------+
 | 1.75           | 1.7802547770700636    |
 +----------------+-----------------------*/
```

The following example shows a SQL UDF that uses the
templated function parameter, `ANY TYPE`, to return the last element of an
array of any type.

```sql
CREATE TEMP FUNCTION LastArrayElement(arr ANY TYPE)
AS (
  arr[ORDINAL(ARRAY_LENGTH(arr))]
);

SELECT
  names[OFFSET(0)] AS first_name,
  LastArrayElement(names) AS last_name
FROM
  (
    SELECT ['Fred', 'McFeely', 'Rogers'] AS names UNION ALL
    SELECT ['Marie', 'Skłodowska', 'Curie']
  );

/*------------+-----------+
 | first_name | last_name |
 +------------+-----------+
 | Fred       | Rogers    |
 | Marie      | Curie     |
 +------------+-----------*/
```

## JavaScript UDFs

A JavaScript user-defined function (UDF) runs JavaScript code and returns the
result as a single value.

### Create a JavaScript UDF

You can create a JavaScript UDF using the following syntax:

```sql
CREATE
  [ { TEMPORARY | TEMP } ] FUNCTION
  function_name ( [ function_parameter [, ...] ] )
  RETURNS data_type
  [ determinism_specifier ]
  LANGUAGE js AS function_body

function_parameter:
  parameter_name
  { data_type | ANY TYPE }
  [ DEFAULT default_value ]

determinism_specifier:
  { IMMUTABLE | DETERMINISTIC | NOT DETERMINISTIC | VOLATILE | STABLE }
```

This syntax consists of the following components:

- `CREATE ... FUNCTION`: Creates a new function. A function
  can have zero or more function parameters.
  - `TEMPORARY` or `TEMP`: Indicates that the function is temporary, meaning
    that it exists for the lifetime of the session. A temporary function can
    have the same name as a built-in function. If this happens, the
    temporary function hides the built-in function for the duration of the
    temporary function's lifetime.

- `function_name`: The name of the function.
- `function_parameter`: A parameter for the function.
  - `parameter_name`: The name of the function parameter.

  - `data_type`: A SQL [data type][data-types].
    See [SQL type encodings in JavaScript][javascript-data-types] to learn
    how SQL represents JavaScript types.

  - `ANY TYPE`: The function will accept an argument of any type for this
    function parameter. If more than one parameter includes `ANY TYPE`,
    a relationship isn't enforced between these parameters when the function
    is defined. However, if the type of argument passed into the function at
    call time is incompatible with the function definition, this will
    result in an error.

    `ANY TYPE` is a [_templated function parameter_][templated-parameters].

  - `DEFAULT default_value`: If an argument isn't provided for a function
    parameter, `default_value` is used. `default_value` must be a literal
    or `NULL` value. All function parameters following this one
    must also have default values.

- `determinism_specifier`: Identifies the determinism property of the
  function, which impacts query semantics and planning. Your choices are:
  - `IMMUTABLE` or `DETERMINISTIC`: The function always returns the same
    result when passed the same arguments. For example, if the function
    `add_one(i)` always returns `i + 1`, the function is deterministic.

  - `NOT DETERMINISTIC`: The function doesn't always return the same result
    when passed the same arguments. The `VOLATILE` and `STABLE` keywords are
    subcategories of `NOT DETERMINISTIC`.

  - `VOLATILE`: The function doesn't always return the same result when
    passed the same arguments, even within the same run of a query
    statement. For example if `add_random(i)` returns `i + rand()`, the
    function is volatile, because every call to the function can return a
    different result.

  - `STABLE`: Within one execution of a statement, the function will
    consistently return the same result for the same argument values.
    However, the result could change for different executions of the
    same statement. For example if you invoke the function
    `CURRENT_TIMESTAMP` multiple times within a single statement, it will
    return the same result, but it may return different results in
    subsequent statement executions.

- `RETURNS data_type`: Specifies the SQL data type that the
  function returns.
- `function_body`: Quoted string literal that represents
  the JavaScript code that defines the function body.
  To learn more about the different types of quoted string literals you can
  use, see [Formats for quoted literals][quoted-literals].

[quoted-literals]: ../syntax/lexical.md#quoted-literals

### Call a JavaScript UDF

You can call a JavaScript UDF the same way that you call a built-in
function. For details, see [Function calls][function-calls].

### SQL type encodings in JavaScript

[SQL data types][data-types] represent
[JavaScript data types][javascript-types] as follows:

| SQL data type | JavaScript data type | Notes                                                                                                                                                                                                                                                                                                                             |
| ------------- | -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ARRAY         | Array                | An array of arrays isn't supported. To get around this limitation, use JavaScript `Array<Object<Array>>` and SQL `ARRAY<STRUCT<ARRAY>>`.                                                                                                                                                                                          |
| BOOL          | Boolean              |                                                                                                                                                                                                                                                                                                                                   |
| VARBINARY     | String               | Base64-encoded String.                                                                                                                                                                                                                                                                                                            |
| DOUBLE        | Number               |                                                                                                                                                                                                                                                                                                                                   |
| FLOAT         | Number               |                                                                                                                                                                                                                                                                                                                                   |
| NUMERIC       | Number or String     | If a NUMERIC value can be represented exactly as an [IEEE 754 floating-point](https://en.wikipedia.org/wiki/Floating-point_arithmetic#IEEE_754:_floating_point_in_modern_computers) value and has no fractional part, it's encoded as a Number. These values are in the range [-2^53, 2^53]. Otherwise, it's encoded as a String. |
| NUMERIC       | Number or String     | Same as NUMERIC.                                                                                                                                                                                                                                                                                                                  |
| INTEGER       | Number               |                                                                                                                                                                                                                                                                                                                                   |
| UINTEGER      | Number               |                                                                                                                                                                                                                                                                                                                                   |
| BIGINT        | String               |                                                                                                                                                                                                                                                                                                                                   |
| UBIGINT       | N/A                  | UBIGINT is unsupported as an input type for JavaScript UDFs. Instead, use DOUBLE to represent integer values as a number, or VARCHAR to represent integer values as a string.                                                                                                                                                     |
| VARCHAR       | String               |                                                                                                                                                                                                                                                                                                                                   |
| STRUCT        | Object               | Object where each STRUCT field is a named property in the Object. Unnamed field in STRUCT isn't supported.                                                                                                                                                                                                                        |
| TIMESTAMP     | Date object          | See the documentation for your database engine.                                                                                                                                                                                                                                                                                   |
| DATE          | Date object          |                                                                                                                                                                                                                                                                                                                                   |

Some SQL types have a direct mapping to JavaScript types, but
others don't.

For example, because JavaScript doesn't support a 64-bit integer type,
`BIGINT` is unsupported as an input type for JavaScript UDFs. Instead,
use `DOUBLE` to represent integer values as a number,
or `VARCHAR` to represent integer values as a string.

SQL does support `BIGINT` as a return type in JavaScript UDFs.
In this case, the JavaScript function body can return either a JavaScript
`Number` or a `String`. SQL then converts either of
these types to `BIGINT`.

In addition, some SQL and JavaScript data types have different
rules. For example, in JavaScript, you can have an array of arrays
(`Array<Array>`), whereas in SQL, you can't. Before using
encodings, ensure they are compatible. To learn more about SQL
data types, see [SQL data types][data-types]. To learn more about
JavaScript data types, see [JavaScript data types][javascript-types].

### JavaScript UDF examples

The following example illustrates a simple JavaScript UDF with a
regular expression. Because there is a regular expression in the
function body, the function body needs to be a [raw string][quoted-literals].

Note: If you aren't sure which quoting style to use for the function body,
a raw string provides the most consistent results.

```sql
CREATE TEMP FUNCTION ExtractLetters(x VARCHAR)
RETURNS VARCHAR
LANGUAGE js
AS r'''
var re = /[a-z]/g;
return x.match(re);
''';

SELECT val, ExtractLetters(val) AS result
FROM UNNEST(['ab-c', 'd_e', '!']) AS val;

/*-------*---------+
 | val   | result  |
 +-------+---------+
 | ab-c  | [a,b,c] |
 | d_e   | [d,e]   |
 | !     | NULL    |
 +-------*---------*/
```

The following example illustrates a single-statement JavaScript UDF.
Because the function body doesn't contain
[escape sequences][escape-sequences] or [regular expressions][quoted-literals],
it can be a quoted or triple-quoted string literal.

```sql
CREATE TEMP FUNCTION PlusOne(x DOUBLE)
RETURNS DOUBLE
LANGUAGE js
AS 'return x+1;';

SELECT val, PlusOne(val) AS result
FROM UNNEST([1, 2, 3]) AS val;

/*-----------+-----------+
 | val       | result    |
 +-----------+-----------+
 | 1         | 2         |
 | 2         | 3         |
 | 3         | 4         |
 +-----------+-----------*/
```

The following example illustrates a more complex multi-statement JavaScript UDF.

```sql
CREATE TEMP FUNCTION CustomGreeting(a VARCHAR)
RETURNS VARCHAR
LANGUAGE js
AS r'''
  var d = new Date();
  if (d.getHours() < 12) {
    return 'Good Morning, ' + a + '!';
  } else {
    return 'Good Evening, ' + a + '!';
  }
''';

SELECT CustomGreeting(names) as everyone
FROM UNNEST(["Hannah", "Max", "Jakob"]) AS names;

/*-----------------------+
 | everyone              |
 +-----------------------+
 | Good Morning, Hannah! |
 | Good Morning, Max!    |
 | Good Morning, Jakob!  |
 +-----------------------*/
```

The following example creates a persistent JavaScript UDF.

```sql
CREATE FUNCTION MultiplyInputs(x DOUBLE, y DOUBLE)
RETURNS DOUBLE
LANGUAGE js
AS r'''
  return x*y;
''';

WITH numbers AS
  (SELECT 1 AS x, 5 as y
  UNION ALL
  SELECT 2 AS x, 10 as y
  UNION ALL
  SELECT 3 as x, 15 as y)
SELECT x, y, MultiplyInputs(x, y) as product
FROM numbers;

/*-----+-----+--------------+
 | x   | y   | product      |
 +-----+-----+--------------+
 | 1   | 5   | 5            |
 | 2   | 10  | 20           |
 | 3   | 15  | 45           |
 +-----+-----+--------------*/
```

The following example creates a temporary JavaScript UDF.

```sql
CREATE TEMP FUNCTION MultiplyInputs(x DOUBLE, y DOUBLE)
RETURNS DOUBLE
LANGUAGE js
AS r'''
  return x*y;
''';

WITH numbers AS
  (SELECT 1 AS x, 5 as y
  UNION ALL
  SELECT 2 AS x, 10 as y
  UNION ALL
  SELECT 3 as x, 15 as y)
SELECT x, y, MultiplyInputs(x, y) as product
FROM numbers;

/*-----+-----+--------------+
 | x   | y   | product      |
 +-----+-----+--------------+
 | 1   | 5   | 5            |
 | 2   | 10  | 20           |
 | 3   | 15  | 45           |
 +-----+-----+--------------*/
```

You can create multiple JavaScript UDFs before a query. For example:

```sql
CREATE TEMP FUNCTION MultiplyInputs(x DOUBLE, y DOUBLE)
RETURNS DOUBLE
LANGUAGE js
AS r'''
  return x*y;
''';

CREATE TEMP FUNCTION DivideByTwo(x DOUBLE)
RETURNS DOUBLE
LANGUAGE js
AS r'''
  return x / 2;
''';

WITH numbers AS
  (SELECT 1 AS x, 5 as y
  UNION ALL
  SELECT 2 AS x, 10 as y
  UNION ALL
  SELECT 3 as x, 15 as y)
SELECT x,
  y,
  MultiplyInputs(x, y) as product,
  DivideByTwo(x) as half_x,
  DivideByTwo(y) as half_y
FROM numbers;

/*-----+-----+--------------+--------+--------+
 | x   | y   | product      | half_x | half_y |
 +-----+-----+--------------+--------+--------+
 | 1   | 5   | 5            | 0.5    | 2.5    |
 | 2   | 10  | 20           | 1      | 5      |
 | 3   | 15  | 45           | 1.5    | 7.5    |
 +-----+-----+--------------+--------+--------*/
```

You can pass the result of a JavaScript UDF as input to another UDF.
For example:

```sql
CREATE TEMP FUNCTION MultiplyInputs(x DOUBLE, y DOUBLE)
RETURNS DOUBLE
LANGUAGE js
AS r'''
  return x*y;
''';

CREATE TEMP FUNCTION DivideByTwo(x DOUBLE)
RETURNS DOUBLE
LANGUAGE js
AS r'''
  return x/2;
''';

WITH numbers AS
  (SELECT 1 AS x, 5 as y
  UNION ALL
  SELECT 2 AS x, 10 as y
  UNION ALL
  SELECT 3 as x, 15 as y)
SELECT x,
  y,
  MultiplyInputs(DivideByTwo(x), DivideByTwo(y)) as half_product
FROM numbers;

/*-----+-----+--------------+
 | x   | y   | half_product |
 +-----+-----+--------------+
 | 1   | 5   | 1.25         |
 | 2   | 10  | 5            |
 | 3   | 15  | 11.25        |
 +-----+-----+--------------*/
```

The following example shows how you can use a JavaScript UDF with
default values.

```sql
CREATE TEMP FUNCTION AddValues(x BIGINT, y BIGINT DEFAULT 50, z BIGINT DEFAULT 100)
RETURNS BIGINT
LANGUAGE js
AS r'''
  return x*y*z;
''';

SELECT AddValues(1, 2) AS result;

/*--------+
 | result |
 +--------+
 | 200    |
 +--------*/
```

[quoted-literals]: ../syntax/lexical.md#string-and_bytes_literals

The following example sums the values of all
fields named `foo` in the given JSON string.

```sql
CREATE TEMP FUNCTION SumFieldsNamedFoo(json_row VARCHAR)
RETURNS DOUBLE PRECISION
LANGUAGE js
AS r'''
function SumFoo(obj) {
  var sum = 0;
  for (var field in obj) {
    if (obj.hasOwnProperty(field) && obj[field] != null) {
      if (typeof obj[field] == "object") {
        sum += SumFoo(obj[field]);
      } else if (field == "foo") {
        sum += obj[field];
      }
    }
  }
  return sum;
}
var row = JSON.parse(json_row);
return SumFoo(row);
''';

WITH
  Input AS (
    SELECT
      STRUCT(1 AS foo, 2 AS bar, STRUCT('foo' AS x, 3.14 AS foo) AS baz) AS s,
      10 AS foo
    UNION ALL
    SELECT NULL, 4 AS foo
    UNION ALL
    SELECT
      STRUCT(NULL, 2 AS bar, STRUCT('fizz' AS x, 1.59 AS foo) AS baz) AS s,
      NULL AS foo
  )
SELECT
  TO_JSON_STRING(t) AS json_row,
  SumFieldsNamedFoo(TO_JSON_STRING(t)) AS foo_sum
FROM Input AS t;

/*---------------------------------------------------------------------+---------+
 | json_row                                                            | foo_sum |
 +---------------------------------------------------------------------+---------+
 | {"s":{"foo":1,"bar":2,"baz":{"x":"foo","foo":3.14}},"foo":10}       | 14.14   |
 | {"s":null,"foo":4}                                                  | 4       |
 | {"s":{"foo":null,"bar":2,"baz":{"x":"fizz","foo":1.59}},"foo":null} | 1.59    |
 +---------------------------------------------------------------------+---------*/
```

## Lua UDFs

A Lua user-defined function (UDF) runs Lua code and returns the result as a
single value.

### Create a Lua UDF

You can create a Lua UDF using the following syntax:

```sql
CREATE
  [ { TEMPORARY | TEMP } ] FUNCTION
  function_name ( [ function_parameter [, ...] ] )
  RETURNS data_type
  [ determinism_specifier ]
  LANGUAGE lua AS function_body

function_parameter:
  parameter_name
  { data_type | ANY TYPE }
  [ DEFAULT default_value ]

determinism_specifier:
  { IMMUTABLE | DETERMINISTIC | NOT DETERMINISTIC | VOLATILE | STABLE }
```

This syntax consists of the following components:

- `CREATE ... FUNCTION`: Creates a new function. A function
  can have zero or more function parameters.
  - `TEMPORARY` or `TEMP`: Indicates that the function is temporary, meaning
    that it exists for the lifetime of the session. A temporary function can
    have the same name as a built-in function. If this happens, the
    temporary function hides the built-in function for the duration of the
    temporary function's lifetime.

- `function_name`: The name of the function.
- `function_parameter`: A parameter for the function.
  - `parameter_name`: The name of the function parameter.

  - `data_type`: A SQL [data type][data-types].

  - `ANY TYPE`: The function will accept an argument of any type for this
    function parameter. If more than one parameter includes `ANY TYPE`,
    a relationship isn't enforced between these parameters when the function
    is defined. However, if the type of argument passed into the function at
    call time is incompatible with the function definition, this will
    result in an error.

    `ANY TYPE` is a [_templated function parameter_][templated-parameters].

  - `DEFAULT default_value`: If an argument isn't provided for a function
    parameter, `default_value` is used. `default_value` must be a literal
    or `NULL` value. All function parameters following this one
    must also have default values.

- `determinism_specifier`: Identifies the determinism property of the
  function, which impacts query semantics and planning. Your choices are:
  - `IMMUTABLE` or `DETERMINISTIC`: The function always returns the same
    result when passed the same arguments. For example, if the function
    `add_one(i)` always returns `i + 1`, the function is deterministic.

  - `NOT DETERMINISTIC`: The function doesn't always return the same result
    when passed the same arguments. The `VOLATILE` and `STABLE` keywords are
    subcategories of `NOT DETERMINISTIC`.

  - `VOLATILE`: The function doesn't always return the same result when
    passed the same arguments, even within the same run of a query
    statement. For example if `add_random(i)` returns `i + rand()`, the
    function is volatile, because every call to the function can return a
    different result.

  - `STABLE`: Within one execution of a statement, the function will
    consistently return the same result for the same argument values.
    However, the result could change for different executions of the
    same statement. For example if you invoke the function
    `CURRENT_TIMESTAMP` multiple times within a single statement, it will
    return the same result, but it may return different results in
    subsequent statement executions.

- `RETURNS data_type`: Specifies the SQL data type that the
  function returns.
- `function_body`: Quoted string literal that represents
  the Lua code that defines the function body.
  To learn more about the different types of quoted string literals you can
  use, see [Formats for quoted literals][quoted-literals].

[quoted-literals]: ../syntax/lexical.md#quoted-literals

### Call a Lua UDF

You can call a Lua UDF in the same way that you call a built-in
function. For details, see [Function calls][function-calls].

### Data types supported

The valid data types for arguments and return values are INTEGER, UINTEGER, BIGINT, UBIGINT, FLOAT, DOUBLE, VARCHAR, BOOL, and . If the function returns the Lua type nil, it gets converted to a NULL value of the specified return data type.

### Lua UDF examples

The following example illustrates a simple Lua UDF with some
Lua escape sequences. Because there is a Lua escape sequence in the
function body, the function body needs to be a [raw string][quoted-literals].

Note: If you aren't sure which quoting style to use for the function body,
a raw string provides the most consistent results.

```sql
CREATE TEMP FUNCTION Alphabet()
RETURNS VARCHAR
LANGUAGE lua
AS r'''
  return 'A\nB\nC';
''';

SELECT Alphabet() AS characters;

/*------------+
 | characters |
 +------------+
 | A          |
 | B          |
 | C          |
 +------------*/
```

The following example illustrates a single-statement Lua UDF.
Because the function body doesn't contain
[escape sequences][escape-sequences] or [regular expressions][quoted-literals],
it can be a quoted or triple-quoted string literal.

```sql
CREATE TEMP FUNCTION PlusOne(x DOUBLE)
RETURNS DOUBLE
LANGUAGE lua
AS 'return x+1;';

SELECT val, PlusOne(val) AS result
FROM UNNEST([1, 2, 3]) AS val;

/*-----------+-----------+
 | val       | result    |
 +-----------+-----------+
 | 1         | 2         |
 | 2         | 3         |
 | 3         | 4         |
 +-----------+-----------*/
```

The following example illustrates a multi-statement Lua UDF.

```sql
CREATE TEMP FUNCTION CustomGreeting(i INTEGER)
RETURNS VARCHAR
LANGUAGE lua
AS r'''
  if i < 12 then
    return 'Good Morning!'
  else
    return 'Good Evening!'
  end
''';

SELECT CustomGreeting(13) AS message;

/*---------------+
 | message       |
 +---------------+
 | Good Evening! |
 +---------------*/
```

The following example creates a persistent Lua UDF.

```sql
CREATE FUNCTION MultiplyInputs(x DOUBLE, y DOUBLE)
RETURNS DOUBLE
LANGUAGE lua
AS r'''
  return x*y;
''';

WITH numbers AS
  (SELECT 1 AS x, 5 as y
  UNION ALL
  SELECT 2 AS x, 10 as y
  UNION ALL
  SELECT 3 as x, 15 as y)
SELECT x, y, MultiplyInputs(x, y) as product
FROM numbers;

/*-----+-----+--------------+
 | x   | y   | product      |
 +-----+-----+--------------+
 | 1   | 5   | 5            |
 | 2   | 10  | 20           |
 | 3   | 15  | 45           |
 +-----+-----+--------------*/
```

The following example creates a temporary Lua UDF.

```sql
CREATE TEMP FUNCTION MultiplyInputs(x DOUBLE, y DOUBLE)
RETURNS DOUBLE
LANGUAGE lua
AS r'''
  return x*y;
''';

WITH numbers AS
  (SELECT 1 AS x, 5 as y
  UNION ALL
  SELECT 2 AS x, 10 as y
  UNION ALL
  SELECT 3 as x, 15 as y)
SELECT x, y, MultiplyInputs(x, y) as product
FROM numbers;

/*-----+-----+--------------+
 | x   | y   | product      |
 +-----+-----+--------------+
 | 1   | 5   | 5            |
 | 2   | 10  | 20           |
 | 3   | 15  | 45           |
 +-----+-----+--------------*/
```

You can create multiple Lua UDFs before a query. For example:

```sql
CREATE TEMP FUNCTION MultiplyInputs(x DOUBLE, y DOUBLE)
RETURNS DOUBLE
LANGUAGE lua
AS r'''
  return x*y;
''';

CREATE TEMP FUNCTION DivideByTwo(x DOUBLE)
RETURNS DOUBLE
LANGUAGE lua
AS r'''
  return x / 2;
''';

WITH numbers AS
  (SELECT 1 AS x, 5 as y
  UNION ALL
  SELECT 2 AS x, 10 as y
  UNION ALL
  SELECT 3 as x, 15 as y)
SELECT x,
  y,
  MultiplyInputs(x, y) as product,
  DivideByTwo(x) as half_x,
  DivideByTwo(y) as half_y
FROM numbers;

/*-----+-----+--------------+--------+--------+
 | x   | y   | product      | half_x | half_y |
 +-----+-----+--------------+--------+--------+
 | 1   | 5   | 5            | 0.5    | 2.5    |
 | 2   | 10  | 20           | 1      | 5      |
 | 3   | 15  | 45           | 1.5    | 7.5    |
 +-----+-----+--------------+--------+--------*/
```

You can pass the result of a Lua UDF as input to another UDF.
For example:

```sql
CREATE TEMP FUNCTION MultiplyInputs(x DOUBLE, y DOUBLE)
RETURNS DOUBLE
LANGUAGE lua
AS r'''
  return x*y;
''';

CREATE TEMP FUNCTION DivideByTwo(x DOUBLE)
RETURNS DOUBLE
LANGUAGE lua
AS r'''
  return x/2;
''';

WITH numbers AS
  (SELECT 1 AS x, 5 as y
  UNION ALL
  SELECT 2 AS x, 10 as y
  UNION ALL
  SELECT 3 as x, 15 as y)
SELECT x,
  y,
  MultiplyInputs(DivideByTwo(x), DivideByTwo(y)) as half_product
FROM numbers;

/*-----+-----+--------------+
 | x   | y   | half_product |
 +-----+-----+--------------+
 | 1   | 5   | 1.25         |
 | 2   | 10  | 5            |
 | 3   | 15  | 11.25        |
 +-----+-----+--------------*/
```

The following example shows how you can use a Lua UDF with
default values.

```sql
CREATE TEMP FUNCTION AddValues(x BIGINT, y BIGINT DEFAULT 50, z BIGINT DEFAULT 100)
RETURNS BIGINT
LANGUAGE lua
AS r'''
  return x*y*z;
''';

SELECT AddValues(1, 2) AS result;

/*--------+
 | result |
 +--------+
 | 200    |
 +--------*/
```

[quoted-literals]: ../syntax/lexical.md#string-and_bytes_literals

## Templated function parameters

A templated function parameter can match more than one argument type at
function call time. If a function signature includes a
templated function parameter, SQL allows function calls
to pass to the function any argument type as long as the function body is
valid for that argument type.

[javascript-types]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects
[templated-parameters]: #templated-function_parameters
[javascript-data-types]: #javascript-udf_data_types
[data-types]: ../types/data_types.md
[function-calls]: functions_reference.md
[quoted-literals]: ../syntax/lexical.md#quoted-literals
[escape-sequences]: ../syntax/lexical.md#escape-sequences
