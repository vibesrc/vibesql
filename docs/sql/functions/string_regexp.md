# Regular Expression Functions

Functions for pattern matching with regular expressions.

## `REGEXP_CONTAINS`

```sql
REGEXP_CONTAINS(value, regexp)
```

**Description**

Returns `TRUE` if `value` is a partial match for the regular expression,
`regexp`.

If the `regexp` argument is invalid, the function returns an error.

You can search for a full match by using `^` (beginning of text) and `$` (end of
text). Due to regular expression operator precedence, it's good practice to use
parentheses around everything between `^` and `$`.

Note: SQL provides regular expression support using the
[re2][string-link-to-re2] library; see that documentation for its
regular expression syntax.

**Return type**

`BOOL`

**Examples**

The following queries check to see if an email is valid:

```sql
SELECT
  'foo@example.com' AS email,
  REGEXP_CONTAINS('foo@example.com', r'@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+') AS is_valid

/*-----------------+----------+
 | email           | is_valid |
 +-----------------+----------+
 | foo@example.com | TRUE     |
 +-----------------+----------*/
```

```sql
SELECT
 'www.example.net' AS email,
 REGEXP_CONTAINS('www.example.net', r'@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+') AS is_valid

/*-----------------+----------+
| email           | is_valid |
+-----------------+----------+
| www.example.net | FALSE    |
+-----------------+----------*/
```

The following queries check to see if an email is valid. They
perform a full match, using `^` and `$`. Due to regular expression operator
precedence, it's good practice to use parentheses around everything between `^`
and `$`.

```sql
SELECT
  'a@foo.com' AS email,
  REGEXP_CONTAINS('a@foo.com', r'^([\w.+-]+@foo\.com|[\w.+-]+@bar\.org)$') AS valid_email_address,
  REGEXP_CONTAINS('a@foo.com', r'^[\w.+-]+@foo\.com|[\w.+-]+@bar\.org$') AS without_parentheses;

/*----------------+---------------------+---------------------+
 | email          | valid_email_address | without_parentheses |
 +----------------+---------------------+---------------------+
 | a@foo.com      | true                | true                |
 +----------------+---------------------+---------------------*/
```

```sql
SELECT
  'a@foo.computer' AS email,
  REGEXP_CONTAINS('a@foo.computer', r'^([\w.+-]+@foo\.com|[\w.+-]+@bar\.org)$') AS valid_email_address,
  REGEXP_CONTAINS('a@foo.computer', r'^[\w.+-]+@foo\.com|[\w.+-]+@bar\.org$') AS without_parentheses;

/*----------------+---------------------+---------------------+
 | email          | valid_email_address | without_parentheses |
 +----------------+---------------------+---------------------+
 | a@foo.computer | false               | true                |
 +----------------+---------------------+---------------------*/
```

```sql
SELECT
  'b@bar.org' AS email,
  REGEXP_CONTAINS('b@bar.org', r'^([\w.+-]+@foo\.com|[\w.+-]+@bar\.org)$') AS valid_email_address,
  REGEXP_CONTAINS('b@bar.org', r'^[\w.+-]+@foo\.com|[\w.+-]+@bar\.org$') AS without_parentheses;

/*----------------+---------------------+---------------------+
 | email          | valid_email_address | without_parentheses |
 +----------------+---------------------+---------------------+
 | b@bar.org      | true                | true                |
 +----------------+---------------------+---------------------*/
```

```sql
SELECT
  '!b@bar.org' AS email,
  REGEXP_CONTAINS('!b@bar.org', r'^([\w.+-]+@foo\.com|[\w.+-]+@bar\.org)$') AS valid_email_address,
  REGEXP_CONTAINS('!b@bar.org', r'^[\w.+-]+@foo\.com|[\w.+-]+@bar\.org$') AS without_parentheses;

/*----------------+---------------------+---------------------+
 | email          | valid_email_address | without_parentheses |
 +----------------+---------------------+---------------------+
 | !b@bar.org     | false               | true                |
 +----------------+---------------------+---------------------*/
```

```sql
SELECT
  'c@buz.net' AS email,
  REGEXP_CONTAINS('c@buz.net', r'^([\w.+-]+@foo\.com|[\w.+-]+@bar\.org)$') AS valid_email_address,
  REGEXP_CONTAINS('c@buz.net', r'^[\w.+-]+@foo\.com|[\w.+-]+@bar\.org$') AS without_parentheses;

/*----------------+---------------------+---------------------+
 | email          | valid_email_address | without_parentheses |
 +----------------+---------------------+---------------------+
 | c@buz.net      | false               | false               |
 +----------------+---------------------+---------------------*/
```

