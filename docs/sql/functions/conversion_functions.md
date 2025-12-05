# Conversion functions

SQL supports conversion functions. These data type
conversions are explicit, but some conversions can happen implicitly. You can
learn more about implicit and explicit conversion [here][conversion-rules].

## Function list

| Name                                                                               | Summary                                                                                                                                                                                                         |
| ---------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`ARRAY_TO_STRING`](array_functions.md#array-to-string)                            | Produces a concatenation of the elements in an array as a `VARCHAR` value. For more information, see [Array functions](array_functions.md).                                                                     |
| [`BIT_CAST_TO_INTEGER`](bit_functions.md#bit-cast-to-int32)                        | Cast bits to an `INTEGER` value. For more information, see [Bit functions](bit_functions.md).                                                                                                                   |
| [`BIT_CAST_TO_BIGINT`](bit_functions.md#bit-cast-to-int64)                         | Cast bits to an `BIGINT` value. For more information, see [Bit functions](bit_functions.md).                                                                                                                    |
| [`BIT_CAST_TO_UINTEGER`](bit_functions.md#bit-cast-to-uint32)                      | Cast bits to an `UINTEGER` value. For more information, see [Bit functions](bit_functions.md).                                                                                                                  |
| [`BIT_CAST_TO_UBIGINT`](bit_functions.md#bit-cast-to-uint64)                       | Cast bits to an `UBIGINT` value. For more information, see [Bit functions](bit_functions.md).                                                                                                                   |
| [`BOOL`](json_functions.md#bool-for-json)                                          | Converts a JSON boolean to a SQL `BOOL` value. For more information, see [JSON functions](json_functions.md).                                                                                                   |
| [`BOOL_ARRAY`](json_functions.md#bool-array-for-json)                              | Converts a JSON array of booleans to a SQL `ARRAY<BOOL>` value. For more information, see [JSON functions](json_functions.md).                                                                                  |
| [`CAST`](conversion_functions.md#cast)                                             | Convert the results of an expression to the given type.                                                                                                                                                         |
| [`CHR`](string_functions.md#chr)                                                   | Converts a Unicode code point to a character. For more information, see [String functions](string_functions.md).                                                                                                |
| [`CODE_POINTS_TO_BYTES`](string_functions.md#code-points-to-bytes)                 | Converts an array of extended ASCII code points to a `VARBINARY` value. For more information, see [String aggregate functions](string_functions.md).                                                            |
| [`CODE_POINTS_TO_STRING`](string_functions.md#code-points-to-string)               | Converts an array of extended ASCII code points to a `VARCHAR` value. For more information, see [String aggregate functions](string_functions.md).                                                              |
| [`DATE_FROM_UNIX_DATE`](date_functions.md#date-from-unix-date)                     | Interprets an `BIGINT` expression as the number of days since 1970-01-01. For more information, see [Date functions](date_functions.md).                                                                        |
| [`FROM_BASE32`](string_functions.md#from-base32)                                   | Converts a base32-encoded `VARCHAR` value into a `VARBINARY` value. For more information, see [String functions](string_functions.md).                                                                          |
| [`FROM_BASE64`](string_functions.md#from-base64)                                   | Converts a base64-encoded `VARCHAR` value into a `VARBINARY` value. For more information, see [String functions](string_functions.md).                                                                          |
| [`FROM_HEX`](string_functions.md#from-hex)                                         | Converts a hexadecimal-encoded `VARCHAR` value into a `VARBINARY` value. For more information, see [String functions](string_functions.md).                                                                     |
| [`INTEGER`](json_functions.md#int32-for-json)                                      | Converts a JSON number to a SQL `INTEGER` value. For more information, see [JSON functions](json_functions.md).                                                                                                 |
| [`INTEGER_ARRAY`](json_functions.md#int32-array-for-json)                          | Converts a JSON number to a SQL `ARRAY<INTEGER>` value. For more information, see [JSON functions](json_functions.md).                                                                                          |
| [`BIGINT`](json_functions.md#int64-for-json)                                       | Converts a JSON number to a SQL `BIGINT` value. For more information, see [JSON functions](json_functions.md).                                                                                                  |
| [`BIGINT_ARRAY`](json_functions.md#int64-array-for-json)                           | Converts a JSON array of numbers to a SQL `ARRAY<BIGINT>` value. For more information, see [JSON functions](json_functions.md).                                                                                 |
| [`LAX_BOOL`](json_functions.md#lax-bool)                                           | Attempts to convert a JSON value to a SQL `BOOL` value. For more information, see [JSON functions](json_functions.md).                                                                                          |
| [`LAX_BOOL_ARRAY`](json_functions.md#lax-bool-array)                               | Attempts to convert a JSON value to a SQL `ARRAY<BOOL>` value. For more information, see [JSON functions](json_functions.md).                                                                                   |
| [`LAX_DOUBLE`](json_functions.md#lax-double)                                       | Attempts to convert a JSON value to a SQL `DOUBLE` value. For more information, see [JSON functions](json_functions.md).                                                                                        |
| [`LAX_DOUBLE_ARRAY`](json_functions.md#lax-double-array)                           | Attempts to convert a JSON value to a SQL `ARRAY<DOUBLE>` value. For more information, see [JSON functions](json_functions.md).                                                                                 |
| [`LAX_FLOAT`](json_functions.md#lax-float)                                         | Attempts to convert a JSON value to a SQL `FLOAT` value. For more information, see [JSON functions](json_functions.md).                                                                                         |
| [`LAX_FLOAT_ARRAY`](json_functions.md#lax-float-array)                             | Attempts to convert a JSON value to a SQL `ARRAY>FLOAT<` value. For more information, see [JSON functions](json_functions.md).                                                                                  |
| [`LAX_INTEGER`](json_functions.md#lax-int32)                                       | Attempts to convert a JSON value to a SQL `INTEGER` value. For more information, see [JSON functions](json_functions.md).                                                                                       |
| [`LAX_INTEGER_ARRAY`](json_functions.md#lax-int32-array)                           | Attempts to convert a JSON value to a SQL `ARRAY<INTEGER>` value. For more information, see [JSON functions](json_functions.md).                                                                                |
| [`LAX_BIGINT`](json_functions.md#lax-int64)                                        | Attempts to convert a JSON value to a SQL `BIGINT` value. For more information, see [JSON functions](json_functions.md).                                                                                        |
| [`LAX_BIGINT_ARRAY`](json_functions.md#lax-int64-array)                            | Attempts to convert a JSON value to a SQL `ARRAY<BIGINT>` value. For more information, see [JSON functions](json_functions.md).                                                                                 |
| [`LAX_STRING`](json_functions.md#lax-string)                                       | Attempts to convert a JSON value to a SQL `VARCHAR` value. For more information, see [JSON functions](json_functions.md).                                                                                       |
| [`LAX_STRING_ARRAY`](json_functions.md#lax-string-array)                           | Attempts to convert a JSON value to a SQL `ARRAY<VARCHAR>`value. For more information, see [JSON functions](json_functions.md).                                                                                 |
| [`LAX_UINTEGER`](json_functions.md#lax-uint32)                                     | Attempts to convert a JSON value to a SQL `UINTEGER` value. For more information, see [JSON functions](json_functions.md).                                                                                      |
| [`LAX_UBIGINT`](json_functions.md#lax-uint64)                                      | Attempts to convert a JSON value to a SQL `UBIGINT` value. For more information, see [JSON functions](json_functions.md).                                                                                       |
| [`LAX_UBIGINT_ARRAY`](json_functions.md#lax-uint64-array)                          | Attempts to convert a JSON value to a SQL `ARRAY<UBIGINT>` value. For more information, see [JSON functions](json_functions.md).                                                                                |
| [`PARSE_NUMERIC`](conversion_functions.md#parse-bignumeric)                        | Converts a `VARCHAR` value to a `NUMERIC` value.                                                                                                                                                                |
| [`PARSE_DATE`](date_functions.md#parse-date)                                       | Converts a `VARCHAR` value to a `DATE` value. For more information, see [Date functions](date_functions.md).                                                                                                    |
| [`PARSE_DATETIME`](datetime_functions.md#parse-datetime)                           | Converts a `VARCHAR` value to a `DATETIME` value. For more information, see [Datetime functions](datetime_functions.md).                                                                                        |
| [`PARSE_JSON`](json_functions.md#parse-json)                                       | Converts a JSON-formatted `VARCHAR` value to a `JSON` value. For more information, see [JSON functions](json_functions.md).                                                                                     |
| [`PARSE_NUMERIC`](conversion_functions.md#parse-numeric)                           | Converts a `VARCHAR` value to a `NUMERIC` value.                                                                                                                                                                |
| [`PARSE_TIME`](time_functions.md#parse-time)                                       | Converts a `VARCHAR` value to a `TIME` value. For more information, see [Time functions](time_functions.md).                                                                                                    |
| [`PARSE_TIMESTAMP`](timestamp_functions.md#parse-timestamp)                        | Converts a `VARCHAR` value to a `TIMESTAMP` value. For more information, see [Timestamp functions](timestamp_functions.md).                                                                                     |
| [`CAST`](conversion_functions.md#safe-casting)                                     | Similar to the `CAST` function, but returns `NULL` when a runtime error is produced.                                                                                                                            |
| [`SAFE_CONVERT_BYTES_TO_STRING`](string_functions.md#safe-convert-bytes-to-string) | Converts a `VARBINARY` value to a `VARCHAR` value and replace any invalid UTF-8 characters with the Unicode replacement character, `U+FFFD`. For more information, see [String functions](string_functions.md). |
| [`VARCHAR` (JSON)](json_functions.md#string-for-json)                              | Converts a JSON string to a SQL `VARCHAR` value. For more information, see [JSON functions](json_functions.md).                                                                                                 |
| [`STRING_ARRAY`](json_functions.md#string-array-for-json)                          | Converts a JSON array of strings to a SQL `ARRAY<VARCHAR>` value. For more information, see [JSON functions](json_functions.md).                                                                                |
| [`VARCHAR` (Timestamp)](timestamp_functions.md#string)                             | Converts a `TIMESTAMP` value to a `VARCHAR` value. For more information, see [Timestamp functions](timestamp_functions.md).                                                                                     |
| [`TIMESTAMP_MICROS`](timestamp_functions.md#timestamp-micros)                      | Converts the number of microseconds since 1970-01-01 00:00:00 UTC to a `TIMESTAMP`. For more information, see [Timestamp functions](timestamp_functions.md).                                                    |
| [`TIMESTAMP_MILLIS`](timestamp_functions.md#timestamp-millis)                      | Converts the number of milliseconds since 1970-01-01 00:00:00 UTC to a `TIMESTAMP`. For more information, see [Timestamp functions](timestamp_functions.md).                                                    |
| [`TIMESTAMP_SECONDS`](timestamp_functions.md#timestamp-seconds)                    | Converts the number of seconds since 1970-01-01 00:00:00 UTC to a `TIMESTAMP`. For more information, see [Timestamp functions](timestamp_functions.md).                                                         |
| [`TO_BASE32`](string_functions.md#to-base32)                                       | Converts a `VARBINARY` value to a base32-encoded `VARCHAR` value. For more information, see [String functions](string_functions.md).                                                                            |
| [`TO_BASE64`](string_functions.md#to-base64)                                       | Converts a `VARBINARY` value to a base64-encoded `VARCHAR` value. For more information, see [String functions](string_functions.md).                                                                            |
| [`TO_CODE_POINTS`](string_functions.md#to-code-points)                             | Converts a `VARCHAR` or `VARBINARY` value into an array of extended ASCII code points. For more information, see [String functions](string_functions.md).                                                       |
| [`TO_HEX`](string_functions.md#to-hex)                                             | Converts a `VARBINARY` value to a hexadecimal `VARCHAR` value. For more information, see [String functions](string_functions.md).                                                                               |
| [`TO_JSON`](json_functions.md#to-json)                                             | Converts a SQL value to a JSON value. For more information, see [JSON functions](json_functions.md).                                                                                                            |
| [`TO_JSON_STRING`](json_functions.md#to-json-string)                               | Converts a SQL value to a JSON-formatted `VARCHAR` value. For more information, see [JSON functions](json_functions.md).                                                                                        |
| [`UINTEGER`](json_functions.md#uint32-for-json)                                    | Converts a JSON number to a SQL `UINTEGER` value. For more information, see [JSON functions](json_functions.md).                                                                                                |
| [`UINTEGER_ARRAY`](json_functions.md#uint32-array-for-json)                        | Converts a JSON number to a SQL `ARRAY<UINTEGER>` value. For more information, see [JSON functions](json_functions.md).                                                                                         |
| [`UBIGINT`](json_functions.md#uint64-for-json)                                     | Converts a JSON number to a SQL `UBIGINT` value. For more information, see [JSON functions](json_functions.md).                                                                                                 |
| [`UBIGINT_ARRAY`](json_functions.md#uint64-array-for-json)                         | Converts a JSON number to a SQL `ARRAY<UBIGINT>` value. For more information, see [JSON functions](json_functions.md).                                                                                          |
| [`UNIX_DATE`](date_functions.md#unix-date)                                         | Converts a `DATE` value to the number of days since 1970-01-01. For more information, see [Date functions](date_functions.md).                                                                                  |
| [`UNIX_MICROS`](timestamp_functions.md#unix-micros)                                | Converts a `TIMESTAMP` value to the number of microseconds since 1970-01-01 00:00:00 UTC. For more information, see [Timestamp functions](timestamp_functions.md).                                              |
| [`UNIX_MILLIS`](timestamp_functions.md#unix-millis)                                | Converts a `TIMESTAMP` value to the number of milliseconds since 1970-01-01 00:00:00 UTC. For more information, see [Timestamp functions](timestamp_functions.md).                                              |
| [`UNIX_SECONDS`](timestamp_functions.md#unix-seconds)                              | Converts a `TIMESTAMP` value to the number of seconds since 1970-01-01 00:00:00 UTC. For more information, see [Timestamp functions](timestamp_functions.md).                                                   |

## `BIT_CAST_TO_INTEGER`

```sql
BIT_CAST_TO_INTEGER(value)
```

**Description**

SQL supports bit casting to `INTEGER`. A bit
cast is a cast in which the order of bits is preserved instead of the value
those bytes represent.

The `value` parameter can represent:

- `INTEGER`
- `UINTEGER`

**Return Data Type**

`INTEGER`

**Examples**

```sql
SELECT BIT_CAST_TO_UINTEGER(-1) as UINTEGER_value, BIT_CAST_TO_INTEGER(BIT_CAST_TO_UINTEGER(-1)) as bit_cast_value;

/*---------------+----------------------+
 | UINTEGER_value  | bit_cast_value       |
 +---------------+----------------------+
 | 4294967295    | -1                   |
 +---------------+----------------------*/
```

## `BIT_CAST_TO_BIGINT`

```sql
BIT_CAST_TO_BIGINT(value)
```

**Description**

SQL supports bit casting to `BIGINT`. A bit
cast is a cast in which the order of bits is preserved instead of the value
those bytes represent.

The `value` parameter can represent:

- `BIGINT`
- `UBIGINT`

**Return Data Type**

`BIGINT`

**Example**

```sql
SELECT BIT_CAST_TO_UBIGINT(-1) as UBIGINT_value, BIT_CAST_TO_BIGINT(BIT_CAST_TO_UBIGINT(-1)) as bit_cast_value;

/*-----------------------+----------------------+
 | UBIGINT_value          | bit_cast_value       |
 +-----------------------+----------------------+
 | 18446744073709551615  | -1                   |
 +-----------------------+----------------------*/
```

## `BIT_CAST_TO_UINTEGER`

```sql
BIT_CAST_TO_UINTEGER(value)
```

**Description**

SQL supports bit casting to `UINTEGER`. A bit
cast is a cast in which the order of bits is preserved instead of the value
those bytes represent.

The `value` parameter can represent:

- `INTEGER`
- `UINTEGER`

**Return Data Type**

`UINTEGER`

**Examples**

```sql
SELECT -1 as UINTEGER_value, BIT_CAST_TO_UINTEGER(-1) as bit_cast_value;

/*--------------+----------------------+
 | UINTEGER_value | bit_cast_value       |
 +--------------+----------------------+
 | -1           | 4294967295           |
 +--------------+----------------------*/
```

## `BIT_CAST_TO_UBIGINT`

```sql
BIT_CAST_TO_UBIGINT(value)
```

**Description**

SQL supports bit casting to `UBIGINT`. A bit
cast is a cast in which the order of bits is preserved instead of the value
those bytes represent.

The `value` parameter can represent:

- `BIGINT`
- `UBIGINT`

**Return Data Type**

`UBIGINT`

**Example**

```sql
SELECT -1 as BIGINT_value, BIT_CAST_TO_UBIGINT(-1) as bit_cast_value;

/*--------------+----------------------+
 | BIGINT_value  | bit_cast_value       |
 +--------------+----------------------+
 | -1           | 18446744073709551615 |
 +--------------+----------------------*/
```

## `CAST`

```sql
CAST(expression AS typename [format_clause])
```

**Description**

Cast syntax is used in a query to indicate that the result type of an
expression should be converted to some other type.

When using `CAST`, a query can fail if SQL is unable to perform
the cast. If you want to protect your queries from these types of errors, you
can use [CAST][con-func-safecast].

Casts between supported types that don't successfully map from the original
value to the target domain produce runtime errors. For example, casting
`VARBINARY` to `VARCHAR` where the byte sequence isn't valid UTF-8 results in a
runtime error.

Other examples include:

- Casting `BIGINT` to `INTEGER` where the value overflows `INTEGER`.
- Casting `VARCHAR` to `INTEGER` where the `VARCHAR` contains non-digit characters.

Some casts can include a [format clause][formatting-syntax], which provides
instructions for how to conduct the
cast. For example, you could
instruct a cast to convert a sequence of bytes to a BASE64-encoded string
instead of a UTF-8-encoded string.

The structure of the format clause is unique to each type of cast and more
information is available in the section for that cast.

**Examples**

The following query results in `"true"` if `x` is `1`, `"false"` for any other
non-`NULL` value, and `NULL` if `x` is `NULL`.

```sql
CAST(x=1 AS VARCHAR)
```

### CAST AS ARRAY

```sql
CAST(expression AS ARRAY)
```

**Description**

SQL supports [casting][con-func-cast] to `ARRAY`. The
`expression` parameter can represent an expression for these data types:

- `ARRAY`

**Conversion rules**

| From    | To      | Rule(s) when casting `x`                                                                                                                                                                                                                                             |
| ------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ARRAY` | `ARRAY` | The element types of the input array must be castable to the element types of the target array. For example, casting from type `ARRAY<BIGINT>` to `ARRAY<DOUBLE>` or `ARRAY<VARCHAR>` is valid; casting from type `ARRAY<BIGINT>` to `ARRAY<VARBINARY>` isn't valid. |

### CAST AS NUMERIC

```sql
CAST(expression AS NUMERIC)
```

**Description**

SQL supports [casting][con-func-cast] to `NUMERIC`. The
`expression` parameter can represent an expression for these data types:

- `INTEGER`
- `UINTEGER`
- `BIGINT`
- `UBIGINT`
- `FLOAT`
- `DOUBLE`
- `NUMERIC`
- `NUMERIC`
- `VARCHAR`

**Conversion rules**

| From           | To        | Rule(s) when casting `x`                                                                                                                                                                                                                                                                                                                                                               |
| -------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Floating Point | `NUMERIC` | The floating point number will round [half away from zero](https://en.wikipedia.org/wiki/Rounding#Round-half-away-from-zero). Casting a `NaN`, `+inf` or `-inf` will return an error. Casting a value outside the range of `NUMERIC` returns an overflow error.                                                                                                                        |
| `VARCHAR`      | `NUMERIC` | The numeric literal contained in the string must not exceed the maximum precision or range of the `NUMERIC` type, or an error will occur. If the number of digits after the decimal point exceeds 38, then the resulting `NUMERIC` value will round [half away from zero](https://en.wikipedia.org/wiki/Rounding#Round-half-away-from-zero) to have 38 digits after the decimal point. |

### CAST AS BOOL

```sql
CAST(expression AS BOOL)
```

**Description**

SQL supports [casting][con-func-cast] to `BOOL`. The
`expression` parameter can represent an expression for these data types:

- `INTEGER`
- `UINTEGER`
- `BIGINT`
- `UBIGINT`
- `BOOL`
- `VARCHAR`

**Conversion rules**

| From      | To     | Rule(s) when casting `x`                                                                                                                                                                                             |
| --------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Integer   | `BOOL` | Returns `FALSE` if `x` is `0`, `TRUE` otherwise.                                                                                                                                                                     |
| `VARCHAR` | `BOOL` | Returns `TRUE` if `x` is `"true"` and `FALSE` if `x` is `"false"` All other values of `x` are invalid and throw an error instead of casting to a boolean. A string is case-insensitive when converting to a boolean. |

### CAST AS VARBINARY

```sql
CAST(expression AS VARBINARY [format_clause])
```

**Description**

SQL supports [casting][con-func-cast] to `VARBINARY`. The
`expression` parameter can represent an expression for these data types:

- `VARBINARY`
- `VARCHAR`
- **Format clause**

When an expression of one type is cast to another type, you can use the
[format clause][formatting-syntax] to provide instructions for how to conduct
the cast. You can use the format clause in this section if `expression` is a
`VARCHAR`.

- [Format string as bytes][format-string-as-bytes]

**Conversion rules**

| From      | To          | Rule(s) when casting `x`                                                                                                                                            |
| --------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VARCHAR` | `VARBINARY` | Strings are cast to bytes using UTF-8 encoding. For example, the string "&copy;", when cast to bytes, would become a 2-byte sequence with the hex values C2 and A9. |
|           | `VARBINARY` | Returns the proto2 wire format bytes of `x`.                                                                                                                        |

### CAST AS DATE

```sql
CAST(expression AS DATE [format_clause])
```

**Description**

SQL supports [casting][con-func-cast] to `DATE`. The `expression`
parameter can represent an expression for these data types:

- `VARCHAR`
- `DATETIME`
- `TIMESTAMP`

**Format clause**

When an expression of one type is cast to another type, you can use the
[format clause][formatting-syntax] to provide instructions for how to conduct
the cast. You can use the format clause in this section if `expression` is a
`VARCHAR`.

- [Format string as date and time][format-string-as-date-time]

**Conversion rules**

| From        | To     | Rule(s) when casting `x`                                                                                                                                                                                                                                            |
| ----------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VARCHAR`   | `DATE` | When casting from string to date, the string must conform to the supported date literal format, and is independent of time zone. If the string expression is invalid or represents a date that's outside of the supported min/max range, then an error is produced. |
| `TIMESTAMP` | `DATE` | Casting from a timestamp to date effectively truncates the timestamp as of the default time zone.                                                                                                                                                                   |

### CAST AS DATETIME

```sql
CAST(expression AS DATETIME [format_clause])
```

**Description**

SQL supports [casting][con-func-cast] to `DATETIME`. The
`expression` parameter can represent an expression for these data types:

- `VARCHAR`
- `DATETIME`
- `TIMESTAMP`

**Format clause**

When an expression of one type is cast to another type, you can use the
[format clause][formatting-syntax] to provide instructions for how to conduct
the cast. You can use the format clause in this section if `expression` is a
`VARCHAR`.

- [Format string as date and time][format-string-as-date-time]

**Conversion rules**

| From        | To         | Rule(s) when casting `x`                                                                                                                                                                                                                                                        |
| ----------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VARCHAR`   | `DATETIME` | When casting from string to datetime, the string must conform to the supported datetime literal format, and is independent of time zone. If the string expression is invalid or represents a datetime that's outside of the supported min/max range, then an error is produced. |
| `TIMESTAMP` | `DATETIME` | Casting from a timestamp to datetime effectively truncates the timestamp as of the default time zone.                                                                                                                                                                           |

### CAST AS ENUM

```sql
CAST(expression AS ENUM)
```

**Description**

SQL supports [casting][con-func-cast] to `ENUM`. The `expression`
parameter can represent an expression for these data types:

- `INTEGER`
- `UINTEGER`
- `BIGINT`
- `UBIGINT`
- `VARCHAR`
- `ENUM`

**Conversion rules**

| From   | To     | Rule(s) when casting `x`      |
| ------ | ------ | ----------------------------- |
| `ENUM` | `ENUM` | Must have the same enum name. |

### CAST AS Floating Point

```sql
CAST(expression AS DOUBLE)
```

```sql
CAST(expression AS FLOAT)
```

**Description**

SQL supports [casting][con-func-cast] to floating point types.
The `expression` parameter can represent an expression for these data types:

- `INTEGER`
- `UINTEGER`
- `BIGINT`
- `UBIGINT`
- `FLOAT`
- `DOUBLE`
- `NUMERIC`
- `NUMERIC`
- `VARCHAR`

**Conversion rules**

| From      | To             | Rule(s) when casting `x`                                                                                                                                                                                                                                                     |
| --------- | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Integer   | Floating Point | Returns a close but potentially not exact floating point value.                                                                                                                                                                                                              |
| `NUMERIC` | Floating Point | `NUMERIC` will convert to the closest floating point number with a possible loss of precision.                                                                                                                                                                               |
| `NUMERIC` | Floating Point | `NUMERIC` will convert to the closest floating point number with a possible loss of precision.                                                                                                                                                                               |
| `VARCHAR` | Floating Point | Returns `x` as a floating point value, interpreting it as having the same form as a valid floating point literal. Also supports casts from `"[+,-]inf"` to `[,-]Infinity`, `"[+,-]infinity"` to `[,-]Infinity`, and `"[+,-]nan"` to `NaN`. Conversions are case-insensitive. |

### CAST AS Integer

```sql
CAST(expression AS INTEGER)
```

```sql
CAST(expression AS UINTEGER)
```

```sql
CAST(expression AS BIGINT)
```

```sql
CAST(expression AS UBIGINT)
```

**Description**

SQL supports [casting][con-func-cast] to integer types.
The `expression` parameter can represent an expression for these data types:

- `INTEGER`
- `UINTEGER`
- `BIGINT`
- `UBIGINT`
- `FLOAT`
- `DOUBLE`
- `NUMERIC`
- `NUMERIC`
- `ENUM`
- `BOOL`
- `VARCHAR`

**Conversion rules**

| From           | To      | Rule(s) when casting `x`                                                                     |
| -------------- | ------- | -------------------------------------------------------------------------------------------- |
| Floating Point | Integer | Returns the closest integer value. Halfway cases such as 1.5 or -0.5 round away from zero.   |
| `BOOL`         | Integer | Returns `1` if `x` is `TRUE`, `0` otherwise.                                                 |
| `VARCHAR`      | Integer | A hex string can be cast to an integer. For example, `0x123` to `291` or `-0x123` to `-291`. |

**Examples**

If you are working with hex strings (`0x123`), you can cast those strings as
integers:

```sql
SELECT '0x123' as hex_value, CAST('0x123' as BIGINT) as hex_to_int;

/*-----------+------------+
 | hex_value | hex_to_int |
 +-----------+------------+
 | 0x123     | 291        |
 +-----------+------------*/
```

```sql
SELECT '-0x123' as hex_value, CAST('-0x123' as BIGINT) as hex_to_int;

/*-----------+------------+
 | hex_value | hex_to_int |
 +-----------+------------+
 | -0x123    | -291       |
 +-----------+------------*/
```

### CAST AS INTERVAL

```sql
CAST(expression AS INTERVAL)
```

**Description**

SQL supports [casting][con-func-cast] to `INTERVAL`. The
`expression` parameter can represent an expression for these data types:

- `VARCHAR`

**Conversion rules**

| From      | To         | Rule(s) when casting `x`                                                                                                                                                                                                                                                                                                                                                                                                                           |
| --------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VARCHAR` | `INTERVAL` | When casting from string to interval, the string must conform to either [ISO 8601 Duration](https://en.wikipedia.org/wiki/ISO_8601#Durations) standard or to interval literal format 'Y-M D H:M:S.F'. Partial interval literal formats are also accepted when they aren't ambiguous, for example 'H:M:S'. If the string expression is invalid or represents an interval that is outside of the supported min/max range, then an error is produced. |

**Examples**

```sql
SELECT input, CAST(input AS INTERVAL) AS output
FROM UNNEST([
  '1-2 3 10:20:30.456',
  '1-2',
  '10:20:30',
  'P1Y2M3D',
  'PT10H20M30,456S'
]) input

/*--------------------+--------------------+
 | input              | output             |
 +--------------------+--------------------+
 | 1-2 3 10:20:30.456 | 1-2 3 10:20:30.456 |
 | 1-2                | 1-2 0 0:0:0        |
 | 10:20:30           | 0-0 0 10:20:30     |
 | P1Y2M3D            | 1-2 3 0:0:0        |
 | PT10H20M30,456S    | 0-0 0 10:20:30.456 |
 +--------------------+--------------------*/
```

### CAST AS NUMERIC

```sql
CAST(expression AS NUMERIC)
```

**Description**

SQL supports [casting][con-func-cast] to `NUMERIC`. The
`expression` parameter can represent an expression for these data types:

- `INTEGER`
- `UINTEGER`
- `BIGINT`
- `UBIGINT`
- `FLOAT`
- `DOUBLE`
- `NUMERIC`
- `NUMERIC`
- `VARCHAR`

**Conversion rules**

| From           | To        | Rule(s) when casting `x`                                                                                                                                                                                                                                                                                                                                                                   |
| -------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Floating Point | `NUMERIC` | The floating point number will round [half away from zero](https://en.wikipedia.org/wiki/Rounding#Round-half-away-from-zero). Casting a `NaN`, `+inf` or `-inf` will return an error. Casting a value outside the range of `NUMERIC` returns an overflow error.                                                                                                                            |
| `VARCHAR`      | `NUMERIC` | The numeric literal contained in the string must not exceed the maximum precision or range of the `NUMERIC` type, or an error will occur. If the number of digits after the decimal point exceeds nine, then the resulting `NUMERIC` value will round [half away from zero](https://en.wikipedia.org/wiki/Rounding#Round-half-away-from-zero) to have nine digits after the decimal point. |

### CAST AS

```sql
CAST(expression AS )
```

**Description**

SQL supports [casting][con-func-cast] to . The
`expression` parameter can represent an expression for these data types:

- `VARCHAR`
- `VARBINARY`
- **Conversion rules**

| From        | To  | Rule(s) when casting `x`                                                                                                                           |
| ----------- | --- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VARCHAR`   |     | Returns the that results from parsing from proto2 text format. Throws an error if parsing fails, e.g., if not all required fields are set.         |
| `VARBINARY` |     | Returns the that results from parsing `x` from the proto2 wire format. Throws an error if parsing fails, e.g., if not all required fields are set. |
|             |     | Must have the same name.                                                                                                                           |

**Example**

This example references a called `Award`.

```sqlproto
message Award {
  required int32 year = 1;
  optional int32 month = 2;
  repeated Type type = 3;

  message Type {
    optional string award_name = 1;
    optional string category = 2;
  }
}
```

```sql
SELECT
  CAST(
    '''
    year: 2001
    month: 9
    type { award_name: 'Best Artist' category: 'Artist' }
    type { award_name: 'Best Album' category: 'Album' }
    '''
    AS )
  AS award_col

/*---------------------------------------------------------+
 | award_col                                               |
 +---------------------------------------------------------+
 | {                                                       |
 |   year: 2001                                            |
 |   month: 9                                              |
 |   type { award_name: "Best Artist" category: "Artist" } |
 |   type { award_name: "Best Album" category: "Album" }   |
 | }                                                       |
 +---------------------------------------------------------*/
```

### CAST AS RANGE

```sql
CAST(expression AS RANGE)
```

**Description**

SQL supports [casting][con-func-cast] to `RANGE`. The
`expression` parameter can represent an expression for these data types:

- `VARCHAR`

**Conversion rules**

| From      | To      | Rule(s) when casting `x`                                                                                                                                                                                                                      |
| --------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VARCHAR` | `RANGE` | When casting from string to range, the string must conform to the supported range literal format. If the string expression is invalid or represents a range that's outside of the supported subtype min/max range, then an error is produced. |

**Examples**

```sql
SELECT CAST(
  '[2020-01-01, 2020-01-02)'
  AS RANGE) AS string_to_range

/*----------------------------------------+
 | string_to_range                        |
 +----------------------------------------+
 | [DATE '2020-01-01', DATE '2020-01-02') |
 +----------------------------------------*/
```

```sql
SELECT CAST(
  '[2014-09-27 12:30:00.45, 2016-10-17 11:15:00.33)'
  AS RANGE) AS string_to_range

/*------------------------------------------------------------------------+
 | string_to_range                                                        |
 +------------------------------------------------------------------------+
 | [DATETIME '2014-09-27 12:30:00.45', DATETIME '2016-10-17 11:15:00.33') |
 +------------------------------------------------------------------------*/
```

```sql
SELECT CAST(
  '[2014-09-27 12:30:00+08, 2016-10-17 11:15:00+08)'
  AS RANGE) AS string_to_range

-- Results depend upon where this query was executed.
/*--------------------------------------------------------------------------+
 | string_to_range                                                          |
 +--------------------------------------------------------------------------+
 | [TIMESTAMP '2014-09-27 12:30:00+08', TIMESTAMP '2016-10-17 11:15:00+08') |
 +--------------------------------------------------------------------------*/
```

```sql
SELECT CAST(
  '[UNBOUNDED, 2020-01-02)'
  AS RANGE) AS string_to_range

/*--------------------------------+
 | string_to_range                |
 +--------------------------------+
 | [UNBOUNDED, DATE '2020-01-02') |
 +--------------------------------*/
```

```sql
SELECT CAST(
  '[2020-01-01, NULL)'
  AS RANGE) AS string_to_range

/*--------------------------------+
 | string_to_range                |
 +--------------------------------+
 | [DATE '2020-01-01', UNBOUNDED) |
 +--------------------------------*/
```

### CAST AS VARCHAR

```sql
CAST(expression AS VARCHAR [format_clause [AT TIME ZONE timezone_expr]])
```

**Description**

SQL supports [casting][con-func-cast] to `VARCHAR`. The
`expression` parameter can represent an expression for these data types:

- `INTEGER`
- `UINTEGER`
- `BIGINT`
- `UBIGINT`
- `FLOAT`
- `DOUBLE`
- `NUMERIC`
- `NUMERIC`
- `ENUM`
- `BOOL`
- `VARBINARY`
-
- `TIME`
- `DATE`
- `DATETIME`
- `TIMESTAMP`
- `RANGE`
- `INTERVAL`
- `VARCHAR`

**Format clause**

When an expression of one type is cast to another type, you can use the
[format clause][formatting-syntax] to provide instructions for how to conduct
the cast. You can use the format clause in this section if `expression` is one
of these data types:

- `INTEGER`
- `UINTEGER`
- `BIGINT`
- `UBIGINT`
- `FLOAT`
- `DOUBLE`
- `NUMERIC`
- `NUMERIC`
- `VARBINARY`
- `TIME`
- `DATE`
- `DATETIME`
- `TIMESTAMP`

The format clause for `VARCHAR` has an additional optional clause called
`AT TIME ZONE timezone_expr`, which you can use to specify a specific time zone
to use during formatting of a `TIMESTAMP`. If this optional clause isn't
included when formatting a `TIMESTAMP`, the default time zone,
which is implementation defined, is used.

For more information, see the following topics:

- [Format bytes as string][format-bytes-as-string]
- [Format date and time as string][format-date-time-as-string]
- [Format numeric type as string][format-numeric-type-as-string]

**Conversion rules**

| From           | To        | Rule(s) when casting `x`                                                                                                                                                                                                                                                                                    |
| -------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Floating Point | `VARCHAR` | Returns an approximate string representation. A returned `NaN` or `0` will not be signed.                                                                                                                                                                                                                   |
| `BOOL`         | `VARCHAR` | Returns `"true"` if `x` is `TRUE`, `"false"` otherwise.                                                                                                                                                                                                                                                     |
| `VARBINARY`    | `VARCHAR` | Returns `x` interpreted as a UTF-8 string. For example, the bytes literal `b'\xc2\xa9'`, when cast to a string, is interpreted as UTF-8 and becomes the unicode character "&copy;". An error occurs if `x` isn't valid UTF-8.                                                                               |
| `ENUM`         | `VARCHAR` | Returns the canonical enum value name of `x`. If an enum value has multiple names (aliases), the canonical name/alias for that value is used.                                                                                                                                                               |
|                | `VARCHAR` | Returns the proto2 text format representation of `x`.                                                                                                                                                                                                                                                       |
| `TIME`         | `VARCHAR` | Casting from a time type to a string is independent of time zone and is of the form `HH:MM:SS`.                                                                                                                                                                                                             |
| `DATE`         | `VARCHAR` | Casting from a date type to a string is independent of time zone and is of the form `YYYY-MM-DD`.                                                                                                                                                                                                           |
| `DATETIME`     | `VARCHAR` | Casting from a datetime type to a string is independent of time zone and is of the form `YYYY-MM-DD HH:MM:SS`.                                                                                                                                                                                              |
| `TIMESTAMP`    | `VARCHAR` | When casting from timestamp types to string, the timestamp is interpreted using the default time zone, which is implementation defined. The number of subsecond digits produced depends on the number of trailing zeroes in the subsecond part: the CAST function will truncate zero, three, or six digits. |
| `INTERVAL`     | `VARCHAR` | Casting from an interval to a string is of the form `Y-M D H:M:S`.                                                                                                                                                                                                                                          |

**Examples**

```sql
SELECT CAST(CURRENT_DATE() AS VARCHAR) AS current_date

/*---------------+
 | current_date  |
 +---------------+
 | 2021-03-09    |
 +---------------*/
```

```sql
SELECT CAST(CURRENT_DATE() AS VARCHAR FORMAT 'DAY') AS current_day

/*-------------+
 | current_day |
 +-------------+
 | MONDAY      |
 +-------------*/
```

```sql
SELECT CAST(
  TIMESTAMP '2008-12-25 00:00:00+00:00'
  AS VARCHAR FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM') AS date_time_to_string

-- Results depend upon where this query was executed.
/*------------------------------+
 | date_time_to_string          |
 +------------------------------+
 | 2008-12-24 16:00:00 -08:00   |
 +------------------------------*/
```

```sql
SELECT CAST(
  TIMESTAMP '2008-12-25 00:00:00+00:00'
  AS VARCHAR FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM'
  AT TIME ZONE 'Asia/Kolkata') AS date_time_to_string

-- Because the time zone is specified, the result is always the same.
/*------------------------------+
 | date_time_to_string          |
 +------------------------------+
 | 2008-12-25 05:30:00 +05:30   |
 +------------------------------*/
```

```sql
SELECT CAST(INTERVAL 3 DAY AS VARCHAR) AS interval_to_string

/*--------------------+
 | interval_to_string |
 +--------------------+
 | 0-0 3 0:0:0        |
 +--------------------*/
```

```sql
SELECT CAST(
  INTERVAL "1-2 3 4:5:6.789" YEAR TO SECOND
  AS VARCHAR) AS interval_to_string

/*--------------------+
 | interval_to_string |
 +--------------------+
 | 1-2 3 4:5:6.789    |
 +--------------------*/
```

### CAST AS STRUCT

```sql
CAST(expression AS STRUCT)
```

**Description**

SQL supports [casting][con-func-cast] to `STRUCT`. The
`expression` parameter can represent an expression for these data types:

- `STRUCT`

**Conversion rules**

| From                                                                                                                                                  | To       | Rule(s) when casting `x`                                                                        |
| ----------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ----------------------------------------------------------------------------------------------- |
| `STRUCT`                                                                                                                                              | `STRUCT` | Allowed if the following conditions are met: 1. The two structs have the same number of fields. |
| 2. The original struct field types can be explicitly cast to the corresponding target struct field types (as defined by field order, not field name). |

### CAST AS TIME

```sql
CAST(expression AS TIME [format_clause])
```

**Description**

SQL supports [casting][con-func-cast] to TIME. The `expression`
parameter can represent an expression for these data types:

- `VARCHAR`
- `TIME`
- `DATETIME`
- `TIMESTAMP`

**Format clause**

When an expression of one type is cast to another type, you can use the
[format clause][formatting-syntax] to provide instructions for how to conduct
the cast. You can use the format clause in this section if `expression` is a
`VARCHAR`.

- [Format string as date and time][format-string-as-date-time]

**Conversion rules**

| From      | To     | Rule(s) when casting `x`                                                                                                                                                                                                                                            |
| --------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VARCHAR` | `TIME` | When casting from string to time, the string must conform to the supported time literal format, and is independent of time zone. If the string expression is invalid or represents a time that's outside of the supported min/max range, then an error is produced. |

### CAST AS TIMESTAMP

```sql
CAST(expression AS TIMESTAMP [format_clause [AT TIME ZONE timezone_expr]])
```

**Description**

SQL supports [casting][con-func-cast] to `TIMESTAMP`. The
`expression` parameter can represent an expression for these data types:

- `VARCHAR`
- `DATETIME`
- `TIMESTAMP`

**Format clause**

When an expression of one type is cast to another type, you can use the
[format clause][formatting-syntax] to provide instructions for how to conduct
the cast. You can use the format clause in this section if `expression` is a
`VARCHAR`.

- [Format string as date and time][format-string-as-date-time]

The format clause for `TIMESTAMP` has an additional optional clause called
`AT TIME ZONE timezone_expr`, which you can use to specify a specific time zone
to use during formatting. If this optional clause isn't included, the default
time zone, which is implementation defined, is used.

**Conversion rules**

| From       | To          | Rule(s) when casting `x`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VARCHAR`  | `TIMESTAMP` | When casting from string to a timestamp, `string_expression` must conform to the supported timestamp literal formats, or else a runtime error occurs. The `string_expression` may itself contain a time zone. If there is a time zone in the `string_expression`, that time zone is used for conversion, otherwise the default time zone, which is implementation defined, is used. If the string has fewer than six digits, then it's implicitly widened. An error is produced if the `string_expression` is invalid, has more than six subsecond digits (i.e., precision greater than microseconds), or represents a time outside of the supported timestamp range. |
| `DATE`     | `TIMESTAMP` | Casting from a date to a timestamp interprets `date_expression` as of midnight (start of the day) in the default time zone, which is implementation defined.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `DATETIME` | `TIMESTAMP` | Casting from a datetime to a timestamp interprets `datetime_expression` in the default time zone, which is implementation defined. Most valid datetime values have exactly one corresponding timestamp in each time zone. However, there are certain combinations of valid datetime values and time zones that have zero or two corresponding timestamp values. This happens in a time zone when clocks are set forward or set back, such as for Daylight Savings Time. When there are two valid timestamps, the earlier one is used. When there is no valid timestamp, the length of the gap in time (typically one hour) is added to the datetime.                  |

**Examples**

The following example casts a string-formatted timestamp as a timestamp:

```sql
SELECT CAST("2020-06-02 17:00:53.110+00:00" AS TIMESTAMP) AS as_timestamp

-- Results depend upon where this query was executed.
/*----------------------------+
 | as_timestamp               |
 +----------------------------+
 | 2020-06-03 00:00:53.110+00 |
 +----------------------------*/
```

The following examples cast a string-formatted date and time as a timestamp.
These examples return the same output as the previous example.

```sql
SELECT CAST('06/02/2020 17:00:53.110' AS TIMESTAMP FORMAT 'MM/DD/YYYY HH24:MI:SS.FF3' AT TIME ZONE 'UTC') AS as_timestamp
```

```sql
SELECT CAST('06/02/2020 17:00:53.110' AS TIMESTAMP FORMAT 'MM/DD/YYYY HH24:MI:SS.FF3' AT TIME ZONE '+00') AS as_timestamp
```

```sql
SELECT CAST('06/02/2020 17:00:53.110 +00' AS TIMESTAMP FORMAT 'MM/DD/YYYY HH24:MI:SS.FF3 TZH') AS as_timestamp
```

[formatting-syntax]: ../types/format_elements.md#formatting-syntax
[format-string-as-bytes]: ../types/format_elements.md#format-string_as_bytes
[format-bytes-as-string]: ../types/format_elements.md#format-bytes_as_string
[format-date-time-as-string]: ../types/format_elements.md#format-date_time_as_string
[format-string-as-date-time]: ../types/format_elements.md#format-string_as_datetime
[format-numeric-type-as-string]: ../types/format_elements.md#format-numeric_type_as_string
[con-func-cast]: #cast
[con-func-safecast]: #safe-casting

## `PARSE_NUMERIC`

```sql
PARSE_NUMERIC(string_expression)
```

**Description**

Converts a `VARCHAR` to a `NUMERIC` value.

The numeric literal contained in the string must not exceed the
[maximum precision or range][bignumeric-type] of the `NUMERIC` type, or an
error occurs. If the number of digits after the decimal point exceeds 38, then
the resulting `NUMERIC` value rounds
[half away from zero][half-from-zero-wikipedia] to have 38 digits after the
decimal point.

```

-- This example shows how a string with a decimal point is parsed.
SELECT PARSE_NUMERIC("123.45") AS parsed;

/*--------+
 | parsed |
 +--------+
 | 123.45 |
 +--------*/

-- This example shows how a string with an exponent is parsed.
SELECT PARSE_NUMERIC("123.456E37") AS parsed;

/*-----------------------------------------+
 | parsed                                  |
 +-----------------------------------------+
 | 123400000000000000000000000000000000000 |
 +-----------------------------------------*/

-- This example shows the rounding when digits after the decimal point exceeds 38.
SELECT PARSE_NUMERIC("1.123456789012345678901234567890123456789") as parsed;

/*------------------------------------------+
 | parsed                                   |
 +------------------------------------------+
 | 1.12345678901234567890123456789012345679 |
 +------------------------------------------*/
```

This function is similar to using the [`CAST AS NUMERIC`][cast-bignumeric]
function except that the `PARSE_NUMERIC` function only accepts string inputs
and allows the following in the string:

- Spaces between the sign (+/-) and the number
- Signs (+/-) after the number

Rules for valid input strings:

| Rule                                                                                          | Example Input     | Output     |
| --------------------------------------------------------------------------------------------- | ----------------- | ---------- |
| The string can only contain digits, commas, decimal points and signs.                         | "- 12,34567,89.0" | -123456789 |
| Whitespaces are allowed anywhere except between digits.                                       | " - 12.345 "      | -12.345    |
| Only digits and commas are allowed before the decimal point.                                  | " 12,345,678"     | 12345678   |
| Only digits are allowed after the decimal point.                                              | "1.234 "          | 1.234      |
| Use `E` or `e` for exponents. After the `e`, digits and a leading sign indicator are allowed. | " 123.45e-1"      | 12.345     |
| If the integer part isn't empty, then it must contain at least one digit.                     | " 0,.12 -"        | -0.12      |
| If the string contains a decimal point, then it must contain at least one digit.              | " .1"             | 0.1        |
| The string can't contain more than one sign.                                                  | " 0.5 +"          | 0.5        |

**Return Data Type**

`NUMERIC`

**Examples**

This example shows an input with spaces before, after, and between the
sign and the number:

```sql
SELECT PARSE_NUMERIC("  -  12.34 ") as parsed;

/*--------+
 | parsed |
 +--------+
 | -12.34 |
 +--------*/
```

This example shows an input with an exponent as well as the sign after the
number:

```sql
SELECT PARSE_NUMERIC("12.34e-1-") as parsed;

/*--------+
 | parsed |
 +--------+
 | -1.234 |
 +--------*/
```

This example shows an input with multiple commas in the integer part of the
number:

```sql
SELECT PARSE_NUMERIC("  1,2,,3,.45 + ") as parsed;

/*--------+
 | parsed |
 +--------+
 | 123.45 |
 +--------*/
```

This example shows an input with a decimal point and no digits in the whole
number part:

```sql
SELECT PARSE_NUMERIC(".1234  ") as parsed;

/*--------+
 | parsed |
 +--------+
 | 0.1234 |
 +--------*/
```

**Examples of invalid inputs**

This example is invalid because the whole number part contains no digits:

```sql
SELECT PARSE_NUMERIC(",,,.1234  ") as parsed;
```

This example is invalid because there are whitespaces between digits:

```sql
SELECT PARSE_NUMERIC("1  23.4 5  ") as parsed;
```

This example is invalid because the number is empty except for an exponent:

```sql
SELECT PARSE_NUMERIC("  e1 ") as parsed;
```

This example is invalid because the string contains multiple signs:

```sql
SELECT PARSE_NUMERIC("  - 12.3 - ") as parsed;
```

This example is invalid because the value of the number falls outside the range
of `NUMERIC`:

```sql
SELECT PARSE_NUMERIC("12.34E100 ") as parsed;
```

This example is invalid because the string contains invalid characters:

```sql
SELECT PARSE_NUMERIC("$12.34") as parsed;
```

[half-from-zero-wikipedia]: https://en.wikipedia.org/wiki/Rounding#Round_half_away_from_zero
[cast-bignumeric]: #cast-bignumeric
[bignumeric-type]: ../types/data_types.md#decimal-types

## `PARSE_NUMERIC`

```sql
PARSE_NUMERIC(string_expression)
```

**Description**

Converts a `VARCHAR` to a `NUMERIC` value.

The numeric literal contained in the string must not exceed the
[maximum precision or range][numeric-type] of the `NUMERIC` type, or an error
occurs. If the number of digits after the decimal point exceeds nine, then the
resulting `NUMERIC` value rounds
[half away from zero][half-from-zero-wikipedia] to have nine digits after the
decimal point.

```

-- This example shows how a string with a decimal point is parsed.
SELECT PARSE_NUMERIC("123.45") AS parsed;

/*--------+
 | parsed |
 +--------+
 | 123.45 |
 +--------*/

-- This example shows how a string with an exponent is parsed.
SELECT PARSE_NUMERIC("12.34E27") as parsed;

/*-------------------------------+
 | parsed                        |
 +-------------------------------+
 | 12340000000000000000000000000 |
 +-------------------------------*/

-- This example shows the rounding when digits after the decimal point exceeds 9.
SELECT PARSE_NUMERIC("1.0123456789") as parsed;

/*-------------+
 | parsed      |
 +-------------+
 | 1.012345679 |
 +-------------*/
```

This function is similar to using the [`CAST AS NUMERIC`][cast-numeric] function
except that the `PARSE_NUMERIC` function only accepts string inputs and allows
the following in the string:

- Spaces between the sign (+/-) and the number
- Signs (+/-) after the number

Rules for valid input strings:

| Rule                                                                                          | Example Input     | Output     |
| --------------------------------------------------------------------------------------------- | ----------------- | ---------- |
| The string can only contain digits, commas, decimal points and signs.                         | "- 12,34567,89.0" | -123456789 |
| Whitespaces are allowed anywhere except between digits.                                       | " - 12.345 "      | -12.345    |
| Only digits and commas are allowed before the decimal point.                                  | " 12,345,678"     | 12345678   |
| Only digits are allowed after the decimal point.                                              | "1.234 "          | 1.234      |
| Use `E` or `e` for exponents. After the `e`, digits and a leading sign indicator are allowed. | " 123.45e-1"      | 12.345     |
| If the integer part isn't empty, then it must contain at least one digit.                     | " 0,.12 -"        | -0.12      |
| If the string contains a decimal point, then it must contain at least one digit.              | " .1"             | 0.1        |
| The string can't contain more than one sign.                                                  | " 0.5 +"          | 0.5        |

**Return Data Type**

`NUMERIC`

**Examples**

This example shows an input with spaces before, after, and between the
sign and the number:

```sql
SELECT PARSE_NUMERIC("  -  12.34 ") as parsed;

/*--------+
 | parsed |
 +--------+
 | -12.34 |
 +--------*/
```

This example shows an input with an exponent as well as the sign after the
number:

```sql
SELECT PARSE_NUMERIC("12.34e-1-") as parsed;

/*--------+
 | parsed |
 +--------+
 | -1.234 |
 +--------*/
```

This example shows an input with multiple commas in the integer part of the
number:

```sql
SELECT PARSE_NUMERIC("  1,2,,3,.45 + ") as parsed;

/*--------+
 | parsed |
 +--------+
 | 123.45 |
 +--------*/
```

This example shows an input with a decimal point and no digits in the whole
number part:

```sql
SELECT PARSE_NUMERIC(".1234  ") as parsed;

/*--------+
 | parsed |
 +--------+
 | 0.1234 |
 +--------*/
```

**Examples of invalid inputs**

This example is invalid because the whole number part contains no digits:

```sql
SELECT PARSE_NUMERIC(",,,.1234  ") as parsed;
```

This example is invalid because there are whitespaces between digits:

```sql
SELECT PARSE_NUMERIC("1  23.4 5  ") as parsed;
```

This example is invalid because the number is empty except for an exponent:

```sql
SELECT PARSE_NUMERIC("  e1 ") as parsed;
```

This example is invalid because the string contains multiple signs:

```sql
SELECT PARSE_NUMERIC("  - 12.3 - ") as parsed;
```

This example is invalid because the value of the number falls outside the range
of `NUMERIC`:

```sql
SELECT PARSE_NUMERIC("12.34E100 ") as parsed;
```

This example is invalid because the string contains invalid characters:

```sql
SELECT PARSE_NUMERIC("$12.34") as parsed;
```

[half-from-zero-wikipedia]: https://en.wikipedia.org/wiki/Rounding#Round_half_away_from_zero
[cast-numeric]: #cast-numeric
[numeric-type]: ../types/data_types.md#decimal-types

## `CAST`

```sql

`CAST(expression AS typename [format_clause])`
```

**Description**

When using `CAST`, a query can fail if SQL is unable to perform
the cast. For example, the following query generates an error:

```sql
SELECT CAST("apple" AS BIGINT) AS not_a_number;
```

If you want to protect your queries from these types of errors, you can use
`CAST`. `CAST` replaces runtime errors with `NULL`s. However, during
static analysis, impossible casts between two non-castable types still produce
an error because the query is invalid.

```sql
SELECT CAST("apple" AS BIGINT) AS not_a_number;

/*--------------+
 | not_a_number |
 +--------------+
 | NULL         |
 +--------------*/
```

Some casts can include a [format clause][formatting-syntax], which provides
instructions for how to conduct the
cast. For example, you could
instruct a cast to convert a sequence of bytes to a BASE64-encoded string
instead of a UTF-8-encoded string.

The structure of the format clause is unique to each type of cast and more
information is available in the section for that cast.

If you are casting from bytes to strings, you can also use the
function, [`SAFE_CONVERT_BYTES_TO_STRING`][SC_BTS]. Any invalid UTF-8 characters
are replaced with the unicode replacement character, `U+FFFD`.

[SC_BTS]: string_functions.md#safe-convert_bytes_to_string
[formatting-syntax]: ../types/format_elements.md#formatting-syntax
[conversion-rules]: ../types/conversion_rules.md
