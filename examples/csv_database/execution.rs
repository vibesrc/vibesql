//! Query execution utilities.

use std::collections::HashMap;

use crate::result::Row;

/// A combined row from potentially multiple tables with column metadata.
#[derive(Clone)]
pub struct ExecutionRow {
    /// Values in order.
    pub values: Vec<String>,
    /// Map from "table.column" -> index and "column" -> index.
    pub col_map: HashMap<String, usize>,
}

impl ExecutionRow {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            col_map: HashMap::new(),
        }
    }

    pub fn add_table(&mut self, alias: &str, columns: &[String], values: &[String]) {
        let start_idx = self.values.len();
        for (i, col) in columns.iter().enumerate() {
            let qualified = format!("{}.{}", alias.to_lowercase(), col.to_lowercase());
            let unqualified = col.to_lowercase();
            self.col_map.insert(qualified, start_idx + i);
            // Only add unqualified if not already present (avoid ambiguity)
            self.col_map.entry(unqualified).or_insert(start_idx + i);
        }
        self.values.extend(values.iter().cloned());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.col_map
            .get(&key.to_lowercase())
            .and_then(|&i| self.values.get(i))
    }
}

/// Table info for execution.
pub struct TableInfo {
    pub alias: String,
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
}