[string-link-to-re2]: https://github.com/google/re2/wiki/Syntax

## `REGEXP_EXTRACT`

```sql
REGEXP_EXTRACT(value, regexp[, position[, occurrence]])
```

**Description**

Returns the substring in `value` that matches the
[re2 regular expression][string-link-to-re2], `regexp`.
Returns `NULL` if there is no match.

If the regular expression contains a capturing group (`(...)`), and there is a
match for that capturing group, that match is returned. If there
are multiple matches for a capturing group, the first match is returned.

To extract matches for multiple capturing groups in a single call, use
[`REGEXP_EXTRACT_GROUPS`][regexp-extract-groups].

If `position` is specified, the search starts at this
position in `value`, otherwise it starts at the beginning of `value`. The
`position` must be a positive integer and can't be 0. If `position` is greater
than the length of `value`, `NULL` is returned.

If `occurrence` is specified, the search returns a specific occurrence of the
`regexp` in `value`, otherwise returns the first match. If `occurrence` is
greater than the number of matches found, `NULL` is returned. For
`occurrence` > 1, the function searches for additional occurrences beginning
with the character following the previous occurrence.

Returns an error if:

- The regular expression is invalid
- The regular expression has more than one capturing group
- The `position` isn't a positive integer
- The `occurrence` isn't a positive integer

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT REGEXP_EXTRACT('foo@example.com', r'^[a-zA-Z0-9_.+-]+') AS user_name

/*-----------+
 | user_name |
 +-----------+
 | foo       |
 +-----------*/
```

```sql
SELECT REGEXP_EXTRACT('foo@example.com', r'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.([a-zA-Z0-9-.]+$)')

/*------------------+
 | top_level_domain |
 +------------------+
 | com              |
 +------------------*/
```

```sql
SELECT
  REGEXP_EXTRACT('ab', '.b') AS result_a,
  REGEXP_EXTRACT('ab', '(.)b') AS result_b,
  REGEXP_EXTRACT('xyztb', '(.)+b') AS result_c,
  REGEXP_EXTRACT('ab', '(z)?b') AS result_d

/*-------------------------------------------+
 | result_a | result_b | result_c | result_d |
 +-------------------------------------------+
 | ab       | a        | t        | NULL     |
 +-------------------------------------------*/
```

```sql
WITH example AS
(SELECT 'Hello Helloo and Hellooo' AS value, 'H?ello+' AS regex, 1 as position,
1 AS occurrence UNION ALL
SELECT 'Hello Helloo and Hellooo', 'H?ello+', 1, 2 UNION ALL
SELECT 'Hello Helloo and Hellooo', 'H?ello+', 1, 3 UNION ALL
SELECT 'Hello Helloo and Hellooo', 'H?ello+', 1, 4 UNION ALL
SELECT 'Hello Helloo and Hellooo', 'H?ello+', 2, 1 UNION ALL
SELECT 'Hello Helloo and Hellooo', 'H?ello+', 3, 1 UNION ALL
SELECT 'Hello Helloo and Hellooo', 'H?ello+', 3, 2 UNION ALL
SELECT 'Hello Helloo and Hellooo', 'H?ello+', 3, 3 UNION ALL
SELECT 'Hello Helloo and Hellooo', 'H?ello+', 20, 1 UNION ALL
SELECT 'cats&dogs&rabbits' ,'\\w+&', 1, 2 UNION ALL
SELECT 'cats&dogs&rabbits', '\\w+&', 2, 3
)
SELECT value, regex, position, occurrence, REGEXP_EXTRACT(value, regex,
position, occurrence) AS regexp_value FROM example;

/*--------------------------+---------+----------+------------+--------------+
 | value                    | regex   | position | occurrence | regexp_value |
 +--------------------------+---------+----------+------------+--------------+
 | Hello Helloo and Hellooo | H?ello+ | 1        | 1          | Hello        |
 | Hello Helloo and Hellooo | H?ello+ | 1        | 2          | Helloo       |
 | Hello Helloo and Hellooo | H?ello+ | 1        | 3          | Hellooo      |
 | Hello Helloo and Hellooo | H?ello+ | 1        | 4          | NULL         |
 | Hello Helloo and Hellooo | H?ello+ | 2        | 1          | ello         |
 | Hello Helloo and Hellooo | H?ello+ | 3        | 1          | Helloo       |
 | Hello Helloo and Hellooo | H?ello+ | 3        | 2          | Hellooo      |
 | Hello Helloo and Hellooo | H?ello+ | 3        | 3          | NULL         |
 | Hello Helloo and Hellooo | H?ello+ | 20       | 1          | NULL         |
 | cats&dogs&rabbits        | \w+&    | 1        | 2          | dogs&        |
 | cats&dogs&rabbits        | \w+&    | 2        | 3          | NULL         |
 +--------------------------+---------+----------+------------+--------------*/
