//! Function signature definitions.

use crate::types::SqlType;

/// Function signature for built-in and user-defined functions.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    /// Function name (uppercase).
    pub name: String,
    /// Parameter types (None means any type is accepted).
    pub parameters: Vec<FunctionParameter>,
    /// Return type.
    pub return_type: SqlType,
    /// Whether this is an aggregate function.
    pub is_aggregate: bool,
    /// Whether this is a window function.
    pub is_window: bool,
    /// Whether the function is deterministic.
    pub is_deterministic: bool,
    /// Minimum number of arguments.
    pub min_args: usize,
    /// Maximum number of arguments (None = unlimited).
    pub max_args: Option<usize>,
}

/// A function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    /// Parameter name (optional).
    pub name: Option<String>,
    /// Expected type (None = any type).
    pub data_type: Option<SqlType>,
    /// Whether the parameter is optional.
    pub optional: bool,
    /// Whether this parameter accepts variable arguments.
    pub variadic: bool,
}

impl FunctionSignature {
    /// Create a scalar function signature.
    pub fn scalar(name: impl Into<String>, return_type: SqlType) -> Self {
        Self {
            name: name.into().to_uppercase(),
            parameters: Vec::new(),
            return_type,
            is_aggregate: false,
            is_window: false,
            is_deterministic: true,
            min_args: 0,
            max_args: None,
        }
    }

    /// Create an aggregate function signature.
    pub fn aggregate(name: impl Into<String>, return_type: SqlType) -> Self {
        Self {
            name: name.into().to_uppercase(),
            parameters: Vec::new(),
            return_type,
            is_aggregate: true,
            is_window: false,
            is_deterministic: true,
            min_args: 0,
            max_args: None,
        }
    }

    /// Create a window function signature.
    pub fn window(name: impl Into<String>, return_type: SqlType) -> Self {
        Self {
            name: name.into().to_uppercase(),
            parameters: Vec::new(),
            return_type,
            is_aggregate: false,
            is_window: true,
            is_deterministic: true,
            min_args: 0,
            max_args: None,
        }
    }

    /// Set minimum number of arguments.
    pub fn with_min_args(mut self, min: usize) -> Self {
        self.min_args = min;
        self
    }

    /// Set maximum number of arguments.
    pub fn with_max_args(mut self, max: usize) -> Self {
        self.max_args = Some(max);
        self
    }

    /// Set exact number of arguments.
    pub fn with_args(mut self, count: usize) -> Self {
        self.min_args = count;
        self.max_args = Some(count);
        self
    }

    /// Add a parameter.
    pub fn with_param(mut self, param: FunctionParameter) -> Self {
        self.parameters.push(param);
        self
    }

    /// Mark as non-deterministic.
    pub fn non_deterministic(mut self) -> Self {
        self.is_deterministic = false;
        self
    }

    /// Check if the given number of arguments is valid.
    pub fn accepts_arg_count(&self, count: usize) -> bool {
        if count < self.min_args {
            return false;
        }
        match self.max_args {
            Some(max) => count <= max,
            None => true,
        }
    }

    /// Check if the function can be used in a window context.
    pub fn can_be_window(&self) -> bool {
        self.is_window || self.is_aggregate
    }
}

impl FunctionParameter {
    /// Create a new parameter.
    pub fn new(name: impl Into<String>, data_type: SqlType) -> Self {
        Self {
            name: Some(name.into()),
            data_type: Some(data_type),
            optional: false,
            variadic: false,
        }
    }

    /// Create a parameter that accepts any type.
    pub fn any(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            data_type: None,
            optional: false,
            variadic: false,
        }
    }

    /// Create an unnamed parameter.
    pub fn unnamed(data_type: SqlType) -> Self {
        Self {
            name: None,
            data_type: Some(data_type),
            optional: false,
            variadic: false,
        }
    }

    /// Mark as optional.
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Mark as variadic (accepts multiple values).
    pub fn variadic(mut self) -> Self {
        self.variadic = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_signature() {
        let count = FunctionSignature::aggregate("COUNT", SqlType::Int64)
            .with_min_args(1)
            .with_max_args(1);

        assert!(count.is_aggregate);
        assert!(!count.is_window);
        assert!(count.accepts_arg_count(1));
        assert!(!count.accepts_arg_count(0));
        assert!(!count.accepts_arg_count(2));
    }

    #[test]
    fn test_window_function() {
        let row_number = FunctionSignature::window("ROW_NUMBER", SqlType::Int64).with_args(0);

        assert!(row_number.is_window);
        assert!(row_number.can_be_window());
        assert!(row_number.accepts_arg_count(0));
    }
}
