# String Manipulation Functions

Functions for building, modifying, and splitting strings.

## `ASCII`

```sql
ASCII(value)
```

**Description**

Returns the ASCII code for the first character or byte in `value`. Returns
`0` if `value` is empty or the ASCII code is `0` for the first character
or byte.

**Return type**

`BIGINT`

**Examples**

```sql
SELECT ASCII('abcd') as A, ASCII('a') as B, ASCII('') as C, ASCII(NULL) as D;

/*-------+-------+-------+-------+
 | A     | B     | C     | D     |
 +-------+-------+-------+-------+
 | 97    | 97    | 0     | NULL  |
 +-------+-------+-------+-------*/
```

## `BYTE_LENGTH`

```sql
BYTE_LENGTH(value)
```

**Description**

Gets the number of `VARBINARY` in a `VARCHAR` or `VARBINARY` value,
regardless of whether the value is a `VARCHAR` or `VARBINARY` type.

**Return type**

`BIGINT`

**Examples**

```sql
SELECT BYTE_LENGTH('абвгд') AS string_example;

/*----------------+
 | string_example |
 +----------------+
 | 10             |
 +----------------*/
```

```sql
SELECT BYTE_LENGTH(b'абвгд') AS bytes_example;

/*----------------+
 | bytes_example  |
 +----------------+
 | 10             |
 +----------------*/
```

## `CHAR_LENGTH`

```sql
CHAR_LENGTH(value)
```

**Description**

Gets the number of characters in a `VARCHAR` value.

**Return type**

`BIGINT`

**Examples**

```sql
SELECT CHAR_LENGTH('абвгд') AS char_length;

/*-------------+
 | char_length |
 +-------------+
 | 5           |
 +------------ */
```

## `CHARACTER_LENGTH`

```sql
CHARACTER_LENGTH(value)
```

**Description**

Synonym for [CHAR_LENGTH][string-link-to-char-length].

**Return type**

`BIGINT`

**Examples**

```sql
SELECT
  'абвгд' AS characters,
  CHARACTER_LENGTH('абвгд') AS char_length_example

/*------------+---------------------+
 | characters | char_length_example |
 +------------+---------------------+
 | абвгд      |                   5 |
 +------------+---------------------*/
```

[string-link-to-char-length]: #char-length

## `CHR`

```sql
CHR(value)
```

**Description**

Takes a Unicode [code point][string-link-to-code-points-wikipedia] and returns
the character that matches the code point. Each valid code point should fall
within the range of [0, 0xD7FF] and [0xE000, 0x10FFFF]. Returns an empty string
if the code point is `0`. If an invalid Unicode code point is specified, an
error is returned.

To work with an array of Unicode code points, see
[`CODE_POINTS_TO_STRING`][string-link-to-codepoints-to-string]

**Return type**

`VARCHAR`

**Examples**

```sql
SELECT CHR(65) AS A, CHR(255) AS B, CHR(513) AS C, CHR(1024)  AS D;

/*-------+-------+-------+-------+
 | A     | B     | C     | D     |
 +-------+-------+-------+-------+
 | A     | ÿ     | ȁ     | Ѐ     |
 +-------+-------+-------+-------*/
```

```sql
SELECT CHR(97) AS A, CHR(0xF9B5) AS B, CHR(0) AS C, CHR(NULL) AS D;

/*-------+-------+-------+-------+
 | A     | B     | C     | D     |
 +-------+-------+-------+-------+
 | a     | 例    |       | NULL  |
 +-------+-------+-------+-------*/
```

[string-link-to-code-points-wikipedia]: https://en.wikipedia.org/wiki/Code_point
[string-link-to-codepoints-to-string]: #code-points_to_string

## `CONCAT`

```sql
CONCAT(value1[, ...])
```

**Description**

Concatenates one or more values into a single result. All values must be
`VARBINARY` or data types that can be cast to `VARCHAR`.

The function returns `NULL` if any input argument is `NULL`.

Note: You can also use the
[|| concatenation operator][string-link-to-operators] to concatenate
values into a string.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT CONCAT('T.P.', ' ', 'Bar') as author;

/*---------------------+
 | author              |
 +---------------------+
 | T.P. Bar            |
 +---------------------*/
```

```sql
SELECT CONCAT('Summer', ' ', 1923) as release_date;

