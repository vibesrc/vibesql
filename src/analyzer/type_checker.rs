//! Type checking for SQL expressions.

use super::error::AnalyzerError;
use super::scope::{ColumnLookupResult, Scope};
use crate::ast::*;
use crate::catalog::Catalog;
use crate::types::SqlType;

/// Type checker for SQL expressions.
pub struct TypeChecker<'a, C: Catalog> {
    catalog: &'a C,
}

/// Result of type checking an expression.
#[derive(Debug, Clone)]
pub struct TypedExpr {
    /// The resolved type.
    pub data_type: SqlType,
    /// Whether the expression can be null.
    pub nullable: bool,
    /// Whether the expression contains an aggregate.
    pub contains_aggregate: bool,
    /// Whether the expression contains a window function.
    pub contains_window: bool,
}

impl TypedExpr {
    /// Create a non-nullable typed expression.
    pub fn non_null(data_type: SqlType) -> Self {
        Self {
            data_type,
            nullable: false,
            contains_aggregate: false,
            contains_window: false,
        }
    }

    /// Create a nullable typed expression.
    pub fn nullable(data_type: SqlType) -> Self {
        Self {
            data_type,
            nullable: true,
            contains_aggregate: false,
            contains_window: false,
        }
    }
}

impl<'a, C: Catalog> TypeChecker<'a, C> {
    /// Create a new type checker.
    pub fn new(catalog: &'a C) -> Self {
        Self { catalog }
    }

