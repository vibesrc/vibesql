# Subscript and Field Access Operators

Operators for accessing array elements, struct fields, and JSON data.

### Field access operator

```sql
expression.fieldname[. ...]
```

**Description**

Gets the value of a field. Alternatively known as the dot operator. Can be
used to access nested fields. For example, `expression.fieldname1.fieldname2`.

Input values:

- `STRUCT`
-
- `JSON`
- `GRAPH_ELEMENT`

Note: If the field to access is within a `STRUCT`, you can use the
[struct subscript operator][struct-subscript-operator] to access the field by
its position within the `STRUCT` instead of by its name. Accessing by
a field by position is useful when fields are un-named or have ambiguous names.

**Return type**

- For `STRUCT`: SQL data type of `fieldname`. If a field isn't found in
  the struct, an error is thrown.
- For : SQL data type of `fieldname`. If a field isn't found in
  the , an error is thrown.
- For `JSON`: `JSON`. If a field isn't found in a JSON value, a SQL `NULL` is
  returned.
- For `GRAPH_ELEMENT`: SQL data type of `fieldname`. If a field (property)
  isn't found in the graph element, an error is returned.

**Example**

In the following example, the field access operations are `.address` and
`.country`.

```sql
SELECT
  STRUCT(
    STRUCT('Yonge Street' AS street, 'Canada' AS country)
      AS address).address.country

/*---------+
 | country |
 +---------+
 | Canada  |
 +---------*/
```

[struct-subscript-operator]: #struct-subscript_operator

### Array subscript operator

Note: Syntax characters enclosed in double quotes (`""`) are literal and
required.

```sql
array_expression "[" array_subscript_specifier "]"

array_subscript_specifier:
  { index | position_keyword(index) }

position_keyword:
  { OFFSET | OFFSET | ORDINAL | ORDINAL }
```

**Description**

Gets a value from an array at a specific position.

Input values:

- `array_expression`: The input array.
- `position_keyword(index)`: Determines where the index for the array should
  start and how out-of-range indexes are handled. The index is an integer that
  represents a specific position in the array.
  - `OFFSET(index)`: The index starts at zero. Produces an error if the index is
    out of range. To produce `NULL` instead of an error, use
    `OFFSET(index)`. This
    position keyword produces the same result as `index` by itself.
  - `OFFSET(index)`: The index starts at
    zero. Returns `NULL` if the index is out of range.
  - `ORDINAL(index)`: The index starts at one.
    Produces an error if the index is out of range.
    To produce `NULL` instead of an error, use `ORDINAL(index)`.
  - `ORDINAL(index)`: The index starts at
    one. Returns `NULL` if the index is out of range.
- `index`: An integer that represents a specific position in the array. If used
  by itself without a position keyword, the index starts at zero and produces
  an error if the index is out of range. To produce `NULL` instead of an error,
  use the `OFFSET(index)` or `ORDINAL(index)` position keyword.

**Return type**

`T` where `array_expression` is `ARRAY<T>`.

**Examples**

In following query, the array subscript operator is used to return values at
specific position in `item_array`. This query also shows what happens when you
reference an index (`6`) in an array that's out of range. If the `SAFE` prefix
is included, `NULL` is returned, otherwise an error is produced.

```sql
SELECT
  ["coffee", "tea", "milk"] AS item_array,
  ["coffee", "tea", "milk"][0] AS item_index,
  ["coffee", "tea", "milk"][OFFSET(0)] AS item_offset,
  ["coffee", "tea", "milk"][ORDINAL(1)] AS item_ordinal,
  ["coffee", "tea", "milk"][OFFSET(6)] AS item_safe_offset

/*---------------------+------------+-------------+--------------+------------------+
 | item_array          | item_index | item_offset | item_ordinal | item_safe_offset |
 +---------------------+------------+-------------+--------------+------------------+
 | [coffee, tea, milk] | coffee     | coffee      | coffee       | NULL             |
 +----------------------------------+-------------+--------------+------------------*/
```

When you reference an index that's out of range in an array, and a positional
keyword that begins with `SAFE` isn't included, an error is produced.
For example:

