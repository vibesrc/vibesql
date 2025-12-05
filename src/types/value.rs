//! SQL value types.

/// A SQL value.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    #[default]
    Null,
    Boolean(bool),
    Int64(i64),
    Float64(f64),
    String(String),
    Bytes(Vec<u8>),
    Date(i32),      // Days since Unix epoch
    Time(i64),      // Microseconds since midnight
    Datetime(i64),  // Microseconds since Unix epoch
    Timestamp(i64), // Microseconds since Unix epoch (with timezone)
    Interval(Interval),
    Array(Vec<Value>),
    Struct(Vec<(String, Value)>),
    Json(String),
}

/// Interval value.
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub months: i32,
    pub days: i32,
    pub micros: i64,
}

impl Value {
    /// Check if this value is NULL.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Get the type of this value.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "NULL",
            Value::Boolean(_) => "BOOLEAN",
            Value::Int64(_) => "BIGINT",
            Value::Float64(_) => "DOUBLE PRECISION",
            Value::String(_) => "VARCHAR",
            Value::Bytes(_) => "VARBINARY",
            Value::Date(_) => "DATE",
            Value::Time(_) => "TIME",
            Value::Datetime(_) => "DATETIME",
            Value::Timestamp(_) => "TIMESTAMP",
            Value::Interval(_) => "INTERVAL",
            Value::Array(_) => "ARRAY",
            Value::Struct(_) => "STRUCT",
            Value::Json(_) => "JSON",
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int64(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float64(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Bytes(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_bool() {
        assert_eq!(Value::from(true), Value::Boolean(true));
        assert_eq!(Value::from(false), Value::Boolean(false));
    }

    #[test]
    fn test_value_from_i64() {
        assert_eq!(Value::from(42i64), Value::Int64(42));
        assert_eq!(Value::from(-1i64), Value::Int64(-1));
        assert_eq!(Value::from(0i64), Value::Int64(0));
    }

    #[test]
    fn test_value_from_f64() {
        assert_eq!(Value::from(3.14f64), Value::Float64(3.14));
        assert_eq!(Value::from(-0.5f64), Value::Float64(-0.5));
    }

    #[test]
    fn test_value_from_string() {
        assert_eq!(
            Value::from("hello".to_string()),
            Value::String("hello".to_string())
        );
        assert_eq!(Value::from(String::new()), Value::String(String::new()));
    }

    #[test]
    fn test_value_from_str() {
        assert_eq!(Value::from("hello"), Value::String("hello".to_string()));
        assert_eq!(Value::from(""), Value::String(String::new()));
    }

    #[test]
    fn test_value_from_bytes() {
        assert_eq!(Value::from(vec![1u8, 2, 3]), Value::Bytes(vec![1, 2, 3]));
        assert_eq!(Value::from(Vec::<u8>::new()), Value::Bytes(vec![]));
    }

    #[test]
    fn test_value_is_null() {
        assert!(Value::Null.is_null());
        assert!(!Value::Boolean(true).is_null());
        assert!(!Value::Int64(0).is_null());
        assert!(!Value::String("".to_string()).is_null());
    }

    #[test]
    fn test_value_type_name() {
        assert_eq!(Value::Null.type_name(), "NULL");
        assert_eq!(Value::Boolean(true).type_name(), "BOOLEAN");
        assert_eq!(Value::Int64(42).type_name(), "BIGINT");
        assert_eq!(Value::Float64(3.14).type_name(), "DOUBLE PRECISION");
        assert_eq!(Value::String("test".to_string()).type_name(), "VARCHAR");
        assert_eq!(Value::Bytes(vec![]).type_name(), "VARBINARY");
        assert_eq!(Value::Date(0).type_name(), "DATE");
        assert_eq!(Value::Time(0).type_name(), "TIME");
        assert_eq!(Value::Datetime(0).type_name(), "DATETIME");
        assert_eq!(Value::Timestamp(0).type_name(), "TIMESTAMP");
        assert_eq!(
            Value::Interval(Interval {
                months: 0,
                days: 0,
                micros: 0
            })
            .type_name(),
            "INTERVAL"
        );
        assert_eq!(Value::Array(vec![]).type_name(), "ARRAY");
        assert_eq!(Value::Struct(vec![]).type_name(), "STRUCT");
        assert_eq!(Value::Json("{}".to_string()).type_name(), "JSON");
    }

    #[test]
    fn test_value_default() {
        assert_eq!(Value::default(), Value::Null);
    }
}
