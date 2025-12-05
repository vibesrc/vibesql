//! Schema definitions for tables and columns.

use crate::types::SqlType;

/// Schema information for a table.
#[derive(Debug, Clone, PartialEq)]
pub struct TableSchema {
    /// Table name.
    pub name: String,
    /// Columns in the table.
    pub columns: Vec<ColumnSchema>,
}

impl TableSchema {
    /// Create a new table schema.
    pub fn new(name: impl Into<String>, columns: Vec<ColumnSchema>) -> Self {
        Self {
            name: name.into(),
            columns,
        }
    }

    /// Get a column by name (case-insensitive).
    pub fn get_column(&self, name: &str) -> Option<&ColumnSchema> {
        self.columns
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(name))
    }

    /// Get column index by name (case-insensitive).
    pub fn get_column_index(&self, name: &str) -> Option<usize> {
        self.columns
            .iter()
            .position(|c| c.name.eq_ignore_ascii_case(name))
    }

    /// Check if the table has a column.
    pub fn has_column(&self, name: &str) -> bool {
        self.get_column(name).is_some()
    }

    /// Get all column names.
    pub fn column_names(&self) -> Vec<&str> {
        self.columns.iter().map(|c| c.name.as_str()).collect()
    }
}

/// Schema information for a column.
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnSchema {
    /// Column name.
    pub name: String,
    /// Data type.
    pub data_type: SqlType,
    /// Whether the column is nullable.
    pub nullable: bool,
    /// Whether the column is a primary key.
    pub is_primary_key: bool,
    /// Default value expression (as string for now).
    pub default_value: Option<String>,
    /// Column description/comment.
    pub description: Option<String>,
}

impl ColumnSchema {
    /// Create a new column schema.
    pub fn new(name: impl Into<String>, data_type: SqlType) -> Self {
        Self {
            name: name.into(),
            data_type,
            nullable: true,
            is_primary_key: false,
            default_value: None,
            description: None,
        }
    }

    /// Mark column as not nullable.
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    /// Mark column as primary key.
    pub fn primary_key(mut self) -> Self {
        self.is_primary_key = true;
        self.nullable = false;
        self
    }

    /// Set default value.
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default_value = Some(default.into());
        self
    }

    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Builder for creating table schemas.
#[derive(Debug, Default)]
pub struct TableSchemaBuilder {
    name: String,
    columns: Vec<ColumnSchema>,
}

impl TableSchemaBuilder {
    /// Create a new table schema builder.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            columns: Vec::new(),
        }
    }

    /// Add a column.
    pub fn column(mut self, col: ColumnSchema) -> Self {
        self.columns.push(col);
        self
    }

    /// Add a column with name and type.
    pub fn add_column(mut self, name: impl Into<String>, data_type: SqlType) -> Self {
        self.columns.push(ColumnSchema::new(name, data_type));
        self
    }

    /// Build the table schema.
    pub fn build(self) -> TableSchema {
        TableSchema {
            name: self.name,
            columns: self.columns,
        }
    }
}

/// Resolved reference to a column in a query.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedColumn {
    /// Table alias or name this column belongs to.
    pub table_ref: Option<String>,
    /// Column name.
    pub column_name: String,
    /// Column index in the table.
    pub column_index: usize,
    /// Data type of the column.
    pub data_type: SqlType,
    /// Whether the column is nullable.
    pub nullable: bool,
}

impl ResolvedColumn {
    /// Create a new resolved column.
    pub fn new(
        table_ref: Option<String>,
        column_name: String,
        column_index: usize,
        data_type: SqlType,
        nullable: bool,
    ) -> Self {
        Self {
            table_ref,
            column_name,
            column_index,
            data_type,
            nullable,
        }
    }

    /// Get the fully qualified name.
    pub fn qualified_name(&self) -> String {
        match &self.table_ref {
            Some(table) => format!("{}.{}", table, self.column_name),
            None => self.column_name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_schema() {
        let table = TableSchemaBuilder::new("users")
            .column(ColumnSchema::new("id", SqlType::Int64).primary_key())
            .column(ColumnSchema::new("name", SqlType::Varchar).not_null())
            .column(ColumnSchema::new("email", SqlType::Varchar))
            .build();

        assert_eq!(table.name, "users");
        assert_eq!(table.columns.len(), 3);

        let id_col = table.get_column("id").unwrap();
        assert!(!id_col.nullable);
        assert!(id_col.is_primary_key);

        let email_col = table.get_column("EMAIL").unwrap(); // Case-insensitive
        assert!(email_col.nullable);
    }
}
