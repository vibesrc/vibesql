# Collation

SQL supports collation. Collation
defines rules to sort and compare strings in certain
[operations][collate-operations], such as conditional expressions, joins, and
groupings.

By default, SQL sorts strings case-sensitively. This means that `a` and
`A` are treated as different letters, and `Z` would come before `a`.

**Example default sorting:** Apple, Zebra, apple

By contrast, collation lets you sort and compare strings case-insensitively or
according to specific language rules.

**Example case-insensitive collation:** Apple, apple, Zebra

To customize collation for a
collation-supported operation, you typically [assign a collation
specification][collate-define] to at least one string in the operation inputs.
Some operations can't use collation, but can [propagate collation through
them][collate-propagate].

Collation is useful when you need fine-tuned control over how values are sorted,
joined, or grouped in tables.

## Operations affected by collation

The following example query operations are affected by collation when
sorting and comparing strings:

| Operations                                                 |
| ---------------------------------------------------------- |
| Collation-supported [comparison operations][collate-funcs] |
| [Join operations][join-types]                              |
| [`ORDER BY`][order-by-clause]                              |
| [`GROUP BY`][group-by-clause]                              |
| [`WINDOW`][window-clause] for window functions             |
| Collation-supported [scalar functions][collate-funcs]      |
| Collation-supported [aggregate functions][collate-funcs]   |
| [Set operations][set-operators]                            |
| [`NULLIF` conditional expression][nullif]                  |

## Operations that propagate collation

Collation can pass through some query operations to other parts
of a query. When collation passes through an operation in a
query, this is known as _propagation_. During propagation:

- If an input contains no collation specification or an empty
  collation specification and another input contains an explicitly defined
  collation, the explicitly defined collation is used for all of the inputs.
- All inputs with a non-empty explicitly defined collation specification must
  have the same type of collation specification, otherwise an error is thrown.

SQL has several [functions][functions-propagation],
[operators][operators-propagation], and [expressions][expressions-propagation]
that can propagate collation.

In the following example, the `'und:ci'` collation specification is propagated
from the `character` column to the `ORDER BY` operation.

```sql
-- With collation
SELECT *
FROM UNNEST([
  COLLATE('B', 'und:ci'),
  'b',
  'a'
]) AS character
ORDER BY character

/*-----------+
 | character |
 +-----------+
 | a         |
 | B         |
 | b         |
 +-----------*/
```

```sql
-- Without collation
SELECT *
FROM UNNEST([
  'B',
  'b',
  'a'
]) AS character
ORDER BY character

/*-----------+
 | character |
 +-----------+
 | B         |
 | a         |
 | b         |
 +-----------*/
```

### Functions

The following example functions propagate collation.