```sql
-- Error. Array index 6 is out of bounds.
SELECT ["coffee", "tea", "milk"][6] AS item_offset
```

```sql
-- Error. Array index 6 is out of bounds.
SELECT ["coffee", "tea", "milk"][OFFSET(6)] AS item_offset
```

### Struct subscript operator

Note: Syntax characters enclosed in double quotes (`""`) are literal and
required.

```sql
struct_expression "[" struct_subscript_specifier "]"

struct_subscript_specifier:
  { index | position_keyword(index) }

position_keyword:
  { OFFSET | ORDINAL }
```

**Description**

Gets the value of a field at a selected position in a struct.

**Input types**

- `struct_expression`: The input struct.
- `position_keyword(index)`: Determines where the index for the struct should
  start and how out-of-range indexes are handled. The index is an
  integer literal or constant that represents a specific position in the struct.
  - `OFFSET(index)`: The index starts at zero. Produces an error if the index is
    out of range. Produces the same
    result as `index` by itself.
  - `ORDINAL(index)`: The index starts at one. Produces an error if the index
    is out of range.
- `index`: An integer literal or constant that represents a specific position in
  the struct. If used by itself without a position keyword, the index starts at
  zero and produces an error if the index is out of range.

Note: The struct subscript operator doesn't support `SAFE` positional keywords
at this time.

**Examples**

In following query, the struct subscript operator is used to return values at
specific locations in `item_struct` using position keywords. This query also
shows what happens when you reference an index (`6`) in an struct that's out of
range.

```sql
SELECT
  STRUCT(23, "tea", FALSE)[0] AS field_index,
  STRUCT(23, "tea", FALSE)[OFFSET(0)] AS field_offset,
  STRUCT(23, "tea", FALSE)[ORDINAL(1)] AS field_ordinal

/*-------------+--------------+---------------+
 | field_index | field_offset | field_ordinal |
 +-------------+--------------+---------------+
 | 23          | 23           | 23            |
 +-------------+--------------+---------------*/
```

When you reference an index that's out of range in a struct, an error is
produced. For example:

```sql
-- Error: Field ordinal 6 is out of bounds in STRUCT
SELECT STRUCT(23, "tea", FALSE)[6] AS field_offset
```

```sql
-- Error: Field ordinal 6 is out of bounds in STRUCT
SELECT STRUCT(23, "tea", FALSE)[OFFSET(6)] AS field_offset
```

### JSON subscript operator

Note: Syntax characters enclosed in double quotes (`""`) are literal and
required.

```sql
json_expression "[" array_element_id "]"
```

```sql
json_expression "[" field_name "]"
```

**Description**

Gets a value of an array element or field in a JSON expression. Can be
used to access nested data.

Input values:

- `JSON expression`: The `JSON` expression that contains an array element or
  field to return.
- `[array_element_id]`: An `BIGINT` expression that represents a zero-based index
  in the array. If a negative value is entered, or the value is greater than
  or equal to the size of the array, or the JSON expression doesn't represent
  a JSON array, a SQL `NULL` is returned.
- `[field_name]`: A `VARCHAR` expression that represents the name of a field in
  JSON. If the field name isn't found, or the JSON expression isn't a
  JSON object, a SQL `NULL` is returned.

**Return type**

`JSON`

**Example**

In the following example:

- `json_value` is a JSON expression.
- `.class` is a JSON field access.
- `.students` is a JSON field access.
- `[0]` is a JSON subscript expression with an element offset that
  accesses the zeroth element of an array in the JSON value.
- `['name']` is a JSON subscript expression with a field name that
  accesses a field.

```sql
SELECT json_value.class.students[0]['name'] AS first_student
FROM
  UNNEST(
    [
      JSON '{"class" : {"students" : [{"name" : "Jane"}]}}',
      JSON '{"class" : {"students" : []}}',
      JSON '{"class" : {"students" : [{"name" : "John"}, {"name": "Jamie"}]}}'])
    AS json_value;

/*-----------------+
 | first_student   |
 +-----------------+
 | "Jane"          |
 | NULL            |
 | "John"          |
 +-----------------*/
```

### map subscript operator