    /// Check the type of an expression.
    pub fn check_expr(&self, expr: &Expr, scope: &Scope) -> Result<TypedExpr, AnalyzerError> {
        match &expr.kind {
            // Literals
            ExprKind::Null => Ok(TypedExpr::nullable(SqlType::Unknown)),
            ExprKind::Boolean(_) => Ok(TypedExpr::non_null(SqlType::Bool)),
            ExprKind::Integer(_) => Ok(TypedExpr::non_null(SqlType::Int64)),
            ExprKind::Float(_) => Ok(TypedExpr::non_null(SqlType::Float64)),
            ExprKind::String(_) => Ok(TypedExpr::non_null(SqlType::Varchar)),
            ExprKind::Bytes(_) => Ok(TypedExpr::non_null(SqlType::Varbinary)),

            // Identifiers
            ExprKind::Identifier(ident) => self.check_column(&ident.value, None, scope),
            ExprKind::CompoundIdentifier(parts) => {
                if parts.len() == 2 {
                    self.check_column(&parts[1].value, Some(&parts[0].value), scope)
                } else if parts.len() == 1 {
                    self.check_column(&parts[0].value, None, scope)
                } else {
                    // For longer paths, use the last as column, second-to-last as table
                    let col = &parts[parts.len() - 1].value;
                    let table = &parts[parts.len() - 2].value;
                    self.check_column(col, Some(table), scope)
                }
            }

            // Operators
            ExprKind::BinaryOp { op, left, right } => self.check_binary_op(*op, left, right, scope),
            ExprKind::UnaryOp { op, expr } => self.check_unary_op(*op, expr, scope),

            // Comparisons
            ExprKind::Between {
                expr, low, high, ..
            } => {
                self.check_expr(expr, scope)?;
                self.check_expr(low, scope)?;
                self.check_expr(high, scope)?;
                Ok(TypedExpr::non_null(SqlType::Bool))
            }
            ExprKind::In { expr, list, .. } => {
                self.check_expr(expr, scope)?;
                match list {
                    InList::Values(values) => {
                        for v in values {
                            self.check_expr(v, scope)?;
                        }
                    }
                    InList::Subquery(_) => {}
                }
                Ok(TypedExpr::non_null(SqlType::Bool))
            }
            ExprKind::Like { expr, pattern, .. } => {
                self.check_expr(expr, scope)?;
                self.check_expr(pattern, scope)?;
                Ok(TypedExpr::non_null(SqlType::Bool))
            }
            ExprKind::IsExpr { .. } => Ok(TypedExpr::non_null(SqlType::Bool)),
            ExprKind::IsDistinct { .. } => Ok(TypedExpr::non_null(SqlType::Bool)),

            // Functions
            ExprKind::Function(func) => self.check_function(func, scope),
            ExprKind::Aggregate(agg) => self.check_aggregate(agg, scope),
            ExprKind::WindowFunction(wf) => self.check_window_function(wf, scope),

            // Type operations
            ExprKind::Cast { data_type, .. } => {
                let sql_type = self.data_type_to_sql_type(data_type);
                Ok(TypedExpr::nullable(sql_type))
            }
            ExprKind::Extract { .. } => Ok(TypedExpr::nullable(SqlType::Int64)),

            // Conditional
            ExprKind::Case {
                conditions,
                else_result,
                ..
            } => {
                let mut result_type = SqlType::Unknown;
                for (_, result) in conditions {
                    let typed = self.check_expr(result, scope)?;
                    if result_type == SqlType::Unknown {
                        result_type = typed.data_type;
                    } else if let Some(common) = result_type.common_supertype(&typed.data_type) {
                        result_type = common;
                    }
                }
                if let Some(else_expr) = else_result {
                    let typed = self.check_expr(else_expr, scope)?;
                    if let Some(common) = result_type.common_supertype(&typed.data_type) {
                        result_type = common;
                    }
                }
                Ok(TypedExpr::nullable(result_type))
            }
            ExprKind::If {
                then_expr,
                else_expr,
                ..
            } => {
                let then_typed = self.check_expr(then_expr, scope)?;
                let else_typed = self.check_expr(else_expr, scope)?;
                let result_type = then_typed
                    .data_type
                    .common_supertype(&else_typed.data_type)
                    .unwrap_or(SqlType::Unknown);
                Ok(TypedExpr::nullable(result_type))
            }
            ExprKind::Coalesce(exprs) => {
                let mut result_type = SqlType::Unknown;
                for expr in exprs {
                    let typed = self.check_expr(expr, scope)?;
                    if result_type == SqlType::Unknown {
                        result_type = typed.data_type;
                    } else if let Some(common) = result_type.common_supertype(&typed.data_type) {
                        result_type = common;
                    }
                }
                Ok(TypedExpr::nullable(result_type))
            }
            ExprKind::Nullif { left, .. } => {
                let typed = self.check_expr(left, scope)?;
                Ok(TypedExpr::nullable(typed.data_type))
            }
            ExprKind::IfNull { expr, .. } => {
                let typed = self.check_expr(expr, scope)?;
                Ok(TypedExpr::nullable(typed.data_type))
            }

            // Arrays and structs
            ExprKind::Array {
                elements,
                element_type,
            } => {
                let elem_type = if let Some(et) = element_type {
                    self.data_type_to_sql_type(et)
                } else if let Some(first) = elements.first() {
                    self.check_expr(first, scope)?.data_type
                } else {
                    SqlType::Unknown
                };
                Ok(TypedExpr::non_null(SqlType::Array(Box::new(elem_type))))
            }
            ExprKind::Struct { fields } => {
                let mut struct_fields = Vec::new();
                for field in fields {
                    let typed = self.check_expr(&field.value, scope)?;
                    struct_fields.push(crate::types::StructField {
                        name: field.name.as_ref().map(|i| i.value.clone()),
                        data_type: typed.data_type,
                    });
                }
                Ok(TypedExpr::non_null(SqlType::Struct(struct_fields)))
            }

            // Subqueries
            ExprKind::Subquery(_) => Ok(TypedExpr::nullable(SqlType::Unknown)),
            ExprKind::Exists { .. } => Ok(TypedExpr::non_null(SqlType::Bool)),
            ExprKind::InSubquery { .. } => Ok(TypedExpr::non_null(SqlType::Bool)),
            ExprKind::SubqueryOp { .. } => Ok(TypedExpr::non_null(SqlType::Bool)),

            // Other
            ExprKind::Parameter(_) => Ok(TypedExpr::nullable(SqlType::Unknown)),
            ExprKind::ArraySubscript { array, .. } => {
                let typed = self.check_expr(array, scope)?;
                let elem_type = match &typed.data_type {
                    SqlType::Array(elem) => (**elem).clone(),
                    _ => SqlType::Unknown,
                };
                Ok(TypedExpr::nullable(elem_type))
            }
            ExprKind::SafeArraySubscript { array, .. } => {
                let typed = self.check_expr(array, scope)?;
                let elem_type = match &typed.data_type {
                    SqlType::Array(elem) => (**elem).clone(),
                    _ => SqlType::Unknown,
                };
                Ok(TypedExpr::nullable(elem_type))
            }
            ExprKind::FieldAccess { expr, field } => {
                let typed = self.check_expr(expr, scope)?;
                match &typed.data_type {
                    SqlType::Struct(fields) => {
                        for f in fields {
                            if let Some(name) = &f.name {
                                if name.eq_ignore_ascii_case(&field.value) {
                                    return Ok(TypedExpr::nullable(f.data_type.clone()));
                                }
                            }
                        }
                        Err(AnalyzerError::column_not_found(&field.value, None))
                    }
                    SqlType::Json => Ok(TypedExpr::nullable(SqlType::Json)),
                    _ => Err(AnalyzerError::column_not_found(&field.value, None)),
                }
            }
            ExprKind::JsonSubscript { expr, .. } => {
                self.check_expr(expr, scope)?;
                Ok(TypedExpr::nullable(SqlType::Json))
            }
            ExprKind::Interval { .. } => Ok(TypedExpr::non_null(SqlType::Interval)),
            ExprKind::TypedLiteral { data_type, .. } => {
                let sql_type = match data_type {
                    TypedLiteralType::Date => SqlType::Date,
                    TypedLiteralType::Time => SqlType::Time,
                    TypedLiteralType::Timestamp => SqlType::Timestamp,
                    TypedLiteralType::Datetime => SqlType::Datetime,
                    TypedLiteralType::Json => SqlType::Json,
                    TypedLiteralType::Numeric => SqlType::Numeric {
                        precision: None,
                        scale: None,
                    },
                    TypedLiteralType::Bignumeric => SqlType::Numeric {
                        precision: None,
                        scale: None,
                    },
                    TypedLiteralType::Range => SqlType::Unknown,
                };
                Ok(TypedExpr::non_null(sql_type))
            }
            ExprKind::Parenthesized(inner) => self.check_expr(inner, scope),
            ExprKind::Row(exprs) => {
                let mut fields = Vec::new();
                for (i, expr) in exprs.iter().enumerate() {
                    let typed = self.check_expr(expr, scope)?;
                    fields.push(crate::types::StructField {
                        name: Some(format!("_{}", i)),
                        data_type: typed.data_type,
                    });
                }
                Ok(TypedExpr::non_null(SqlType::Struct(fields)))
            }
        }
    }