/*---------------------+
 | release_date        |
 +---------------------+
 | Summer 1923         |
 +---------------------*/
```

```

With Employees AS
  (SELECT
    'John' AS first_name,
    'Doe' AS last_name
  UNION ALL
  SELECT
    'Jane' AS first_name,
    'Smith' AS last_name
  UNION ALL
  SELECT
    'Joe' AS first_name,
    'Jackson' AS last_name)

SELECT
  CONCAT(first_name, ' ', last_name)
  AS full_name
FROM Employees;

/*---------------------+
 | full_name           |
 +---------------------+
 | John Doe            |
 | Jane Smith          |
 | Joe Jackson         |
 +---------------------*/
```

[string-link-to-operators]: ../syntax/operators.md

## `LEFT`

```sql
LEFT(value, length)
```

**Description**

Returns a `VARCHAR` or `VARBINARY` value that consists of the specified
number of leftmost characters or bytes from `value`. The `length` is an
`BIGINT` that specifies the length of the returned
value. If `value` is of type `VARBINARY`, `length` is the number of leftmost bytes
to return. If `value` is `VARCHAR`, `length` is the number of leftmost characters
to return.

If `length` is 0, an empty `VARCHAR` or `VARBINARY` value will be
returned. If `length` is negative, an error will be returned. If `length`
exceeds the number of characters or bytes from `value`, the original `value`
will be returned.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT LEFT('banana', 3) AS results

/*---------+
 | results |
  +--------+
 | ban     |
 +---------*/
```

```sql
SELECT LEFT(b'\xab\xcd\xef\xaa\xbb', 3) AS results

/*--------------+
 | results      |
 +--------------+
 | \xab\xcd\xef |
 +--------------*/
```

## `LENGTH`

```sql
LENGTH(value)
```

**Description**

Returns the length of the `VARCHAR` or `VARBINARY` value. The returned
value is in characters for `VARCHAR` arguments and in bytes for the `VARBINARY`
argument.

**Return type**

`BIGINT`

**Examples**

```sql
SELECT
  LENGTH('абвгд') AS string_example,
  LENGTH(CAST('абвгд' AS VARBINARY)) AS bytes_example;

/*----------------+---------------+
 | string_example | bytes_example |
 +----------------+---------------+
 | 5              | 10            |
 +----------------+---------------*/
```

## `LPAD`

```sql
LPAD(original_value, return_length[, pattern])
```

**Description**

Returns a `VARCHAR` or `VARBINARY` value that consists of `original_value` prepended
with `pattern`. The `return_length` is an `BIGINT` that
specifies the length of the returned value. If `original_value` is of type
`VARBINARY`, `return_length` is the number of bytes. If `original_value` is
of type `VARCHAR`, `return_length` is the number of characters.

The default value of `pattern` is a blank space.

Both `original_value` and `pattern` must be the same data type.

If `return_length` is less than or equal to the `original_value` length, this
function returns the `original_value` value, truncated to the value of
`return_length`. For example, `LPAD('hello world', 7);` returns `'hello w'`.

If `original_value`, `return_length`, or `pattern` is `NULL`, this function
returns `NULL`.

This function returns an error if:

- `return_length` is negative
- `pattern` is empty

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT FORMAT('%T', LPAD('c', 5)) AS results

/*---------+
 | results |
 +---------+
 | "    c" |
 +---------*/
```

```sql
SELECT LPAD('b', 5, 'a') AS results

/*---------+
 | results |
 +---------+
 | aaaab   |
 +---------*/
```

```sql
SELECT LPAD('abc', 10, 'ghd') AS results

/*------------+
 | results    |
 +------------+
 | ghdghdgabc |
 +------------*/
```

```sql
SELECT LPAD('abc', 2, 'd') AS results

/*---------+
 | results |
 +---------+
 | ab      |
 +---------*/
```

```sql
SELECT FORMAT('%T', LPAD(b'abc', 10, b'ghd')) AS results

/*---------------+
 | results       |
 +---------------+
 | b"ghdghdgabc" |
 +---------------*/
```

## `LTRIM`

```sql
LTRIM(value1[, value2])
```

**Description**

Identical to [TRIM][string-link-to-trim], but only removes leading characters.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT CONCAT('#', LTRIM('   apple   '), '#') AS example

/*-------------+
 | example     |
 +-------------+
 | #apple   #  |
 +-------------*/
```

