//! Data type specifications for the SQL AST.
//!
//! This module defines how data types are represented in the AST,
//! separate from the runtime type system. Type names follow ISO SQL standards.

use super::Ident;
use crate::error::Span;

/// Data type specification as it appears in SQL.
#[derive(Debug, Clone, PartialEq)]
pub struct DataTypeSpec {
    pub kind: DataTypeKind,
    pub span: Span,
}

impl DataTypeSpec {
    pub fn new(kind: DataTypeKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// The kind of data type.
///
/// These are the canonical types used in the AST. The parser handles
/// all standard SQL type aliases (INTEGER, BIGINT, VARCHAR, etc.) and
/// normalizes them to these canonical forms.
#[derive(Debug, Clone, PartialEq)]
pub enum DataTypeKind {
    /// Boolean type (BOOLEAN, BOOL)
    Bool,

    /// 32-bit signed integer (INTEGER, INT, INT32)
    Int32,

    /// 64-bit signed integer (BIGINT, INT64)
    Int64,

    /// 32-bit unsigned integer (UINTEGER, UINT32)
    Uint32,

    /// 64-bit unsigned integer (UBIGINT, UINT64)
    Uint64,

    /// 32-bit floating point (REAL, FLOAT32)
    Float32,

    /// 64-bit floating point (DOUBLE PRECISION, DOUBLE, FLOAT, FLOAT64)
    Float64,

    /// Fixed precision decimal (NUMERIC, DECIMAL)
    Numeric {
        precision: Option<u8>,
        scale: Option<u8>,
    },

    /// Variable-length character string (VARCHAR, TEXT, STRING, CHAR)
    Varchar { max_length: Option<u64> },

    /// Variable-length binary data (VARBINARY, BYTEA, BYTES, BLOB)
    Varbinary { max_length: Option<u64> },

    /// Date (year, month, day)
    Date,

    /// Time of day
    Time,

    /// Date and time without timezone (DATETIME, TIMESTAMP WITHOUT TIME ZONE)
    Datetime,

    /// Date and time with timezone (TIMESTAMP, TIMESTAMP WITH TIME ZONE)
    Timestamp,

    /// Time interval
    Interval,

    /// Array of elements
    Array(Box<DataTypeSpec>),

    /// Struct with named fields (ROW type in standard SQL)
    Struct(Vec<StructField>),

    /// JSON data
    Json,

    /// Range of values
    Range(Box<DataTypeSpec>),

    /// UUID type
    Uuid,

    /// Named types (for user-defined types)
    Named(Vec<Ident>),
}

/// Struct field in a STRUCT type.
#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: Option<Ident>,
    pub data_type: DataTypeSpec,
}

impl DataTypeKind {
    /// Check if this type is a numeric type.
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            DataTypeKind::Int32
                | DataTypeKind::Int64
                | DataTypeKind::Uint32
                | DataTypeKind::Uint64
                | DataTypeKind::Float32
                | DataTypeKind::Float64
                | DataTypeKind::Numeric { .. }
        )
    }

    /// Check if this type is an integer type.
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            DataTypeKind::Int32 | DataTypeKind::Int64 | DataTypeKind::Uint32 | DataTypeKind::Uint64
        )
    }

    /// Check if this type is a floating-point type.
    pub fn is_floating_point(&self) -> bool {
        matches!(self, DataTypeKind::Float32 | DataTypeKind::Float64)
    }

    /// Check if this type is a date/time type.
    pub fn is_datetime(&self) -> bool {
        matches!(
            self,
            DataTypeKind::Date
                | DataTypeKind::Time
                | DataTypeKind::Datetime
                | DataTypeKind::Timestamp
        )
    }

    /// Check if this type is a string type.
    pub fn is_string(&self) -> bool {
        matches!(self, DataTypeKind::Varchar { .. })
    }
}

impl std::fmt::Display for DataTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTypeKind::Bool => write!(f, "BOOLEAN"),
            DataTypeKind::Int32 => write!(f, "INTEGER"),
            DataTypeKind::Int64 => write!(f, "BIGINT"),
            DataTypeKind::Uint32 => write!(f, "UINTEGER"),
            DataTypeKind::Uint64 => write!(f, "UBIGINT"),
            DataTypeKind::Float32 => write!(f, "REAL"),
            DataTypeKind::Float64 => write!(f, "DOUBLE PRECISION"),
            DataTypeKind::Numeric { precision, scale } => {
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
            DataTypeKind::Varchar { max_length } => {
                write!(f, "VARCHAR")?;
                if let Some(len) = max_length {
                    write!(f, "({})", len)?;
                }
                Ok(())
            }
            DataTypeKind::Varbinary { max_length } => {
                write!(f, "VARBINARY")?;
                if let Some(len) = max_length {
                    write!(f, "({})", len)?;
                }
                Ok(())
            }
            DataTypeKind::Date => write!(f, "DATE"),
            DataTypeKind::Time => write!(f, "TIME"),
            DataTypeKind::Datetime => write!(f, "DATETIME"),
            DataTypeKind::Timestamp => write!(f, "TIMESTAMP"),
            DataTypeKind::Interval => write!(f, "INTERVAL"),
            DataTypeKind::Array(elem) => write!(f, "ARRAY<{}>", elem.kind),
            DataTypeKind::Struct(fields) => {
                write!(f, "STRUCT<")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if let Some(name) = &field.name {
                        write!(f, "{} ", name)?;
                    }
                    write!(f, "{}", field.data_type.kind)?;
                }
                write!(f, ">")
            }
            DataTypeKind::Json => write!(f, "JSON"),
            DataTypeKind::Range(elem) => write!(f, "RANGE<{}>", elem.kind),
            DataTypeKind::Uuid => write!(f, "UUID"),
            DataTypeKind::Named(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ".")?;
                    }
                    write!(f, "{}", part)?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(format!("{}", DataTypeKind::Int64), "BIGINT");
        assert_eq!(format!("{}", DataTypeKind::Int32), "INTEGER");
        assert_eq!(format!("{}", DataTypeKind::Float64), "DOUBLE PRECISION");
        assert_eq!(format!("{}", DataTypeKind::Float32), "REAL");
        assert_eq!(format!("{}", DataTypeKind::Bool), "BOOLEAN");
        assert_eq!(
            format!(
                "{}",
                DataTypeKind::Numeric {
                    precision: Some(10),
                    scale: Some(2)
                }
            ),
            "NUMERIC(10, 2)"
        );
        assert_eq!(
            format!(
                "{}",
                DataTypeKind::Varchar {
                    max_length: Some(255)
                }
            ),
            "VARCHAR(255)"
        );
        assert_eq!(
            format!(
                "{}",
                DataTypeKind::Varbinary {
                    max_length: Some(100)
                }
            ),
            "VARBINARY(100)"
        );
    }

    #[test]
    fn test_type_classification() {
        assert!(DataTypeKind::Int64.is_numeric());
        assert!(DataTypeKind::Int64.is_integer());
        assert!(!DataTypeKind::Int64.is_floating_point());

        assert!(DataTypeKind::Float64.is_numeric());
        assert!(!DataTypeKind::Float64.is_integer());
        assert!(DataTypeKind::Float64.is_floating_point());

        assert!(DataTypeKind::Date.is_datetime());
        assert!(DataTypeKind::Varchar { max_length: None }.is_string());
    }
}
