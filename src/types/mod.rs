//! Type system for SQL values and expressions.
//!
//! This module provides runtime type information for SQL expressions
//! and values, used during semantic analysis.

pub mod sql_type;
pub mod value;

pub use sql_type::*;
pub use value::*;
