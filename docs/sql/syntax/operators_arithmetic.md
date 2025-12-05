# Arithmetic Operators

Operators for numeric, date, time, and interval arithmetic.

### Arithmetic operators

All arithmetic operators accept input of numeric type `T`, and the result type
has type `T` unless otherwise indicated in the description below:

| Name           | Syntax  |
| -------------- | ------- |
| Addition       | `X + Y` |
| Subtraction    | `X - Y` |
| Multiplication | `X * Y` |
| Division       | `X / Y` |
| Unary Plus     | `+ X`   |
| Unary Minus    | `- X`   |

NOTE: Divide by zero operations return an error. To return a different result,
consider the `IEEE_DIVIDE` or `SAFE_DIVIDE` functions.

Result types for Addition and Multiplication:

| INPUT      | `INTEGER` | `BIGINT`  | `UINTEGER` | `UBIGINT` | `NUMERIC` | `NUMERIC` | `FLOAT`  | `DOUBLE` |
| ---------- | --------- | --------- | ---------- | --------- | --------- | --------- | -------- | -------- |
| `INTEGER`  | `BIGINT`  | `BIGINT`  | `BIGINT`   | ERROR     | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `BIGINT`   | `BIGINT`  | `BIGINT`  | `BIGINT`   | ERROR     | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `UINTEGER` | `BIGINT`  | `BIGINT`  | `UBIGINT`  | `UBIGINT` | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `UBIGINT`  | ERROR     | ERROR     | `UBIGINT`  | `UBIGINT` | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `FLOAT`    | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`  | `DOUBLE` | `DOUBLE` |
| `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`  | `DOUBLE` | `DOUBLE` |

Result types for Subtraction:

| INPUT      | `INTEGER` | `BIGINT`  | `UINTEGER` | `UBIGINT` | `NUMERIC` | `NUMERIC` | `FLOAT`  | `DOUBLE` |
| ---------- | --------- | --------- | ---------- | --------- | --------- | --------- | -------- | -------- |
| `INTEGER`  | `BIGINT`  | `BIGINT`  | `BIGINT`   | ERROR     | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `BIGINT`   | `BIGINT`  | `BIGINT`  | `BIGINT`   | ERROR     | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `UINTEGER` | `BIGINT`  | `BIGINT`  | `BIGINT`   | `BIGINT`  | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `UBIGINT`  | ERROR     | ERROR     | `BIGINT`   | `BIGINT`  | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `FLOAT`    | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`  | `DOUBLE` | `DOUBLE` |
| `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`  | `DOUBLE` | `DOUBLE` |

Result types for Division:

| INPUT      | `INTEGER` | `BIGINT`  | `UINTEGER` | `UBIGINT` | `NUMERIC` | `NUMERIC` | `FLOAT`  | `DOUBLE` |
| ---------- | --------- | --------- | ---------- | --------- | --------- | --------- | -------- | -------- |
| `INTEGER`  | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `BIGINT`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `UINTEGER` | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `UBIGINT`  | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC`  | `NUMERIC` | `NUMERIC` | `NUMERIC` | `DOUBLE` | `DOUBLE` |
| `FLOAT`    | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`  | `DOUBLE` | `DOUBLE` |
| `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`   | `DOUBLE`  | `DOUBLE`  | `DOUBLE`  | `DOUBLE` | `DOUBLE` |

Result types for Unary Plus:

| INPUT  | `INTEGER` | `BIGINT` | `UINTEGER` | `UBIGINT` | `NUMERIC` | `NUMERIC` | `FLOAT` | `DOUBLE` |
| ------ | --------- | -------- | ---------- | --------- | --------- | --------- | ------- | -------- |
| OUTPUT | `INTEGER` | `BIGINT` | `UINTEGER` | `UBIGINT` | `NUMERIC` | `NUMERIC` | `FLOAT` | `DOUBLE` |

Result types for Unary Minus:

| INPUT  | `INTEGER` | `BIGINT` | `UINTEGER` | `UBIGINT` | `NUMERIC` | `NUMERIC` | `FLOAT` | `DOUBLE` |
| ------ | --------- | -------- | ---------- | --------- | --------- | --------- | ------- | -------- |
| OUTPUT | `INTEGER` | `BIGINT` | ERROR      | ERROR     | `NUMERIC` | `NUMERIC` | `FLOAT` | `DOUBLE` |

### Date arithmetics operators

Operators '+' and '-' can be used for arithmetic operations on dates.

```sql
date_expression + int64_expression
int64_expression + date_expression
date_expression - int64_expression
```

**Description**

Adds or subtracts `int64_expression` days to or from `date_expression`. This is
equivalent to `DATE_ADD` or `DATE_SUB` functions, when interval is expressed in
days.

**Return Data Type**

`DATE`

**Example**

```sql
SELECT DATE "2020-09-22" + 1 AS day_later, DATE "2020-09-22" - 7 AS week_ago

/*------------+------------+
 | day_later  | week_ago   |
 +------------+------------+
 | 2020-09-23 | 2020-09-15 |
 +------------+------------*/
```

### Datetime subtraction

```sql
date_expression - date_expression
timestamp_expression - timestamp_expression
datetime_expression - datetime_expression
```

**Description**

Computes the difference between two datetime values as an interval.

**Return Data Type**

`INTERVAL`

**Example**

```sql
SELECT
  DATE "2021-05-20" - DATE "2020-04-19" AS date_diff,
  TIMESTAMP "2021-06-01 12:34:56.789" - TIMESTAMP "2021-05-31 00:00:00" AS time_diff

/*-------------------+------------------------+
 | date_diff         | time_diff              |
 +-------------------+------------------------+
 | 0-0 396 0:0:0     | 0-0 0 36:34:56.789     |
 +-------------------+------------------------*/
```

### Interval arithmetic operators

**Addition and subtraction**

```sql
date_expression + interval_expression = DATETIME
date_expression - interval_expression = DATETIME
timestamp_expression + interval_expression = TIMESTAMP
timestamp_expression - interval_expression = TIMESTAMP
datetime_expression + interval_expression = DATETIME
datetime_expression - interval_expression = DATETIME

```

**Description**

Adds an interval to a datetime value or subtracts an interval from a datetime
value.

**Example**

```sql
SELECT
  DATE "2021-04-20" + INTERVAL 25 HOUR AS date_plus,
  TIMESTAMP "2021-05-02 00:01:02.345+00" - INTERVAL 10 SECOND AS time_minus;

/*-------------------------+--------------------------------+
 | date_plus               | time_minus                     |
 +-------------------------+--------------------------------+
 | 2021-04-21 01:00:00     | 2021-05-02 00:00:52.345+00     |
 +-------------------------+--------------------------------*/
```

**Multiplication and division**

```sql
interval_expression * integer_expression = INTERVAL
interval_expression / integer_expression = INTERVAL

```

**Description**

Multiplies or divides an interval value by an integer.

**Example**

```sql
SELECT
  INTERVAL '1:2:3' HOUR TO SECOND * 10 AS mul1,
  INTERVAL 35 SECOND * 4 AS mul2,
  INTERVAL 10 YEAR / 3 AS div1,
  INTERVAL 1 MONTH / 12 AS div2

/*----------------+--------------+-------------+--------------+
 | mul1           | mul2         | div1        | div2         |
 +----------------+--------------+-------------+--------------+
 | 0-0 0 10:20:30 | 0-0 0 0:2:20 | 3-4 0 0:0:0 | 0-0 2 12:0:0 |
 +----------------+--------------+-------------+--------------*/
```