| Function                                           | Notes                                                                          |
| -------------------------------------------------- | ------------------------------------------------------------------------------ |
| [`AEAD.DECRYPT_STRING`][aead_decrypt_string]       |
| [`ANY_VALUE`][any-value]                           |
| [`ARRAY_AGG`][array-agg]                           | Collation on input arguments are propagated as collation on the array element. |
| [`ARRAY_FIRST`][array-first]                       |
| [`ARRAY_LAST`][array-last]                         |
| [`ARRAY_SLICE`][array-slice]                       |
| [`ARRAY_TO_STRING`][array-to-string]               | Collation on array elements are propagated to output.                          |
| [`COLLATE`][collate]                               |
| [`CONCAT`][concat]                                 |
| [`FORMAT`][format-func]                            | Collation from `format_string` to the returned string is propagated.           |
| [`FORMAT_DATE`][format-date]                       | Collation from `format_string` to the returned string is propagated.           |
| [`FORMAT_DATETIME`][format-datetime]               | Collation from `format_string` to the returned string is propagated.           |
| [`FORMAT_TIME`][format-time]                       | Collation from `format_string` to the returned string is propagated.           |
| [`FORMAT_TIMESTAMP`][format-timestamp]             | Collation from `format_string` to the returned string is propagated.           |
| [`GREATEST`][greatest]                             |
| [`LAG`][lag]                                       |
| [`LEAD`][lead]                                     |
| [`LEAST`][least]                                   |
| [`LEFT`][left]                                     |
| [`LOWER`][lower]                                   |
| [`LPAD`][lpad]                                     |
| [`MAX`][max]                                       |
| [`MIN`][min]                                       |
| [`NET.HOST`][nethost]                              |
| [`NET.MAKE_NET`][netmake-net]                      |
| [`NET.PUBLIC_SUFFIX`][netpublic-suffix]            |
| [`NET.REG_DOMAIN`][netreg-domain]                  |
| [`NTH_VALUE`][nth-value]                           |
| [`NORMALIZE`][normalize]                           |
| [`NORMALIZE_AND_CASEFOLD`][normalize-and-casefold] |
| [`NULLIFERROR`][nulliferror]                       |
| [`REPEAT`][repeat]                                 |
| [`REPLACE`][replace]                               |
| [`REVERSE`][reverse]                               |
| [`RIGHT`][right]                                   |
| [`RPAD`][rpad]                                     |
| [`SOUNDEX`][soundex]                               |
| [`SPLIT`][split]                                   | Collation on input arguments are propagated as collation on the array element. |
| [`STRING_AGG`][string-agg]                         |
| [`SUBSTR`][substr]                                 |
| [`UPPER`][upper]                                   |

### Operators

The following example operators propagate collation.

| Operator                                                | Notes                                                                                             |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| [`\|\|` concatenation operator][concat-op]              |
| [Array subscript operator][array-subscript-operator]    | Propagated to output.                                                                             |
| [Set operators][set-operators]                          | Collation of an output column is decided by the collations of input columns at the same position. |
| [`STRUCT` field access operator][field-access-operator] | When getting a `STRUCT`, collation on the `STRUCT` field is propagated as the output collation.   |
| [`UNNEST`][unnest-operator]                             | Collation on the input array element is propagated to output.                                     |

### Expressions

The following example expressions propagate collation.

| Expression               | Notes                                                                                                     |
| ------------------------ | --------------------------------------------------------------------------------------------------------- |
| [`ARRAY`][array-dt]      | When you construct an `ARRAY`, collation on input arguments is propagated on the elements in the `ARRAY`. |
| [`CASE`][case]           |
| [`CASE` expr][case-expr] |
| [`COALESCE`][coalesce]   |
| [`IF`][if]               |
| [`IFNULL`][ifnull]       |
| [`NULLIF`][nullif]       |
| [`STRUCT`][struct-dt]    | When you construct a `STRUCT`, collation on input arguments is propagated on the fields in the `STRUCT`.  |

## Where you can assign a collation specification

You can assign a [collation specification][collate-spec-details] to these
collation-supported types:

- A `VARCHAR`
- A `VARCHAR` field in a `STRUCT`
- A `VARCHAR` element in an `ARRAY`

In addition:

- You can assign a default collation specification to a schema when you
  create or alter it. This assigns a default collation specification to all
  future tables that are added to the schema if the tables don't have their
  own default collation specifications.
- You can assign a default collation specification to a table when you create
  or alter it. This assigns a collation specification to all future
  collation-supported columns that are added to the table if the columns don't
  have collation specifications. This overrides a
  default collation specification on a schema.
- You can assign a collation specification to a collation-supported type
  in a column. A column that contains a collation-supported type in its
  column schema is a collation-supported column. This overrides a
  default collation specification on a table.
- You can assign a collation specification to a collation-supported
  query operation.
- You can assign a collation specification to a collation-supported expression
  with the `COLLATE` function. This overrides any collation specifications set
  previously.

In summary:

You can define a default collation specification for a schema. For example:

```sql
CREATE SCHEMA (...)
DEFAULT COLLATE 'und:ci'
```

