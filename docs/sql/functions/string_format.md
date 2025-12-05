# String Formatting and Encoding Functions

Functions for formatting strings and encoding/decoding.

## `CODE_POINTS_TO_BYTES`

```sql
CODE_POINTS_TO_BYTES(ascii_code_points)
```

**Description**

Takes an array of extended ASCII
[code points][string-link-to-code-points-wikipedia]
as `ARRAY<BIGINT>` and returns `VARBINARY`.

To convert from `VARBINARY` to an array of code points, see
[TO_CODE_POINTS][string-link-to-code-points].

**Return type**

`VARBINARY`

**Examples**

The following is a basic example using `CODE_POINTS_TO_BYTES`.

```sql
SELECT CODE_POINTS_TO_BYTES([65, 98, 67, 100]) AS bytes;

/*----------+
 | bytes    |
 +----------+
 | AbCd     |
 +----------*/
```

The following example uses a rotate-by-13 places (ROT13) algorithm to encode a
string.

```sql
SELECT CODE_POINTS_TO_BYTES(ARRAY_AGG(
  (SELECT
      CASE
        WHEN chr BETWEEN b'a' and b'z'
          THEN TO_CODE_POINTS(b'a')[offset(0)] +
            MOD(code+13-TO_CODE_POINTS(b'a')[offset(0)],26)
        WHEN chr BETWEEN b'A' and b'Z'
          THEN TO_CODE_POINTS(b'A')[offset(0)] +
            MOD(code+13-TO_CODE_POINTS(b'A')[offset(0)],26)
        ELSE code
      END
   FROM
     (SELECT code, CODE_POINTS_TO_BYTES([code]) chr)
  ) ORDER BY OFFSET)) AS encoded_string
FROM UNNEST(TO_CODE_POINTS(b'Test String!')) code WITH OFFSET;

/*------------------+
 | encoded_string   |
 +------------------+
 | Grfg Fgevat!     |
 +------------------*/
```

[string-link-to-code-points-wikipedia]: https://en.wikipedia.org/wiki/Code_point
[string-link-to-code-points]: #to-code_points

## `CODE_POINTS_TO_STRING`

```sql
CODE_POINTS_TO_STRING(unicode_code_points)
```

**Description**

Takes an array of Unicode [code points][string-link-to-code-points-wikipedia]
as `ARRAY<BIGINT>` and returns a `VARCHAR`.

To convert from a string to an array of code points, see
[TO_CODE_POINTS][string-link-to-code-points].

**Return type**

`VARCHAR`

**Examples**

The following are basic examples using `CODE_POINTS_TO_STRING`.

```sql
SELECT CODE_POINTS_TO_STRING([65, 255, 513, 1024]) AS string;

/*--------+
 | string |
 +--------+
 | AÿȁЀ   |
 +--------*/
```

```sql
SELECT CODE_POINTS_TO_STRING([97, 0, 0xF9B5]) AS string;

/*--------+
 | string |
 +--------+
 | a例    |
 +--------*/
```

```sql
SELECT CODE_POINTS_TO_STRING([65, 255, NULL, 1024]) AS string;

/*--------+
 | string |
 +--------+
 | NULL   |
 +--------*/
```

The following example computes the frequency of letters in a set of words.

```sql
WITH Words AS (
  SELECT word
  FROM UNNEST(['foo', 'bar', 'baz', 'giraffe', 'llama']) AS word
)
SELECT
  CODE_POINTS_TO_STRING([code_point]) AS letter,
  COUNT(*) AS letter_count
FROM Words,
  UNNEST(TO_CODE_POINTS(word)) AS code_point
GROUP BY 1
ORDER BY 2 DESC;

/*--------+--------------+
 | letter | letter_count |
 +--------+--------------+
 | a      | 5            |
 | f      | 3            |
 | r      | 2            |
 | b      | 2            |
 | l      | 2            |
 | o      | 2            |
 | g      | 1            |
 | z      | 1            |
 | e      | 1            |
 | m      | 1            |
 | i      | 1            |
 +--------+--------------*/
```

[string-link-to-code-points-wikipedia]: https://en.wikipedia.org/wiki/Code_point
[string-link-to-code-points]: #to-code_points

## `FORMAT`

```sql
FORMAT(format_string_expression, data_type_expression[, ...])
```

**Description**

`FORMAT` formats a data type expression as a string.

