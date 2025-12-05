# Functions

Function reference and calling conventions.

## Overview

| File                                                     | Description                    |
| -------------------------------------------------------- | ------------------------------ |
| [functions_reference.md](functions_reference.md)         | Function call syntax and rules |
| [conditional_expressions.md](conditional_expressions.md) | CASE, IF, COALESCE, NULLIF     |

## Aggregate Functions

| File                                                                     | Description                   |
| ------------------------------------------------------------------------ | ----------------------------- |
| [aggregate_function_calls.md](aggregate_function_calls.md)               | Aggregate calling conventions |
| [aggregate_functions.md](aggregate_functions.md)                         | COUNT, SUM, AVG, MIN, MAX     |
| [approximate_aggregate_functions.md](approximate_aggregate_functions.md) | APPROX_COUNT_DISTINCT         |
| [statistical_aggregate_functions.md](statistical_aggregate_functions.md) | STDDEV, VARIANCE, CORR        |

## Window Functions

| File                                                 | Description                         |
| ---------------------------------------------------- | ----------------------------------- |
| [window_function_calls.md](window_function_calls.md) | OVER clause, PARTITION BY, frames   |
| [navigation_functions.md](navigation_functions.md)   | LAG, LEAD, FIRST_VALUE, LAST_VALUE  |
| [numbering_functions.md](numbering_functions.md)     | ROW_NUMBER, RANK, DENSE_RANK, NTILE |

## String Functions

| File                                             | Description                                     |
| ------------------------------------------------ | ----------------------------------------------- |
| [string_functions.md](string_functions.md)       | String function overview                        |
| [string_manipulation.md](string_manipulation.md) | CONCAT, SUBSTR, REPLACE, TRIM, SPLIT            |
| [string_search.md](string_search.md)             | INSTR, STRPOS, STARTS_WITH, ENDS_WITH           |
| [string_regexp.md](string_regexp.md)             | REGEXP_CONTAINS, REGEXP_EXTRACT, REGEXP_REPLACE |
| [string_format.md](string_format.md)             | FORMAT, BASE64, HEX encoding                    |
| [string_case.md](string_case.md)                 | UPPER, LOWER, COLLATE, NORMALIZE                |

## JSON Functions

| File                                   | Description                              |
| -------------------------------------- | ---------------------------------------- |
| [json_functions.md](json_functions.md) | JSON function overview                   |
| [json_core.md](json_core.md)           | JSON_ARRAY, JSON_OBJECT, JSON_TYPE       |
| [json_extract.md](json_extract.md)     | JSON_QUERY, JSON_VALUE, JSON_EXTRACT     |
| [json_modify.md](json_modify.md)       | JSON_SET, JSON_REMOVE, JSON_ARRAY_APPEND |
| [json_convert.md](json_convert.md)     | TO_JSON, PARSE_JSON, type converters     |

## Other Scalar Functions

| File                                                   | Description                        |
| ------------------------------------------------------ | ---------------------------------- |
| [mathematical_functions.md](mathematical_functions.md) | ABS, ROUND, FLOOR, CEIL, SQRT, POW |
| [date_functions.md](date_functions.md)                 | DATE, DATE_ADD, DATE_DIFF          |
| [time_functions.md](time_functions.md)                 | TIME, TIME_ADD, TIME_DIFF          |
| [datetime_functions.md](datetime_functions.md)         | DATETIME operations                |
| [timestamp_functions.md](timestamp_functions.md)       | TIMESTAMP operations               |
| [interval_functions.md](interval_functions.md)         | MAKE_INTERVAL, EXTRACT             |
| [array_functions.md](array_functions.md)               | ARRAY_LENGTH, ARRAY_TO_STRING      |
| [bit_functions.md](bit_functions.md)                   | BIT_COUNT, BIT_AND, BIT_OR         |
| [conversion_functions.md](conversion_functions.md)     | CAST, parse/format functions       |
| [range_functions.md](range_functions.md)               | RANGE operations                   |

## Time Series & Table Functions

| File                                                     | Description               |
| -------------------------------------------------------- | ------------------------- |
| [time_series_functions.md](time_series_functions.md)     | GAP_FILL, time buckets    |
| [table_functions.md](table_functions.md)                 | Table-valued functions    |
| [user_defined_functions.md](user_defined_functions.md)   | CREATE FUNCTION           |
| [user_defined_aggregates.md](user_defined_aggregates.md) | CREATE AGGREGATE FUNCTION |