    /// Check a column reference.
    fn check_column(
        &self,
        col_name: &str,
        table_name: Option<&str>,
        scope: &Scope,
    ) -> Result<TypedExpr, AnalyzerError> {
        if let Some(table) = table_name {
            if let Some(col) = scope.lookup_qualified_column(table, col_name) {
                Ok(TypedExpr {
                    data_type: col.data_type.clone(),
                    nullable: col.nullable,
                    contains_aggregate: false,
                    contains_window: false,
                })
            } else {
                Err(AnalyzerError::column_not_found(
                    col_name,
                    Some(table.to_string()),
                ))
            }
        } else {
            match scope.lookup_column(col_name) {
                ColumnLookupResult::Found(_, col) => Ok(TypedExpr {
                    data_type: col.data_type.clone(),
                    nullable: col.nullable,
                    contains_aggregate: false,
                    contains_window: false,
                }),
                ColumnLookupResult::NotFound => {
                    Err(AnalyzerError::column_not_found(col_name, None))
                }
                ColumnLookupResult::Ambiguous(tables) => {
                    Err(AnalyzerError::ambiguous_column(col_name, tables))
                }
            }
        }
    }

    /// Check a binary operation.
    fn check_binary_op(
        &self,
        op: BinaryOp,
        left: &Expr,
        right: &Expr,
        scope: &Scope,
    ) -> Result<TypedExpr, AnalyzerError> {
        let left_typed = self.check_expr(left, scope)?;
        let right_typed = self.check_expr(right, scope)?;

        let result_type = match op {
            // Comparison operators return Bool
            BinaryOp::Eq
            | BinaryOp::NotEq
            | BinaryOp::Lt
            | BinaryOp::LtEq
            | BinaryOp::Gt
            | BinaryOp::GtEq => SqlType::Bool,

            // Logical operators
            BinaryOp::And | BinaryOp::Or => SqlType::Bool,

            // Arithmetic operators
            BinaryOp::Plus
            | BinaryOp::Minus
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo => left_typed
                .data_type
                .common_supertype(&right_typed.data_type)
                .unwrap_or(SqlType::Float64),

            // String concatenation
            BinaryOp::Concat => SqlType::Varchar,

            // Bitwise operators
            BinaryOp::BitwiseAnd
            | BinaryOp::BitwiseOr
            | BinaryOp::BitwiseXor
            | BinaryOp::LeftShift
            | BinaryOp::RightShift => SqlType::Int64,
        };

        Ok(TypedExpr {
            data_type: result_type,
            nullable: left_typed.nullable || right_typed.nullable,
            contains_aggregate: left_typed.contains_aggregate || right_typed.contains_aggregate,
            contains_window: left_typed.contains_window || right_typed.contains_window,
        })
    }

