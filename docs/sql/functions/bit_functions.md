# Bit functions

SQL supports the following bit functions.

## Function list

| Name                                                          | Summary                                                                                                                     |
| ------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| [`BIT_AND`](aggregate_functions.md#bit-and)                   | Performs a bitwise AND operation on an expression. For more information, see [Aggregate functions](aggregate_functions.md). |
| [`BIT_CAST_TO_INTEGER`](bit_functions.md#bit-cast-to-int32)   | Cast bits to an `INTEGER` value.                                                                                            |
| [`BIT_CAST_TO_BIGINT`](bit_functions.md#bit-cast-to-int64)    | Cast bits to an `BIGINT` value.                                                                                             |
| [`BIT_CAST_TO_UINTEGER`](bit_functions.md#bit-cast-to-uint32) | Cast bits to an `UINTEGER` value.                                                                                           |
| [`BIT_CAST_TO_UBIGINT`](bit_functions.md#bit-cast-to-uint64)  | Cast bits to an `UBIGINT` value.                                                                                            |
| [`BIT_COUNT`](bit_functions.md#bit-count)                     | Gets the number of bits that are set in an input expression.                                                                |
| [`BIT_OR`](aggregate_functions.md#bit-or)                     | Performs a bitwise OR operation on an expression. For more information, see [Aggregate functions](aggregate_functions.md).  |
| [`BIT_XOR`](aggregate_functions.md#bit-xor)                   | Performs a bitwise XOR operation on an expression. For more information, see [Aggregate functions](aggregate_functions.md). |

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

## `BIT_COUNT`

```sql
BIT_COUNT(expression)
```

**Description**

The input, `expression`, must be an
integer or `VARBINARY`.

Returns the number of bits that are set in the input `expression`.
For signed integers, this is the number of bits in two's complement form.

**Return Data Type**

`BIGINT`

**Example**

```sql
SELECT a, BIT_COUNT(a) AS a_bits, FORMAT("%T", b) as b, BIT_COUNT(b) AS b_bits
FROM UNNEST([
  STRUCT(0 AS a, b'' AS b), (0, b'\x00'), (5, b'\x05'), (8, b'\x00\x08'),
  (0xFFFF, b'\xFF\xFF'), (-2, b'\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE'),
  (-1, b'\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF'),
  (NULL, b'\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF')
]) AS x;

/*-------+--------+---------------------------------------------+--------+
 | a     | a_bits | b                                           | b_bits |
 +-------+--------+---------------------------------------------+--------+
 | 0     | 0      | b""                                         | 0      |
 | 0     | 0      | b"\x00"                                     | 0      |
 | 5     | 2      | b"\x05"                                     | 2      |
 | 8     | 1      | b"\x00\x08"                                 | 1      |
 | 65535 | 16     | b"\xff\xff"                                 | 16     |
 | -2    | 63     | b"\xff\xff\xff\xff\xff\xff\xff\xfe"         | 63     |
 | -1    | 64     | b"\xff\xff\xff\xff\xff\xff\xff\xff"         | 64     |
 | NULL  | NULL   | b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff" | 80     |
 +-------+--------+---------------------------------------------+--------*/
```