- `format_string_expression`: Can contain zero or more
  [format specifiers][format-specifiers]. Each format specifier is introduced
  by the `%` symbol, and must map to one or more of the remaining arguments.
  In general, this is a one-to-one mapping, except when the `*` specifier is
  present. For example, `%.*i` maps to two arguments&mdash;a length argument
  and a signed integer argument. If the number of arguments related to the
  format specifiers isn't the same as the number of arguments, an error occurs.
- `data_type_expression`: The value to format as a string. This can be any
  SQL data type.

**Return type**

`VARCHAR`

**Examples**

| Description                     | Statement                                                         | Result                             |
| ------------------------------- | ----------------------------------------------------------------- | ---------------------------------- |
| Simple integer                  | FORMAT('%d', 10)                                                  | 10                                 |
| Integer with left blank padding | FORMAT('\|%10d\|', 11)                                            | \|        11\|                     |
| Integer with left zero padding  | FORMAT('+%010d+', 12)                                             | +0000000012+                       |
| Integer with commas             | FORMAT("%'d", 123456789)                                          | 123,456,789                        |
| VARCHAR                         | FORMAT('-%s-', 'abcd efg')                                        | -abcd efg-                         |
| DOUBLE                          | FORMAT('%f %E', 1.1, 2.2)                                         | 1.100000 2.200000E+00              |
| DATE                            | FORMAT('%t', date '2015-09-01')                                   | 2015-09-01                         |
| TIMESTAMP                       | FORMAT('%t', timestamp '2015-09-01 12:34:56 America/Los_Angeles') | 2015-09-01 19:34:56+00             |

The `FORMAT()` function doesn't provide fully customizable formatting for all
types and values, nor formatting that's sensitive to locale.

If custom formatting is necessary for a type, you must first format it using
type-specific format functions, such as `FORMAT_DATE()` or `FORMAT_TIMESTAMP()`.
For example:

```sql
SELECT FORMAT('date: %s!', FORMAT_DATE('%B %d, %Y', date '2015-01-02'));
```

Returns

```sql
date: January 02, 2015!
```

#### Supported format specifiers

```sql
%[flags][width][.precision]specifier
```

A [format specifier][format-specifier-list] adds formatting when casting a
value to a string. It can optionally contain these sub-specifiers:

- [Flags][flags]
- [Width][width]
- [Precision][precision]

Additional information about format specifiers:

- [%g and %G behavior][g-and-g-behavior]
- [%p and %P behavior][p-and-p-behavior]
- [%t and %T behavior][t-and-t-behavior]
- [Error conditions][error-format-specifiers]
- [NULL argument handling][null-format-specifiers]
- [Additional semantic rules][rules-format-specifiers]

##### Format specifiers

The format specifier can optionally contain the sub-specifiers identified above
in the specifier prototype.

These sub-specifiers must comply with the following specifications.

##### Flags

Flags may be specified in any order. Duplicate flags aren't an error. When
flags aren't relevant for some element type, they are ignored.

##### Width##### Precision

##### %g and %G behavior

