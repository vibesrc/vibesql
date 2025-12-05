//! Builder pattern for creating customized catalogs.
//!
//! The catalog builder provides a fluent API for creating catalogs with:
//! - Custom functions (scalar, aggregate, window)
//! - Custom tables and schemas
//! - Custom type aliases
//! - Built-in function selection

use super::{ColumnSchema, FunctionSignature, MemoryCatalog, TableSchema, TypeRegistry};
use crate::types::SqlType;

/// Builder for creating customized `MemoryCatalog` instances.
///
/// # Example
///
/// ```
/// use vibesql::catalog::{CatalogBuilder, ColumnSchema, FunctionSignature};
/// use vibesql::types::SqlType;
///
/// let catalog = CatalogBuilder::new()
///     .with_builtins()
///     .add_scalar_function("MY_FUNC", SqlType::Int64)
///     .add_table("users", |t| {
///         t.column("id", SqlType::Int64)
///          .column("name", SqlType::Varchar)
///     })
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct CatalogBuilder {
    catalog: MemoryCatalog,
    type_registry: TypeRegistry,
    include_builtins: bool,
}

impl CatalogBuilder {
    /// Create a new catalog builder.
    pub fn new() -> Self {
        Self {
            catalog: MemoryCatalog::new(),
            type_registry: TypeRegistry::new(),
            include_builtins: false,
        }
    }

    /// Include standard built-in functions (COUNT, SUM, CONCAT, etc.).
    pub fn with_builtins(mut self) -> Self {
        self.include_builtins = true;
        self
    }

    /// Add a custom type alias.
    ///
    /// # Example
    ///
    /// ```
    /// use vibesql::catalog::CatalogBuilder;
    /// use vibesql::types::SqlType;
    ///
    /// let catalog = CatalogBuilder::new()
    ///     .add_type_alias("SERIAL", SqlType::Int32)
    ///     .add_type_alias("MONEY", SqlType::Numeric { precision: Some(19), scale: Some(4) })
    ///     .build();
    /// ```
    pub fn add_type_alias(mut self, alias: impl Into<String>, sql_type: SqlType) -> Self {
        self.type_registry.add_alias(alias, sql_type);
        self
    }

    /// Add a scalar function.
    ///
    /// # Example
    ///
    /// ```
    /// use vibesql::catalog::CatalogBuilder;
    /// use vibesql::types::SqlType;
    ///
    /// let catalog = CatalogBuilder::new()
    ///     .add_scalar_function("HASH", SqlType::Int64)
    ///     .add_scalar_function("ENCRYPT", SqlType::Varbinary)
    ///     .build();
    /// ```
    pub fn add_scalar_function(mut self, name: impl Into<String>, return_type: SqlType) -> Self {
        self.catalog
            .add_function(FunctionSignature::scalar(name, return_type));
        self
    }

    /// Add an aggregate function.
    ///
    /// # Example
    ///
    /// ```
    /// use vibesql::catalog::CatalogBuilder;
    /// use vibesql::types::SqlType;
    ///
    /// let catalog = CatalogBuilder::new()
    ///     .add_aggregate_function("MY_SUM", SqlType::Float64)
    ///     .build();
    /// ```
    pub fn add_aggregate_function(mut self, name: impl Into<String>, return_type: SqlType) -> Self {
        self.catalog
            .add_function(FunctionSignature::aggregate(name, return_type));
        self
    }

    /// Add a window function.
    ///
    /// # Example
    ///
    /// ```
    /// use vibesql::catalog::CatalogBuilder;
    /// use vibesql::types::SqlType;
    ///
    /// let catalog = CatalogBuilder::new()
    ///     .add_window_function("CUSTOM_RANK", SqlType::Int64)
    ///     .build();
    /// ```
    pub fn add_window_function(mut self, name: impl Into<String>, return_type: SqlType) -> Self {
        self.catalog
            .add_function(FunctionSignature::window(name, return_type));
        self
    }

    /// Add a function with full signature control.
    ///
    /// # Example
    ///
    /// ```
    /// use vibesql::catalog::{CatalogBuilder, FunctionSignature};
    /// use vibesql::types::SqlType;
    ///
    /// let catalog = CatalogBuilder::new()
    ///     .add_function(FunctionSignature::scalar("MY_FUNC", SqlType::Varchar))
    ///     .build();
    /// ```
    pub fn add_function(mut self, signature: FunctionSignature) -> Self {
        self.catalog.add_function(signature);
        self
    }

    /// Add a table using a builder closure.
    ///
    /// # Example
    ///
    /// ```
    /// use vibesql::catalog::CatalogBuilder;
    /// use vibesql::types::SqlType;
    ///
    /// let catalog = CatalogBuilder::new()
    ///     .add_table("users", |t| {
    ///         t.column("id", SqlType::Int64)
    ///          .column("name", SqlType::Varchar)
    ///          .column("email", SqlType::Varchar)
    ///     })
    ///     .build();
    /// ```
    pub fn add_table<F>(mut self, name: impl Into<String>, builder_fn: F) -> Self
    where
        F: FnOnce(TableBuilder) -> TableBuilder,
    {
        let name = name.into();
        let builder = TableBuilder::new(&name);
        let table = builder_fn(builder).build();
        self.catalog.add_table(table);
        self
    }

