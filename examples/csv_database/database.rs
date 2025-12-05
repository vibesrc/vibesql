//! CSV database implementation.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use vibesql::{
    AnalyzedQuery, Analyzer, Catalog, ColumnSchema, ExprKind, JoinCondition, JoinType,
    MemoryCatalog, Parser, Query, QueryBody, Select, SelectItem, SqlType, StatementKind, TableRef,
    TableSchemaBuilder,
};

use crate::execution::{ExecutionRow, TableInfo};
use crate::result::{QueryResult, Row};

/// A simple CSV database.
pub struct CsvDatabase {
    /// Directory containing CSV files.
    data_dir: PathBuf,
    /// Catalog with table schemas.
    catalog: MemoryCatalog,
    /// Cached table data.
    tables: HashMap<String, Vec<Row>>,
}

impl CsvDatabase {
    /// Create a new CSV database from a directory.
    pub fn new(data_dir: impl AsRef<Path>) -> std::io::Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        let mut db = Self {
            data_dir,
            catalog: MemoryCatalog::new(),
            tables: HashMap::new(),
        };
        db.catalog.register_builtins();
        db.discover_tables()?;
        db.register_information_schema();
        Ok(db)
    }

    /// Check if a table exists.
    pub fn has_table(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }

    /// Register information_schema virtual tables.
    fn register_information_schema(&mut self) {
        // Create the information_schema schema
        let schema = self.catalog.add_schema("information_schema");

        // Register TABLES table
        let tables_schema = TableSchemaBuilder::new("tables")
            .column(ColumnSchema::new("table_catalog", SqlType::Varchar))
            .column(ColumnSchema::new("table_schema", SqlType::Varchar))
            .column(ColumnSchema::new("table_name", SqlType::Varchar))
            .column(ColumnSchema::new("table_type", SqlType::Varchar))
            .build();
        schema.tables.insert("tables".to_string(), tables_schema);

        // Register COLUMNS table
        let columns_schema = TableSchemaBuilder::new("columns")
            .column(ColumnSchema::new("table_catalog", SqlType::Varchar))
            .column(ColumnSchema::new("table_schema", SqlType::Varchar))
            .column(ColumnSchema::new("table_name", SqlType::Varchar))
            .column(ColumnSchema::new("column_name", SqlType::Varchar))
            .column(ColumnSchema::new("ordinal_position", SqlType::Varchar))
            .column(ColumnSchema::new("data_type", SqlType::Varchar))
            .column(ColumnSchema::new("is_nullable", SqlType::Varchar))
            .build();
        schema.tables.insert("columns".to_string(), columns_schema);
    }

    /// Generate information_schema.tables data dynamically.
    fn get_information_schema_tables(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for table_name in self.tables.keys() {
            if table_name.starts_with("information_schema.") {
                continue;
            }
            rows.push(vec![
                "csvdb".to_string(),
                "default".to_string(),
                table_name.clone(),
                "BASE TABLE".to_string(),
            ]);
        }
        rows.sort_by(|a, b| a[2].cmp(&b[2]));
        rows
    }

    /// Generate information_schema.columns data dynamically.
    fn get_information_schema_columns(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for table_name in self.tables.keys() {
            if table_name.starts_with("information_schema.") {
                continue;
            }
            if let Ok(Some(schema)) = self.catalog.resolve_table(&[table_name.clone()]) {
                for (i, col) in schema.columns.iter().enumerate() {
                    rows.push(vec![
                        "csvdb".to_string(),
                        "default".to_string(),
                        table_name.clone(),
                        col.name.clone(),
                        (i + 1).to_string(),
                        format!("{:?}", col.data_type),
                        if col.nullable { "YES" } else { "NO" }.to_string(),
                    ]);
                }
            }
        }
        rows.sort_by(|a, b| (&a[2], &a[4]).cmp(&(&b[2], &b[4])));
        rows
    }

    /// Discover CSV files and register them as tables.
    fn discover_tables(&mut self) -> std::io::Result<()> {
        if !self.data_dir.exists() {
            std::fs::create_dir_all(&self.data_dir)?;
        }

        for entry in std::fs::read_dir(&self.data_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "csv").unwrap_or(false) {
                self.load_table(&path)?;
            }
        }
        Ok(())
    }

    /// Load a CSV file as a table.
    fn load_table(&mut self, path: &Path) -> std::io::Result<()> {
        let table_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let header = match lines.next() {
            Some(Ok(h)) => h,
            _ => return Ok(()),
        };

        let columns: Vec<&str> = header.split(',').map(|s| s.trim()).collect();

        let mut builder = TableSchemaBuilder::new(&table_name);
        for col_name in &columns {
            builder = builder.column(ColumnSchema::new(*col_name, SqlType::Varchar));
        }
        let schema = builder.build();
        self.catalog.add_table(schema);

        let mut rows = Vec::new();
        for line in lines {
            let line = line?;
            let values: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
            if values.len() == columns.len() {
                rows.push(values);
            }
        }

        self.tables.insert(table_name, rows);
        Ok(())
    }

    /// Execute a SQL query and return results.
    pub fn execute(&mut self, sql: &str) -> Result<QueryResult, String> {
        let mut parser = Parser::new(sql);
        let statements = parser.parse().map_err(|e| format!("Parse error: {}", e))?;

        if statements.is_empty() {
            return Err("No statements to execute".to_string());
        }

        let stmt = &statements[0];

        match &stmt.kind {
            StatementKind::Query(query) => self.execute_query(query),
            StatementKind::Insert(insert) => {
                let table_name = insert
                    .table
                    .parts
                    .last()
                    .map(|i| i.value.clone())
                    .unwrap_or_default();

                if let vibesql::InsertSource::Values(rows) = &insert.source {
                    for row in rows {
                        let values: Vec<String> =
                            row.iter().map(|expr| self.eval_literal(expr)).collect();

                        if let Some(table_data) = self.tables.get_mut(&table_name) {
                            table_data.push(values.clone());
                        }
                    }
                    self.save_table(&table_name)?;
                    Ok(QueryResult::new(
                        vec!["result".to_string()],
                        vec![vec!["Inserted".to_string()]],
                    ))
                } else {
                    Err("Only INSERT VALUES is supported".to_string())
                }
            }
            StatementKind::CreateTable(create) => {
                let table_name = create
                    .name
                    .parts
                    .last()
                    .map(|i| i.value.clone())
                    .unwrap_or_default();

                let mut builder = TableSchemaBuilder::new(&table_name);
                for col in &create.columns {
                    builder = builder.column(ColumnSchema::new(&col.name.value, SqlType::Varchar));
                }
                let schema = builder.build();
                self.catalog.add_table(schema);

                self.tables.insert(table_name.clone(), Vec::new());
                self.save_table(&table_name)?;

                Ok(QueryResult::new(
                    vec!["result".to_string()],
                    vec![vec![format!("Created table {}", table_name)]],
                ))
            }
            _ => Err("Unsupported statement type".to_string()),
        }
    }

    /// Execute a SELECT query.
    fn execute_query(&self, query: &Query) -> Result<QueryResult, String> {
        let mut analyzer = Analyzer::with_catalog(self.catalog.clone());
        let analyzed = analyzer
            .analyze_query_result(query)
            .map_err(|e| format!("Analysis error: {}", e))?;

        match &query.body {
            QueryBody::Select(select) => self.execute_select(select, query, &analyzed),
            _ => Err("Only simple SELECT is supported".to_string()),
        }
    }

    /// Execute a SELECT statement.
    fn execute_select(
        &self,
        select: &Select,
        query: &Query,
        analyzed: &AnalyzedQuery,
    ) -> Result<QueryResult, String> {
        let from = select.from.as_ref().ok_or("SELECT requires FROM clause")?;

        if from.tables.is_empty() {
            return Err("No tables in FROM clause".to_string());
        }

        let mut exec_rows = self.process_from_clause(&from.tables[0])?;

        if let Some(where_expr) = &select.where_clause {
            exec_rows.retain(|row| self.eval_where(where_expr, row));
        }

        let result_rows = if analyzed.has_aggregation {
            self.execute_aggregation(select, &exec_rows, analyzed)?
        } else {
            self.project_columns(select, &exec_rows, analyzed)?
        };

        let limited_rows = if let Some(limit) = &query.limit {
            if let Some(count) = &limit.count {
                let n = self.eval_int(count) as usize;
                result_rows.into_iter().take(n).collect()
            } else {
                result_rows
            }
        } else {
            result_rows
        };

        Ok(QueryResult::new(
            analyzed.columns.iter().map(|c| c.name.clone()).collect(),
            limited_rows,
        ))
    }

    /// Execute aggregation query.
    fn execute_aggregation(
        &self,
        select: &Select,
        exec_rows: &[ExecutionRow],
        analyzed: &AnalyzedQuery,
    ) -> Result<Vec<Row>, String> {
        let groups = self.group_rows(select, exec_rows);

        let mut result = Vec::new();

        for (_group_key, group_rows) in groups {
            let mut row = Vec::new();

            for (i, item) in select.projection.iter().enumerate() {
                match item {
                    SelectItem::Expr { expr, .. } => {
                        let val = self.eval_aggregate_expr(expr, &group_rows);
                        row.push(val);
                    }
                    _ => {
                        if let Some(col) = analyzed.columns.get(i) {
                            if let Some(first_row) = group_rows.first() {
                                let val = first_row.get(&col.name).cloned().unwrap_or_default();
                                row.push(val);
                            } else {
                                row.push(String::new());
                            }
                        }
                    }
                }
            }

            result.push(row);
        }

        if result.is_empty() && select.group_by.is_none() {
            let mut row = Vec::new();
            for item in &select.projection {
                match item {
                    SelectItem::Expr { expr, .. } => {
                        let val = self.eval_aggregate_expr(expr, &[]);
                        row.push(val);
                    }
                    _ => row.push(String::new()),
                }
            }
            result.push(row);
        }

        Ok(result)
    }

    /// Group rows by GROUP BY columns.
    fn group_rows(
        &self,
        select: &Select,
        exec_rows: &[ExecutionRow],
    ) -> Vec<(String, Vec<ExecutionRow>)> {
        let group_by_exprs: Vec<&vibesql::Expr> = if let Some(group_by) = &select.group_by {
            group_by
                .items
                .iter()
                .filter_map(|item| {
                    if let vibesql::GroupByItem::Expr(expr) = item {
                        Some(expr.as_ref())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        if group_by_exprs.is_empty() {
            return vec![("".to_string(), exec_rows.to_vec())];
        }

        let mut groups: HashMap<String, Vec<ExecutionRow>> = HashMap::new();

        for row in exec_rows {
            let key: Vec<String> = group_by_exprs
                .iter()
                .map(|expr| self.eval_expr_row(expr, row))
                .collect();
            let key_str = key.join("|");

            groups
                .entry(key_str)
                .or_insert_with(Vec::new)
                .push(row.clone());
        }

        groups.into_iter().collect()
    }

    /// Evaluate an expression that may contain aggregates.
    fn eval_aggregate_expr(&self, expr: &vibesql::Expr, rows: &[ExecutionRow]) -> String {
        match &expr.kind {
            ExprKind::Aggregate(agg) => {
                let func_name = agg
                    .function
                    .name
                    .parts
                    .last()
                    .map(|i| i.value.to_uppercase())
                    .unwrap_or_default();

                let arg_values: Vec<String> = if agg.function.args.is_empty() {
                    vec![]
                } else {
                    rows.iter()
                        .filter_map(|row| {
                            if let Some(vibesql::FunctionArg::Unnamed(arg_expr)) =
                                agg.function.args.first()
                            {
                                Some(self.eval_expr_row(arg_expr, row))
                            } else {
                                None
                            }
                        })
                        .collect()
                };

                self.compute_aggregate(&func_name, &arg_values, rows.len())
            }
            ExprKind::Function(func) => {
                let func_name = func
                    .name
                    .parts
                    .last()
                    .map(|i| i.value.to_uppercase())
                    .unwrap_or_default();

                if matches!(func_name.as_str(), "COUNT" | "SUM" | "AVG" | "MIN" | "MAX") {
                    let arg_values: Vec<String> = rows
                        .iter()
                        .filter_map(|row| {
                            if let Some(vibesql::FunctionArg::Unnamed(arg_expr)) = func.args.first()
                            {
                                Some(self.eval_expr_row(arg_expr, row))
                            } else {
                                None
                            }
                        })
                        .collect();

                    self.compute_aggregate(&func_name, &arg_values, rows.len())
                } else {
                    rows.first()
                        .map(|row| self.eval_expr_row(expr, row))
                        .unwrap_or_default()
                }
            }
            ExprKind::Identifier(_) | ExprKind::CompoundIdentifier(_) => rows
                .first()
                .map(|row| self.eval_expr_row(expr, row))
                .unwrap_or_default(),
            ExprKind::BinaryOp { op, left, right } => {
                let left_val = self.eval_aggregate_expr(left, rows);
                let right_val = self.eval_aggregate_expr(right, rows);

                // Try numeric arithmetic
                if let (Ok(l), Ok(r)) = (left_val.parse::<f64>(), right_val.parse::<f64>()) {
                    let result = match op {
                        vibesql::BinaryOp::Plus => l + r,
                        vibesql::BinaryOp::Minus => l - r,
                        vibesql::BinaryOp::Multiply => l * r,
                        vibesql::BinaryOp::Divide => {
                            if r != 0.0 {
                                l / r
                            } else {
                                return "NULL".to_string();
                            }
                        }
                        vibesql::BinaryOp::Modulo => {
                            if r != 0.0 {
                                l % r
                            } else {
                                return "NULL".to_string();
                            }
                        }
                        _ => return format!("{} {} {}", left_val, op, right_val),
                    };
                    if result.fract() == 0.0 {
                        (result as i64).to_string()
                    } else {
                        format!("{:.2}", result)
                    }
                } else {
                    // Non-numeric, return concatenated for debugging
                    format!("{}{}{}", left_val, right_val, "")
                }
            }
            ExprKind::Integer(n) => n.to_string(),
            ExprKind::Float(f) => f.to_string(),
            _ => rows
                .first()
                .map(|row| self.eval_expr_row(expr, row))
                .unwrap_or_default(),
        }
    }

    /// Compute an aggregate function.
    fn compute_aggregate(&self, func_name: &str, values: &[String], row_count: usize) -> String {
        match func_name {
            "COUNT" => {
                if values.is_empty() {
                    row_count.to_string()
                } else {
                    values.iter().filter(|v| !v.is_empty()).count().to_string()
                }
            }
            "SUM" => {
                let sum: f64 = values.iter().filter_map(|v| v.parse::<f64>().ok()).sum();
                if sum.fract() == 0.0 {
                    (sum as i64).to_string()
                } else {
                    format!("{:.2}", sum)
                }
            }
            "AVG" => {
                let nums: Vec<f64> = values
                    .iter()
                    .filter_map(|v| v.parse::<f64>().ok())
                    .collect();
                if nums.is_empty() {
                    "NULL".to_string()
                } else {
                    let avg = nums.iter().sum::<f64>() / nums.len() as f64;
                    format!("{:.2}", avg)
                }
            }
            "MIN" => {
                let nums: Vec<f64> = values
                    .iter()
                    .filter_map(|v| v.parse::<f64>().ok())
                    .collect();
                if !nums.is_empty() {
                    let min = nums.iter().cloned().fold(f64::INFINITY, f64::min);
                    if min.fract() == 0.0 {
                        (min as i64).to_string()
                    } else {
                        format!("{:.2}", min)
                    }
                } else {
                    values
                        .iter()
                        .filter(|v| !v.is_empty())
                        .min()
                        .cloned()
                        .unwrap_or_else(|| "NULL".to_string())
                }
            }
            "MAX" => {
                let nums: Vec<f64> = values
                    .iter()
                    .filter_map(|v| v.parse::<f64>().ok())
                    .collect();
                if !nums.is_empty() {
                    let max = nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    if max.fract() == 0.0 {
                        (max as i64).to_string()
                    } else {
                        format!("{:.2}", max)
                    }
                } else {
                    values
                        .iter()
                        .filter(|v| !v.is_empty())
                        .max()
                        .cloned()
                        .unwrap_or_else(|| "NULL".to_string())
                }
            }
            _ => "NULL".to_string(),
        }
    }

    /// Process FROM clause and return execution rows.
    fn process_from_clause(&self, table_ref: &TableRef) -> Result<Vec<ExecutionRow>, String> {
        match table_ref {
            TableRef::Table { name, alias, .. } => {
                let table_name = name
                    .parts
                    .iter()
                    .map(|i| i.value.clone())
                    .collect::<Vec<_>>()
                    .join(".");

                let table_alias =
                    alias
                        .as_ref()
                        .map(|a| a.name.value.clone())
                        .unwrap_or_else(|| {
                            name.parts
                                .last()
                                .map(|i| i.value.clone())
                                .unwrap_or_default()
                        });

                let table_info = self.get_table_info(&table_name, &table_alias)?;

                let mut exec_rows = Vec::new();
                for row in &table_info.rows {
                    let mut exec_row = ExecutionRow::new();
                    exec_row.add_table(&table_info.alias, &table_info.columns, row);
                    exec_rows.push(exec_row);
                }
                Ok(exec_rows)
            }
            TableRef::Join {
                left,
                right,
                join_type,
                condition,
            } => {
                let left_rows = self.process_from_clause(left)?;
                let right_rows = self.process_from_clause(right)?;

                self.execute_join(&left_rows, &right_rows, join_type, condition)
            }
            _ => Err("Unsupported table reference type".to_string()),
        }
    }

    /// Get table info for execution.
    fn get_table_info(&self, table_name: &str, alias: &str) -> Result<TableInfo, String> {
        let normalized_name = table_name.to_lowercase();

        if normalized_name == "information_schema.tables" {
            let columns = vec![
                "table_catalog".to_string(),
                "table_schema".to_string(),
                "table_name".to_string(),
                "table_type".to_string(),
            ];
            return Ok(TableInfo {
                alias: alias.to_string(),
                columns,
                rows: self.get_information_schema_tables(),
            });
        } else if normalized_name == "information_schema.columns" {
            let columns = vec![
                "table_catalog".to_string(),
                "table_schema".to_string(),
                "table_name".to_string(),
                "column_name".to_string(),
                "ordinal_position".to_string(),
                "data_type".to_string(),
                "is_nullable".to_string(),
            ];
            return Ok(TableInfo {
                alias: alias.to_string(),
                columns,
                rows: self.get_information_schema_columns(),
            });
        }

        let schema = self
            .catalog
            .resolve_table(&[table_name.to_string()])
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Table '{}' not found", table_name))?;

        let rows = self
            .tables
            .get(table_name)
            .cloned()
            .ok_or_else(|| format!("Data for '{}' not found", table_name))?;

        let columns: Vec<String> = schema.columns.iter().map(|c| c.name.clone()).collect();

        Ok(TableInfo {
            alias: alias.to_string(),
            columns,
            rows,
        })
    }

    /// Execute a JOIN operation.
    fn execute_join(
        &self,
        left_rows: &[ExecutionRow],
        right_rows: &[ExecutionRow],
        join_type: &JoinType,
        condition: &Option<JoinCondition>,
    ) -> Result<Vec<ExecutionRow>, String> {
        let mut result = Vec::new();

        match join_type {
            JoinType::Inner | JoinType::Natural => {
                for left in left_rows {
                    for right in right_rows {
                        let combined = self.combine_rows(left, right);
                        if self.check_join_condition(&combined, condition) {
                            result.push(combined);
                        }
                    }
                }
            }
            JoinType::Left | JoinType::LeftSemi | JoinType::LeftAnti => {
                for left in left_rows {
                    let mut found_match = false;
                    for right in right_rows {
                        let combined = self.combine_rows(left, right);
                        if self.check_join_condition(&combined, condition) {
                            result.push(combined);
                            found_match = true;
                        }
                    }
                    if !found_match {
                        let combined = self.combine_rows_with_nulls(left, right_rows.first());
                        result.push(combined);
                    }
                }
            }
            JoinType::Right | JoinType::RightSemi | JoinType::RightAnti => {
                for right in right_rows {
                    let mut found_match = false;
                    for left in left_rows {
                        let combined = self.combine_rows(left, right);
                        if self.check_join_condition(&combined, condition) {
                            result.push(combined);
                            found_match = true;
                        }
                    }
                    if !found_match {
                        let combined = self.combine_rows_with_nulls_left(left_rows.first(), right);
                        result.push(combined);
                    }
                }
            }
            JoinType::Full => {
                let mut right_matched: Vec<bool> = vec![false; right_rows.len()];

                for left in left_rows {
                    let mut found_match = false;
                    for (i, right) in right_rows.iter().enumerate() {
                        let combined = self.combine_rows(left, right);
                        if self.check_join_condition(&combined, condition) {
                            result.push(combined);
                            found_match = true;
                            right_matched[i] = true;
                        }
                    }
                    if !found_match {
                        let combined = self.combine_rows_with_nulls(left, right_rows.first());
                        result.push(combined);
                    }
                }

                for (i, right) in right_rows.iter().enumerate() {
                    if !right_matched[i] {
                        let combined = self.combine_rows_with_nulls_left(left_rows.first(), right);
                        result.push(combined);
                    }
                }
            }
            JoinType::Cross => {
                for left in left_rows {
                    for right in right_rows {
                        result.push(self.combine_rows(left, right));
                    }
                }
            }
        }

        Ok(result)
    }

    /// Combine two execution rows.
    fn combine_rows(&self, left: &ExecutionRow, right: &ExecutionRow) -> ExecutionRow {
        let mut combined = ExecutionRow::new();
        combined.values = left.values.clone();
        combined.col_map = left.col_map.clone();

        let offset = combined.values.len();
        for (key, &idx) in &right.col_map {
            combined.col_map.insert(key.clone(), idx + offset);
        }
        combined.values.extend(right.values.iter().cloned());

        combined
    }

    /// Combine left row with NULLs for right side.
    fn combine_rows_with_nulls(
        &self,
        left: &ExecutionRow,
        right_template: Option<&ExecutionRow>,
    ) -> ExecutionRow {
        let mut combined = ExecutionRow::new();
        combined.values = left.values.clone();
        combined.col_map = left.col_map.clone();

        if let Some(right) = right_template {
            let offset = combined.values.len();
            for (key, &idx) in &right.col_map {
                combined.col_map.insert(key.clone(), idx + offset);
            }
            combined
                .values
                .extend(std::iter::repeat(String::new()).take(right.values.len()));
        }

        combined
    }

    /// Combine NULLs for left side with right row.
    fn combine_rows_with_nulls_left(
        &self,
        left_template: Option<&ExecutionRow>,
        right: &ExecutionRow,
    ) -> ExecutionRow {
        let mut combined = ExecutionRow::new();

        if let Some(left) = left_template {
            combined.col_map = left.col_map.clone();
            combined
                .values
                .extend(std::iter::repeat(String::new()).take(left.values.len()));
        }

        let offset = combined.values.len();
        for (key, &idx) in &right.col_map {
            combined.col_map.insert(key.clone(), idx + offset);
        }
        combined.values.extend(right.values.iter().cloned());

        combined
    }

    /// Check if a join condition is satisfied.
    fn check_join_condition(&self, row: &ExecutionRow, condition: &Option<JoinCondition>) -> bool {
        match condition {
            Some(JoinCondition::On(expr)) => self.eval_where(expr, row),
            Some(JoinCondition::Using(_)) => true,
            None => true,
        }
    }

    /// Project columns from execution rows.
    fn project_columns(
        &self,
        select: &Select,
        exec_rows: &[ExecutionRow],
        analyzed: &AnalyzedQuery,
    ) -> Result<Vec<Row>, String> {
        let mut result = Vec::new();

        for exec_row in exec_rows {
            let mut row = Vec::new();

            for (i, item) in select.projection.iter().enumerate() {
                match item {
                    SelectItem::Wildcard => {
                        row.extend(exec_row.values.iter().cloned());
                    }
                    SelectItem::QualifiedWildcard { qualifier } => {
                        let table_prefix = qualifier
                            .parts
                            .last()
                            .map(|i| i.value.to_lowercase())
                            .unwrap_or_default();

                        for (key, &idx) in &exec_row.col_map {
                            if key.starts_with(&format!("{}.", table_prefix)) {
                                if let Some(val) = exec_row.values.get(idx) {
                                    row.push(val.clone());
                                }
                            }
                        }
                    }
                    SelectItem::Expr { expr, .. } => {
                        let val = self.eval_expr_row(expr, exec_row);
                        row.push(val);
                    }
                    _ => {
                        if let Some(col) = analyzed.columns.get(i) {
                            let val = exec_row.get(&col.name).cloned().unwrap_or_default();
                            row.push(val);
                        }
                    }
                }
            }

            result.push(row);
        }

        Ok(result)
    }

    /// Evaluate a WHERE clause expression.
    fn eval_where(&self, expr: &vibesql::Expr, row: &ExecutionRow) -> bool {
        match &expr.kind {
            ExprKind::BinaryOp { op, left, right } => match op {
                vibesql::BinaryOp::And => self.eval_where(left, row) && self.eval_where(right, row),
                vibesql::BinaryOp::Or => self.eval_where(left, row) || self.eval_where(right, row),
                _ => {
                    let left_val = self.eval_expr_row(left, row);
                    let right_val = self.eval_expr_row(right, row);

                    match op {
                        vibesql::BinaryOp::Eq => left_val == right_val,
                        vibesql::BinaryOp::NotEq => left_val != right_val,
                        vibesql::BinaryOp::Lt => {
                            self.compare_values(&left_val, &right_val) == std::cmp::Ordering::Less
                        }
                        vibesql::BinaryOp::LtEq => {
                            self.compare_values(&left_val, &right_val)
                                != std::cmp::Ordering::Greater
                        }
                        vibesql::BinaryOp::Gt => {
                            self.compare_values(&left_val, &right_val)
                                == std::cmp::Ordering::Greater
                        }
                        vibesql::BinaryOp::GtEq => {
                            self.compare_values(&left_val, &right_val) != std::cmp::Ordering::Less
                        }
                        _ => false,
                    }
                }
            },
            ExprKind::Boolean(b) => *b,
            _ => true,
        }
    }

    /// Evaluate an expression against an execution row.
    fn eval_expr_row(&self, expr: &vibesql::Expr, row: &ExecutionRow) -> String {
        match &expr.kind {
            ExprKind::Identifier(ident) => row.get(&ident.value).cloned().unwrap_or_default(),
            ExprKind::CompoundIdentifier(parts) => {
                if parts.len() >= 2 {
                    let key = format!(
                        "{}.{}",
                        parts[parts.len() - 2].value,
                        parts[parts.len() - 1].value
                    );
                    row.get(&key).cloned().unwrap_or_default()
                } else if parts.len() == 1 {
                    row.get(&parts[0].value).cloned().unwrap_or_default()
                } else {
                    String::new()
                }
            }
            ExprKind::String(s) => s.clone(),
            ExprKind::Integer(n) => n.to_string(),
            ExprKind::Float(f) => f.to_string(),
            ExprKind::Boolean(b) => b.to_string(),
            ExprKind::Null => String::new(),
            _ => String::new(),
        }
    }

    /// Compare two string values (try numeric comparison first).
    fn compare_values(&self, a: &str, b: &str) -> std::cmp::Ordering {
        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
            a_num
                .partial_cmp(&b_num)
                .unwrap_or(std::cmp::Ordering::Equal)
        } else {
            a.cmp(b)
        }
    }

    /// Evaluate a literal expression.
    fn eval_literal(&self, expr: &vibesql::Expr) -> String {
        match &expr.kind {
            ExprKind::String(s) => s.clone(),
            ExprKind::Integer(n) => n.to_string(),
            ExprKind::Float(f) => f.to_string(),
            ExprKind::Boolean(b) => b.to_string(),
            ExprKind::Null => String::new(),
            _ => String::new(),
        }
    }

    /// Evaluate an integer expression.
    fn eval_int(&self, expr: &vibesql::Expr) -> i64 {
        match &expr.kind {
            ExprKind::Integer(n) => *n,
            _ => 0,
        }
    }

    /// Save a table to CSV.
    fn save_table(&self, table_name: &str) -> Result<(), String> {
        let schema = self
            .catalog
            .resolve_table(&[table_name.to_string()])
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Schema for '{}' not found", table_name))?;

        let path = self.data_dir.join(format!("{}.csv", table_name));
        let mut file = File::create(&path).map_err(|e| e.to_string())?;

        let header: Vec<&str> = schema.columns.iter().map(|c| c.name.as_str()).collect();
        writeln!(file, "{}", header.join(",")).map_err(|e| e.to_string())?;

        if let Some(rows) = self.tables.get(table_name) {
            for row in rows {
                writeln!(file, "{}", row.join(",")).map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}