The `%g` and `%G` format specifiers choose either the decimal notation (like
the `%f` and `%F` specifiers) or the scientific notation (like the `%e` and `%E`
specifiers), depending on the input value's exponent and the specified
[precision](#precision).

Let p stand for the specified [precision](#precision) (defaults to 6; 1 if the
specified precision is less than 1). The input value is first converted to
scientific notation with precision = (p - 1). If the resulting exponent part x
is less than -4 or no less than p, the scientific notation with precision =
(p - 1) is used; otherwise the decimal notation with precision = (p - 1 - x) is
used.

Unless [`#` flag](#flags) is present, the trailing zeros after the decimal point
are removed, and the decimal point is also removed if there is no digit after
it.

##### %p and %P behavior

The `%p` format specifier produces a one-line printable string. The `%P`
format specifier produces a multi-line printable string. You can use these
format specifiers with the following data types:##### %t and %T behavior

The `%t` and `%T` format specifiers are defined for all types. The
[width](#width), [precision](#precision), and [flags](#flags) act as they do
for `%s`: the [width](#width) is the minimum width and the `VARCHAR` will be
padded to that size, and [precision](#precision) is the maximum width
of content to show and the `VARCHAR` will be truncated to that size, prior to
padding to width.

The `%t` specifier is always meant to be a readable form of the value.

The `%T` specifier is always a valid SQL literal of a similar type, such as a
wider numeric type. The literal will not include casts or a type name,
except for the special case of non-finite floating point values.

The `VARCHAR` is formatted as follows:##### Error conditions

If a format specifier is invalid, or isn't compatible with the related
argument type, or the wrong number or arguments are provided, then an error is
produced. For example, the following `<format_string>` expressions are invalid:

```sql
FORMAT('%s', 1)
```

```sql
FORMAT('%')
```

##### NULL argument handling

A `NULL` format string results in a `NULL` output `VARCHAR`. Any other arguments
are ignored in this case.

The function generally produces a `NULL` value if a `NULL` argument is present.
For example, `FORMAT('%i', NULL_expression)` produces a `NULL VARCHAR` as
output.

However, there are some exceptions: if the format specifier is %t or %T
(both of which produce `VARCHAR`s that effectively match CAST and literal value
semantics), a `NULL` value produces 'NULL' (without the quotes) in the result
`VARCHAR`. For example, the function:

```sql
FORMAT('00-%t-00', NULL_expression);
```

Returns

```sql
00-NULL-00
```

##### Additional semantic rules

`DOUBLE` and
`FLOAT` values can be `+/-inf` or `NaN`.
When an argument has one of those values, the result of the format specifiers
`%f`, `%F`, `%e`, `%E`, `%g`, `%G`, and `%t` are `inf`, `-inf`, or `nan`
(or the same in uppercase) as appropriate. This is consistent with how
SQL casts these values to `VARCHAR`. For `%T`,
SQL returns quoted strings for
`DOUBLE` values that don't have non-string literal
representations.

[format-specifiers]: #format-specifiers
[format-specifier-list]: #format-specifier_list
[flags]: #flags
[width]: #width
[precision]: #precision
[g-and-g-behavior]: #g-and_g_behavior
[p-and-p-behavior]: #p-and_p_behavior
[t-and-t-behavior]: #t-and_t_behavior
[error-format-specifiers]: #error-format_specifiers
[null-format-specifiers]: #null-format_specifiers
[rules-format-specifiers]: #rules-format_specifiers

## `FROM_BASE32`

```sql
FROM_BASE32(string_expr)
```

**Description**

Converts the base32-encoded input `string_expr` into `VARBINARY` format. To convert
`VARBINARY` to a base32-encoded `VARCHAR`, use [TO_BASE32][string-link-to-base32].

**Return type**

`VARBINARY`

**Example**

```sql
SELECT FROM_BASE32('MFRGGZDF74======') AS byte_data;

/*-----------+
 | byte_data |
 +-----------+
 | abcde\xff |
 +-----------*/
```

[string-link-to-base32]: #to-base32

## `FROM_BASE64`

```sql
FROM_BASE64(string_expr)
```

**Description**

Converts the base64-encoded input `string_expr` into
`VARBINARY` format. To convert
`VARBINARY` to a base64-encoded `VARCHAR`,
use [TO_BASE64][string-link-to-to-base64].

There are several base64 encodings in common use that vary in exactly which
alphabet of 65 ASCII characters are used to encode the 64 digits and padding.
See [RFC 4648][RFC-4648] for details. This
function expects the alphabet `[A-Za-z0-9+/=]`.

**Return type**

`VARBINARY`

**Example**

```sql
SELECT FROM_BASE64('/+A=') AS byte_data;

/*-----------+
 | byte_data |
 +-----------+
 | \377\340  |
 +-----------*/
```

To work with an encoding using a different base64 alphabet, you might need to
compose `FROM_BASE64` with the `REPLACE` function. For instance, the
`base64url` url-safe and filename-safe encoding commonly used in web programming
uses `-_=` as the last characters rather than `+/=`. To decode a
`base64url`-encoded string, replace `-` and `_` with `+` and `/` respectively.

```sql
SELECT FROM_BASE64(REPLACE(REPLACE('_-A=', '-', '+'), '_', '/')) AS binary;

/*-----------+
 | binary    |
 +-----------+
 | \377\340  |
 +-----------*/
```

[RFC-4648]: https://tools.ietf.org/html/rfc4648#section-4
[string-link-to-to-base64]: #to-base64

## `FROM_HEX`

```sql
FROM_HEX(string)
```

**Description**

Converts a hexadecimal-encoded `VARCHAR` into `VARBINARY` format. Returns an error
if the input `VARCHAR` contains characters outside the range
`(0..9, A..F, a..f)`. The lettercase of the characters doesn't matter. If the
input `VARCHAR` has an odd number of characters, the function acts as if the
input has an additional leading `0`. To convert `VARBINARY` to a hexadecimal-encoded
`VARCHAR`, use [TO_HEX][string-link-to-to-hex].

**Return type**

`VARBINARY`

**Example**

```sql
WITH Input AS (
  SELECT '00010203aaeeefff' AS hex_str UNION ALL
  SELECT '0AF' UNION ALL
  SELECT '666f6f626172'
)
SELECT hex_str, FROM_HEX(hex_str) AS bytes_str
FROM Input;

/*------------------+----------------------------------+
 | hex_str          | bytes_str                        |
 +------------------+----------------------------------+
 | 0AF              | \x00\xaf                         |
 | 00010203aaeeefff | \x00\x01\x02\x03\xaa\xee\xef\xff |
 | 666f6f626172     | foobar                           |
 +------------------+----------------------------------*/
```

[string-link-to-to-hex]: #to-hex

## `SAFE_CONVERT_BYTES_TO_STRING`

```sql
SAFE_CONVERT_BYTES_TO_STRING(value)
```

**Description**

Converts a sequence of `VARBINARY` to a `VARCHAR`. Any invalid UTF-8 characters are
replaced with the Unicode replacement character, `U+FFFD`.

**Return type**

`VARCHAR`

**Examples**

The following statement returns the Unicode replacement character, &#65533;.

```sql
SELECT SAFE_CONVERT_BYTES_TO_STRING(b'\xc2') as safe_convert;
```

## `TO_BASE32`

```sql
TO_BASE32(bytes_expr)
```

**Description**

Converts a sequence of `VARBINARY` into a base32-encoded `VARCHAR`. To convert a
base32-encoded `VARCHAR` into `VARBINARY`, use [FROM_BASE32][string-link-to-from-base32].

**Return type**

`VARCHAR`

**Example**

```sql
SELECT TO_BASE32(b'abcde\xFF') AS base32_string;

/*------------------+
 | base32_string    |
 +------------------+
 | MFRGGZDF74====== |
 +------------------*/
```

[string-link-to-from-base32]: #from-base32

## `TO_BASE64`

```sql
TO_BASE64(bytes_expr)
```

**Description**

Converts a sequence of `VARBINARY` into a base64-encoded `VARCHAR`. To convert a
base64-encoded `VARCHAR` into `VARBINARY`, use [FROM_BASE64][string-link-to-from-base64].

There are several base64 encodings in common use that vary in exactly which
alphabet of 65 ASCII characters are used to encode the 64 digits and padding.
See [RFC 4648][RFC-4648] for details. This
function adds padding and uses the alphabet `[A-Za-z0-9+/=]`.

**Return type**

`VARCHAR`

**Example**

```sql
SELECT TO_BASE64(b'\377\340') AS base64_string;

/*---------------+
 | base64_string |
 +---------------+
 | /+A=          |
 +---------------*/
```

To work with an encoding using a different base64 alphabet, you might need to
compose `TO_BASE64` with the `REPLACE` function. For instance, the
`base64url` url-safe and filename-safe encoding commonly used in web programming
uses `-_=` as the last characters rather than `+/=`. To encode a
`base64url`-encoded string, replace `+` and `/` with `-` and `_` respectively.

```sql
SELECT REPLACE(REPLACE(TO_BASE64(b'\377\340'), '+', '-'), '/', '_') as websafe_base64;

/*----------------+
 | websafe_base64 |
 +----------------+
 | _-A=           |
 +----------------*/
```

[string-link-to-from-base64]: #from-base64
[RFC-4648]: https://tools.ietf.org/html/rfc4648#section-4

## `TO_CODE_POINTS`

```sql
TO_CODE_POINTS(value)
```

**Description**

Takes a `VARCHAR` or `VARBINARY` value and returns an array of `BIGINT` values that
represent code points or extended ASCII character values.

- If `value` is a `VARCHAR`, each element in the returned array represents a
  [code point][string-link-to-code-points-wikipedia]. Each code point falls
  within the range of [0, 0xD7FF] and [0xE000, 0x10FFFF].
- If `value` is `VARBINARY`, each element in the array is an extended ASCII
  character value in the range of [0, 255].

To convert from an array of code points to a `VARCHAR` or `VARBINARY`, see
[CODE_POINTS_TO_STRING][string-link-to-codepoints-to-string] or
[CODE_POINTS_TO_BYTES][string-link-to-codepoints-to-bytes].

**Return type**

`ARRAY`

**Examples**

The following examples get the code points for each element in an array of
words.

```sql
SELECT
  'foo' AS word,
  TO_CODE_POINTS('foo') AS code_points

/*---------+------------------------------------+
 | word    | code_points                        |
 +---------+------------------------------------+
 | foo     | [102, 111, 111]                    |
 +---------+------------------------------------*/
```

```sql
SELECT
  'bar' AS word,
  TO_CODE_POINTS('bar') AS code_points

/*---------+------------------------------------+
 | word    | code_points                        |
 +---------+------------------------------------+
 | bar     | [98, 97, 114]                      |
 +---------+------------------------------------*/
```

```sql
SELECT
  'baz' AS word,
  TO_CODE_POINTS('baz') AS code_points

/*---------+------------------------------------+
 | word    | code_points                        |
 +---------+------------------------------------+
 | baz     | [98, 97, 122]                      |
 +---------+------------------------------------*/
```

```sql
SELECT
  'giraffe' AS word,
  TO_CODE_POINTS('giraffe') AS code_points

/*---------+------------------------------------+
 | word    | code_points                        |
 +---------+------------------------------------+
 | giraffe | [103, 105, 114, 97, 102, 102, 101] |
 +---------+------------------------------------*/
```

```sql
SELECT
  'llama' AS word,
  TO_CODE_POINTS('llama') AS code_points

/*---------+------------------------------------+
 | word    | code_points                        |
 +---------+------------------------------------+
 | llama   | [108, 108, 97, 109, 97]            |
 +---------+------------------------------------*/
```

The following examples convert integer representations of `VARBINARY` to their
corresponding ASCII character values.

```sql
SELECT
  b'\x66\x6f\x6f' AS bytes_value,
  TO_CODE_POINTS(b'\x66\x6f\x6f') AS bytes_value_as_integer

/*------------------+------------------------+
 | bytes_value      | bytes_value_as_integer |
 +------------------+------------------------+
 | foo              | [102, 111, 111]        |
 +------------------+------------------------*/
```

```sql
SELECT
  b'\x00\x01\x10\xff' AS bytes_value,
  TO_CODE_POINTS(b'\x00\x01\x10\xff') AS bytes_value_as_integer

/*------------------+------------------------+
 | bytes_value      | bytes_value_as_integer |
 +------------------+------------------------+
 | \x00\x01\x10\xff | [0, 1, 16, 255]        |
 +------------------+------------------------*/
```

The following example demonstrates the difference between a `VARBINARY` result and a
`VARCHAR` result. Notice that the character `Ā` is represented as a two-byte
Unicode sequence. As a result, the `VARBINARY` version of `TO_CODE_POINTS` returns
an array with two elements, while the `VARCHAR` version returns an array with a
single element.

```sql
SELECT TO_CODE_POINTS(b'Ā') AS b_result, TO_CODE_POINTS('Ā') AS s_result;

/*------------+----------+
 | b_result   | s_result |
 +------------+----------+
 | [196, 128] | [256]    |
 +------------+----------*/
```

[string-link-to-code-points-wikipedia]: https://en.wikipedia.org/wiki/Code_point
[string-link-to-codepoints-to-string]: #code-points_to_string
[string-link-to-codepoints-to-bytes]: #code-points_to_bytes

## `TO_HEX`

```sql
TO_HEX(bytes)
```

**Description**

Converts a sequence of `VARBINARY` into a hexadecimal `VARCHAR`. Converts each byte
in the `VARCHAR` as two hexadecimal characters in the range
`(0..9, a..f)`. To convert a hexadecimal-encoded
`VARCHAR` to `VARBINARY`, use [FROM_HEX][string-link-to-from-hex].

**Return type**

`VARCHAR`

**Example**

```sql
SELECT
  b'\x00\x01\x02\x03\xAA\xEE\xEF\xFF' AS byte_string,
  TO_HEX(b'\x00\x01\x02\x03\xAA\xEE\xEF\xFF') AS hex_string

/*----------------------------------+------------------+
 | byte_string                      | hex_string       |
 +----------------------------------+------------------+
 | \x00\x01\x02\x03\xaa\xee\xef\xff | 00010203aaeeefff |
 +----------------------------------+------------------*/
```

[string-link-to-from-hex]: #from-hex