```sql
SELECT LTRIM('***apple***', '*') AS example

/*-----------+
 | example   |
 +-----------+
 | apple***  |
 +-----------*/
```

```sql
SELECT LTRIM('xxxapplexxx', 'xyz') AS example

/*-----------+
 | example   |
 +-----------+
 | applexxx  |
 +-----------*/
```

[string-link-to-trim]: #trim

## `OCTET_LENGTH`

```sql
OCTET_LENGTH(value)
```

Alias for [`BYTE_LENGTH`][byte-length].

[byte-length]: #byte-length

## `REPEAT`

```sql
REPEAT(original_value, repetitions)
```

**Description**

Returns a `VARCHAR` or `VARBINARY` value that consists of `original_value`, repeated.
The `repetitions` parameter specifies the number of times to repeat
`original_value`. Returns `NULL` if either `original_value` or `repetitions`
are `NULL`.

This function returns an error if the `repetitions` value is negative.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT REPEAT('abc', 3) AS results

/*-----------+
 | results   |
 |-----------|
 | abcabcabc |
 +-----------*/
```

```sql
SELECT REPEAT('abc', NULL) AS results

/*---------+
 | results |
 |---------|
 | NULL    |
 +---------*/
```

```sql
SELECT REPEAT(NULL, 3) AS results

/*---------+
 | results |
 |---------|
 | NULL    |
 +---------*/
```

## `REPLACE`

```sql
REPLACE(original_value, from_pattern, to_pattern)
```

**Description**

Replaces all occurrences of `from_pattern` with `to_pattern` in
`original_value`. If `from_pattern` is empty, no replacement is made.

This function supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
WITH desserts AS
  (SELECT 'apple pie' as dessert
  UNION ALL
  SELECT 'blackberry pie' as dessert
  UNION ALL
  SELECT 'cherry pie' as dessert)

SELECT
  REPLACE (dessert, 'pie', 'cobbler') as example
FROM desserts;

/*--------------------+
 | example            |
 +--------------------+
 | apple cobbler      |
 | blackberry cobbler |
 | cherry cobbler     |
 +--------------------*/
```

## `REVERSE`

```sql
REVERSE(value)
```

**Description**

Returns the reverse of the input `VARCHAR` or `VARBINARY`.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT REVERSE('abc') AS results

/*---------+
 | results |
 +---------+
 | cba     |
 +---------*/
```

```sql
SELECT FORMAT('%T', REVERSE(b'1a3')) AS results

/*---------+
 | results |
 +---------+
 | b"3a1"  |
 +---------*/
```

## `RIGHT`

```sql
RIGHT(value, length)
```

**Description**

Returns a `VARCHAR` or `VARBINARY` value that consists of the specified
number of rightmost characters or bytes from `value`. The `length` is an
`BIGINT` that specifies the length of the returned
value. If `value` is `VARBINARY`, `length` is the number of rightmost bytes to
return. If `value` is `VARCHAR`, `length` is the number of rightmost characters
to return.

If `length` is 0, an empty `VARCHAR` or `VARBINARY` value will be
returned. If `length` is negative, an error will be returned. If `length`
exceeds the number of characters or bytes from `value`, the original `value`
will be returned.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT 'apple' AS example, RIGHT('apple', 3) AS right_example

/*---------+---------------+
 | example | right_example |
 +---------+---------------+
 | apple   | ple           |
 +---------+---------------*/
```

```sql
SELECT b'apple' AS example, RIGHT(b'apple', 3) AS right_example

/*----------------------+---------------+
 | example              | right_example |
 +----------------------+---------------+
 | apple                | ple           |
 +----------------------+---------------*
```

## `RPAD`

```sql
RPAD(original_value, return_length[, pattern])
```

**Description**

Returns a `VARCHAR` or `VARBINARY` value that consists of `original_value` appended
with `pattern`. The `return_length` parameter is an
`BIGINT` that specifies the length of the
returned value. If `original_value` is `VARBINARY`,
`return_length` is the number of bytes. If `original_value` is `VARCHAR`,
`return_length` is the number of characters.

The default value of `pattern` is a blank space.

Both `original_value` and `pattern` must be the same data type.

If `return_length` is less than or equal to the `original_value` length, this
function returns the `original_value` value, truncated to the value of
`return_length`. For example, `RPAD('hello world', 7);` returns `'hello w'`.