```sql
proto_map_field_expression[proto_subscript_specifier]

proto_subscript_specifier:
  key_name | key_keyword(key_name)

key_keyword:
  { KEY | SAFE_KEY }
```

**Description**

Returns the value in a [ map][proto-map] for a
given key.

Input values:

- `proto_map_field_expression`: A map field.
- `key_keyword(key_name)`: Determines whether to produce `NULL` or
  an error if the key isn't present in the map field.
  - `KEY(key_name)`: Returns an error if the key isn't present in the
    map field.
  - `SAFE_KEY(key_name)`: Returns `NULL` if the key isn't present in the
    map field.
  - `key_name`: When `key_name` is provided without a wrapping keyword,
    it's the same as `KEY(key_name)`.
- `key_name`: The key in the map field. This operator returns
  `NULL` if the key is `NULL`.

**Return type**

In the input map field, `V` as represented in `map<K,V>`.

**Examples**

To illustrate the use of this function, we use the message
`Item`.

```sqlproto
message Item {
  optional map purchased = 1;
};
```

In the following example, the subscript operator returns the value when the key
is present.

```sql
SELECT
  m.purchased[KEY('A')] AS map_value
FROM
  (SELECT AS VALUE CAST("purchased { key: 'A' value: 2 }" AS Item)) AS m;

/*-----------+
 | map_value |
 +-----------+
 | 2         |
 +-----------*/
```

When the key doesn't exist in the map field and you use `KEY`, an error is
produced. For example:

```sql
-- ERROR: Key not found in map: 2
SELECT
  m.purchased[KEY('B')] AS value
FROM
  (SELECT AS VALUE CAST("purchased { key: 'A' value: 2 }" AS Item)) AS m;
```

When the key doesn't exist in the map field and you use `SAFE_KEY`,
the subscript operator returns `NULL`. For example:

```sql
SELECT
  CAST(m.purchased[SAFE_KEY('B')] AS safe_key_missing
FROM
  (SELECT AS VALUE CAST("purchased { key: 'A' value: 2 }" AS Item)) AS m;

/*------------------+
 | safe_key_missing |
 +------------------+
 | NULL             |
 +------------------*/
```

The subscript operator returns `NULL` when the map field or key is `NULL`.
For example:

```sql
SELECT
  CAST(NULL AS Item).purchased[KEY('A')] AS null_map,
  m.purchased[KEY(NULL)] AS null_key
FROM
  (SELECT AS VALUE CAST("purchased { key: 'A' value: 2 }" AS Item)) AS m;

/*-----------------------+
 | null_map  | null_key  |
 +-----------------------+
 | NULL      | NULL      |
 +-----------------------*/
```

When a key is used without `KEY()` or `SAFE_KEY()`, it has the same behavior
as if `KEY()` had been used. For example:

```sql
SELECT
  m.purchased['A'] AS map_value
FROM
  (SELECT AS VALUE CAST("purchased { key: 'A' value: 2 }" AS Item)) AS m;

/*-----------+
 | map_value |
 +-----------+
 | 2         |
 +-----------*/
```

[proto-map]: https://developers.google.com/protocol-buffers/docs/proto3#maps

### Array elements field access operator

Note: Syntax characters enclosed in double quotes (`""`) are literal and
required.

```sql
array_expression.field_or_element[. ...]

field_or_element:
  { fieldname | array_element }

array_element:
  array_fieldname "[" array_subscript_specifier "]"
```

**Description**

The array elements field access operation lets you traverse through the
levels of a nested data type inside an array.

Input values:

- `array_expression`: An expression that evaluates to an array value.
- `field_or_element[. ...]`: The field to access. This can also be a position
  in an array-typed field.
- `fieldname`: The name of the field to access.

  For example, this query returns all values for the `items` field inside of the
  `my_array` array expression:

  ```sql
  WITH MyTable AS ( SELECT [STRUCT(['foo', 'bar'] AS items)] AS my_array )
  SELECT FLATTEN(my_array.items)
  FROM MyTable
  ```

  These data types have fields:

- `STRUCT`
-
- `JSON`

