# String Search Functions

Functions for searching and comparing strings.

## `EDIT_DISTANCE`

```sql
EDIT_DISTANCE(
  value1,
  value2,
  [ max_distance => max_distance_value ]
)
```

**Description**

Computes the [Levenshtein distance][l-distance] between two `VARCHAR` or
`VARBINARY` values.

**Definitions**

- `value1`: The first `VARCHAR` or `VARBINARY` value to compare.
- `value2`: The second `VARCHAR` or `VARBINARY` value to compare.
- `max_distance`: A named argument with a `BIGINT` value that's greater than
  or equal to zero. Represents the maximum distance between the two values
  to compute.

  If this distance is exceeded, the function returns this value.
  The default value for this argument is the maximum size of
  `value1` and `value2`.

**Details**

If `value1` or `value2` is `NULL`, `NULL` is returned.

You can only compare values of the same type. Otherwise, an error is produced.

**Return type**

`BIGINT`

**Examples**

In the following example, the first character in both strings is different:

```sql
SELECT EDIT_DISTANCE('a', 'b') AS results;

/*---------+
 | results |
 +---------+
 | 1       |
 +---------*/
```

In the following example, the first and second characters in both strings are
different:

```sql
SELECT EDIT_DISTANCE('aa', 'b') AS results;

/*---------+
 | results |
 +---------+
 | 2       |
 +---------*/
```

In the following example, only the first character in both strings is
different:

```sql
SELECT EDIT_DISTANCE('aa', 'ba') AS results;

/*---------+
 | results |
 +---------+
 | 1       |
 +---------*/
```

In the following example, the last six characters are different, but because
the maximum distance is `2`, this function exits early and returns `2`, the
maximum distance:

```sql
SELECT EDIT_DISTANCE('abcdefg', 'a', max_distance => 2) AS results;

/*---------+
 | results |
 +---------+
 | 2       |
 +---------*/
```

[l-distance]: https://en.wikipedia.org/wiki/Levenshtein_distance

## `ENDS_WITH`

```sql
ENDS_WITH(value, suffix)
```

**Description**

Takes two `VARCHAR` or `VARBINARY` values. Returns `TRUE` if `suffix`
is a suffix of `value`.

This function supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return type**

`BOOL`

**Examples**

```sql
SELECT ENDS_WITH('apple', 'e') as example

/*---------+
 | example |
 +---------+
 |    True |
 +---------*/
```

## `INSTR`

```sql
INSTR(value, subvalue[, position[, occurrence]])
```

**Description**

Returns the lowest 1-based position of `subvalue` in `value`.
`value` and `subvalue` must be the same type, either
`VARCHAR` or `VARBINARY`.

If `position` is specified, the search starts at this position in
`value`, otherwise it starts at `1`, which is the beginning of
`value`. If `position` is negative, the function searches backwards
from the end of `value`, with `-1` indicating the last character.
`position` is of type `BIGINT` and can't be `0`.

If `occurrence` is specified, the search returns the position of a specific
instance of `subvalue` in `value`. If not specified, `occurrence`
defaults to `1` and returns the position of the first occurrence.
For `occurrence` > `1`, the function includes overlapping occurrences.
`occurrence` is of type `BIGINT` and must be positive.

This function supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

Returns `0` if:

- No match is found.
- If `occurrence` is greater than the number of matches found.
- If `position` is greater than the length of `value`.

Returns `NULL` if:

- Any input argument is `NULL`.

Returns an error if:

- `position` is `0`.
- `occurrence` is `0` or negative.

**Return type**

`BIGINT`

**Examples**

```sql
SELECT
  'banana' AS value, 'an' AS subvalue, 1 AS position, 1 AS occurrence,
  INSTR('banana', 'an', 1, 1) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | banana       | an           | 1        | 1          | 2     |
 +--------------+--------------+----------+------------+-------*/
```

```sql
SELECT
  'banana' AS value, 'an' AS subvalue, 1 AS position, 2 AS occurrence,
  INSTR('banana', 'an', 1, 2) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | banana       | an           | 1        | 2          | 4     |
 +--------------+--------------+----------+------------+-------*/
```

