//! Type registry for managing type aliases and custom display names.
//!
//! The type registry allows users to:
//! - Define custom type aliases (e.g., "TEXT" -> VARCHAR)
//! - Customize how types are displayed in error messages and output
//! - Register custom composite types

use crate::types::SqlType;
use std::collections::HashMap;

/// Registry for SQL type aliases and display customization.
///
/// # Example
///
/// ```
/// use vibesql::catalog::TypeRegistry;
/// use vibesql::types::SqlType;
///
/// let mut registry = TypeRegistry::new();
///
/// // Add custom alias
/// registry.add_alias("TEXT", SqlType::Varchar);
/// registry.add_alias("SERIAL", SqlType::Int32);
///
/// // Resolve alias to canonical type
/// assert_eq!(registry.resolve("TEXT"), Some(&SqlType::Varchar));
/// assert_eq!(registry.resolve("VARCHAR"), Some(&SqlType::Varchar));
/// ```
#[derive(Debug, Clone)]
pub struct TypeRegistry {
    /// Map from alias name (uppercase) to canonical SqlType
    aliases: HashMap<String, SqlType>,
    /// Custom display names for types (optional override)
    display_names: HashMap<SqlType, String>,
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeRegistry {
    /// Create a new type registry with standard SQL aliases pre-registered.
    pub fn new() -> Self {
        let mut registry = Self {
            aliases: HashMap::new(),
            display_names: HashMap::new(),
        };
        registry.register_standard_aliases();
        registry
    }

    /// Create an empty type registry without standard aliases.
    pub fn empty() -> Self {
        Self {
            aliases: HashMap::new(),
            display_names: HashMap::new(),
        }
    }

    /// Register standard SQL type aliases.
    fn register_standard_aliases(&mut self) {
        // Boolean aliases
        self.aliases.insert("BOOL".to_string(), SqlType::Bool);
        self.aliases.insert("BOOLEAN".to_string(), SqlType::Bool);

        // Integer aliases
        self.aliases.insert("INT".to_string(), SqlType::Int32);
        self.aliases.insert("INT32".to_string(), SqlType::Int32);
        self.aliases.insert("INTEGER".to_string(), SqlType::Int32);
        self.aliases.insert("SMALLINT".to_string(), SqlType::Int32);
        self.aliases.insert("TINYINT".to_string(), SqlType::Int32);

        self.aliases.insert("INT64".to_string(), SqlType::Int64);
        self.aliases.insert("BIGINT".to_string(), SqlType::Int64);

        self.aliases.insert("UINT32".to_string(), SqlType::Uint32);
        self.aliases.insert("UINTEGER".to_string(), SqlType::Uint32);

        self.aliases.insert("UINT64".to_string(), SqlType::Uint64);
        self.aliases.insert("UBIGINT".to_string(), SqlType::Uint64);

        // Floating point aliases
        self.aliases.insert("FLOAT".to_string(), SqlType::Float32);
        self.aliases.insert("FLOAT32".to_string(), SqlType::Float32);
        self.aliases.insert("REAL".to_string(), SqlType::Float32);

        self.aliases.insert("FLOAT64".to_string(), SqlType::Float64);
        self.aliases.insert("DOUBLE".to_string(), SqlType::Float64);
        self.aliases
            .insert("DOUBLE PRECISION".to_string(), SqlType::Float64);

        // String aliases
        self.aliases.insert("VARCHAR".to_string(), SqlType::Varchar);
        self.aliases.insert("STRING".to_string(), SqlType::Varchar);
        self.aliases.insert("TEXT".to_string(), SqlType::Varchar);
        self.aliases.insert("CHAR".to_string(), SqlType::Varchar);

        // Binary aliases
        self.aliases
            .insert("VARBINARY".to_string(), SqlType::Varbinary);
        self.aliases.insert("BYTES".to_string(), SqlType::Varbinary);
        self.aliases.insert("BYTEA".to_string(), SqlType::Varbinary);
        self.aliases.insert("BLOB".to_string(), SqlType::Varbinary);

        // Date/time aliases
        self.aliases.insert("DATE".to_string(), SqlType::Date);
        self.aliases.insert("TIME".to_string(), SqlType::Time);
        self.aliases
            .insert("DATETIME".to_string(), SqlType::Datetime);
        self.aliases
            .insert("TIMESTAMP".to_string(), SqlType::Timestamp);
        self.aliases
            .insert("INTERVAL".to_string(), SqlType::Interval);

        // Other types
        self.aliases.insert("JSON".to_string(), SqlType::Json);
        self.aliases.insert("UUID".to_string(), SqlType::Uuid);
    }

    /// Add a custom type alias.
    ///
    /// # Arguments
    ///
    /// * `alias` - The alias name (will be uppercased)
    /// * `sql_type` - The canonical type this alias resolves to
    ///
    /// # Example
    ///
    /// ```
    /// use vibesql::catalog::TypeRegistry;
    /// use vibesql::types::SqlType;
    ///
    /// let mut registry = TypeRegistry::new();
    /// registry.add_alias("SERIAL", SqlType::Int32);
    /// ```
    pub fn add_alias(&mut self, alias: impl Into<String>, sql_type: SqlType) {
        self.aliases.insert(alias.into().to_uppercase(), sql_type);
    }

    /// Remove a type alias.
    pub fn remove_alias(&mut self, alias: &str) -> Option<SqlType> {
        self.aliases.remove(&alias.to_uppercase())
    }

    /// Resolve a type name to its canonical SqlType.
    ///
    /// Returns `None` if the type name is not registered.
    pub fn resolve(&self, type_name: &str) -> Option<&SqlType> {
        self.aliases.get(&type_name.to_uppercase())
    }

    /// Check if a type alias exists.
    pub fn has_alias(&self, alias: &str) -> bool {
        self.aliases.contains_key(&alias.to_uppercase())
    }

    /// Set a custom display name for a type.
    ///
    /// This overrides the default Display implementation for error messages.
    pub fn set_display_name(&mut self, sql_type: SqlType, display_name: impl Into<String>) {
        self.display_names.insert(sql_type, display_name.into());
    }

    /// Get the display name for a type.
    ///
    /// Returns the custom display name if set, otherwise uses the type's Display impl.
    pub fn display_name(&self, sql_type: &SqlType) -> String {
        self.display_names
            .get(sql_type)
            .cloned()
            .unwrap_or_else(|| sql_type.to_string())
    }

    /// Get all registered aliases.
    pub fn aliases(&self) -> impl Iterator<Item = (&str, &SqlType)> {
        self.aliases.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Get the number of registered aliases.
    pub fn len(&self) -> usize {
        self.aliases.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.aliases.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_aliases() {
        let registry = TypeRegistry::new();

        // Check standard aliases work
        assert_eq!(registry.resolve("INT"), Some(&SqlType::Int32));
        assert_eq!(registry.resolve("INTEGER"), Some(&SqlType::Int32));
        assert_eq!(registry.resolve("BIGINT"), Some(&SqlType::Int64));
        assert_eq!(registry.resolve("VARCHAR"), Some(&SqlType::Varchar));
        assert_eq!(registry.resolve("TEXT"), Some(&SqlType::Varchar));
        assert_eq!(registry.resolve("BOOLEAN"), Some(&SqlType::Bool));

        // Case insensitive
        assert_eq!(registry.resolve("int"), Some(&SqlType::Int32));
        assert_eq!(registry.resolve("Int"), Some(&SqlType::Int32));
    }

    #[test]
    fn test_custom_alias() {
        let mut registry = TypeRegistry::new();

        registry.add_alias("SERIAL", SqlType::Int32);
        registry.add_alias("BIGSERIAL", SqlType::Int64);

        assert_eq!(registry.resolve("SERIAL"), Some(&SqlType::Int32));
        assert_eq!(registry.resolve("BIGSERIAL"), Some(&SqlType::Int64));
    }

    #[test]
    fn test_remove_alias() {
        let mut registry = TypeRegistry::new();
        registry.add_alias("CUSTOM", SqlType::Int64);

        assert!(registry.has_alias("CUSTOM"));
        registry.remove_alias("CUSTOM");
        assert!(!registry.has_alias("CUSTOM"));
    }

    #[test]
    fn test_display_name_override() {
        let mut registry = TypeRegistry::new();

        // Default display name
        assert_eq!(registry.display_name(&SqlType::Int64), "BIGINT");

        // Override
        registry.set_display_name(SqlType::Int64, "INT64");
        assert_eq!(registry.display_name(&SqlType::Int64), "INT64");
    }

    #[test]
    fn test_empty_registry() {
        let registry = TypeRegistry::empty();
        assert!(registry.is_empty());
        assert_eq!(registry.resolve("INT"), None);
    }
}
