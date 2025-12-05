# String Case and Collation Functions

Functions for case conversion, normalization, and collation.

## `COLLATE`

```sql
COLLATE(value, collate_specification)
```

Takes a `VARCHAR` and a [collation specification][link-collation-spec]. Returns
a `VARCHAR` with a collation specification. If `collate_specification` is empty,
returns a value with collation removed from the `VARCHAR`.

The collation specification defines how the resulting `VARCHAR` can be compared
and sorted. To learn more, see
[Collation][link-collation-concepts].

- `collation_specification` must be a string literal, otherwise an error is
  thrown.
- Returns `NULL` if `value` is `NULL`.

**Return type**

`VARCHAR`

**Examples**

In this example, the weight of `a` is less than the weight of `Z`. This
is because the collate specification, `und:ci` assigns more weight to `Z`.

```sql
WITH Words AS (
  SELECT
    COLLATE('a', 'und:ci') AS char1,
    COLLATE('Z', 'und:ci') AS char2
)
SELECT ( Words.char1 < Words.char2 ) AS a_less_than_Z
FROM Words;

/*----------------+
 | a_less_than_Z  |
 +----------------+
 | TRUE           |
 +----------------*/
```

In this example, the weight of `a` is greater than the weight of `Z`. This
is because the default collate specification assigns more weight to `a`.

```sql
WITH Words AS (
  SELECT
    'a' AS char1,
    'Z' AS char2
)
SELECT ( Words.char1 < Words.char2 ) AS a_less_than_Z
FROM Words;

/*----------------+
 | a_less_than_Z  |
 +----------------+
 | FALSE          |
 +----------------*/
```

[link-collation-spec]: ../types/collation_concepts.md#collate-spec_details
[link-collation-concepts]: ../types/collation_concepts.md

## `INITCAP`

```sql
INITCAP(value[, delimiters])
```

**Description**

Takes a `VARCHAR` and returns it with the first character in each word in
uppercase and all other characters in lowercase. Non-alphabetic characters
remain the same.

`delimiters` is an optional string argument that's used to override the default
set of characters used to separate words. If `delimiters` isn't specified, it
defaults to the following characters: \
`<whitespace> [ ] ( ) { } / | \ < > ! ? @ " ^ # $ & ~ _ , . : ; * % + -`

If `value` or `delimiters` is `NULL`, the function returns `NULL`.

**Return type**

`VARCHAR`

**Examples**

```sql
SELECT
  'Hello World-everyone!' AS value,
  INITCAP('Hello World-everyone!') AS initcap_value

/*-------------------------------+-------------------------------+
 | value                         | initcap_value                 |
 +-------------------------------+-------------------------------+
 | Hello World-everyone!         | Hello World-Everyone!         |
 +-------------------------------+-------------------------------*/
```

```sql
SELECT
  'Apples1oranges2pears' as value,
  '12' AS delimiters,
  INITCAP('Apples1oranges2pears' , '12') AS initcap_value

/*----------------------+------------+----------------------+
 | value                | delimiters | initcap_value        |
 +----------------------+------------+----------------------+
 | Apples1oranges2pears | 12         | Apples1Oranges2Pears |
 +----------------------+------------+----------------------*/
```

## `LOWER`

```sql
LOWER(value)
```

**Description**

For `VARCHAR` arguments, returns the original string with all alphabetic
characters in lowercase. Mapping between lowercase and uppercase is done
according to the
[Unicode Character Database][string-link-to-unicode-character-definitions]
without taking into account language-specific mappings.

For `VARBINARY` arguments, the argument is treated as ASCII text, with all bytes
greater than 127 left intact.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT
  LOWER('FOO BAR BAZ') AS example
FROM items;

/*-------------+
 | example     |
 +-------------+
 | foo bar baz |
 +-------------*/
```

[string-link-to-unicode-character-definitions]: http://unicode.org/ucd/

## `NORMALIZE`

```sql
NORMALIZE(value[, normalization_mode])
```

**Description**

Takes a string value and returns it as a normalized string. If you don't
provide a normalization mode, `NFC` is used.

[Normalization][string-link-to-normalization-wikipedia] is used to ensure that
two strings are equivalent. Normalization is often used in situations in which
two strings render the same on the screen but have different Unicode code
points.

`NORMALIZE` supports four optional normalization modes:

| Value  | Name                                           | Description                                                                                                         |
| ------ | ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `NFC`  | Normalization Form Canonical Composition       | Decomposes and recomposes characters by canonical equivalence.                                                      |
| `NFKC` | Normalization Form Compatibility Composition   | Decomposes characters by compatibility, then recomposes them by canonical equivalence.                              |
| `NFD`  | Normalization Form Canonical Decomposition     | Decomposes characters by canonical equivalence, and multiple combining characters are arranged in a specific order. |
| `NFKD` | Normalization Form Compatibility Decomposition | Decomposes characters by compatibility, and multiple combining characters are arranged in a specific order.         |

**Return type**

`VARCHAR`

**Examples**

The following example normalizes different language characters:

````sql
SELECT
  NORMALIZE('\u00ea') as a,
  NORMALIZE('\u0065\u0302') as b,
  NORMALIZE('\u00ea') = NORMALIZE('\u0065\u0302') as normalized;

/*---+---+------------+
 | a | b | normalized |
 +---+---+------------+
 | ê | ê | TRUE       |
 +---+---+------------*/
```sql
The following examples normalize different space characters:

```sql
SELECT NORMALIZE('Raha\u2004Mahan', NFKC) AS normalized_name

/*-----------------+
 | normalized_name |
 +-----------------+
 | Raha Mahan      |
 +-----------------*/
````

```sql
SELECT NORMALIZE('Raha\u2005Mahan', NFKC) AS normalized_name