```sql
SELECT
  'banana' AS value, 'an' AS subvalue, 1 AS position, 3 AS occurrence,
  INSTR('banana', 'an', 1, 3) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | banana       | an           | 1        | 3          | 0     |
 +--------------+--------------+----------+------------+-------*/
```

```sql
SELECT
  'banana' AS value, 'an' AS subvalue, 3 AS position, 1 AS occurrence,
  INSTR('banana', 'an', 3, 1) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | banana       | an           | 3        | 1          | 4     |
 +--------------+--------------+----------+------------+-------*/
```

```sql
SELECT
  'banana' AS value, 'an' AS subvalue, -1 AS position, 1 AS occurrence,
  INSTR('banana', 'an', -1, 1) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | banana       | an           | -1       | 1          | 4     |
 +--------------+--------------+----------+------------+-------*/
```

```sql
SELECT
  'banana' AS value, 'an' AS subvalue, -3 AS position, 1 AS occurrence,
  INSTR('banana', 'an', -3, 1) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | banana       | an           | -3       | 1          | 4     |
 +--------------+--------------+----------+------------+-------*/
```

```sql
SELECT
  'banana' AS value, 'ann' AS subvalue, 1 AS position, 1 AS occurrence,
  INSTR('banana', 'ann', 1, 1) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | banana       | ann          | 1        | 1          | 0     |
 +--------------+--------------+----------+------------+-------*/
```

```sql
SELECT
  'helloooo' AS value, 'oo' AS subvalue, 1 AS position, 1 AS occurrence,
  INSTR('helloooo', 'oo', 1, 1) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | helloooo     | oo           | 1        | 1          | 5     |
 +--------------+--------------+----------+------------+-------*/
```

```sql
SELECT
  'helloooo' AS value, 'oo' AS subvalue, 1 AS position, 2 AS occurrence,
  INSTR('helloooo', 'oo', 1, 2) AS instr;

/*--------------+--------------+----------+------------+-------+
 | value        | subvalue     | position | occurrence | instr |
 +--------------+--------------+----------+------------+-------+
 | helloooo     | oo           | 1        | 2          | 6     |
 +--------------+--------------+----------+------------+-------*/
```

## `SOUNDEX`

```sql
SOUNDEX(value)
```

**Description**

Returns a `VARCHAR` that represents the
[Soundex][string-link-to-soundex-wikipedia] code for `value`.

SOUNDEX produces a phonetic representation of a string. It indexes words by
sound, as pronounced in English. It's typically used to help determine whether
two strings, such as the family names _Levine_ and _Lavine_, or the words _to_
and _too_, have similar English-language pronunciation.

The result of the SOUNDEX consists of a letter followed by 3 digits. Non-latin
characters are ignored. If the remaining string is empty after removing
non-Latin characters, an empty `VARCHAR` is returned.

**Return type**

`VARCHAR`

**Examples**

```sql
SELECT 'Ashcraft' AS value, SOUNDEX('Ashcraft') AS soundex

/*----------------------+---------+
 | value                | soundex |
 +----------------------+---------+
 | Ashcraft             | A261    |
 +----------------------+---------*/
```

[string-link-to-soundex-wikipedia]: https://en.wikipedia.org/wiki/Soundex

## `STARTS_WITH`

```sql
STARTS_WITH(value, prefix)
```

**Description**

Takes two `VARCHAR` or `VARBINARY` values. Returns `TRUE` if `prefix` is a
prefix of `value`.

This function supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return type**

`BOOL`

**Examples**

```sql
SELECT STARTS_WITH('bar', 'b') AS example

/*---------+
 | example |
 +---------+
 |    True |
 +---------*/
```

## `STRPOS`

```sql
STRPOS(value, subvalue)
```

**Description**

Takes two `VARCHAR` or `VARBINARY` values. Returns the 1-based position of the first
occurrence of `subvalue` inside `value`. Returns `0` if `subvalue` isn't found.

This function supports specifying [collation][collation].

[collation]: ../types/collation_concepts.md

**Return type**

`BIGINT`

**Examples**

```sql
SELECT STRPOS('foo@example.com', '@') AS example

/*---------+
 | example |
 +---------+
 |       4 |
 +---------*/
```