You can define a default collation specification for a table. For example:

```sql
CREATE TABLE (...)
DEFAULT COLLATE 'und:ci'
```

You can define a collation specification for a collation-supported column.
For example:

```sql
CREATE TABLE (
  case_insensitive_column VARCHAR COLLATE 'und:ci'
)
```

You can specify a collation specification for a collation-supported expression
with the `COLLATE` function. For example:

```sql
SELECT COLLATE('a', 'und:ci') AS character
```

In the `ORDER BY` clause, you can specify a collation specification for a
collation-supported column. This overrides any
collation specifications set previously.

For example:

```sql
SELECT Place
FROM Locations
ORDER BY Place COLLATE "und:ci"
```

### DDL statements

You can assign a collation specification to the following DDL statements.

| Location | Support                          | Notes                                        |
| -------- | -------------------------------- | -------------------------------------------- |
| Schema   | [`CREATE SCHEMA`][create-schema] | Create a schema and optionally add a default |

: : : collation specification to the schema. :
| Schema | [`ALTER SCHEMA`][alter-schema] | Updates the default collation specification |
: : : for a schema. :
| Table | [`CREATE TABLE`][create-table] | Create a table and optionally add a default |
: : : collation specification to a table :
: : : or a collation specification to a :
: : : collation-supported type in a column. :
: : : :
: : : You can't have collation on a column used :
: : : with `CLUSTERING`. :
| Table | [`ALTER TABLE`][alter-table] | Update the default collation specification |
: : : for collation-supported type in a table. :
| Column | [`ADD COLUMN`][add-column] | Add a collation specification to a |
: : : collation-supported type in a new column :
: : : in an existing table. :### Data types

You can assign a collation specification to the following data types.

| Type                   | Notes                                               |
| ---------------------- | --------------------------------------------------- |
| [`VARCHAR`][string-dt] | You can apply a collation specification directly to |

: : this data type. :
| [`STRUCT`][struct-dt] | You can apply a collation specification to a |
: : `VARCHAR` field in a `STRUCT`. A `STRUCT` can :
: : have `VARCHAR` fields with different :
: : collation specifications. :
: : A `STRUCT` can only be used in comparisons with the :
: : following operators and conditional expressions&#58;:
: : `=`, `!=`, `IN`, `NULLIF`, and `CASE`. :
| [`ARRAY`][array-dt] | You can apply a collation specification to a |
: : `VARCHAR` element in an `ARRAY`. An `ARRAY` can :
: : have `VARCHAR` elements with different :
: : collation specifications. :

Note: Use the [`COLLATE`][collate] function to apply a collation specification
to collation-supported expressions.

### Query statements

You can assign a collation specification to the following query statements.

| Type    | Support                              |
| ------- | ------------------------------------ |
| Sorting | [`ORDER BY` clause][order-by-clause] |

### Functions, operators, and conditional expressions

You can assign a collation specification to the following functions, operators,
and conditional expressions.#### Functions

| Type   | Support                  | Notes |
| ------ | ------------------------ | ----- |
| Scalar | [`COLLATE`][collate]     |       |
| Scalar | [`ENDS_WITH`][ends-with] |       |
| Scalar | [`GREATEST`][greatest]   |       |

: : : :
| Scalar | [`INSTR`][instr] | |
| Scalar | [`LEAST`][least] | |
| Scalar | [`REPLACE`][replace] | |
: : : :
| Scalar | [`SPLIT`][split] | |
| Scalar | [`STARTS_WITH`][starts-with] | |
| Scalar | [`STRPOS`][strpos] | |
| Aggregate | [`COUNT`][count] | This operator is only affected by |
: : : collation when the input includes :
: : : the `DISTINCT` argument. :
| Aggregate | [`MAX`][max] | |
| Aggregate | [`MIN`][min] | |
: : : :

#### Operators

