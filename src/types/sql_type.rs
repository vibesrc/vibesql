//! SQL type system for semantic analysis.
//!
//! This module defines the runtime type system used during query analysis
//! and execution planning. Type names follow ISO SQL standards.

use std::fmt;

/// A SQL data type used during semantic analysis.
///
/// Type names follow ISO SQL standards:
/// - `Int32` displays as "INTEGER"
/// - `Int64` displays as "BIGINT"
/// - `Float32` displays as "REAL"
/// - `Float64` displays as "DOUBLE PRECISION"
/// - `Varchar` displays as "VARCHAR"
/// - `Varbinary` displays as "VARBINARY"
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SqlType {
    /// Boolean type (BOOLEAN)
    Bool,

    /// 32-bit signed integer (INTEGER)
    Int32,

    /// 64-bit signed integer (BIGINT)
    Int64,

    /// 32-bit unsigned integer (UINTEGER) - extension
    Uint32,

    /// 64-bit unsigned integer (UBIGINT) - extension
    Uint64,

    /// 32-bit floating point (REAL)
    Float32,

    /// 64-bit floating point (DOUBLE PRECISION)
    Float64,

    /// Fixed-precision decimal (NUMERIC/DECIMAL)
    Numeric {
        precision: Option<u8>,
        scale: Option<u8>,
    },

    /// Variable-length character string (VARCHAR)
    Varchar,

    /// Variable-length binary data (VARBINARY)
    Varbinary,

    /// Date (year, month, day)
    Date,

    /// Time of day
    Time,

    /// Date and time without timezone (TIMESTAMP WITHOUT TIME ZONE)
    Datetime,

    /// Date and time with timezone (TIMESTAMP WITH TIME ZONE)
    Timestamp,

    /// Time interval
    Interval,

    /// Array of elements
    Array(Box<SqlType>),

    /// Struct with named fields (ROW type in standard SQL)
    Struct(Vec<StructField>),

    /// JSON data
    Json,

    /// Range of values
    Range(Box<SqlType>),

    /// Universally unique identifier (UUID)
    Uuid,

    /// Unknown type (for unresolved expressions)
    Unknown,

    /// Any type (for polymorphic functions)
    Any,
}

/// A field in a struct type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructField {
    pub name: Option<String>,
    pub data_type: SqlType,
}

