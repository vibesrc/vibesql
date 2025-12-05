//! Scope management for semantic analysis.

use crate::types::SqlType;
use std::collections::HashMap;

/// A scope for name resolution.
#[derive(Debug, Clone)]
pub struct Scope {
    /// Tables available in this scope (keyed by alias or name).
    tables: HashMap<String, ScopeTable>,
    /// CTEs available in this scope.
    ctes: HashMap<String, CteRef>,
    /// Named expressions (for SELECT aliases that can be referenced).
    named_exprs: HashMap<String, ExprRef>,
    /// Whether this scope allows aggregates.
    pub allows_aggregates: bool,
    /// Whether we're inside an aggregate function.
    pub in_aggregate: bool,
    /// Whether we're inside a window function.
    pub in_window: bool,
    /// Columns that must be in GROUP BY (for validation).
    pub group_by_columns: Vec<String>,
    /// Whether GROUP BY is present.
    pub has_group_by: bool,
}

/// Reference to a table in scope (renamed to avoid collision with ast::TableRef).
#[derive(Debug, Clone)]
pub struct ScopeTable {
    /// The alias (or original name if no alias).
    pub alias: String,
    /// The original table name.
    pub original_name: Vec<String>,
    /// Columns in this table.
    pub columns: Vec<ScopeColumn>,
}

/// Reference to a column in scope.
#[derive(Debug, Clone)]
pub struct ScopeColumn {
    /// Column name.
    pub name: String,
    /// Resolved type.
    pub data_type: SqlType,
    /// Whether the column is nullable.
    pub nullable: bool,
    /// Source table alias.
    pub table_alias: String,
    /// Index in the source table.
    pub column_index: usize,
}

/// Reference to a CTE.
#[derive(Debug, Clone)]
pub struct CteRef {
    /// CTE name.
    pub name: String,
    /// Columns produced by the CTE.
    pub columns: Vec<ScopeColumn>,
    /// Whether this is a recursive CTE.
    pub is_recursive: bool,
}

/// Reference to a named expression (SELECT alias).
#[derive(Debug, Clone)]
pub struct ExprRef {
    /// The alias name.
    pub name: String,
    /// The resolved type.
    pub data_type: SqlType,
    /// Whether the expression is nullable.
    pub nullable: bool,
    /// Position in select list.
    pub ordinal: usize,
}