| Support                              | Notes                           |
| ------------------------------------ | ------------------------------- |
| [`<`][comparison-op]                 |                                 |
| [`<=`][comparison-op]                |                                 |
| [`>`][comparison-op]                 |                                 |
| [`>=`][comparison-op]                |                                 |
| [`=`][comparison-op]                 |                                 |
| [`!=`][comparison-op]                |                                 |
| [`[NOT] BETWEEN`][comparison-op]     |                                 |
| [`[NOT] IN`][in-op]                  | [Limitations apply][in-op].     |
| [`[NOT] LIKE`][like-op]              | [Limitations apply][like-op].   |
| [Quantified `[NOT] LIKE`][q-like-op] | [Limitations apply][q-like-op]. |

#### Conditional expressions

| Support                  |     |
| ------------------------ | --- |
| [`CASE`][case]           |     |
| [`CASE` expr][case-expr] |     |
| [`NULLIF`][nullif]       |     |

The preceding collation-supported operations
(functions, operators, and conditional expressions)
can include input with explicitly defined collation specifications for
collation-supported types. In a collation-supported operation:

- All inputs with a non-empty, explicitly defined collation specification must
  be the same, otherwise an error is thrown.
- If an input doesn't contain an explicitly defined collation
  and another input contains an explicitly defined collation, the
  explicitly defined collation is used for both.

For example:

```sql
-- Assume there's a table with this column declaration:
CREATE TABLE table_a
(
    col_a VARCHAR COLLATE 'und:ci',
    col_b VARCHAR COLLATE '',
    col_c VARCHAR,
    col_d VARCHAR COLLATE 'und:ci'
);

-- This runs. Column 'b' has a collation specification and the
-- column 'c' doesn't.
SELECT STARTS_WITH(col_b_expression, col_c_expression)
FROM table_a;

-- This runs. Column 'a' and 'd' have the same collation specification.
SELECT STARTS_WITH(col_a_expression, col_d_expression)
FROM table_a;

-- This runs. Even though column 'a' and 'b' have different
-- collation specifications, column 'b' is considered the default collation
-- because it's assigned to an empty collation specification.
SELECT STARTS_WITH(col_a_expression, col_b_expression)
FROM table_a;

-- This works. Even though column 'a' and 'b' have different
-- collation specifications, column 'b' is updated to use the same
-- collation specification as column 'a'.
SELECT STARTS_WITH(col_a_expression, COLLATE(col_b_expression, 'und:ci'))
FROM table_a;

-- This runs. Column 'c' doesn't have a collation specification, so it uses the
-- collation specification of column 'd'.
SELECT STARTS_WITH(col_c_expression, col_d_expression)
FROM table_a;
```

## Collation specification details

A collation specification determines how strings are sorted and compared in
[collation-supported operations][collate-operations]. You can define a
collation specification for [collation-supported types][collate-define].
These types of collation specifications are available:

- [Binary collation specification][binary-collation]
- [Unicode collation specification][unicode-collation]

If a collation specification isn't defined, the default collation specification
is used. To learn more, see the next section.

### Default collation specification

When a collation specification isn't assigned or is empty,
`'binary'` collation is used. Binary collation indicates that the
operation should return data in [Unicode code point order][unicode-code-point].
You can't set binary collation explicitly.

In general, the following behavior occurs when an empty string is included in
collation:

- If a string has `und:ci` collation, the string comparison is
  case-insensitive.
- If a string has empty collation, the string comparison is case-sensitive.
- If string not assigned collation, the string comparison is case-sensitive.
- A column with unassigned collation inherit the table's default
  collation.
- A column with empty collation doesn't inherit the table's default collation.

### Binary collation specification

```sql
collation_specification:
  'language_tag'
```

A binary collation specification indicates that the operation should
return data in [Unicode code point order][unicode-code-point]. The
collation specification can be a `VARCHAR` literal or a query parameter.

The language tag determines how strings are generally sorted and compared.
The allowed value for the `language_tag` is `binary`.

