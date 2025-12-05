# Bitwise and Logical Operators

Operators for bitwise operations and boolean logic.

### Bitwise operators

All bitwise operators return the same type
and the same length as
the first operand.

| Name        | Syntax     | Input Data Type                                   | Description                                                                                                                                                                                                                                    |
| ----------- | ---------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bitwise not | `~ X`      | Integer or `VARBINARY`                            | Performs logical negation on each bit, forming the ones' complement of the given binary value.                                                                                                                                                 |
| Bitwise or  | `X \| Y`   | `X`: Integer or `VARBINARY` `Y`: Same type as `X` | Takes two bit patterns of equal length and performs the logical inclusive `OR` operation on each pair of the corresponding bits. This operator throws an error if `X` and `Y` are bytes of different lengths.                                 |
| Bitwise xor | `X ^ Y`    | `X`: Integer or `VARBINARY` `Y`: Same type as `X` | Takes two bit patterns of equal length and performs the logical exclusive `OR` operation on each pair of the corresponding bits. This operator throws an error if `X` and `Y` are bytes of different lengths.                                 |
| Bitwise and | `X & Y`    | `X`: Integer or `VARBINARY` `Y`: Same type as `X` | Takes two bit patterns of equal length and performs the logical `AND` operation on each pair of the corresponding bits. This operator throws an error if `X` and `Y` are bytes of different lengths.                                          |
| Left shift  | `X << Y`   | `X`: Integer or `VARBINARY` `Y`: `BIGINT`         | Shifts the first operand `X` to the left. This operator returns `0` or a byte sequence of `b'\x00'` if `Y` is greater than or equal to the bit length of `X` (e.g., `64` if `X` is `BIGINT`). Throws an error if `Y` is negative.              |
| Right shift | `X >> Y`   | `X`: Integer or `VARBINARY` `Y`: `BIGINT`         | Shifts the first operand `X` to the right. Doesn't perform sign bit extension (fills vacant bits on the left with `0`). Returns `0` or `b'\x00'` if `Y` >= bit length of `X` (e.g., `64` if `X` is `BIGINT`). Throws an error if `Y` is negative. |

### Logical operators

SQL supports the `AND`, `OR`, and `NOT` logical operators.
Logical operators allow only `BOOL` or `NULL` input
and use [three-valued logic][three-valued-logic]
to produce a result. The result can be `TRUE`, `FALSE`, or `NULL`:

| `x`     | `y`     | `x AND y` | `x OR y` |
| ------- | ------- | --------- | -------- |
| `TRUE`  | `TRUE`  | `TRUE`    | `TRUE`   |
| `TRUE`  | `FALSE` | `FALSE`   | `TRUE`   |
| `TRUE`  | `NULL`  | `NULL`    | `TRUE`   |
| `FALSE` | `TRUE`  | `FALSE`   | `TRUE`   |
| `FALSE` | `FALSE` | `FALSE`   | `FALSE`  |
| `FALSE` | `NULL`  | `FALSE`   | `NULL`   |
| `NULL`  | `TRUE`  | `NULL`    | `TRUE`   |
| `NULL`  | `FALSE` | `FALSE`   | `NULL`   |
| `NULL`  | `NULL`  | `NULL`    | `NULL`   |

| `x`     | `NOT x` |
| ------- | ------- |
| `TRUE`  | `FALSE` |
| `FALSE` | `TRUE`  |
| `NULL`  | `NULL`  |

The order of evaluation of operands to `AND` and `OR` can vary, and evaluation
can be skipped if unnecessary.

**Examples**

The examples in this section reference a table called `entry_table`:

```sql
/*-------+
 | entry |
 +-------+
 | a     |
 | b     |
 | c     |
 | NULL  |
 +-------*/
```

```sql
SELECT 'a' FROM entry_table WHERE entry = 'a'

-- a => 'a' = 'a' => TRUE
-- b => 'b' = 'a' => FALSE
-- NULL => NULL = 'a' => NULL

/*-------+
 | entry |
 +-------+
 | a     |
 +-------*/
```

```sql
SELECT entry FROM entry_table WHERE NOT (entry = 'a')

-- a => NOT('a' = 'a') => NOT(TRUE) => FALSE
-- b => NOT('b' = 'a') => NOT(FALSE) => TRUE
-- NULL => NOT(NULL = 'a') => NOT(NULL) => NULL

/*-------+
 | entry |
 +-------+
 | b     |
 | c     |
 +-------*/
```

```sql
SELECT entry FROM entry_table WHERE entry IS NULL

-- a => 'a' IS NULL => FALSE
-- b => 'b' IS NULL => FALSE
-- NULL => NULL IS NULL => TRUE

/*-------+
 | entry |
 +-------+
 | NULL  |
 +-------*/
```

[three-valued-logic]: https://en.wikipedia.org/wiki/Three-valued_logic