impl Scope {
    /// Create a new empty scope.
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            ctes: HashMap::new(),
            named_exprs: HashMap::new(),
            allows_aggregates: true,
            in_aggregate: false,
            in_window: false,
            group_by_columns: Vec::new(),
            has_group_by: false,
        }
    }

    /// Add a table to this scope.
    pub fn add_table(&mut self, table: ScopeTable) {
        let key = table.alias.to_lowercase();
        self.tables.insert(key, table);
    }

    /// Add a CTE to this scope.
    pub fn add_cte(&mut self, cte: CteRef) {
        let key = cte.name.to_lowercase();
        self.ctes.insert(key, cte);
    }

    /// Add a named expression (SELECT alias).
    pub fn add_named_expr(&mut self, expr: ExprRef) {
        let key = expr.name.to_lowercase();
        self.named_exprs.insert(key, expr);
    }

    /// Look up a table by name (case-insensitive).
    pub fn lookup_table(&self, name: &str) -> Option<&ScopeTable> {
        self.tables.get(&name.to_lowercase())
    }

    /// Look up a CTE by name (case-insensitive).
    pub fn lookup_cte(&self, name: &str) -> Option<&CteRef> {
        self.ctes.get(&name.to_lowercase())
    }

    /// Look up a named expression by name (case-insensitive).
    pub fn lookup_named_expr(&self, name: &str) -> Option<&ExprRef> {
        self.named_exprs.get(&name.to_lowercase())
    }

    /// Look up a column by name across all tables.
    /// Returns (table_ref, column_ref) or None.
    /// Returns an error indicator if ambiguous.
    pub fn lookup_column(&self, name: &str) -> ColumnLookupResult {
        let name_lower = name.to_lowercase();
        let mut found: Vec<(&ScopeTable, &ScopeColumn)> = Vec::new();

        for table in self.tables.values() {
            for col in &table.columns {
                if col.name.to_lowercase() == name_lower {
                    found.push((table, col));
                }
            }
        }

        match found.len() {
            0 => ColumnLookupResult::NotFound,
            1 => ColumnLookupResult::Found(found[0].0.clone(), found[0].1.clone()),
            _ => {
                let tables: Vec<String> = found.iter().map(|(t, _)| t.alias.clone()).collect();
                ColumnLookupResult::Ambiguous(tables)
            }
        }
    }

    /// Look up a qualified column (table.column).
    pub fn lookup_qualified_column(
        &self,
        table_name: &str,
        column_name: &str,
    ) -> Option<ScopeColumn> {
        let table = self.lookup_table(table_name)?;
        let col_lower = column_name.to_lowercase();
        table
            .columns
            .iter()
            .find(|c| c.name.to_lowercase() == col_lower)
            .cloned()
    }

    /// Get all tables in scope.
    pub fn all_tables(&self) -> impl Iterator<Item = &ScopeTable> {
        self.tables.values()
    }

    /// Get all columns across all tables.
    pub fn all_columns(&self) -> Vec<&ScopeColumn> {
        self.tables
            .values()
            .flat_map(|t| t.columns.iter())
            .collect()
    }

    /// Check if a table name exists in scope.
    pub fn has_table(&self, name: &str) -> bool {
        self.tables.contains_key(&name.to_lowercase())
    }

    /// Check if a CTE name exists in scope.
    pub fn has_cte(&self, name: &str) -> bool {
        self.ctes.contains_key(&name.to_lowercase())
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of looking up a column.
#[derive(Debug, Clone)]
pub enum ColumnLookupResult {
    /// Column found uniquely.
    Found(ScopeTable, ScopeColumn),
    /// Column not found.
    NotFound,
    /// Column found in multiple tables (ambiguous).
    Ambiguous(Vec<String>),
}

impl ScopeTable {
    /// Create a new table reference.
    pub fn new(alias: String, original_name: Vec<String>, columns: Vec<ScopeColumn>) -> Self {
        Self {
            alias,
            original_name,
            columns,
        }
    }

    /// Get a column by name.
    pub fn get_column(&self, name: &str) -> Option<&ScopeColumn> {
        let name_lower = name.to_lowercase();
        self.columns
            .iter()
            .find(|c| c.name.to_lowercase() == name_lower)
    }
}

impl ScopeColumn {
    /// Create a new column reference.
    pub fn new(
        name: String,
        data_type: SqlType,
        nullable: bool,
        table_alias: String,
        column_index: usize,
    ) -> Self {
        Self {
            name,
            data_type,
            nullable,
            table_alias,
            column_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_table_lookup() {
        let mut scope = Scope::new();

        let table = ScopeTable::new(
            "users".to_string(),
            vec!["users".to_string()],
            vec![
                ScopeColumn::new(
                    "id".to_string(),
                    SqlType::Int64,
                    false,
                    "users".to_string(),
                    0,
                ),
                ScopeColumn::new(
                    "name".to_string(),
                    SqlType::Varchar,
                    true,
                    "users".to_string(),
                    1,
                ),
            ],
        );
        scope.add_table(table);

        assert!(scope.has_table("users"));
        assert!(scope.has_table("USERS")); // case-insensitive
        assert!(!scope.has_table("orders"));

        let col = scope.lookup_qualified_column("users", "id");
        assert!(col.is_some());
        assert_eq!(col.unwrap().data_type, SqlType::Int64);
    }

    #[test]
    fn test_scope_ambiguous_column() {
        let mut scope = Scope::new();

        scope.add_table(ScopeTable::new(
            "t1".to_string(),
            vec!["table1".to_string()],
            vec![ScopeColumn::new(
                "id".to_string(),
                SqlType::Int64,
                false,
                "t1".to_string(),
                0,
            )],
        ));
        scope.add_table(ScopeTable::new(
            "t2".to_string(),
            vec!["table2".to_string()],
            vec![ScopeColumn::new(
                "id".to_string(),
                SqlType::Int64,
                false,
                "t2".to_string(),
                0,
            )],
        ));

        match scope.lookup_column("id") {
            ColumnLookupResult::Ambiguous(tables) => {
                assert_eq!(tables.len(), 2);
            }
            _ => panic!("Expected ambiguous result"),
        }
    }
}