/*-----------------+
 | normalized_name |
 +-----------------+
 | Raha Mahan      |
 +-----------------*/
```

```sql
SELECT NORMALIZE('Raha\u2006Mahan', NFKC) AS normalized_name

/*-----------------+
 | normalized_name |
 +-----------------+
 | Raha Mahan      |
 +-----------------*/
```

```sql
SELECT NORMALIZE('Raha Mahan', NFKC) AS normalized_name

/*-----------------+
 | normalized_name |
 +-----------------+
 | Raha Mahan      |
 +-----------------*/
```

[string-link-to-normalization-wikipedia]: https://en.wikipedia.org/wiki/Unicode_equivalence#Normalization

## `NORMALIZE_AND_CASEFOLD`

```sql
NORMALIZE_AND_CASEFOLD(value[, normalization_mode])
```

**Description**

Takes a string value and returns it as a normalized string. If you don't
provide a normalization mode, `NFC` is used.

[Normalization][string-link-to-normalization-wikipedia] is used to ensure that
two strings are equivalent. Normalization is often used in situations in which
two strings render the same on the screen but have different Unicode code
points.

[Case folding][string-link-to-case-folding-wikipedia] is used for the caseless
comparison of strings. If you need to compare strings and case shouldn't be
considered, use `NORMALIZE_AND_CASEFOLD`, otherwise use
[`NORMALIZE`][string-link-to-normalize].

`NORMALIZE_AND_CASEFOLD` supports four optional normalization modes:

| Value  | Name                                           | Description                                                                                                         |
| ------ | ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `NFC`  | Normalization Form Canonical Composition       | Decomposes and recomposes characters by canonical equivalence.                                                      |
| `NFKC` | Normalization Form Compatibility Composition   | Decomposes characters by compatibility, then recomposes them by canonical equivalence.                              |
| `NFD`  | Normalization Form Canonical Decomposition     | Decomposes characters by canonical equivalence, and multiple combining characters are arranged in a specific order. |
| `NFKD` | Normalization Form Compatibility Decomposition | Decomposes characters by compatibility, and multiple combining characters are arranged in a specific order.         |

**Return type**

`VARCHAR`

**Examples**

```sql
SELECT
  NORMALIZE('The red barn') = NORMALIZE('The Red Barn') AS normalized,
  NORMALIZE_AND_CASEFOLD('The red barn')
    = NORMALIZE_AND_CASEFOLD('The Red Barn') AS normalized_with_case_folding;

/*------------+------------------------------+
 | normalized | normalized_with_case_folding |
 +------------+------------------------------+
 | FALSE      | TRUE                         |
 +------------+------------------------------*/
```

```sql
SELECT
  '\u2168' AS a,
  'IX' AS b,
  NORMALIZE_AND_CASEFOLD('\u2168', NFD)=NORMALIZE_AND_CASEFOLD('IX', NFD) AS nfd,
  NORMALIZE_AND_CASEFOLD('\u2168', NFC)=NORMALIZE_AND_CASEFOLD('IX', NFC) AS nfc,
  NORMALIZE_AND_CASEFOLD('\u2168', NFKD)=NORMALIZE_AND_CASEFOLD('IX', NFKD) AS nfkd,
  NORMALIZE_AND_CASEFOLD('\u2168', NFKC)=NORMALIZE_AND_CASEFOLD('IX', NFKC) AS nfkc;

/*---+----+-------+-------+------+------+
 | a | b  | nfd   | nfc   | nfkd | nfkc |
 +---+----+-------+-------+------+------+
 | Ⅸ | IX | false | false | true | true |
 +---+----+-------+-------+------+------*/
```

```sql
SELECT
  '\u0041\u030A' AS a,
  '\u00C5' AS b,
  NORMALIZE_AND_CASEFOLD('\u0041\u030A', NFD)=NORMALIZE_AND_CASEFOLD('\u00C5', NFD) AS nfd,
  NORMALIZE_AND_CASEFOLD('\u0041\u030A', NFC)=NORMALIZE_AND_CASEFOLD('\u00C5', NFC) AS nfc,
  NORMALIZE_AND_CASEFOLD('\u0041\u030A', NFKD)=NORMALIZE_AND_CASEFOLD('\u00C5', NFKD) AS nkfd,
  NORMALIZE_AND_CASEFOLD('\u0041\u030A', NFKC)=NORMALIZE_AND_CASEFOLD('\u00C5', NFKC) AS nkfc;

/*---+----+-------+-------+------+------+
 | a | b  | nfd   | nfc   | nkfd | nkfc |
 +---+----+-------+-------+------+------+
 | Å | Å  | true  | true  | true | true |
 +---+----+-------+-------+------+------*/
```

[string-link-to-normalization-wikipedia]: https://en.wikipedia.org/wiki/Unicode_equivalence#Normalization
[string-link-to-case-folding-wikipedia]: https://en.wikipedia.org/wiki/Letter_case#Case_folding
[string-link-to-normalize]: #normalize

## `UPPER`

```sql
UPPER(value)
```

**Description**

For `VARCHAR` arguments, returns the original string with all alphabetic
characters in uppercase. Mapping between uppercase and lowercase is done
according to the
[Unicode Character Database][string-link-to-unicode-character-definitions]
without taking into account language-specific mappings.

For `VARBINARY` arguments, the argument is treated as ASCII text, with all bytes
greater than 127 left intact.

**Return type**

`VARCHAR` or `VARBINARY`

**Examples**

```sql
SELECT UPPER('foo') AS example

/*---------+
 | example |
 +---------+
 | FOO     |
 +---------*/
```

[string-link-to-unicode-character-definitions]: http://unicode.org/ucd/
[string-link-to-strpos]: #strpos