    /// Add a pre-built table schema.
    pub fn add_table_schema(mut self, table: TableSchema) -> Self {
        self.catalog.add_table(table);
        self
    }

    /// Add a schema to the catalog.
    pub fn add_schema(mut self, name: impl Into<String>) -> Self {
        self.catalog.add_schema(name);
        self
    }

    /// Get access to the type registry for advanced customization.
    pub fn type_registry(&self) -> &TypeRegistry {
        &self.type_registry
    }

    /// Get mutable access to the type registry.
    pub fn type_registry_mut(&mut self) -> &mut TypeRegistry {
        &mut self.type_registry
    }

    /// Build the catalog.
    pub fn build(mut self) -> MemoryCatalog {
        if self.include_builtins {
            self.catalog.register_builtins();
        }
        self.catalog
    }

    /// Build the catalog and return it along with the type registry.
    pub fn build_with_registry(mut self) -> (MemoryCatalog, TypeRegistry) {
        if self.include_builtins {
            self.catalog.register_builtins();
        }
        (self.catalog, self.type_registry)
    }
}

/// Builder for creating table schemas.
#[derive(Debug)]
pub struct TableBuilder {
    name: String,
    columns: Vec<ColumnSchema>,
}

impl TableBuilder {
    /// Create a new table builder.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            columns: Vec::new(),
        }
    }

    /// Add a column with name and type.
    pub fn column(mut self, name: impl Into<String>, data_type: SqlType) -> Self {
        self.columns.push(ColumnSchema::new(name, data_type));
        self
    }

    /// Add a non-nullable column.
    pub fn column_not_null(mut self, name: impl Into<String>, data_type: SqlType) -> Self {
        self.columns
            .push(ColumnSchema::new(name, data_type).not_null());
        self
    }

    /// Add a primary key column.
    pub fn primary_key(mut self, name: impl Into<String>, data_type: SqlType) -> Self {
        self.columns
            .push(ColumnSchema::new(name, data_type).primary_key());
        self
    }

    /// Add a column with full customization.
    pub fn add_column(mut self, column: ColumnSchema) -> Self {
        self.columns.push(column);
        self
    }

    /// Build the table schema.
    pub fn build(self) -> TableSchema {
        TableSchema::new(self.name, self.columns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Catalog;

    #[test]
    fn test_builder_with_builtins() {
        let catalog = CatalogBuilder::new().with_builtins().build();

        // Should have built-in functions
        let count = catalog.resolve_function(&["COUNT".to_string()]).unwrap();
        assert!(count.is_some());
    }

    #[test]
    fn test_builder_custom_function() {
        let catalog = CatalogBuilder::new()
            .add_scalar_function("MY_HASH", SqlType::Int64)
            .add_aggregate_function("MY_AGG", SqlType::Float64)
            .build();

        let hash = catalog.resolve_function(&["MY_HASH".to_string()]).unwrap();
        assert!(hash.is_some());
        assert!(!hash.unwrap().is_aggregate);

        let agg = catalog.resolve_function(&["MY_AGG".to_string()]).unwrap();
        assert!(agg.is_some());
        assert!(agg.unwrap().is_aggregate);
    }

    #[test]
    fn test_builder_table() {
        let catalog = CatalogBuilder::new()
            .add_table("users", |t| {
                t.primary_key("id", SqlType::Int64)
                    .column_not_null("name", SqlType::Varchar)
                    .column("email", SqlType::Varchar)
            })
            .build();

        let table = catalog.resolve_table(&["users".to_string()]).unwrap();
        assert!(table.is_some());

        let table = table.unwrap();
        assert_eq!(table.name, "users");
        assert_eq!(table.columns.len(), 3);

        let id_col = table.get_column("id").unwrap();
        assert!(id_col.is_primary_key);
        assert!(!id_col.nullable);
    }

    #[test]
    fn test_builder_type_alias() {
        let (_, registry) = CatalogBuilder::new()
            .add_type_alias("SERIAL", SqlType::Int32)
            .add_type_alias("BIGSERIAL", SqlType::Int64)
            .build_with_registry();

        assert_eq!(registry.resolve("SERIAL"), Some(&SqlType::Int32));
        assert_eq!(registry.resolve("BIGSERIAL"), Some(&SqlType::Int64));
    }

    #[test]
    fn test_full_builder() {
        let catalog = CatalogBuilder::new()
            .with_builtins()
            .add_scalar_function("CUSTOM_HASH", SqlType::Int64)
            .add_table("products", |t| {
                t.primary_key("id", SqlType::Int64)
                    .column_not_null("name", SqlType::Varchar)
                    .column("price", SqlType::Float64)
            })
            .build();

        // Has built-ins
        assert!(catalog
            .resolve_function(&["COUNT".to_string()])
            .unwrap()
            .is_some());

        // Has custom function
        assert!(catalog
            .resolve_function(&["CUSTOM_HASH".to_string()])
            .unwrap()
            .is_some());

        // Has table
        assert!(catalog
            .resolve_table(&["products".to_string()])
            .unwrap()
            .is_some());
    }
}
