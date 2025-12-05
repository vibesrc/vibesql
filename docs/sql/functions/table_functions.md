# Table-valued functions

SQL supports table-valued functions (TVFs).
A TVF returns an entire output table instead of
a single scalar value, and appears in the `FROM` clause like a table subquery.

## Create a TVF

You can create a TVF using the following syntax:

```sql
CREATE
  [ OR REPLACE ]
  { TEMPORARY | TEMP } TABLE FUNCTION
  [ IF NOT EXISTS ]
  function_name ( [ function_parameter [ DEFAULT value_for_argument ] [, ...] ] )
  [ RETURNS TABLE  ]
  [ { AS query | LANGUAGE language_name AS string_literal } ]

function_parameter:
  parameter_name { data_type | ANY TYPE | ANY TABLE }

column_declaration:
  column_name data_type
```

- `CREATE ... TABLE FUNCTION`: Creates a new
  [table-valued function][table-valued-function] function.
  A function can have zero or more function parameters.
  - `TEMPORARY` or `TEMP`: Indicates that the function is temporary, meaning
    that it exists for the lifetime of the session.

- `OR REPLACE`: Replaces any function with the same name if it exists. Can't
  appear with `IF NOT EXISTS`.
- `IF NOT EXISTS`: If any function exists with the same name, the `CREATE`
  statement has no effect. Can't appear with `OR REPLACE`.
- `function_parameter`: A parameter for the function.
  - `parameter_name`: The name of the parameter.

  - `data_type`: A SQL [data type][data-types].

  - `ANY TYPE`: The function will accept an argument of any type for this
    function parameter. If more than one parameter includes `ANY TYPE`,
    a relationship isn't enforced between these parameters when the function
    is defined. However, if the type of argument passed into the function at
    call time is incompatible with the function definition, this will
    result in an error.

    `ANY TYPE` is a [_templated function parameter_][templated-parameters].

  - `ANY TABLE`. The function will accept an argument of any relation type for
    this argument. However, passing the function arguments of types that are
    incompatible with the function definition will result in an error at
    call time.

    `ANY TABLE` is a [_templated function parameter_][templated-parameters].

- `DEFAULT value_for_argument`: A default argument for a function parameter.
  The `value_for_argument` has to be coercible to the function parameter's
  data type. If any function parameter has a default value, all later
  function parameters must also have default values.
- `RETURNS TABLE`: Specifies the schema of the table that a table-valued
  function returns as a comma-separated list of `column_name` and `TYPE`
  pairs. If `RETURNS TABLE` is absent, SQL infers the
  output schema from the `AS query` statement in the function body.
- `AS query`: If you want to create a SQL TVF, specifies the SQL query to run.
- `LANGUAGE ... AS`: If you want to create an external TVF, specifies the
  language and code to use.
  `language_name` represents the name of the language, such
  as `js` for JavaScript. `string_literal` represents the code that defines
  the function body.

## Specify TVF arguments

When a TVF with function parameters is called, arguments must be passed in for
all function parameters that don't have default values. An argument can be of
any supported SQL type or table, but must be coercible to the
related function parameter's type.

For non-table arguments, you can optionally include default arguments with the
`DEFAULT value_for_argument` clause. The `value_for_argument` expression runs
when `CREATE FUNCTION` is analyzed, and has to be coercible to the argument type
of the argument. If any argument has a default value, all later arguments must
also have default values. Calls to the function may optionally omit values for
arguments that have default values.

Specify a table argument the same way you specify the fields of a
[STRUCT][data-types-struct].

```sql
parameter_name TABLE
```

The table argument can specify a [value table][datamodel-value-tables],
in which each row
is a single column of a specific type. To specify a value table as an argument,
include only the `data_type`, leaving out the `column_name`:

```sql
parameter_name TABLE
```

In many cases, the `data_type` of the single column in the value table is a
; for example:

```sql
CREATE TEMP TABLE FUNCTION AggregatedMovieLogs(
  TicketPurchases TABLE)
```

The function body can refer directly to fields within the proto.

You have the option to specify the input table using the templated type `ANY
TABLE` in place of `TABLE<column_name data_type [, ...]>`. This option enables
you to create a polymorphic TVF that accepts any table as input.

**Example**

The following example implements a pair of TVFs that define parameterized views
of a range of rows from the Customer table. The first returns all rows for a
range of `CustomerIds`; the second calls the first function and applies an
additional filter based on `CustomerType`.

```sql
CREATE TEMP TABLE FUNCTION CustomerRange(MinID BIGINT, MaxID BIGINT)
AS (
  SELECT *
  FROM Customer
  WHERE CustomerId >= MinId AND CustomerId <= MaxId
);

CREATE TEMP TABLE FUNCTION CustomerRangeWithCustomerType(
  MinId BIGINT,
  MaxId BIGINT,
  customer_type
    ads.boulder.schema.CustomerType
      DEFAULT 'CUSTOMER_TYPE_ADVERTISER')
AS (
  SELECT *
  FROM CustomerRange(MinId, MaxId)
  WHERE type = customer_type
);
```

The following function returns all rows from the input table if the first
argument is greater than the second argument; otherwise it returns no rows.

```sql
CREATE TEMP TABLE FUNCTION MyFunction(
     first_value ANY TYPE,
     second_value ANY TYPE,
     MyInputTable ANY TABLE)
   AS
     SELECT *
     FROM MyInputTable
     WHERE first_value > second_value;
```

The following function accepts two integers and a table with any set of columns
and returns rows from the table where the predicate evaluates to true. The input
table `SelectedCustomers` must contain a column named `creation_time`, and
`creation_time` must be a numeric type, or the function will return an error.

```sql
CREATE TEMP TABLE FUNCTION CustomerCreationTimeRange(
    min_creation_time BIGINT,
    max_creation_time BIGINT,
    SelectedCustomers ANY TABLE)
  AS
    SELECT *
    FROM SelectedCustomers
    WHERE creation_time >= min_creation_time
    AND creation_time <= max_creation_time;
```

## Call a TVF

To call a TVF, see [Table function calls][table-function-calls].

## Templated function parameters

A templated function parameter can match more than one argument type at
function call time. If a function signature includes a
templated function parameter, SQL allows function calls
to pass to the function any argument type as long as the function body is
valid for that argument type.

[table-valued-function]: #tvfs
[tvf-syntax]: #tvf-structure
[table-function-calls]: ../syntax/query_syntax.md#table-function_calls
[templated-parameters]: #templated-function_parameters
[data-types]: ../types/data_types.md
[data-types-struct]: ../types/data_types.md#struct-type
[datamodel-value-tables]: ../types/data_model.md#value-tables