impl SqlType {
    /// Check if this type is numeric.
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            SqlType::Int32
                | SqlType::Int64
                | SqlType::Uint32
                | SqlType::Uint64
                | SqlType::Float32
                | SqlType::Float64
                | SqlType::Numeric { .. }
        )
    }

    /// Check if this type is an integer type (signed or unsigned).
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            SqlType::Int32 | SqlType::Int64 | SqlType::Uint32 | SqlType::Uint64
        )
    }

    /// Check if this type is a signed integer type.
    pub fn is_signed_integer(&self) -> bool {
        matches!(self, SqlType::Int32 | SqlType::Int64)
    }

    /// Check if this type is an unsigned integer type.
    pub fn is_unsigned_integer(&self) -> bool {
        matches!(self, SqlType::Uint32 | SqlType::Uint64)
    }

    /// Check if this type is a floating-point type.
    pub fn is_floating_point(&self) -> bool {
        matches!(self, SqlType::Float32 | SqlType::Float64)
    }

    /// Check if this type is a string type.
    pub fn is_string(&self) -> bool {
        matches!(self, SqlType::Varchar)
    }

    /// Check if this type is a date/time type.
    pub fn is_datetime(&self) -> bool {
        matches!(
            self,
            SqlType::Date | SqlType::Time | SqlType::Datetime | SqlType::Timestamp
        )
    }

    /// Check if this type is comparable with another type.
    pub fn is_comparable_with(&self, other: &SqlType) -> bool {
        match (self, other) {
            // Same types are always comparable
            (a, b) if a == b => true,

            // Numeric types are comparable with each other
            (a, b) if a.is_numeric() && b.is_numeric() => true,

            // Date/time types are comparable with each other (with caveats)
            (a, b) if a.is_datetime() && b.is_datetime() => true,

            // Unknown/Any can be compared with anything
            (SqlType::Unknown, _) | (_, SqlType::Unknown) => true,
            (SqlType::Any, _) | (_, SqlType::Any) => true,

            _ => false,
        }
    }

    /// Check if this type can be implicitly coerced to another type.
    pub fn can_coerce_to(&self, target: &SqlType) -> bool {
        match (self, target) {
            // Same types
            (a, b) if a == b => true,

            // Integer widening: smaller -> larger
            (SqlType::Int32, SqlType::Int64) => true,
            (SqlType::Uint32, SqlType::Uint64) => true,
            (SqlType::Uint32, SqlType::Int64) => true,

            // Integers can coerce to floating point
            (SqlType::Int32, SqlType::Float32) => true,
            (SqlType::Int32, SqlType::Float64) => true,
            (SqlType::Int64, SqlType::Float64) => true,
            (SqlType::Uint32, SqlType::Float32) => true,
            (SqlType::Uint32, SqlType::Float64) => true,
            (SqlType::Uint64, SqlType::Float64) => true,

            // Float32 can coerce to Float64
            (SqlType::Float32, SqlType::Float64) => true,

            // Integers can coerce to Numeric
            (SqlType::Int32, SqlType::Numeric { .. }) => true,
            (SqlType::Int64, SqlType::Numeric { .. }) => true,
            (SqlType::Uint32, SqlType::Numeric { .. }) => true,
            (SqlType::Uint64, SqlType::Numeric { .. }) => true,

            // Float can coerce to Numeric
            (SqlType::Float32, SqlType::Numeric { .. }) => true,
            (SqlType::Float64, SqlType::Numeric { .. }) => true,

            // Date can coerce to Datetime/Timestamp
            (SqlType::Date, SqlType::Datetime) => true,
            (SqlType::Date, SqlType::Timestamp) => true,

            // Datetime can coerce to Timestamp
            (SqlType::Datetime, SqlType::Timestamp) => true,

            // Unknown can coerce to anything
            (SqlType::Unknown, _) => true,

            // Anything can coerce to Any
            (_, SqlType::Any) => true,

            // Array coercion if element types can coerce
            (SqlType::Array(a), SqlType::Array(b)) => a.can_coerce_to(b),

            _ => false,
        }
    }

    /// Get the common supertype of two types.
    pub fn common_supertype(&self, other: &SqlType) -> Option<SqlType> {
        match (self, other) {
            // Same types
            (a, b) if a == b => Some(a.clone()),

            // Integer promotions
            (SqlType::Int32, SqlType::Int64) | (SqlType::Int64, SqlType::Int32) => {
                Some(SqlType::Int64)
            }
            (SqlType::Uint32, SqlType::Uint64) | (SqlType::Uint64, SqlType::Uint32) => {
                Some(SqlType::Uint64)
            }
            (SqlType::Int32, SqlType::Uint32) | (SqlType::Uint32, SqlType::Int32) => {
                Some(SqlType::Int64) // Promote to signed 64-bit to avoid overflow
            }
            (SqlType::Int32, SqlType::Uint64) | (SqlType::Uint64, SqlType::Int32) => {
                Some(SqlType::Float64) // No safe integer type, use float
            }
            (SqlType::Int64, SqlType::Uint64) | (SqlType::Uint64, SqlType::Int64) => {
                Some(SqlType::Float64) // No safe integer type, use float
            }
            (SqlType::Uint32, SqlType::Int64) | (SqlType::Int64, SqlType::Uint32) => {
                Some(SqlType::Int64)
            }

            // Float promotions
            (SqlType::Float32, SqlType::Float64) | (SqlType::Float64, SqlType::Float32) => {
                Some(SqlType::Float64)
            }

            // Integer to float promotions
            (SqlType::Int32, SqlType::Float32) | (SqlType::Float32, SqlType::Int32) => {
                Some(SqlType::Float32)
            }
            (SqlType::Int32, SqlType::Float64) | (SqlType::Float64, SqlType::Int32) => {
                Some(SqlType::Float64)
            }
            (SqlType::Int64, SqlType::Float32) | (SqlType::Float32, SqlType::Int64) => {
                Some(SqlType::Float64)
            }
            (SqlType::Int64, SqlType::Float64) | (SqlType::Float64, SqlType::Int64) => {
                Some(SqlType::Float64)
            }
            (SqlType::Uint32, SqlType::Float32) | (SqlType::Float32, SqlType::Uint32) => {
                Some(SqlType::Float32)
            }
            (SqlType::Uint32, SqlType::Float64) | (SqlType::Float64, SqlType::Uint32) => {
                Some(SqlType::Float64)
            }
            (SqlType::Uint64, SqlType::Float32) | (SqlType::Float32, SqlType::Uint64) => {
                Some(SqlType::Float64)
            }
            (SqlType::Uint64, SqlType::Float64) | (SqlType::Float64, SqlType::Uint64) => {
                Some(SqlType::Float64)
            }

            // Numeric supertypes (any integer or float with Numeric)
            (t, SqlType::Numeric { .. }) | (SqlType::Numeric { .. }, t)
                if t.is_integer() || t.is_floating_point() =>
            {
                Some(SqlType::Numeric {
                    precision: None,
                    scale: None,
                })
            }

            // Date/time types
            (SqlType::Date, SqlType::Datetime) | (SqlType::Datetime, SqlType::Date) => {
                Some(SqlType::Datetime)
            }
            (SqlType::Date, SqlType::Timestamp) | (SqlType::Timestamp, SqlType::Date) => {
                Some(SqlType::Timestamp)
            }
            (SqlType::Datetime, SqlType::Timestamp) | (SqlType::Timestamp, SqlType::Datetime) => {
                Some(SqlType::Timestamp)
            }

            // Unknown resolves to the other type
            (SqlType::Unknown, other) | (other, SqlType::Unknown) => Some(other.clone()),

            // Any stays Any
            (SqlType::Any, _) | (_, SqlType::Any) => Some(SqlType::Any),

            // Arrays - find common element type
            (SqlType::Array(a), SqlType::Array(b)) => {
                a.common_supertype(b).map(|t| SqlType::Array(Box::new(t)))
            }

            _ => None,
        }
    }

    /// Get the element type if this is an array.
    pub fn element_type(&self) -> Option<&SqlType> {
        match self {
            SqlType::Array(elem) => Some(elem),
            _ => None,
        }
    }

    /// Get struct fields if this is a struct.
    pub fn struct_fields(&self) -> Option<&[StructField]> {
        match self {
            SqlType::Struct(fields) => Some(fields),
            _ => None,
        }
    }
}