This is what the `binary` language tag looks like when used with the `ORDER BY`
clause:

```sql
SELECT Place
FROM Locations
ORDER BY Place COLLATE 'binary'
```

### Unicode collation specification

```sql
collation_specification:
  'language_tag[:collation_attribute]'
```

A unicode collation specification indicates that the operation should use the
[Unicode Collation Algorithm][tr10-collation-algorithm] to sort and compare
strings. The collation specification can be a `VARCHAR` literal or a
query parameter.

#### The language tag

The language tag determines how strings are generally sorted and compared.
Allowed values for `language_tag` are:

- A standard locale string: This name is usually two or three letters
  that represent the language, optionally followed by an underscore or dash and
  two letters that represent the region &mdash; for example, `en_US`. These
  names are defined by the
  [Common Locale Data Repository (CLDR)][unicode-locale-identifier].
- `und`: A locale string representing the _undetermined_ locale. `und` is a
  special language tag defined in the
  [IANA language subtag registry][iana-language-subtag-registry] and used to
  indicate an undetermined locale. This is also known as the _root_ locale and
  can be considered the _default_ Unicode collation. It defines a reasonable,
  locale agnostic collation. It differs significantly from
  `binary`.
- `unicode`: Identical to `binary`. It's recommended to migrate `unicode`
  to `binary`.

Additionally, you can append a language tag with an extension. To learn more,
see [extensions][collate-extensions] for the language tag.

#### The collation attribute

In addition to the language tag, the unicode collation specification can have
an optional `collation_attribute`, which enables additional rules for sorting
and comparing strings. Allowed values are:

- `ci`: Collation is case-insensitive.
- `cs`: Collation is case-sensitive. By default, `collation_attribute` is
  implicitly `cs`.

If you're using the `unicode` language tag with a collation attribute, these
caveats apply:

- `unicode:cs` is identical to `unicode`.
- `unicode:ci` is identical to `und:ci`. It's recommended to migrate
  `unicode:ci` to `binary`.

#### Collation specification example

This is what the `ci` collation attribute looks like when used with the
`und` language tag in the `COLLATE` function:

```sql
COLLATE('orange1', 'und:ci')
```

This is what the `ci` collation attribute looks like when used with the
`und` language tag in the `ORDER BY` clause:

```sql
SELECT Place
FROM Locations
ORDER BY Place COLLATE 'und:ci'
```

#### Extensions

The [Unicode Collation Algorithm][tr10-collation-algorithm] standard
includes some useful locale extensions. In SQL, a `language_tag`
may be extended by appending `-u-[extension]` to it and replacing `[extension]`
with your desired [Unicode local extension][tr35-collation-settings].

This is what the `kn-true` extension looks like when used with the
`en-us` language tag in the `ORDER BY` clause:

For example:

```sql
SELECT *
FROM UNNEST([
  'a12b',
  'a1b'
]) AS ids
ORDER BY ids COLLATE 'en-us-u-kn-true'

/*-------+
 | ids   |
 +-------+
 | a1b   |
 | a12b  |
 +-------*/
```

```sql
SELECT *
FROM UNNEST([
  'a12b',
  'a1b'
]) AS ids
ORDER BY ids COLLATE 'en-us-u-kn-false'

/*-------+
 | ids   |
 +-------+
 | a12b  |
 | a1b   |
 +-------*/
```

Here are some commonly used extensions:

| Extension         | Name                        | Example            |
| ----------------- | --------------------------- | ------------------ |
| ks-level2         | Case-Insensitive            | "a1" < "A2"        |
| ks-level1         | Accent and Case-Insensitive | "ä1" < "a2" < "A3" |
| ks-level1-kc-true | Accent Insensitive          | "ä1" < "a2"        |
| kn-true           | Numeric Ordering            | "a1b" < "a12b"     |

For a complete list and in depth technical details, consult
[Unicode Locale Data Markup Language Part 5: Collation]
[tr35-collation-settings].