If `original_value`, `return_length`, or `pattern` is `NULL`, this function
returns `NULL`.

This function returns an error if:

- `return_length` is negative
- `pattern` is empty

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT FORMAT('%T', RPAD('c', 5)) AS results

/*---------+
 | results |
 +---------+
 | "c    " |
 +---------*/
```

```sql
SELECT RPAD('b', 5, 'a') AS results

/*---------+
 | results |
 +---------+
 | baaaa   |
 +---------*/
```

```sql
SELECT RPAD('abc', 10, 'ghd') AS results

/*------------+
 | results    |
 +------------+
 | abcghdghdg |
 +------------*/
```

```sql
SELECT RPAD('abc', 2, 'd') AS results

/*---------+
 | results |
 +---------+
 | ab      |
 +---------*/
```

```sql
SELECT FORMAT('%T', RPAD(b'abc', 10, b'ghd')) AS results

/*---------------+
 | results       |
 +---------------+
 | b"abcghdghdg" |
 +---------------*/
```

## `RTRIM`

```sql
RTRIM(value1[, value2])
```

**Description**

Identical to [TRIM][string-link-to-trim], but only removes trailing characters.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT RTRIM('***apple***', '*') AS example

/*-----------+
 | example   |
 +-----------+
 | ***apple  |
 +-----------*/
```

```sql
SELECT RTRIM('applexxz', 'xyz') AS example

/*---------+
 | example |
 +---------+
 | apple   |
 +---------*/
```

[string-link-to-trim]: #trim

## `SPLIT`

```sql
SPLIT(value[, delimiter])
```

**Description**

Splits a `VARCHAR` or `VARBINARY` value, using a delimiter. The `delimiter` argument
must be a literal character or sequence of characters. You can't split with a
regular expression.

For `VARCHAR`, the default delimiter is the comma `,`.

For `VARBINARY`, you must specify a delimiter.

Splitting on an empty delimiter produces an array of UTF-8 characters for
`VARCHAR` values, and an array of `VARBINARY` for `VARBINARY` values.

Splitting an empty `VARCHAR` returns an
`ARRAY` with a single empty
`VARCHAR`.

This function supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return type**

`ARRAY` or `ARRAY`

**Examples**

```sql
WITH letters AS
  (SELECT '' as letter_group
  UNION ALL
  SELECT 'a' as letter_group
  UNION ALL
  SELECT 'b c d' as letter_group)

SELECT SPLIT(letter_group, ' ') as example
FROM letters;

/*----------------------+
 | example              |
 +----------------------+
 | []                   |
 | [a]                  |
 | [b, c, d]            |
 +----------------------*/
```

## `SPLIT_SUBSTR`

```sql
SPLIT_SUBSTR(value, delimiter, start_split[, count])
```

**Description**

Returns a substring from an input `VARCHAR` that's determined by a delimiter, a
location that indicates the first split of the substring to return, and the
number of splits to include in the returned substring.

The `value` argument is the supplied `VARCHAR` value from which a substring is
returned.

The `delimiter` argument is the delimiter used to split the input `VARCHAR`. It
must be a literal character or sequence of characters.

- The `delimiter` argument can't be a regular expression.
- Delimiter matching is from left to right.
- If the delimiter is a sequence of characters, then two instances of the
  delimiter in the input string can't overlap. For example, if the delimiter is
  `**`, then the delimiters in the string `aa***bb***cc` are:
  - The first two asterisks after `aa`.
  - The first two asterisks after `bb`.

The `start_split` argument is an integer that specifies the first split of the
substring to return.

- If `start_split` is `1`, then the returned substring starts from the first
  split.
- If `start_split` is `0` or less than the negative of the number of splits,
  then `start_split` is treated as if it's `1` and returns a substring that
  starts with the first split.
- If `start_split` is greater than the number of splits, then an empty string is
  returned.
- If `start_split` is negative, then the splits are counted from the end of the
  input string. If `start_split` is `-1`, then the last split in the input
  string is returned.

The optional `count` argument is an integer that specifies the maximum number
of splits to include in the returned substring.

- If `count` isn't specified, then the substring from the `start_split`
  position to the end of the input string is returned.
- If `count` is `0`, an empty string is returned.
- If `count` is negative, an error is returned.
- If the sum of `count` plus `start_split` is greater than the number of splits,
  then a substring from `start_split` to the end of the input string is
  returned.