```

[string-link-to-re2]: https://github.com/google/re2/wiki/Syntax
[regexp-extract-groups]: string_functions.md#regexp-extract_groups

## `REGEXP_EXTRACT_GROUPS`

```sql
REGEXP_EXTRACT_GROUPS(value, regexp)
```

**Description**

Returns a `STRUCT` where each field contains a substring from `value` that
matches a capturing group in the [re2 regular expression][string-link-to-re2],
`regexp`. The function returns the substrings from the first place in `value`
where the _entire_ `regexp` pattern matches.

**Details**

This function is similar to [`REGEXP_EXTRACT`][regexp-extract], but it returns a
`STRUCT` with a field for each capturing group in the `regexp`.

The `regexp` must contain at least one capturing group. The fields in the
returned `STRUCT` correspond to these capturing groups:

- If a capturing group is named (for example, `(?...)` or `(?P...)`),
  the corresponding `STRUCT` field will have that name. Both syntaxes are
  equivalent.
- If a capturing group is unnamed, the corresponding `STRUCT` field is
  anonymous. These fields can be accessed by their 0-based position in the
  `STRUCT`.
- The order of fields in the `STRUCT` matches the order of the capturing
  groups in `regexp` from left to right.

Returns `NULL` if `value` is `NULL` or if the overall `regexp` pattern doesn't
match at all. If a specific capturing group doesn't match (for example, if it's
part of an alternation or is optional), the corresponding `STRUCT` field is
`NULL`.

Returns an error if:

- The `regexp` is invalid.
- The `regexp` is not a string literal.
- The `regexp` has no capturing groups.
- A capturing group name is not a valid `STRUCT` field name (for example, starts
  with a digit or contains spaces). Valid names consist of letters, numbers,
  and underscores, and must start with a letter or underscore.
- The same capturing group name is used more than once (case-insensitive).

**Return type**

`STRUCT`

The fields of the `STRUCT` are generally `VARCHAR` (or `VARBINARY` if the inputs are
`VARBINARY`). However, fields can be [auto-casted](#auto-casting) to other types.

**Examples**

Extract unnamed groups:

```sql
SELECT REGEXP_EXTRACT_GROUPS('abc123xyz', r'([a-z]+)([0-9]+)([a-z]+)') AS result

/*---------------------------------+
 | result                          |
 +---------------------------------+
 | {abc, 123, xyz}                 |
 +---------------------------------*/
```

Extract named groups:

```sql
SELECT REGEXP_EXTRACT_GROUPS('2025-09-10', r'(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})') AS result

/*----------------------------------------------+
 | result                                       |
 +----------------------------------------------+
 | {2025 year, 09 month, 10 day}                |
 +----------------------------------------------*/
```

**Expand STRUCT fields into columns**

Because `REGEXP_EXTRACT_GROUPS` returns a `STRUCT`, you can use the `.*` operator
in the `SELECT` list to expand the fields of the `STRUCT` into separate columns.
Expanding `STRUCT` fields into columns is particularly useful when all capturing
groups are named.

```sql
SELECT REGEXP_EXTRACT_GROUPS('PROD-WIDGET-1234', r'(?<env>\w+)-(?<product>\w+)-(?<id>\d+)').*

/*-------+-----------+------+
 | env   | product   | id   |
 +-------+-----------+------+
 | PROD  | WIDGET    | 1234 |
 +-------+-----------+------*/
```

Mix of named and unnamed groups:

```sql
SELECT REGEXP_EXTRACT_GROUPS('id:123', r'(?<key>[a-z]+):([0-9]+)') AS result

/*-----------------------+
 | result                |
 +-----------------------+
 | {id key, 123}         |
 +-----------------------*/
```

No match returns `NULL`:

```sql
SELECT REGEXP_EXTRACT_GROUPS('abc', r'(\d+)') AS result

/*--------+
 | result |
 +--------+
 | NULL   |
 +--------*/