#### Caveats

- Differing strings can be considered equal.
  For instance, `ẞ` (LATIN CAPITAL LETTER SHARP S) is considered equal to `'SS'`
  in some contexts. The following expressions both evaluate to `TRUE`:
  - `COLLATE('ẞ', 'und:ci') > COLLATE('SS', 'und:ci')`
  - `COLLATE('ẞ1', 'und:ci') < COLLATE('SS2', 'und:ci')`

  This is similar to how case insensitivity works.

- In search operations, strings with different lengths could be considered
  equal. To ensure consistency, collation should be used without
  search tailoring.
- There are a wide range of unicode code points (punctuation, symbols, etc),
  that are treated as if they aren't there. So strings with
  and without them are sorted identically. For example, the format control
  code point `U+2060` is ignored when the following strings are sorted:

  ```sql
  SELECT *
  FROM UNNEST([
    COLLATE('oran\u2060ge1', 'und:ci'),
    COLLATE('\u2060orange2', 'und:ci'),
    COLLATE('orange3', 'und:ci')
  ]) AS fruit
  ORDER BY fruit
  ```

/_---------+
| fruit |
+---------+
| orange1 |
| orange2 |
| orange3 |
+---------_/

````sql
+ Ordering _may_ change. The Unicode specification of the `und` collation can
change occasionally, which can affect sorting
order. If you need a stable sort order that's
guaranteed to never change, use `unicode` collation.

## Limitations

Limitations for supported features are captured in the previous
sections, but here are a few general limitations to keep in mind:

+ Table functions can't take table arguments with collated columns.