This function supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return type**

`VARCHAR`

**Examples**

The following example returns an empty string because `count` is `0`:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", 1, 0) AS example

/*---------+
 | example |
 +---------+
 |         |
 +---------*/
```

The following example returns two splits starting with the first split:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", 1, 2) AS example

/*---------+
 | example |
 +---------+
 | www.abc |
 +---------*/
```

The following example returns one split starting with the first split:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", 1, 1) AS example

/*---------+
 | example |
 +---------+
 | www     |
 +---------*/
```

The following example returns splits from the right because `start_split` is a
negative value:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", -1, 1) AS example

/*---------+
 | example |
 +---------+
 | com     |
 +---------*/
```

The following example returns a substring with three splits, starting with the
first split:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", 1, 3) AS example

/*-------------+
 | example     |
 +-------------+
 | www.abc.xyz |
 +------------*/
```

If `start_split` is zero, then it's treated as if it's `1`. The following
example returns three substrings starting with the first split:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", 0, 3) AS example

/*-------------+
 | example     |
 +-------------+
 | www.abc.xyz |
 +------------*/
```

If `start_split` is greater than the number of splits, then an empty string is
returned:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", 5, 3) AS example

/*---------+
 | example |
 +---------+
 |         |
 +--------*/
```

In the following example, the `start_split` value (`-5`) is less than the
negative of the number of splits (`-4`), so `start_split` is treated as `1`:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", -5, 3) AS example

/*-------------+
 | example     |
 +-------------+
 | www.abc.xyz |
 +------------*/
```

In the following example, the substring from `start_split` to the end of the
string is returned because `count` isn't specified:

```sql
SELECT SPLIT_SUBSTR("www.abc.xyz.com", ".", 3) AS example

/*---------+
 | example |
 +---------+
 | xyz.com |
 +--------*/
```

The following two examples demonstrate how `SPLIT_SUBSTR` works with a
multi-character delimiter that has overlapping matches in the input string. In
each example, the input string contains instances of three asterisks in a row
(`***`) and the delimiter is two asterisks (`**`).

```sql
SELECT SPLIT_SUBSTR('aaa***bbb***ccc', '**', 1, 2) AS example

/*-----------+
 | example   |
 +-----------+
 | aaa***bbb |
 +----------*/
```

```sql
SELECT SPLIT_SUBSTR('aaa***bbb***ccc', '**', 2, 2) AS example

/*------------+
 | example    |
 +------------+
 | *bbb***ccc |
 +-----------*/
```

## `SUBSTR`

```sql
SUBSTR(value, position[, length])
```

**Description**

Gets a portion (substring) of the supplied `VARCHAR` or `VARBINARY` value.

The `position` argument is an integer specifying the starting position of the
substring.

- If `position` is `1`, the substring starts from the first character or byte.
- If `position` is `0` or less than `-LENGTH(value)`, `position` is set to `1`,
  and the substring starts from the first character or byte.
- If `position` is greater than the length of `value`, the function produces
  an empty substring.
- If `position` is negative, the function counts from the end of `value`,
  with `-1` indicating the last character or byte.

The `length` argument specifies the maximum number of characters or bytes to
return.

- If `length` isn't specified, the function produces a substring that starts
  at the specified position and ends at the last character or byte of `value`.
- If `length` is `0`, the function produces an empty substring.
- If `length` is negative, the function produces an error.
- The returned substring may be shorter than `length`, for example, when
  `length` exceeds the length of `value`, or when the starting position of the
  substring plus `length` is greater than the length of `value`.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT SUBSTR('apple', 2) AS example

/*---------+
 | example |
 +---------+
 | pple    |
 +---------*/
```

```sql
SELECT SUBSTR('apple', 2, 2) AS example

/*---------+
 | example |
 +---------+
 | pp      |
 +---------*/
```

```sql
SELECT SUBSTR('apple', -2) AS example

/*---------+
 | example |
 +---------+
 | le      |
 +---------*/
```

```sql
SELECT SUBSTR('apple', 1, 123) AS example

/*---------+
 | example |
 +---------+
 | apple   |
 +---------*/
```

```sql
SELECT SUBSTR('apple', 123) AS example

/*---------+
 | example |
 +---------+
 |         |
 +---------*/