```

Optional groups and empty matches:

```sql
WITH inputs AS (
  SELECT 'id:123:extra' AS t UNION ALL
  SELECT 'id:123:' AS t UNION ALL
  SELECT 'id:123' AS t
)
SELECT
  t,
  REGEXP_EXTRACT_GROUPS(t, r'(?<key>\w+):(?<val>\w+)(?::(?<opt>\w*))?') AS result
FROM inputs;

/*-----------------+--------------------------------------+
 | t               | result                               |
 +-----------------+--------------------------------------+
 | id:123:extra    | {id key, 123 val, extra opt}         |
 | id:123:         | {id key, 123 val,  opt}              |
 | id:123          | {id key, 123 val, NULL opt}          |
 +-----------------+--------------------------------------*/
```

Note that in the second row, the optional group `opt` matches an empty string,
which is different from the third row where the group doesn't match at all and
results in `NULL`.

Nested groups:

```sql
SELECT REGEXP_EXTRACT_GROUPS('a=b=c', r'(\w+)=((\w+)=\w+)') AS result

/*-----------------------+
 | result                |
 +-----------------------+
 | {a, b=c, b}           |
 +-----------------------*/
```

Alternation with different groups:

```sql
WITH inputs AS (
  SELECT 'config_id=123' AS t UNION ALL
  SELECT 'option_name=ABC' AS t
)
SELECT
  t,
  REGEXP_EXTRACT_GROUPS(t, r'config_id=(?<id>\d+)|option_name=(?<name>\w+)') AS result
FROM inputs;

/*-----------------+--------------------------+
 | t               | result                   |
 +-----------------+--------------------------+
 | config_id=123   | {123 id, NULL name}      |
 | option_name=ABC | {NULL id, ABC name}      |
 +-----------------+--------------------------*/
```

The `STRUCT` result contains fields for all named capturing groups across all
alternatives in the regular expression. In each row, only the fields
corresponding to the alternative that matched are populated. Other fields are
`NULL`.

##### Auto-casting

You can automatically cast the captured substring to a specific type by
suffixing the capturing group name with a double underscore (`__`) followed by
the type name.

Any type that can be cast from `VARCHAR` (or `VARBINARY` for the `VARBINARY` version
of the function) is supported. Type names are case-insensitive.

The field name in the resulting `STRUCT` will have the `__TYPE` suffix removed.

If the captured substring can't be cast to the specified type, an error is
returned. This includes casting an empty string to a numeric or boolean type.
If the captured substring is `NULL` (due to an optional group not matching), the
cast result is also `NULL`.

**Examples of auto-casting**

```sql
SELECT REGEXP_EXTRACT_GROUPS('val=0x1a', r'val=(?<val__BIGINT>0x[0-9a-fA-F]+)') AS result

/*-------------+
 | result      |
 +-------------+
 | {26 val}    |
 +-------------*/
```

Auto-casted values in expressions with Pipe syntax:

```sql
FROM UNNEST(['02:30:10', '01:02:03']) AS time_str
|> EXTEND REGEXP_EXTRACT_GROUPS(time_str, r'(?<h__BIGINT>\d{2}):(?<m__BIGINT>\d{2}):(?<s__BIGINT>\d{2})').*
|> SELECT time_str, h * 3600 + m * 60 + s AS total_seconds

/*----------+---------------+
 | time_str | total_seconds |
 +----------+---------------+
 | 02:30:10 | 9010          |
 | 01:02:03 | 3723          |
 +----------+---------------*/
```

Expand auto-casted fields into columns:

```sql
SELECT REGEXP_EXTRACT_GROUPS('2025-09-10', r'(?<year__BIGINT>\d{4})-(?<month__BIGINT>\d{2})-(?<day__BIGINT>\d{2})').*

/*--------+---------+-------+
 | year   | month   | day   |
 +--------+---------+-------+
 | 2025   | 9       | 10    |
 +--------+---------+-------*/
```

Cast failure:

```sql {.bad}
-- Error: Bad BIGINT value
SELECT REGEXP_EXTRACT_GROUPS('ID: ABC', r'ID: (?<item_id__BIGINT>\w+)')
```

Cast failure with empty string:

```sql {.bad}
-- Error: Bad BIGINT value
SELECT REGEXP_EXTRACT_GROUPS('ID: ', r'ID: (?<item_id__BIGINT>\d*)')
```

Workaround for empty string cast failure by making the group optional:

```sql
SELECT REGEXP_EXTRACT_GROUPS('ID: ', r'ID: (?<item_id__BIGINT>\d+)?') AS result