```sql
CREATE TABLE FUNCTION my_dataset.my_tvf(x TABLE<col_str VARCHAR>) AS (
  SELECT col_str FROM x
);

SELECT * FROM my_dataset.my_tvf(
  (SELECT COLLATE('abc', 'und:ci') AS col_str)
);

-- User error:
-- "Collation 'und:ci' on column col_str of argument of TVF call isn't allowed"
````

[unicode-code-point]: https://en.wikipedia.org/wiki/List_of_Unicode_characters
[iana-language-subtag-registry]: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
[unicode-locale-identifier]: https://www.unicode.org/reports/tr35/#Unicode_locale_identifier
[tr35-collation-settings]: http://www.unicode.org/reports/tr35/tr35-collation.html#Setting_Options
[tr10-collation-algorithm]: http://www.unicode.org/reports/tr10/
[collate-operations]: #collate-operations
[collate-define]: #collate-define
[collate-propagate]: #collate-propagate
[collate-spec-details]: #collate-spec_details
[collate-funcs]: #collate-funcs
[collate-query]: #collate-query
[collate-dts]: #collate-data_types
[collate-ddl]: #collate-ddl
[unicode-collation]: #unicode-collation
[binary-collation]: #binary-collation
[functions-propagation]: #functions-propagation
[operators-propagation]: #operators-propagation
[expressions-propagation]: #expressions-propagation
[collate-extensions]: #collation-extensions
[limitations]: #limitations
[order-by-clause]: ../syntax/query_syntax.md#order-by_clause
[collate-clause]: ../syntax/query_syntax.md#collate-clause
[create-schema]: #create-schema_statement
[create-table]: #create-table_statement
[alter-schema]: #alter-schema_collate_statement
[alter-table]: #alter-table_collate_statement
[alter-column]: #alter-column_set_data_type_statement
[add-column]: #alter-table_add_column_statement
[string-dt]: data_types.md#string-type
[struct-dt]: data_types.md#struct-type
[array-dt]: data_types.md#array-type
[join-types]: ../syntax/query_syntax.md#join-types
[group-by-clause]: ../syntax/query_syntax.md#group-by_clause
[window-clause]: ../syntax/query_syntax.md#window-clause
[set-operators]: ../syntax/query_syntax.md#set-operators
[unnest-operator]: ../syntax/query_syntax.md#unnest-operator

[aead_decrypt_string]#aeaddecrypt-string

[any-value]: ../functions/aggregate_functions.md#any-value
[array-agg]: ../functions/aggregate_functions.md#array-agg
[array-to-string]: ../functions/array_functions.md#array-to_string
[array-slice]: ../functions/array_functions.md#array-slice
[array-first]: ../functions/array_functions.md#array-first
[array-last]: ../functions/array_functions.md#array-last
[cast]: ../functions/conversion_functions.md
[collate]: ../functions/string_functions.md#collate
[concat]: ../functions/string_functions.md#concat
[count]: ../functions/aggregate_functions.md#count
[ends-with]: ../functions/string_functions.md#ends-with
[format-func]: ../functions/string_functions.md#format-string
[format-date]: ../functions/date_functions.md#format-date
[format-datetime]: ../functions/datetime_functions.md#format-datetime
[format-time]: ../functions/time_functions.md#format-time
[format-timestamp]: ../functions/timestamp_functions.md#format-timestamp
[greatest]: ../functions/mathematical_functions.md#greatest
[initcap]: ../functions/string_functions.md#initcap
[instr]: ../functions/string_functions.md#instr
[json-extract]: ../functions/json_functions.md#json-extract
[json-extract-array]: ../functions/json_functions.md#json-extract_array
[json-extract-scalar]: ../functions/json_functions.md#json-extract_scalar
[json-extract-string-array]: ../functions/json_functions.md#json-extract_string_array
[json-query]: ../functions/json_functions.md#json-query
[json-query-array]: ../functions/json_functions.md#json-query_array
[json-value]: ../functions/json_functions.md#json-value
[json-value-array]: ../functions/json_functions.md#json-value_array
[lag]: ../functions/navigation_functions.md#lag
[lead]: ../functions/navigation_functions.md#lead
[least]: ../functions/mathematical_functions.md#least
[left]: ../functions/string_functions.md#left
[lower]: ../functions/string_functions.md#lower
[lpad]: ../functions/string_functions.md#lpad
[max]: ../functions/aggregate_functions.md#max
[min]: ../functions/aggregate_functions.md#min
[nth-value]: ../functions/navigation_functions.md#nth-value
[normalize]: ../functions/string_functions.md#normalize
[normalize-and-casefold]: ../functions/string_functions.md#normalize-and_casefold
[repeat]: ../functions/string_functions.md#repeat
[replace]: ../functions/string_functions.md#replace
[reverse]: ../functions/string_functions.md#reverse
[right]: ../functions/string_functions.md#right
[rpad]: ../functions/string_functions.md#rpad
[soundex]: ../functions/string_functions.md#soundex
[split]: ../functions/string_functions.md#split
[starts-with]: ../functions/string_functions.md#starts-with
[string-func]: ../functions/conversion_functions.md#other-conv_functions
[string-agg]: ../functions/aggregate_functions.md#string-agg
[strpos]: ../functions/string_functions.md#strpos
[substr]: ../functions/string_functions.md#substr
[upper]: ../functions/string_functions.md#upper
[comparison-op]: ../syntax/operators.md#comparison-operators
[in-op]: ../syntax/operators.md#in-operators
[like-op]: ../syntax/operators.md#like-operator
[q-like-op]: ../syntax/operators.md#like-operator_quantified
[concat-op]: ../syntax/operators.md#concatenation-operator
[field-access-operator]: ../syntax/operators.md#field-access_operator
[array-subscript-operator]: ../syntax/operators.md#array-subscript_operator
[case]: ../functions/conditional_expressions.md#case
[case-expr]: ../functions/conditional_expressions.md#case-expr
[coalesce]: ../functions/conditional_expressions.md#coalesce
[if]: ../functions/conditional_expressions.md#if
[ifnull]: ../functions/conditional_expressions.md#ifnull
[nullif]: ../functions/conditional_expressions.md#nullif