    /// Check a unary operation.
    fn check_unary_op(
        &self,
        op: UnaryOp,
        expr: &Expr,
        scope: &Scope,
    ) -> Result<TypedExpr, AnalyzerError> {
        let typed = self.check_expr(expr, scope)?;
        let result_type = match op {
            UnaryOp::Not => SqlType::Bool,
            UnaryOp::Plus | UnaryOp::Minus => typed.data_type.clone(),
            UnaryOp::BitwiseNot => SqlType::Int64,
        };
        Ok(TypedExpr {
            data_type: result_type,
            nullable: typed.nullable,
            contains_aggregate: typed.contains_aggregate,
            contains_window: typed.contains_window,
        })
    }

    /// Check a function call.
    fn check_function(
        &self,
        func: &FunctionCall,
        scope: &Scope,
    ) -> Result<TypedExpr, AnalyzerError> {
        let func_name = func
            .name
            .parts
            .last()
            .map(|i| i.value.to_uppercase())
            .unwrap_or_default();
        let name_parts: Vec<String> = func.name.parts.iter().map(|i| i.value.clone()).collect();

        // Look up function in catalog
        let sig = self
            .catalog
            .resolve_function(&name_parts)
            .map_err(|_| AnalyzerError::function_not_found(&func_name))?
            .ok_or_else(|| AnalyzerError::function_not_found(&func_name))?;

        // Check argument count
        let arg_count = func.args.len();
        if !sig.accepts_arg_count(arg_count) {
            return Err(AnalyzerError::wrong_argument_count(
                &func_name,
                sig.min_args,
                sig.max_args,
                arg_count,
            ));
        }

        // Type check arguments
        for arg in &func.args {
            if let FunctionArg::Unnamed(expr) = arg {
                self.check_expr(expr, scope)?;
            }
        }

        Ok(TypedExpr {
            data_type: sig.return_type.clone(),
            nullable: true,
            contains_aggregate: sig.is_aggregate,
            contains_window: sig.is_window,
        })
    }

    /// Check an aggregate function call.
    fn check_aggregate(
        &self,
        agg: &AggregateCall,
        scope: &Scope,
    ) -> Result<TypedExpr, AnalyzerError> {
        let func_name = agg
            .function
            .name
            .parts
            .last()
            .map(|i| i.value.to_uppercase())
            .unwrap_or_default();
        let name_parts: Vec<String> = agg
            .function
            .name
            .parts
            .iter()
            .map(|i| i.value.clone())
            .collect();

        let sig = self
            .catalog
            .resolve_function(&name_parts)
            .map_err(|_| AnalyzerError::function_not_found(&func_name))?
            .ok_or_else(|| AnalyzerError::function_not_found(&func_name))?;

        // Type check arguments
        for arg in &agg.function.args {
            if let FunctionArg::Unnamed(expr) = arg {
                self.check_expr(expr, scope)?;
            }
        }

        Ok(TypedExpr {
            data_type: sig.return_type.clone(),
            nullable: true,
            contains_aggregate: true,
            contains_window: false,
        })
    }

    /// Check a window function call.
    fn check_window_function(
        &self,
        wf: &WindowFunctionCall,
        scope: &Scope,
    ) -> Result<TypedExpr, AnalyzerError> {
        let func_name = wf
            .function
            .name
            .parts
            .last()
            .map(|i| i.value.to_uppercase())
            .unwrap_or_default();
        let name_parts: Vec<String> = wf
            .function
            .name
            .parts
            .iter()
            .map(|i| i.value.clone())
            .collect();

        let sig = self
            .catalog
            .resolve_function(&name_parts)
            .map_err(|_| AnalyzerError::function_not_found(&func_name))?
            .ok_or_else(|| AnalyzerError::function_not_found(&func_name))?;

        // Type check arguments
        for arg in &wf.function.args {
            if let FunctionArg::Unnamed(expr) = arg {
                self.check_expr(expr, scope)?;
            }
        }

        Ok(TypedExpr {
            data_type: sig.return_type.clone(),
            nullable: true,
            contains_aggregate: false,
            contains_window: true,
        })
    }

