//! Semantic analyzer for SQL statements.
//!
//! This module provides semantic analysis for parsed SQL AST,
//! including type checking, name resolution, and validation.

mod error;
mod scope;
mod type_checker;

pub use error::{AnalyzerError, AnalyzerErrorKind};
pub use scope::{ColumnLookupResult, CteRef, Scope, ScopeColumn, ScopeTable};
pub use type_checker::{TypeChecker, TypedExpr};

use crate::ast::*;
use crate::catalog::{Catalog, ColumnSchema, MemoryCatalog, TableSchema};
use crate::error::{Error, Result};
use crate::types::SqlType;

/// Semantic analyzer for SQL statements.
pub struct Analyzer<C: Catalog = MemoryCatalog> {
    /// The catalog for resolving tables and functions.
    catalog: C,
    /// Scope stack.
    scopes: Vec<Scope>,
    /// Accumulated errors (for error recovery).
    errors: Vec<AnalyzerError>,
}

/// Analysis result for a query.
#[derive(Debug, Clone)]
pub struct AnalyzedQuery {
    /// The output columns.
    pub columns: Vec<OutputColumn>,
    /// Whether the query has aggregation.
    pub has_aggregation: bool,
    /// Whether the query uses window functions.
    pub has_window_functions: bool,
}

/// An output column from a query.
#[derive(Debug, Clone)]
pub struct OutputColumn {
    /// Column name (or alias).
    pub name: String,
    /// Data type.
    pub data_type: SqlType,
    /// Whether the column is nullable.
    pub nullable: bool,
}

impl<C: Catalog> Analyzer<C> {
    /// Create a new analyzer with the given catalog.
    pub fn with_catalog(catalog: C) -> Self {
        Self {
            catalog,
            scopes: vec![Scope::new()],
            errors: Vec::new(),
        }
    }

    /// Get the catalog.
    pub fn catalog(&self) -> &C {
        &self.catalog
    }

    /// Analyze a statement.
    pub fn analyze(&mut self, stmt: &Statement) -> Result<()> {
        self.errors.clear();
        self.analyze_statement(stmt)
            .map_err(|e| Error::analyzer(e.to_string()))
    }

    /// Analyze a query and return column information.
    pub fn analyze_query_result(&mut self, query: &Query) -> Result<AnalyzedQuery> {
        self.errors.clear();
        self.analyze_query_internal(query)
            .map_err(|e| Error::analyzer(e.to_string()))
    }

    /// Get any accumulated errors.
    pub fn errors(&self) -> &[AnalyzerError] {
        &self.errors
    }

    /// Analyze a statement.
    fn analyze_statement(&mut self, stmt: &Statement) -> std::result::Result<(), AnalyzerError> {
        match &stmt.kind {
            StatementKind::Query(query) => {
                self.analyze_query_internal(query)?;
                Ok(())
            }
            StatementKind::Insert(insert) => self.analyze_insert(insert),
            StatementKind::Update(update) => self.analyze_update(update),
            StatementKind::Delete(delete) => self.analyze_delete(delete),
            StatementKind::Merge(merge) => self.analyze_merge(merge),
            StatementKind::CreateTable(create) => self.analyze_create_table(create),
            StatementKind::CreateView(create) => self.analyze_create_view(create),
            _ => Ok(()), // Other statements don't need deep analysis
        }
    }

    /// Analyze a query.
    fn analyze_query_internal(
        &mut self,
        query: &Query,
    ) -> std::result::Result<AnalyzedQuery, AnalyzerError> {
        // Process WITH clause first (CTEs)
        if let Some(with) = &query.with {
            self.analyze_with_clause(with)?;
        }

        // Analyze the main query body
        let result = self.analyze_query_body(&query.body)?;

        // Analyze ORDER BY
        for order_item in &query.order_by {
            self.analyze_expr(&order_item.expr)?;
        }

        // Analyze LIMIT/OFFSET
        if let Some(limit) = &query.limit {
            if let Some(count) = &limit.count {
                self.analyze_expr_expect_int(count)?;
            }
            if let Some(offset) = &limit.offset {
                self.analyze_expr_expect_int(offset)?;
            }
        }

        Ok(result)
    }