/*-----------------+
 | result          |
 +-----------------+
 | {NULL item_id}  |
 +-----------------*/
```

[string-link-to-re2]: https://github.com/google/re2/wiki/Syntax
[regexp-extract]: string_functions.md#regexp-extract

## `REGEXP_EXTRACT_ALL`

```sql
REGEXP_EXTRACT_ALL(value, regexp)
```

**Description**

Returns an array of all substrings of `value` that match the
[re2 regular expression][string-link-to-re2], `regexp`. Returns an empty array
if there is no match.

If the regular expression contains a capturing group (`(...)`), and there is a
match for that capturing group, that match is added to the results.

The `REGEXP_EXTRACT_ALL` function only returns non-overlapping matches. For
example, using this function to extract `ana` from `banana` returns only one
substring, not two.

Returns an error if:

- The regular expression is invalid
- The regular expression has more than one capturing group

**Return type**

`ARRAY` or `ARRAY`

**Examples**

```sql
SELECT REGEXP_EXTRACT_ALL('Try `func(x)` or `func(y)`', '`(.+?)`') AS example

/*--------------------+
 | example            |
 +--------------------+
 | [func(x), func(y)] |
 +--------------------*/
```

[string-link-to-re2]: https://github.com/google/re2/wiki/Syntax

## `REGEXP_INSTR`

```sql
REGEXP_INSTR(source_value, regexp [, position[, occurrence, [occurrence_position]]])
```

**Description**

Returns the lowest 1-based position of a regular expression, `regexp`, in
`source_value`. `source_value` and `regexp` must be the same type, either
`VARCHAR` or `VARBINARY`.

If `position` is specified, the search starts at this position in
`source_value`, otherwise it starts at `1`, which is the beginning of
`source_value`. `position` is of type `BIGINT` and must be positive.

If `occurrence` is specified, the search returns the position of a specific
instance of `regexp` in `source_value`. If not specified, `occurrence` defaults
to `1` and returns the position of the first occurrence. For `occurrence` > 1,
the function searches for the next, non-overlapping occurrence.
`occurrence` is of type `BIGINT` and must be positive.

You can optionally use `occurrence_position` to specify where a position
in relation to an `occurrence` starts. Your choices are:

- `0`: Returns the start position of `occurrence`.
- `1`: Returns the end position of `occurrence` + `1`. If the
  end of the occurrence is at the end of `source_value `,
  `LENGTH(source_value) + 1` is returned.

Returns `0` if:

- No match is found.
- If `occurrence` is greater than the number of matches found.
- If `position` is greater than the length of `source_value`.
- The regular expression is empty.

Returns `NULL` if:

- `position` is `NULL`.
- `occurrence` is `NULL`.

Returns an error if:

- `position` is `0` or negative.
- `occurrence` is `0` or negative.
- `occurrence_position` is neither `0` nor `1`.
- The regular expression is invalid.
- The regular expression has more than one capturing group.

**Return type**

`BIGINT`

**Examples**

```sql
SELECT
  REGEXP_INSTR('ab@cd-ef',  '@[^-]*') AS instr_a,
  REGEXP_INSTR('ab@d-ef',   '@[^-]*') AS instr_b,
  REGEXP_INSTR('abc@cd-ef', '@[^-]*') AS instr_c,
  REGEXP_INSTR('abc-ef',    '@[^-]*') AS instr_d,

/*---------------------------------------+
 | instr_a | instr_b | instr_c | instr_d |
 +---------------------------------------+
 | 3       | 3       | 4       | 0       |
 +---------------------------------------*/
```

```sql
SELECT
  REGEXP_INSTR('a@cd-ef b@cd-ef', '@[^-]*', 1) AS instr_a,
  REGEXP_INSTR('a@cd-ef b@cd-ef', '@[^-]*', 2) AS instr_b,
  REGEXP_INSTR('a@cd-ef b@cd-ef', '@[^-]*', 3) AS instr_c,
  REGEXP_INSTR('a@cd-ef b@cd-ef', '@[^-]*', 4) AS instr_d,

/*---------------------------------------+
 | instr_a | instr_b | instr_c | instr_d |
 +---------------------------------------+
 | 2       | 2       | 10      | 10      |
 +---------------------------------------*/
```

```sql
SELECT
  REGEXP_INSTR('a@cd-ef b@cd-ef c@cd-ef', '@[^-]*', 1, 1) AS instr_a,
  REGEXP_INSTR('a@cd-ef b@cd-ef c@cd-ef', '@[^-]*', 1, 2) AS instr_b,
  REGEXP_INSTR('a@cd-ef b@cd-ef c@cd-ef', '@[^-]*', 1, 3) AS instr_c