    /// Convert AST data type to SqlType.
    fn data_type_to_sql_type(&self, dt: &DataTypeSpec) -> SqlType {
        Self::convert_data_type(dt)
    }

    /// Convert AST data type to SqlType (static helper).
    fn convert_data_type(dt: &DataTypeSpec) -> SqlType {
        match &dt.kind {
            DataTypeKind::Bool => SqlType::Bool,
            DataTypeKind::Int32 => SqlType::Int32,
            DataTypeKind::Int64 => SqlType::Int64,
            DataTypeKind::Uint32 => SqlType::Uint32,
            DataTypeKind::Uint64 => SqlType::Uint64,
            DataTypeKind::Float32 => SqlType::Float32,
            DataTypeKind::Float64 => SqlType::Float64,
            DataTypeKind::Numeric { .. } => SqlType::Numeric {
                precision: None,
                scale: None,
            },
            DataTypeKind::Varchar { .. } => SqlType::Varchar,
            DataTypeKind::Varbinary { .. } => SqlType::Varbinary,
            DataTypeKind::Date => SqlType::Date,
            DataTypeKind::Time => SqlType::Time,
            DataTypeKind::Datetime => SqlType::Datetime,
            DataTypeKind::Timestamp => SqlType::Timestamp,
            DataTypeKind::Interval => SqlType::Interval,
            DataTypeKind::Json => SqlType::Json,
            DataTypeKind::Uuid => SqlType::Uuid,
            DataTypeKind::Array(elem) => SqlType::Array(Box::new(Self::convert_data_type(elem))),
            DataTypeKind::Struct(fields) => {
                let sql_fields: Vec<crate::types::StructField> = fields
                    .iter()
                    .map(|f| crate::types::StructField {
                        name: f.name.as_ref().map(|i| i.value.clone()),
                        data_type: Self::convert_data_type(&f.data_type),
                    })
                    .collect();
                SqlType::Struct(sql_fields)
            }
            DataTypeKind::Range(elem) => SqlType::Range(Box::new(Self::convert_data_type(elem))),
            DataTypeKind::Named(_) => SqlType::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::MemoryCatalog;
    use crate::error::Span;

    fn make_catalog() -> MemoryCatalog {
        let mut catalog = MemoryCatalog::new();
        catalog.register_builtins();
        catalog
    }

    #[test]
    fn test_literal_types() {
        let catalog = make_catalog();
        let checker = TypeChecker::new(&catalog);
        let scope = Scope::new();

        // Integer literal
        let expr = Expr::new(ExprKind::Integer(42), Span::default());
        let typed = checker.check_expr(&expr, &scope).unwrap();
        assert_eq!(typed.data_type, SqlType::Int64);
        assert!(!typed.nullable);

        // String literal
        let expr = Expr::new(ExprKind::String("hello".to_string()), Span::default());
        let typed = checker.check_expr(&expr, &scope).unwrap();
        assert_eq!(typed.data_type, SqlType::Varchar);

        // Null literal
        let expr = Expr::new(ExprKind::Null, Span::default());
        let typed = checker.check_expr(&expr, &scope).unwrap();
        assert!(typed.nullable);
    }

    #[test]
    fn test_binary_op_types() {
        let catalog = make_catalog();
        let checker = TypeChecker::new(&catalog);
        let scope = Scope::new();

        // Arithmetic: int + int = int
        let expr = Expr::new(
            ExprKind::BinaryOp {
                op: BinaryOp::Plus,
                left: Box::new(Expr::new(ExprKind::Integer(1), Span::default())),
                right: Box::new(Expr::new(ExprKind::Integer(2), Span::default())),
            },
            Span::default(),
        );
        let typed = checker.check_expr(&expr, &scope).unwrap();
        assert_eq!(typed.data_type, SqlType::Int64);

        // Comparison: int < int = bool
        let expr = Expr::new(
            ExprKind::BinaryOp {
                op: BinaryOp::Lt,
                left: Box::new(Expr::new(ExprKind::Integer(1), Span::default())),
                right: Box::new(Expr::new(ExprKind::Integer(2), Span::default())),
            },
            Span::default(),
        );
        let typed = checker.check_expr(&expr, &scope).unwrap();
        assert_eq!(typed.data_type, SqlType::Bool);
    }
}