```

```sql
SELECT SUBSTR('apple', 123, 5) AS example

/*---------+
 | example |
 +---------+
 |         |
 +---------*/
```

## `SUBSTRING`

```sql
SUBSTRING(value, position[, length])
```

Alias for [`SUBSTR`][substr].

[substr]: #substr

## `TRANSLATE`

```sql
TRANSLATE(expression, source_characters, target_characters)
```

**Description**

In `expression`, replaces each character in `source_characters` with the
corresponding character in `target_characters`. All inputs must be the same
type, either `VARCHAR` or `VARBINARY`.

- Each character in `expression` is translated at most once.
- A character in `expression` that isn't present in `source_characters` is left
  unchanged in `expression`.
- A character in `source_characters` without a corresponding character in
  `target_characters` is omitted from the result.
- A duplicate character in `source_characters` results in an error.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT TRANSLATE('This is a cookie', 'sco', 'zku') AS translate

/*------------------+
 | translate        |
 +------------------+
 | Thiz iz a kuukie |
 +------------------*/
```

## `TRIM`

```sql
TRIM(value_to_trim[, set_of_characters_to_remove])
```

**Description**

Takes a `VARCHAR` or `VARBINARY` value to trim.

If the value to trim is a `VARCHAR`, removes from this value all leading and
trailing Unicode code points in `set_of_characters_to_remove`.
The set of code points is optional. If it isn't specified, all
whitespace characters are removed from the beginning and end of the
value to trim.

If the value to trim is `VARBINARY`, removes from this value all leading and
trailing bytes in `set_of_characters_to_remove`. The set of bytes is required.

**Return type**

- `VARCHAR` if `value_to_trim` is a `VARCHAR` value.
- `VARBINARY` if `value_to_trim` is a `VARBINARY` value.

**Examples**

In the following example, all leading and trailing whitespace characters are
removed from `item` because `set_of_characters_to_remove` isn't specified.

```sql
SELECT CONCAT('#', TRIM( '   apple   '), '#') AS example

/*----------+
 | example  |
 +----------+
 | #apple#  |
 +----------*/
```

In the following example, all leading and trailing `*` characters are removed
from '**_apple_**'.

```sql
SELECT TRIM('***apple***', '*') AS example

/*---------+
 | example |
 +---------+
 | apple   |
 +---------*/
```

In the following example, all leading and trailing `x`, `y`, and `z` characters
are removed from 'xzxapplexxy'.

```sql
SELECT TRIM('xzxapplexxy', 'xyz') as example

/*---------+
 | example |
 +---------+
 | apple   |
 +---------*/
```

In the following example, examine how `TRIM` interprets characters as
Unicode code-points. If your trailing character set contains a combining
diacritic mark over a particular letter, `TRIM` might strip the
same diacritic mark from a different letter.

```sql
SELECT
  TRIM('abaW̊', 'Y̊') AS a,
  TRIM('W̊aba', 'Y̊') AS b,
  TRIM('abaŪ̊', 'Y̊') AS c,
  TRIM('Ū̊aba', 'Y̊') AS d

/*------+------+------+------+
 | a    | b    | c    | d    |
 +------+------+------+------+
 | abaW | W̊aba | abaŪ | Ūaba |
 +------+------+------+------*/
```

In the following example, all leading and trailing `b'n'`, `b'a'`, `b'\xab'`
bytes are removed from `item`.

```sql
SELECT b'apple', TRIM(b'apple', b'na\xab') AS example

/*----------------------+------------------+
 | item                 | example          |
 +----------------------+------------------+
 | apple                | pple             |
 +----------------------+------------------*/
```

## `UNICODE`

```sql
UNICODE(value)
```

**Description**

Returns the Unicode [code point][string-code-point] for the first character in
`value`. Returns `0` if `value` is empty, or if the resulting Unicode code
point is `0`.

**Return type**

`BIGINT`

**Examples**

```sql
SELECT UNICODE('âbcd') as A, UNICODE('â') as B, UNICODE('') as C, UNICODE(NULL) as D;

/*-------+-------+-------+-------+
 | A     | B     | C     | D     |
 +-------+-------+-------+-------+
 | 226   | 226   | 0     | NULL  |
 +-------+-------+-------+-------*/
```

[string-code-point]: https://en.wikipedia.org/wiki/Code_point