/*-----------------------------+
 | instr_a | instr_b | instr_c |
 +-----------------------------+
 | 2       | 10      | 18      |
 +-----------------------------*/
```

```sql
SELECT
  REGEXP_INSTR('a@cd-ef', '@[^-]*', 1, 1, 0) AS instr_a,
  REGEXP_INSTR('a@cd-ef', '@[^-]*', 1, 1, 1) AS instr_b

/*-------------------+
 | instr_a | instr_b |
 +-------------------+
 | 2       | 5       |
 +-------------------*/
```

## `REGEXP_MATCH` (Deprecated)

```sql
REGEXP_MATCH(value, regexp)
```

**Description**

Returns `TRUE` if `value` is a full match for the regular expression, `regexp`.

If the `regexp` argument is invalid, the function returns an error.

This function is deprecated. When possible, use
[`REGEXP_CONTAINS`][regexp-contains] to find a partial match for a
regular expression.

Note: SQL provides regular expression support using the
[re2][string-link-to-re2] library; see that documentation for its
regular expression syntax.

**Return type**

`BOOL`

**Examples**

```sql
WITH email_addresses AS
  (SELECT 'foo@example.com' as email
  UNION ALL
  SELECT 'bar@example.org' as email
  UNION ALL
  SELECT 'notavalidemailaddress' as email)

SELECT
  email,
  REGEXP_MATCH(email,
               r'[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+')
               AS valid_email_address
FROM email_addresses;

/*-----------------------+---------------------+
 | email                 | valid_email_address |
 +-----------------------+---------------------+
 | foo@example.com       | true                |
 | bar@example.org       | true                |
 | notavalidemailaddress | false               |
 +-----------------------+---------------------*/
```

[string-link-to-re2]: https://github.com/google/re2/wiki/Syntax
[regexp-contains]: string_functions.md#regexp-contains

## `REGEXP_REPLACE`

```sql
REGEXP_REPLACE(value, regexp, replacement)
```

**Description**

Returns a `VARCHAR` where all substrings of `value` that
match regular expression `regexp` are replaced with `replacement`.

You can use backslashed-escaped digits (\1 to \9) within the `replacement`
argument to insert text matching the corresponding parenthesized group in the
`regexp` pattern. Use \0 to refer to the entire matching text.

To add a backslash in your regular expression, you must first escape it. For
example, `SELECT REGEXP_REPLACE('abc', 'b(.)', 'X\\1');` returns `aXc`. You can
also use [raw strings][string-link-to-lexical-literals] to remove one layer of
escaping, for example `SELECT REGEXP_REPLACE('abc', 'b(.)', r'X\1');`.

The `REGEXP_REPLACE` function only replaces non-overlapping matches. For
example, replacing `ana` within `banana` results in only one replacement, not
two.

If the `regexp` argument isn't a valid regular expression, this function
returns an error.

Note: SQL provides regular expression support using the
[re2][string-link-to-re2] library; see that documentation for its
regular expression syntax.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT REGEXP_REPLACE('# Heading', r'^# ([a-zA-Z0-9\s]+$)', '<h1>\\1</h1>') AS html

/*--------------------------+
 | html                     |
 +--------------------------+
 | <h1>Heading</h1>         |
 +--------------------------*/
```

[string-link-to-re2]: https://github.com/google/re2/wiki/Syntax
[string-link-to-lexical-literals]: ../syntax/lexical.md#string-and_bytes_literals

## `REGEXP_SUBSTR`

```sql
REGEXP_SUBSTR(value, regexp[, position[, occurrence]])
```

**Description**

Synonym for [REGEXP_EXTRACT][string-link-to-regex].

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
WITH example AS
(SELECT 'Hello World Helloo' AS value, 'H?ello+' AS regex, 1 AS position, 1 AS
occurrence
)
SELECT value, regex, position, occurrence, REGEXP_SUBSTR(value, regex,
position, occurrence) AS regexp_value FROM example;

/*--------------------+---------+----------+------------+--------------+
 | value              | regex   | position | occurrence | regexp_value |
 +--------------------+---------+----------+------------+--------------+
 | Hello World Helloo | H?ello+ | 1        | 1          | Hello        |
 +--------------------+---------+----------+------------+--------------*/
```

[string-link-to-regex]: #regexp-extract