- `array_element`: If the field to access is an array field (`array_field`),
  you can additionally access a specific position in the field
  with the [array subscript operator][array-subscript-operator]
  (`[array_subscript_specifier]`). This operation returns only elements at a
  selected position, rather than all elements, in the array field.

  For example, this query only returns values at position 0 in the `items`
  array field:

  ```sql
  WITH MyTable AS ( SELECT [STRUCT(['foo', 'bar'] AS items)] AS my_array )
  SELECT FLATTEN(my_array.items[OFFSET(0)])
  FROM MyTable
  ```

Details:

The array elements field access operation isn't a typical expression
that returns a typed value; it represents a concept outside the type system
and can only be interpreted by the following operations:

- [`FLATTEN` operation][flatten-operation]: Returns an array. For example:

  ````sql
  FLATTEN(my_array.sales.prices)
  ```sql
  ````

- [`UNNEST` operation][operators-link-to-unnest]: Returns a table.
  `array_expression` must be a path expression.
  Implicitly implements the `FLATTEN` operator.
  For example, these do the same thing:

  ```sql
  UNNEST(my_array.sales.prices)
  ```

  ````sql
  UNNEST(FLATTEN(my_array.sales.prices))
  ```sql
  ````

- [`FROM` clause][operators-link-to-from-clause]: Returns a table.
  `array_expression` must be a path expression.
  Implicitly implements the `UNNEST` operator and the `FLATTEN` operator.
  For example, these unnesting operations produce the same values for
  `results`:

  ```sql
  SELECT results FROM SalesTable, SalesTable.my_array.sales.prices AS results;
  ```

  ```sql
  SELECT results FROM SalesTable, UNNEST(my_array.sales.prices) AS results;
  ```

  ```sql
  SELECT results FROM SalesTable, UNNEST(FLATTEN(my_array.sales.prices)) AS results;
  ```

If `NULL` array elements are encountered, they are added to the resulting array.

**Common shapes of this operation**

This operation can take several shapes. The right-most value in
the operation determines what type of array is returned. Here are some example
shapes and a description of what they return:

The following shapes extract the final non-array field from each element of
an array expression and return an array of those non-array field values.

- `array_expression.non_array_field_1`
- `array_expression.non_array_field_1.array_field.non_array_field_2`

The following shapes extract the final array field from each element of the
array expression and concatenate the array fields together.
An empty array or a `NULL` array contributes no elements to the resulting array.

- `array_expression.non_array_field_1.array_field_1`
- `array_expression.non_array_field_1.array_field_1.non_array_field_2.array_field_2`
- `array_expression.non_array_field_1.non_array_field_2.array_field_1`

The following shapes extract the final array field from each element of the
array expression at a specific position. Then they return an array of those
extracted elements. An empty array or a `NULL` array contributes no elements
to the resulting array.

- `array_expression.non_array_field_1.array_field_1[OFFSET(1)]`
- `array_expression.non_array_field_1.array_field_1[OFFSET(1)]`
- `array_expression.non_array_field_1.non_array_field_2.array_field_1[ORDINAL(2)]`
- `array_expression.non_array_field_1.non_array_field_2.array_field_1[ORDINAL(2)]`

**Return Value**

- `FLATTEN` of an array element access operation returns an array.
- `UNNEST` of an array element access operation, whether explicit or implicit,
  returns a table.

**Examples**

The next examples in this section reference a table called `SalesTable`, that
contains a nested struct in an array called `my_array`:

```sql
WITH
  SalesTable AS (
    SELECT
      [
        STRUCT(
          [
            STRUCT([25.0, 75.0] AS prices),
            STRUCT([30.0] AS prices)
          ] AS sales
        )
      ] AS my_array
  )
SELECT * FROM SalesTable;

/*----------------------------------------------+
 | my_array                                     |
 +----------------------------------------------+
 | [{[{[25, 75] prices}, {[30] prices}] sales}] |
 +----------------------------------------------*/
```

This is what the array elements field access operator looks like in the
`FLATTEN` operator:

```sql
SELECT FLATTEN(my_array.sales.prices) AS all_prices FROM SalesTable;

/*--------------+
 | all_prices   |
 +--------------+
 | [25, 75, 30] |
 +--------------*/
```

This is how you use the array subscript operator to only return values at a
specific index in the `prices` array:

```sql
SELECT FLATTEN(my_array.sales.prices[OFFSET(0)]) AS first_prices FROM SalesTable;

/*--------------+
 | first_prices |
 +--------------+
 | [25, 30]     |
 +--------------*/
```

This is an example of an explicit `UNNEST` operation that includes the
array elements field access operator:

```sql
SELECT all_prices FROM SalesTable, UNNEST(my_array.sales.prices) AS all_prices

/*------------+
 | all_prices |
 +------------+
 | 25         |
 | 75         |
 | 30         |
 +------------*/
```

This is an example of an implicit `UNNEST` operation that includes the
array elements field access operator:

```sql
SELECT all_prices FROM SalesTable, SalesTable.my_array.sales.prices AS all_prices

/*------------+
 | all_prices |
 +------------+
 | 25         |
 | 75         |
 | 30         |
 +------------*/
```

This query produces an error because one of the `prices` arrays doesn't have
an element at index `1` and `OFFSET` is used:

```sql
SELECT FLATTEN(my_array.sales.prices[OFFSET(1)]) AS second_prices FROM SalesTable;

-- Error
```

This query is like the previous query, but `OFFSET` is used. This
produces a `NULL` value instead of an error.

```sql
SELECT FLATTEN(my_array.sales.prices[OFFSET(1)]) AS second_prices FROM SalesTable;

/*---------------+
 | second_prices |
 +---------------+
 | [75, NULL]    |
 +---------------*/
```

In this next example, an empty array and a `NULL` field value have been added to
the query. These contribute no elements to the result.

```sql
WITH
  SalesTable AS (
    SELECT
      [
        STRUCT(
          [
            STRUCT([25.0, 75.0] AS prices),
            STRUCT([30.0] AS prices),
            STRUCT(ARRAY[] AS prices),
            STRUCT(NULL AS prices)
          ] AS sales
        )
      ] AS my_array
  )
SELECT FLATTEN(my_array.sales.prices) AS first_prices FROM SalesTable;

/*--------------+
 | first_prices |
 +--------------+
 | [25, 75, 30] |
 +--------------*/
```

The next examples in this section reference a called
`Album` that looks like this:

```sqlproto
message Album {
  optional string album_name = 1;
  repeated string song = 2;
  oneof group_name {
    string solo = 3;
    string duet = 4;
    string band = 5;
  }
}
```

Nested data is common in that have data within repeated
messages. The following example extracts a flattened array of songs from a
table called `AlbumList` that contains a column called `Album` of type .

```sql
WITH
  AlbumList AS (
    SELECT
      [
        NEW Album(
          'One Way' AS album_name,
          ['North', 'South'] AS song,
          'Crossroads' AS band),
        NEW Album(
          'After Hours' AS album_name,
          ['Snow', 'Ice', 'Water'] AS song,
          'Sunbirds' AS band)]
        AS albums_array
  )
SELECT FLATTEN(albums_array.song) AS songs FROM AlbumList

/*------------------------------+
 | songs                        |
 +------------------------------+
 | [North,South,Snow,Ice,Water] |
 +------------------------------*/
```

The following example extracts a flattened array of album names, one album name
per row. The data comes from a table called `AlbumList` that contains a
proto-typed column called `Album`.

```sql
WITH
  AlbumList AS (
    SELECT
      [
        (
          SELECT
            NEW Album(
              'One Way' AS album_name,
              ['North', 'South'] AS song,
              'Crossroads' AS band) AS album_col
        ),
        (
          SELECT
            NEW Album(
              'After Hours' AS album_name,
              ['Snow', 'Ice', 'Water'] AS song,
              'Sunbirds' AS band) AS album_col
        )]
        AS albums_array
  )
SELECT names FROM AlbumList, UNNEST(albums_array.album_name) AS names

/*----------------------+
 | names                |
 +----------------------+
 | One Way              |
 | After Hours          |
 +----------------------*/
```

[array-subscript-operator]: #array-subscript_operator
[flatten-operation]: ../functions/array_functions.md#flatten
[operators-link-to-unnest]: query_syntax.md#unnest-operator
[operators-link-to-from-clause]: query_syntax.md#from-clause