impl fmt::Display for SqlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqlType::Bool => write!(f, "BOOLEAN"),
            SqlType::Int32 => write!(f, "INTEGER"),
            SqlType::Int64 => write!(f, "BIGINT"),
            SqlType::Uint32 => write!(f, "UINTEGER"),
            SqlType::Uint64 => write!(f, "UBIGINT"),
            SqlType::Float32 => write!(f, "REAL"),
            SqlType::Float64 => write!(f, "DOUBLE PRECISION"),
            SqlType::Numeric { precision, scale } => {
                write!(f, "NUMERIC")?;
                if let Some(p) = precision {
                    write!(f, "({}", p)?;
                    if let Some(s) = scale {
                        write!(f, ", {}", s)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            SqlType::Varchar => write!(f, "VARCHAR"),
            SqlType::Varbinary => write!(f, "VARBINARY"),
            SqlType::Date => write!(f, "DATE"),
            SqlType::Time => write!(f, "TIME"),
            SqlType::Datetime => write!(f, "DATETIME"),
            SqlType::Timestamp => write!(f, "TIMESTAMP"),
            SqlType::Interval => write!(f, "INTERVAL"),
            SqlType::Array(elem) => write!(f, "ARRAY<{}>", elem),
            SqlType::Struct(fields) => {
                write!(f, "STRUCT<")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if let Some(name) = &field.name {
                        write!(f, "{} ", name)?;
                    }
                    write!(f, "{}", field.data_type)?;
                }
                write!(f, ">")
            }
            SqlType::Json => write!(f, "JSON"),
            SqlType::Range(elem) => write!(f, "RANGE<{}>", elem),
            SqlType::Uuid => write!(f, "UUID"),
            SqlType::Unknown => write!(f, "UNKNOWN"),
            SqlType::Any => write!(f, "ANY"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_classification() {
        assert!(SqlType::Int32.is_numeric());
        assert!(SqlType::Int64.is_numeric());
        assert!(SqlType::Uint32.is_numeric());
        assert!(SqlType::Uint64.is_numeric());
        assert!(SqlType::Float32.is_numeric());
        assert!(SqlType::Float64.is_numeric());
        assert!(SqlType::Numeric {
            precision: None,
            scale: None
        }
        .is_numeric());
        assert!(!SqlType::Varchar.is_numeric());
    }

    #[test]
    fn test_coercion() {
        // Integer widening
        assert!(SqlType::Int32.can_coerce_to(&SqlType::Int64));
        assert!(SqlType::Uint32.can_coerce_to(&SqlType::Uint64));
        assert!(SqlType::Uint32.can_coerce_to(&SqlType::Int64));

        // Integer to float
        assert!(SqlType::Int32.can_coerce_to(&SqlType::Float32));
        assert!(SqlType::Int32.can_coerce_to(&SqlType::Float64));
        assert!(SqlType::Int64.can_coerce_to(&SqlType::Float64));

        // Float widening
        assert!(SqlType::Float32.can_coerce_to(&SqlType::Float64));

        // To Numeric
        assert!(SqlType::Int64.can_coerce_to(&SqlType::Numeric {
            precision: None,
            scale: None
        }));
        assert!(SqlType::Float32.can_coerce_to(&SqlType::Numeric {
            precision: None,
            scale: None
        }));

        // No coercion
        assert!(!SqlType::Varchar.can_coerce_to(&SqlType::Int64));
        assert!(!SqlType::Int64.can_coerce_to(&SqlType::Int32)); // No narrowing
    }

    #[test]
    fn test_common_supertype() {
        // Integer promotions
        assert_eq!(
            SqlType::Int32.common_supertype(&SqlType::Int64),
            Some(SqlType::Int64)
        );
        assert_eq!(
            SqlType::Uint32.common_supertype(&SqlType::Uint64),
            Some(SqlType::Uint64)
        );
        assert_eq!(
            SqlType::Int32.common_supertype(&SqlType::Uint32),
            Some(SqlType::Int64) // Promote to signed 64-bit
        );

        // Float promotions
        assert_eq!(
            SqlType::Float32.common_supertype(&SqlType::Float64),
            Some(SqlType::Float64)
        );

        // Integer to float
        assert_eq!(
            SqlType::Int64.common_supertype(&SqlType::Float64),
            Some(SqlType::Float64)
        );
        assert_eq!(
            SqlType::Int32.common_supertype(&SqlType::Float32),
            Some(SqlType::Float32)
        );

        // Date/time
        assert_eq!(
            SqlType::Date.common_supertype(&SqlType::Timestamp),
            Some(SqlType::Timestamp)
        );

        // No common supertype
        assert_eq!(SqlType::Varchar.common_supertype(&SqlType::Int64), None);
    }

    #[test]
    fn test_is_integer() {
        assert!(SqlType::Int32.is_integer());
        assert!(SqlType::Int64.is_integer());
        assert!(SqlType::Uint32.is_integer());
        assert!(SqlType::Uint64.is_integer());
        assert!(!SqlType::Float32.is_integer());
        assert!(!SqlType::Float64.is_integer());
        assert!(!SqlType::Numeric {
            precision: None,
            scale: None
        }
        .is_integer());
        assert!(!SqlType::Varchar.is_integer());
    }

    #[test]
    fn test_is_signed_unsigned_integer() {
        // Signed
        assert!(SqlType::Int32.is_signed_integer());
        assert!(SqlType::Int64.is_signed_integer());
        assert!(!SqlType::Uint32.is_signed_integer());
        assert!(!SqlType::Uint64.is_signed_integer());

        // Unsigned
        assert!(SqlType::Uint32.is_unsigned_integer());
        assert!(SqlType::Uint64.is_unsigned_integer());
        assert!(!SqlType::Int32.is_unsigned_integer());
        assert!(!SqlType::Int64.is_unsigned_integer());
    }

    #[test]
    fn test_is_floating_point() {
        assert!(SqlType::Float32.is_floating_point());
        assert!(SqlType::Float64.is_floating_point());
        assert!(!SqlType::Int32.is_floating_point());
        assert!(!SqlType::Int64.is_floating_point());
        assert!(!SqlType::Numeric {
            precision: None,
            scale: None
        }
        .is_floating_point());
        assert!(!SqlType::Varchar.is_floating_point());
    }

    #[test]
    fn test_is_string() {
        assert!(SqlType::Varchar.is_string());
        assert!(!SqlType::Int64.is_string());
        assert!(!SqlType::Varbinary.is_string());
        assert!(!SqlType::Json.is_string());
    }

    #[test]
    fn test_is_datetime() {
        assert!(SqlType::Date.is_datetime());
        assert!(SqlType::Time.is_datetime());
        assert!(SqlType::Datetime.is_datetime());
        assert!(SqlType::Timestamp.is_datetime());
        assert!(!SqlType::Int64.is_datetime());
        assert!(!SqlType::Varchar.is_datetime());
        assert!(!SqlType::Interval.is_datetime());
    }

    #[test]
    fn test_is_comparable_with() {
        // Same types
        assert!(SqlType::Int64.is_comparable_with(&SqlType::Int64));
        assert!(SqlType::Varchar.is_comparable_with(&SqlType::Varchar));

        // Numeric types are comparable
        assert!(SqlType::Int64.is_comparable_with(&SqlType::Float64));
        assert!(SqlType::Float64.is_comparable_with(&SqlType::Numeric {
            precision: None,
            scale: None
        }));

        // Date/time types are comparable
        assert!(SqlType::Date.is_comparable_with(&SqlType::Timestamp));
        assert!(SqlType::Time.is_comparable_with(&SqlType::Datetime));

        // Unknown/Any are comparable with anything
        assert!(SqlType::Unknown.is_comparable_with(&SqlType::Int64));
        assert!(SqlType::Int64.is_comparable_with(&SqlType::Unknown));
        assert!(SqlType::Any.is_comparable_with(&SqlType::Varchar));

        // Incompatible types
        assert!(!SqlType::Varchar.is_comparable_with(&SqlType::Int64));
        assert!(!SqlType::Bool.is_comparable_with(&SqlType::Float64));
    }

    #[test]
    fn test_element_type() {
        let array_int = SqlType::Array(Box::new(SqlType::Int64));
        assert_eq!(array_int.element_type(), Some(&SqlType::Int64));

        let nested = SqlType::Array(Box::new(SqlType::Array(Box::new(SqlType::Varchar))));
        assert_eq!(
            nested.element_type(),
            Some(&SqlType::Array(Box::new(SqlType::Varchar)))
        );

        assert_eq!(SqlType::Int64.element_type(), None);
        assert_eq!(SqlType::Varchar.element_type(), None);
    }

    #[test]
    fn test_struct_fields() {
        let struct_type = SqlType::Struct(vec![
            StructField {
                name: Some("a".to_string()),
                data_type: SqlType::Int64,
            },
            StructField {
                name: Some("b".to_string()),
                data_type: SqlType::Varchar,
            },
        ]);
        let fields = struct_type.struct_fields().unwrap();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, Some("a".to_string()));
        assert_eq!(fields[0].data_type, SqlType::Int64);

        assert_eq!(SqlType::Int64.struct_fields(), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SqlType::Bool), "BOOLEAN");
        assert_eq!(format!("{}", SqlType::Int32), "INTEGER");
        assert_eq!(format!("{}", SqlType::Int64), "BIGINT");
        assert_eq!(format!("{}", SqlType::Uint32), "UINTEGER");
        assert_eq!(format!("{}", SqlType::Uint64), "UBIGINT");
        assert_eq!(format!("{}", SqlType::Float32), "REAL");
        assert_eq!(format!("{}", SqlType::Float64), "DOUBLE PRECISION");
        assert_eq!(
            format!(
                "{}",
                SqlType::Numeric {
                    precision: None,
                    scale: None
                }
            ),
            "NUMERIC"
        );
        assert_eq!(
            format!(
                "{}",
                SqlType::Numeric {
                    precision: Some(10),
                    scale: None
                }
            ),
            "NUMERIC(10)"
        );
        assert_eq!(
            format!(
                "{}",
                SqlType::Numeric {
                    precision: Some(10),
                    scale: Some(2)
                }
            ),
            "NUMERIC(10, 2)"
        );
        assert_eq!(format!("{}", SqlType::Varchar), "VARCHAR");
        assert_eq!(format!("{}", SqlType::Varbinary), "VARBINARY");
        assert_eq!(
            format!("{}", SqlType::Array(Box::new(SqlType::Int64))),
            "ARRAY<BIGINT>"
        );
        assert_eq!(format!("{}", SqlType::Uuid), "UUID");
        assert_eq!(format!("{}", SqlType::Unknown), "UNKNOWN");
        assert_eq!(format!("{}", SqlType::Any), "ANY");
    }
}