    /// Analyze a WITH clause.
    fn analyze_with_clause(&mut self, with: &WithClause) -> std::result::Result<(), AnalyzerError> {
        for cte in &with.ctes {
            // Check for duplicate CTE names
            if self.current_scope().has_cte(&cte.name.value) {
                return Err(AnalyzerError::new(AnalyzerErrorKind::DuplicateCte {
                    name: cte.name.value.clone(),
                }));
            }

            // Analyze the CTE query
            let cte_result = self.analyze_query_internal(&cte.query)?;

            // Add CTE to scope
            let columns: Vec<ScopeColumn> = cte_result
                .columns
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    ScopeColumn::new(
                        col.name.clone(),
                        col.data_type.clone(),
                        col.nullable,
                        cte.name.value.clone(),
                        i,
                    )
                })
                .collect();

            self.current_scope_mut().add_cte(CteRef {
                name: cte.name.value.clone(),
                columns,
                is_recursive: with.recursive,
            });
        }
        Ok(())
    }

    /// Analyze a query body (SELECT, UNION, etc.).
    fn analyze_query_body(
        &mut self,
        body: &QueryBody,
    ) -> std::result::Result<AnalyzedQuery, AnalyzerError> {
        match body {
            QueryBody::Select(select) => self.analyze_select(select),
            QueryBody::SetOperation { left, right, .. } => {
                let left_result = self.analyze_query_body(left)?;
                let right_result = self.analyze_query_body(right)?;

                // Check column count matches
                if left_result.columns.len() != right_result.columns.len() {
                    return Err(AnalyzerError::set_operation_column_mismatch(
                        left_result.columns.len(),
                        right_result.columns.len(),
                    ));
                }

                // Result uses left side column names
                Ok(left_result)
            }
            QueryBody::Parenthesized(query) => self.analyze_query_internal(query),
        }
    }

    /// Analyze a SELECT statement.
    fn analyze_select(
        &mut self,
        select: &Select,
    ) -> std::result::Result<AnalyzedQuery, AnalyzerError> {
        self.push_scope();

        // First, analyze FROM clause to populate scope with tables
        if let Some(from) = &select.from {
            for table_ref in &from.tables {
                self.analyze_table_ref(table_ref)?;
            }
        }

        // Check for GROUP BY
        let has_group_by = select.group_by.is_some();
        self.current_scope_mut().has_group_by = has_group_by;

        if let Some(group_by) = &select.group_by {
            for item in &group_by.items {
                if let GroupByItem::Expr(expr) = item {
                    if let ExprKind::Identifier(ident) = &expr.kind {
                        self.current_scope_mut()
                            .group_by_columns
                            .push(ident.value.clone());
                    }
                }
            }
        }

        // Analyze WHERE clause
        if let Some(where_clause) = &select.where_clause {
            self.analyze_expr_expect_bool(where_clause)?;
        }

        // Analyze SELECT items
        let mut columns = Vec::new();
        let mut has_aggregation = false;
        let mut has_window_functions = false;

        for item in &select.projection {
            match item {
                SelectItem::Expr { expr, alias } => {
                    let typed = self.analyze_expr(expr)?;
                    has_aggregation = has_aggregation || typed.contains_aggregate;
                    has_window_functions = has_window_functions || typed.contains_window;

                    let name = alias
                        .as_ref()
                        .map(|a| a.value.clone())
                        .or_else(|| self.expr_to_name(expr))
                        .unwrap_or_else(|| format!("_col{}", columns.len()));

                    columns.push(OutputColumn {
                        name,
                        data_type: typed.data_type,
                        nullable: typed.nullable,
                    });
                }
                SelectItem::Wildcard => {
                    // Expand * to all columns from all tables in scope
                    for table in self.current_scope().all_tables() {
                        for col in &table.columns {
                            columns.push(OutputColumn {
                                name: col.name.clone(),
                                data_type: col.data_type.clone(),
                                nullable: col.nullable,
                            });
                        }
                    }
                }
                SelectItem::QualifiedWildcard { qualifier } => {
                    // Expand table.* to all columns from that table
                    let table_name = qualifier
                        .parts
                        .last()
                        .map(|i| i.value.clone())
                        .unwrap_or_default();

                    if let Some(table) = self.current_scope().lookup_table(&table_name) {
                        for col in &table.columns {
                            columns.push(OutputColumn {
                                name: col.name.clone(),
                                data_type: col.data_type.clone(),
                                nullable: col.nullable,
                            });
                        }
                    } else {
                        return Err(AnalyzerError::table_not_found(&table_name));
                    }
                }
                SelectItem::WildcardExcept { qualifier, except } => {
                    let table_iter: Vec<_> = if let Some(q) = qualifier {
                        let table_name =
                            q.parts.last().map(|i| i.value.clone()).unwrap_or_default();
                        if let Some(table) = self.current_scope().lookup_table(&table_name) {
                            vec![table.clone()]
                        } else {
                            return Err(AnalyzerError::table_not_found(&table_name));
                        }
                    } else {
                        self.current_scope().all_tables().cloned().collect()
                    };

                    let except_names: Vec<String> =
                        except.iter().map(|i| i.value.to_lowercase()).collect();
                    for table in table_iter {
                        for col in &table.columns {
                            if !except_names.contains(&col.name.to_lowercase()) {
                                columns.push(OutputColumn {
                                    name: col.name.clone(),
                                    data_type: col.data_type.clone(),
                                    nullable: col.nullable,
                                });
                            }
                        }
                    }
                }
                SelectItem::WildcardReplace { qualifier, replace } => {
                    let table_iter: Vec<_> = if let Some(q) = qualifier {
                        let table_name =
                            q.parts.last().map(|i| i.value.clone()).unwrap_or_default();
                        if let Some(table) = self.current_scope().lookup_table(&table_name) {
                            vec![table.clone()]
                        } else {
                            return Err(AnalyzerError::table_not_found(&table_name));
                        }
                    } else {
                        self.current_scope().all_tables().cloned().collect()
                    };

                    let replace_map: std::collections::HashMap<String, &Expr> = replace
                        .iter()
                        .map(|(expr, ident)| (ident.value.to_lowercase(), expr.as_ref()))
                        .collect();

                    for table in table_iter {
                        for col in &table.columns {
                            let col_lower = col.name.to_lowercase();
                            if let Some(replace_expr) = replace_map.get(&col_lower) {
                                let typed = self.analyze_expr(replace_expr)?;
                                columns.push(OutputColumn {
                                    name: col.name.clone(),
                                    data_type: typed.data_type,
                                    nullable: typed.nullable,
                                });
                            } else {
                                columns.push(OutputColumn {
                                    name: col.name.clone(),
                                    data_type: col.data_type.clone(),
                                    nullable: col.nullable,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Analyze HAVING clause
        if let Some(having) = &select.having {
            if !has_group_by && !has_aggregation {
                return Err(AnalyzerError::new(AnalyzerErrorKind::HavingWithoutGroupBy));
            }
            self.analyze_expr_expect_bool(having)?;
        }

        self.pop_scope();

        Ok(AnalyzedQuery {
            columns,
            has_aggregation,
            has_window_functions,
        })
    }

    /// Analyze a table reference in FROM clause.
    fn analyze_table_ref(
        &mut self,
        table_ref: &TableRef,
    ) -> std::result::Result<(), AnalyzerError> {
        match table_ref {
            TableRef::Table { name, alias, .. } => {
                let name_parts: Vec<String> = name.parts.iter().map(|i| i.value.clone()).collect();

                // First check if it's a CTE (search all parent scopes)
                let cte_name = name_parts.last().cloned().unwrap_or_default();
                if let Some(cte) = self.lookup_cte(&cte_name) {
                    let table_alias = alias
                        .as_ref()
                        .map(|a| a.name.value.clone())
                        .unwrap_or_else(|| cte_name.clone());

                    let columns: Vec<ScopeColumn> = cte
                        .columns
                        .iter()
                        .map(|c| {
                            ScopeColumn::new(
                                c.name.clone(),
                                c.data_type.clone(),
                                c.nullable,
                                table_alias.clone(),
                                c.column_index,
                            )
                        })
                        .collect();

                    self.current_scope_mut().add_table(ScopeTable::new(
                        table_alias,
                        name_parts,
                        columns,
                    ));
                    return Ok(());
                }

                // Look up table in catalog
                let table_schema = self
                    .catalog
                    .resolve_table(&name_parts)
                    .map_err(|_| AnalyzerError::table_not_found(&cte_name))?
                    .ok_or_else(|| AnalyzerError::table_not_found(&cte_name))?;

                let table_alias = alias
                    .as_ref()
                    .map(|a| a.name.value.clone())
                    .unwrap_or_else(|| table_schema.name.clone());

                let columns = self.table_schema_to_columns(&table_schema, &table_alias);
                self.current_scope_mut().add_table(ScopeTable::new(
                    table_alias,
                    name_parts,
                    columns,
                ));
            }
            TableRef::Subquery { query, alias } => {
                let result = self.analyze_query_internal(query)?;

                let alias_name = alias
                    .as_ref()
                    .map(|a| a.name.value.clone())
                    .unwrap_or_else(|| "_subquery".to_string());

                let columns: Vec<ScopeColumn> = result
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(i, col)| {
                        ScopeColumn::new(
                            col.name.clone(),
                            col.data_type.clone(),
                            col.nullable,
                            alias_name.clone(),
                            i,
                        )
                    })
                    .collect();

                self.current_scope_mut().add_table(ScopeTable::new(
                    alias_name,
                    vec!["_subquery".to_string()],
                    columns,
                ));
            }
            TableRef::Join {
                left,
                right,
                condition,
                ..
            } => {
                self.analyze_table_ref(left)?;
                self.analyze_table_ref(right)?;

                if let Some(JoinCondition::On(expr)) = condition {
                    self.analyze_expr_expect_bool(expr)?;
                }
            }
            TableRef::Unnest { expr, alias, .. } => {
                let typed = self.analyze_expr(expr)?;

                let elem_type = match &typed.data_type {
                    SqlType::Array(elem) => (**elem).clone(),
                    _ => SqlType::Unknown,
                };

                let alias_name = alias
                    .as_ref()
                    .map(|a| a.name.value.clone())
                    .unwrap_or_else(|| "_unnest".to_string());

                let columns = vec![ScopeColumn::new(
                    "value".to_string(),
                    elem_type,
                    true,
                    alias_name.clone(),
                    0,
                )];

                self.current_scope_mut().add_table(ScopeTable::new(
                    alias_name,
                    vec!["_unnest".to_string()],
                    columns,
                ));
            }
            TableRef::Parenthesized(inner) => {
                self.analyze_table_ref(inner)?;
            }
            TableRef::TableFunction { .. } => {
                // Table functions would need special handling
            }
        }
        Ok(())
    }

    /// Analyze an INSERT statement.
    fn analyze_insert(
        &mut self,
        insert: &InsertStatement,
    ) -> std::result::Result<(), AnalyzerError> {
        let name_parts: Vec<String> = insert.table.parts.iter().map(|i| i.value.clone()).collect();
        let table_name = name_parts.last().cloned().unwrap_or_default();

        // Verify table exists
        let table_schema = self
            .catalog
            .resolve_table(&name_parts)
            .map_err(|_| AnalyzerError::table_not_found(&table_name))?
            .ok_or_else(|| AnalyzerError::table_not_found(&table_name))?;

        // Verify columns if specified
        for col in &insert.columns {
            if table_schema.get_column(&col.value).is_none() {
                return Err(AnalyzerError::column_not_found(
                    &col.value,
                    Some(table_name.clone()),
                ));
            }
        }

        // Analyze the source
        match &insert.source {
            InsertSource::Values(rows) => {
                for row in rows {
                    for expr in row {
                        self.analyze_expr(expr)?;
                    }
                }
            }
            InsertSource::Query(query) => {
                self.analyze_query_internal(query)?;
            }
            InsertSource::DefaultValues => {}
        }

        Ok(())
    }

    /// Analyze an UPDATE statement.
    fn analyze_update(
        &mut self,
        update: &UpdateStatement,
    ) -> std::result::Result<(), AnalyzerError> {
        self.push_scope();

        // Add target table to scope - need to extract name from TableRef
        let (name_parts, table_name, alias_opt) = self.extract_table_info(&update.table)?;

        let table_schema = self
            .catalog
            .resolve_table(&name_parts)
            .map_err(|_| AnalyzerError::table_not_found(&table_name))?
            .ok_or_else(|| AnalyzerError::table_not_found(&table_name))?;

        let alias = alias_opt.unwrap_or_else(|| table_name.clone());

        let columns = self.table_schema_to_columns(&table_schema, &alias);
        self.current_scope_mut()
            .add_table(ScopeTable::new(alias.clone(), name_parts, columns));

        // Analyze assignments
        for assignment in &update.assignments {
            match &assignment.target {
                AssignmentTarget::Column(col) => {
                    if table_schema.get_column(&col.value).is_none() {
                        return Err(AnalyzerError::column_not_found(
                            &col.value,
                            Some(table_name.clone()),
                        ));
                    }
                }
                AssignmentTarget::Path(_) => {}
            }
            self.analyze_expr(&assignment.value)?;
        }

        // Analyze WHERE clause
        if let Some(where_clause) = &update.where_clause {
            self.analyze_expr_expect_bool(where_clause)?;
        }

        self.pop_scope();
        Ok(())
    }

    /// Extract table name information from a TableRef.
    fn extract_table_info(
        &self,
        table_ref: &TableRef,
    ) -> std::result::Result<(Vec<String>, String, Option<String>), AnalyzerError> {
        match table_ref {
            TableRef::Table { name, alias, .. } => {
                let name_parts: Vec<String> = name.parts.iter().map(|i| i.value.clone()).collect();
                let table_name = name_parts.last().cloned().unwrap_or_default();
                let alias_name = alias.as_ref().map(|a| a.name.value.clone());
                Ok((name_parts, table_name, alias_name))
            }
            _ => Err(AnalyzerError::new(AnalyzerErrorKind::Other {
                message: "Expected table reference".to_string(),
            })),
        }
    }

    /// Analyze a DELETE statement.
    fn analyze_delete(
        &mut self,
        delete: &DeleteStatement,
    ) -> std::result::Result<(), AnalyzerError> {
        self.push_scope();

        let name_parts: Vec<String> = delete.table.parts.iter().map(|i| i.value.clone()).collect();
        let table_name = name_parts.last().cloned().unwrap_or_default();

        let table_schema = self
            .catalog
            .resolve_table(&name_parts)
            .map_err(|_| AnalyzerError::table_not_found(&table_name))?
            .ok_or_else(|| AnalyzerError::table_not_found(&table_name))?;

        let alias = delete
            .alias
            .as_ref()
            .map(|a| a.name.value.clone())
            .unwrap_or_else(|| table_name.clone());

        let columns = self.table_schema_to_columns(&table_schema, &alias);
        self.current_scope_mut()
            .add_table(ScopeTable::new(alias, name_parts, columns));

        // Analyze WHERE clause
        if let Some(where_clause) = &delete.where_clause {
            self.analyze_expr_expect_bool(where_clause)?;
        }

        self.pop_scope();
        Ok(())
    }

    /// Analyze a MERGE statement.
    fn analyze_merge(&mut self, merge: &MergeStatement) -> std::result::Result<(), AnalyzerError> {
        self.push_scope();

        // Analyze target table
        self.analyze_table_ref(&merge.target)?;

        // Analyze source table
        self.analyze_table_ref(&merge.source)?;

        // Analyze ON condition
        self.analyze_expr_expect_bool(&merge.on)?;

        // Analyze WHEN clauses
        for clause in &merge.clauses {
            match clause {
                MergeClause::Matched { condition, action } => {
                    if let Some(cond) = condition {
                        self.analyze_expr_expect_bool(cond)?;
                    }
                    match action {
                        MergeMatchedAction::Update { assignments } => {
                            for assignment in assignments {
                                self.analyze_expr(&assignment.value)?;
                            }
                        }
                        MergeMatchedAction::Delete => {}
                    }
                }
                MergeClause::NotMatched { condition, action } => {
                    if let Some(cond) = condition {
                        self.analyze_expr_expect_bool(cond)?;
                    }
                    for expr in &action.values {
                        self.analyze_expr(expr)?;
                    }
                }
                MergeClause::NotMatchedBySource { condition, action } => {
                    if let Some(cond) = condition {
                        self.analyze_expr_expect_bool(cond)?;
                    }
                    match action {
                        MergeMatchedAction::Update { assignments } => {
                            for assignment in assignments {
                                self.analyze_expr(&assignment.value)?;
                            }
                        }
                        MergeMatchedAction::Delete => {}
                    }
                }
            }
        }

        self.pop_scope();
        Ok(())
    }

    /// Analyze a CREATE TABLE statement.
    fn analyze_create_table(
        &mut self,
        create: &CreateTableStatement,
    ) -> std::result::Result<(), AnalyzerError> {
        // Check that the table doesn't already exist (unless IF NOT EXISTS)
        if !create.if_not_exists {
            let name_parts: Vec<String> =
                create.name.parts.iter().map(|i| i.value.clone()).collect();
            if let Ok(Some(_)) = self.catalog.resolve_table(&name_parts) {
                return Err(AnalyzerError::new(AnalyzerErrorKind::Other {
                    message: format!("table '{}' already exists", create.name),
                }));
            }
        }

        // Validate column definitions
        for col in &create.columns {
            // Check for duplicate column names
            let count = create
                .columns
                .iter()
                .filter(|c| c.name.value.eq_ignore_ascii_case(&col.name.value))
                .count();
            if count > 1 {
                return Err(AnalyzerError::new(AnalyzerErrorKind::DuplicateAlias {
                    name: col.name.value.clone(),
                }));
            }
        }

        Ok(())
    }

    /// Analyze a CREATE VIEW statement.
    fn analyze_create_view(
        &mut self,
        create: &CreateViewStatement,
    ) -> std::result::Result<(), AnalyzerError> {
        // Analyze the view query
        self.analyze_query_internal(&create.query)?;
        Ok(())
    }

    // === Helper methods ===

    /// Analyze an expression and return its typed result.
    fn analyze_expr(&self, expr: &Expr) -> std::result::Result<TypedExpr, AnalyzerError> {
        let checker = TypeChecker::new(&self.catalog);
        checker.check_expr(expr, self.current_scope())
    }

    /// Analyze an expression and expect a boolean result.
    fn analyze_expr_expect_bool(&self, expr: &Expr) -> std::result::Result<(), AnalyzerError> {
        let typed = self.analyze_expr(expr)?;
        if typed.data_type != SqlType::Bool
            && typed.data_type != SqlType::Unknown
            && typed.data_type != SqlType::Any
        {
            Err(AnalyzerError::type_mismatch(
                SqlType::Bool,
                typed.data_type,
                "condition",
            ))
        } else {
            Ok(())
        }
    }

    /// Analyze an expression and expect an integer result.
    fn analyze_expr_expect_int(&self, expr: &Expr) -> std::result::Result<(), AnalyzerError> {
        let typed = self.analyze_expr(expr)?;
        if !typed.data_type.is_integer()
            && typed.data_type != SqlType::Unknown
            && typed.data_type != SqlType::Any
        {
            Err(AnalyzerError::type_mismatch(
                SqlType::Int64,
                typed.data_type,
                "LIMIT/OFFSET",
            ))
        } else {
            Ok(())
        }
    }

    /// Convert a table schema to column references.
    fn table_schema_to_columns(&self, schema: &TableSchema, alias: &str) -> Vec<ScopeColumn> {
        schema
            .columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                ScopeColumn::new(
                    col.name.clone(),
                    self.column_schema_to_sql_type(col),
                    col.nullable,
                    alias.to_string(),
                    i,
                )
            })
            .collect()
    }

    /// Convert a column schema to SqlType.
    fn column_schema_to_sql_type(&self, col: &ColumnSchema) -> SqlType {
        col.data_type.clone()
    }

    /// Try to derive a name from an expression.
    fn expr_to_name(&self, expr: &Expr) -> Option<String> {
        match &expr.kind {
            ExprKind::Identifier(ident) => Some(ident.value.clone()),
            ExprKind::CompoundIdentifier(parts) => parts.last().map(|i| i.value.clone()),
            ExprKind::Function(func) => func.name.parts.last().map(|i| i.value.clone()),
            ExprKind::Aggregate(agg) => agg.function.name.parts.last().map(|i| i.value.clone()),
            ExprKind::WindowFunction(wf) => wf.function.name.parts.last().map(|i| i.value.clone()),
            _ => None,
        }
    }

    /// Push a new scope.
    fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Pop the current scope.
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Get the current scope.
    fn current_scope(&self) -> &Scope {
        self.scopes.last().expect("No scope available")
    }

    /// Get the current scope mutably.
    fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("No scope available")
    }

    /// Look up a CTE in all scopes (current and parents).
    fn lookup_cte(&self, name: &str) -> Option<CteRef> {
        for scope in self.scopes.iter().rev() {
            if let Some(cte) = scope.lookup_cte(name) {
                return Some(cte.clone());
            }
        }
        None
    }
}

impl Default for Analyzer<MemoryCatalog> {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer<MemoryCatalog> {
    /// Create a new analyzer with an empty memory catalog.
    pub fn new() -> Self {
        let mut catalog = MemoryCatalog::new();
        catalog.register_builtins();
        Self::with_catalog(catalog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::TableSchemaBuilder;
    use crate::parser::Parser;

    fn setup_test_catalog() -> MemoryCatalog {
        let mut catalog = MemoryCatalog::new();
        catalog.register_builtins();

        // Add test tables
        catalog.add_table(
            TableSchemaBuilder::new("users")
                .column(ColumnSchema::new("id", SqlType::Int64).not_null())
                .column(ColumnSchema::new("name", SqlType::Varchar))
                .column(ColumnSchema::new("age", SqlType::Int64))
                .column(ColumnSchema::new("email", SqlType::Varchar))
                .build(),
        );

        catalog.add_table(
            TableSchemaBuilder::new("orders")
                .column(ColumnSchema::new("id", SqlType::Int64).not_null())
                .column(ColumnSchema::new("user_id", SqlType::Int64))
                .column(ColumnSchema::new("amount", SqlType::Float64))
                .column(ColumnSchema::new("created_at", SqlType::Timestamp))
                .build(),
        );

        catalog
    }

    fn parse_and_analyze(sql: &str, catalog: MemoryCatalog) -> Result<AnalyzedQuery> {
        let mut parser = Parser::new(sql);
        let stmts = parser.parse()?;
        let stmt = stmts
            .into_iter()
            .next()
            .expect("Expected at least one statement");

        if let StatementKind::Query(query) = stmt.kind {
            let mut analyzer = Analyzer::with_catalog(catalog);
            analyzer.analyze_query_result(&query)
        } else {
            panic!("Expected a query statement");
        }
    }

    #[test]
    fn test_simple_select() {
        let catalog = setup_test_catalog();
        let result = parse_and_analyze("SELECT id, name FROM users", catalog).unwrap();

        assert_eq!(result.columns.len(), 2);
        assert_eq!(result.columns[0].name, "id");
        assert_eq!(result.columns[0].data_type, SqlType::Int64);
        assert_eq!(result.columns[1].name, "name");
        assert_eq!(result.columns[1].data_type, SqlType::Varchar);
    }

    #[test]
    fn test_select_star() {
        let catalog = setup_test_catalog();
        let result = parse_and_analyze("SELECT * FROM users", catalog).unwrap();

        assert_eq!(result.columns.len(), 4);
    }

    #[test]
    fn test_select_with_alias() {
        let catalog = setup_test_catalog();
        let result =
            parse_and_analyze("SELECT id AS user_id, name AS username FROM users", catalog)
                .unwrap();

        assert_eq!(result.columns[0].name, "user_id");
        assert_eq!(result.columns[1].name, "username");
    }

    #[test]
    fn test_join() {
        let catalog = setup_test_catalog();
        let result = parse_and_analyze(
            "SELECT u.id, u.name, o.amount FROM users u JOIN orders o ON u.id = o.user_id",
            catalog,
        )
        .unwrap();

        assert_eq!(result.columns.len(), 3);
        assert_eq!(result.columns[2].data_type, SqlType::Float64);
    }

    #[test]
    fn test_aggregate() {
        let catalog = setup_test_catalog();
        let result = parse_and_analyze("SELECT COUNT(*), AVG(age) FROM users", catalog).unwrap();

        assert!(result.has_aggregation);
        assert_eq!(result.columns.len(), 2);
    }

    #[test]
    fn test_table_not_found() {
        let catalog = setup_test_catalog();
        let err = parse_and_analyze("SELECT * FROM nonexistent", catalog).unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_column_not_found() {
        let catalog = setup_test_catalog();
        let err = parse_and_analyze("SELECT nonexistent FROM users", catalog).unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_ambiguous_column() {
        let catalog = setup_test_catalog();
        let err = parse_and_analyze("SELECT id FROM users, orders", catalog).unwrap_err();
        assert!(err.to_string().contains("ambiguous"));
    }

    #[test]
    fn test_where_clause_type_check() {
        let catalog = setup_test_catalog();
        // Valid: boolean condition
        let result = parse_and_analyze("SELECT * FROM users WHERE age > 21", catalog);
        assert!(result.is_ok());
    }

    #[test]
    fn test_union() {
        let catalog = setup_test_catalog();
        let result = parse_and_analyze(
            "SELECT id, name FROM users UNION SELECT id, name FROM users",
            catalog,
        )
        .unwrap();

        assert_eq!(result.columns.len(), 2);
    }

    #[test]
    fn test_cte() {
        let catalog = setup_test_catalog();
        let result = parse_and_analyze(
            "WITH active_users AS (SELECT id, name FROM users WHERE age > 18) SELECT * FROM active_users",
            catalog
        ).unwrap();

        assert_eq!(result.columns.len(), 2);
    }
}
