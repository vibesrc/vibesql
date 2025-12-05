//! DML and DDL statement parsing.

use super::Parser;
use crate::ast::*;
use crate::error::{Error, Result, Span};
use crate::lexer::{Keyword, TokenKind};

impl<'a> Parser<'a> {
    // ========================================================================
    // DML Statements
    // ========================================================================

    /// Parse INSERT statement.
    pub fn parse_insert(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Insert)?;

        // Optional OR action
        let or_action = if self.consume_keyword(Keyword::Or)?.is_some() {
            if self.consume_keyword(Keyword::Ignore)?.is_some() {
                Some(InsertOrAction::Ignore)
            } else if self.consume_keyword(Keyword::Replace)?.is_some() {
                Some(InsertOrAction::Replace)
            } else if self.consume_keyword(Keyword::Update)?.is_some() {
                Some(InsertOrAction::Update)
            } else {
                let token = self.peek()?;
                return Err(Error::unexpected_token(
                    "IGNORE, REPLACE, or UPDATE",
                    format!("{}", token.kind),
                    token.span,
                ));
            }
        } else {
            None
        };

        self.expect_keyword(Keyword::Into)?;
        let table = self.parse_object_name()?;

        // Optional column list
        let columns = if self.consume(&TokenKind::LeftParen)?.is_some() {
            let cols = self.parse_comma_separated(|p| p.parse_identifier())?;
            self.expect(&TokenKind::RightParen)?;
            cols
        } else {
            Vec::new()
        };

        // Parse source
        let source = if self.consume_keyword(Keyword::Default)?.is_some() {
            self.expect_keyword(Keyword::Values)?;
            InsertSource::DefaultValues
        } else if self.consume_keyword(Keyword::Values)?.is_some() {
            let rows = self.parse_comma_separated(|p| {
                p.expect(&TokenKind::LeftParen)?;
                let values = p.parse_comma_separated(|p2| p2.parse_expression())?;
                p.expect(&TokenKind::RightParen)?;
                Ok(values)
            })?;
            InsertSource::Values(rows)
        } else {
            let query = self.parse_query()?;
            InsertSource::Query(Box::new(query))
        };

        // Optional RETURNING clause
        let returning = self.parse_returning_clause()?;

        Ok(StatementKind::Insert(InsertStatement {
            or_action,
            table,
            columns,
            source,
            returning,
        }))
    }

    /// Parse UPDATE statement.
    pub fn parse_update(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Update)?;
        let table = self.parse_table_ref()?;

        self.expect_keyword(Keyword::Set)?;
        let assignments = self.parse_comma_separated(|p| p.parse_assignment())?;

        // Optional FROM clause
        let from = if self.consume_keyword(Keyword::From)?.is_some() {
            let tables = self.parse_comma_separated(|p| p.parse_table_ref())?;
            Some(FromClause { tables })
        } else {
            None
        };

        // Optional WHERE clause
        let where_clause = if self.consume_keyword(Keyword::Where)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Optional RETURNING clause
        let returning = self.parse_returning_clause()?;

        Ok(StatementKind::Update(UpdateStatement {
            table,
            assignments,
            from,
            where_clause,
            returning,
        }))
    }

    /// Parse an assignment (column = expr).
    fn parse_assignment(&mut self) -> Result<Assignment> {
        let first = self.parse_identifier()?;

        // Check for path (col.field.field = value)
        let target = if self.consume(&TokenKind::Dot)?.is_some() {
            let mut parts = vec![first];
            parts.push(self.parse_identifier()?);
            while self.consume(&TokenKind::Dot)?.is_some() {
                parts.push(self.parse_identifier()?);
            }
            AssignmentTarget::Path(parts)
        } else {
            AssignmentTarget::Column(first)
        };

        self.expect(&TokenKind::Eq)?;
        let value = self.parse_expression()?;

        Ok(Assignment { target, value })
    }

    /// Parse DELETE statement.
    pub fn parse_delete(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Delete)?;
        self.expect_keyword(Keyword::From)?;

        let table = self.parse_object_name()?;
        let alias = self.parse_optional_table_alias()?;

        // Optional WHERE clause
        let where_clause = if self.consume_keyword(Keyword::Where)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Optional RETURNING clause
        let returning = self.parse_returning_clause()?;

        Ok(StatementKind::Delete(DeleteStatement {
            table,
            alias,
            where_clause,
            returning,
        }))
    }

    /// Parse MERGE statement.
    pub fn parse_merge(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Merge)?;
        self.expect_keyword(Keyword::Into)?;

        let target = self.parse_table_ref()?;

        self.expect_keyword(Keyword::Using)?;
        let source = self.parse_table_ref()?;

        self.expect_keyword(Keyword::On)?;
        let on = self.parse_expression()?;

        let mut clauses = Vec::new();
        while self.consume_keyword(Keyword::When)?.is_some() {
            let clause = self.parse_merge_clause()?;
            clauses.push(clause);
        }

        Ok(StatementKind::Merge(MergeStatement {
            target,
            source,
            on,
            clauses,
        }))
    }

    /// Parse a MERGE clause (WHEN MATCHED/NOT MATCHED).
    fn parse_merge_clause(&mut self) -> Result<MergeClause> {
        let is_not = self.consume_keyword(Keyword::Not)?.is_some();

        self.expect_keyword(Keyword::Matched)?;

        let by_source = if is_not {
            self.consume_keyword(Keyword::By)?.is_some() && {
                self.expect_keyword(Keyword::Source)?;
                true
            }
        } else {
            false
        };

        // Optional AND condition
        let condition = if self.consume_keyword(Keyword::And)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_keyword(Keyword::Then)?;

        if is_not && !by_source {
            // WHEN NOT MATCHED [BY TARGET] THEN INSERT
            self.expect_keyword(Keyword::Insert)?;

            let columns = if self.consume(&TokenKind::LeftParen)?.is_some() {
                let cols = self.parse_comma_separated(|p| p.parse_identifier())?;
                self.expect(&TokenKind::RightParen)?;
                cols
            } else {
                Vec::new()
            };

            self.expect_keyword(Keyword::Values)?;
            self.expect(&TokenKind::LeftParen)?;
            let values = self.parse_comma_separated(|p| p.parse_expression())?;
            self.expect(&TokenKind::RightParen)?;

            Ok(MergeClause::NotMatched {
                condition,
                action: MergeNotMatchedAction { columns, values },
            })
        } else if is_not && by_source {
            // WHEN NOT MATCHED BY SOURCE THEN UPDATE/DELETE
            let action = self.parse_merge_matched_action()?;
            Ok(MergeClause::NotMatchedBySource { condition, action })
        } else {
            // WHEN MATCHED THEN UPDATE/DELETE
            let action = self.parse_merge_matched_action()?;
            Ok(MergeClause::Matched { condition, action })
        }
    }

    /// Parse MERGE matched action (UPDATE or DELETE).
    fn parse_merge_matched_action(&mut self) -> Result<MergeMatchedAction> {
        if self.consume_keyword(Keyword::Update)?.is_some() {
            self.expect_keyword(Keyword::Set)?;
            let assignments = self.parse_comma_separated(|p| p.parse_assignment())?;
            Ok(MergeMatchedAction::Update { assignments })
        } else if self.consume_keyword(Keyword::Delete)?.is_some() {
            Ok(MergeMatchedAction::Delete)
        } else {
            let token = self.peek()?;
            Err(Error::unexpected_token(
                "UPDATE or DELETE",
                format!("{}", token.kind),
                token.span,
            ))
        }
    }

    /// Parse RETURNING clause.
    fn parse_returning_clause(&mut self) -> Result<Option<ReturningClause>> {
        if self.consume_keyword(Keyword::Returning)?.is_some() {
            let items = self.parse_comma_separated(|p| p.parse_select_item())?;
            let with_action = false; // WITH ACTION not yet supported
            Ok(Some(ReturningClause { items, with_action }))
        } else {
            Ok(None)
        }
    }

    // ========================================================================
    // DDL Statements
    // ========================================================================

    /// Parse CREATE statement.
    pub fn parse_create(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Create)?;

        // Check for OR REPLACE
        let or_replace = if self.consume_keyword(Keyword::Or)?.is_some() {
            self.expect_keyword(Keyword::Replace)?;
            true
        } else {
            false
        };

        // Check for TEMPORARY
        let temporary = self.consume_keyword(Keyword::Temporary)?.is_some()
            || self.consume_keyword(Keyword::Temp)?.is_some();

        if self.consume_keyword(Keyword::Table)?.is_some() {
            self.parse_create_table(or_replace, temporary)
        } else if self.consume_keyword(Keyword::View)?.is_some() {
            self.parse_create_view(or_replace, false)
        } else if self.consume_keyword(Keyword::Materialized)?.is_some() {
            self.expect_keyword(Keyword::View)?;
            self.parse_create_view(or_replace, true)
        } else if self.consume_keyword(Keyword::Index)?.is_some() {
            self.parse_create_index(false)
        } else if self.consume_keyword(Keyword::Unique)?.is_some() {
            self.expect_keyword(Keyword::Index)?;
            self.parse_create_index(true)
        } else if self.consume_keyword(Keyword::Function)?.is_some() {
            self.parse_create_function(or_replace, temporary)
        } else if self.consume_keyword(Keyword::Procedure)?.is_some() {
            self.parse_create_procedure(or_replace)
        } else if self.consume_keyword(Keyword::Database)?.is_some()
            || self.consume_keyword(Keyword::Schema)?.is_some()
        {
            self.parse_create_database()
        } else {
            let token = self.peek()?;
            Err(Error::unexpected_token(
                "TABLE, VIEW, INDEX, FUNCTION, PROCEDURE, or DATABASE",
                format!("{}", token.kind),
                token.span,
            ))
        }
    }

    /// Parse CREATE TABLE statement.
    fn parse_create_table(&mut self, or_replace: bool, temporary: bool) -> Result<StatementKind> {
        let if_not_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Not)?;
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        let name = self.parse_object_name()?;

        // Check for LIKE or CLONE
        let like = if self.consume_keyword(Keyword::Like)?.is_some() {
            Some(self.parse_object_name()?)
        } else {
            None
        };

        let clone = if self.consume_keyword(Keyword::Clone)?.is_some() {
            Some(self.parse_object_name()?)
        } else {
            None
        };

        // Check for AS SELECT
        let as_query = if self.consume_keyword(Keyword::As)?.is_some() {
            Some(Box::new(self.parse_query()?))
        } else {
            None
        };

        // Parse column definitions and constraints
        let (columns, constraints) = if self.consume(&TokenKind::LeftParen)?.is_some() {
            let mut columns = Vec::new();
            let mut constraints = Vec::new();

            loop {
                // Check for table constraint
                if self.check_keyword(Keyword::Primary)?
                    || self.check_keyword(Keyword::Unique)?
                    || self.check_keyword(Keyword::Foreign)?
                    || self.check_keyword(Keyword::Check)?
                    || self.check_keyword(Keyword::Constraint)?
                {
                    constraints.push(self.parse_table_constraint()?);
                } else {
                    columns.push(self.parse_column_def()?);
                }

                if self.consume(&TokenKind::Comma)?.is_none() {
                    break;
                }
            }

            self.expect(&TokenKind::RightParen)?;
            (columns, constraints)
        } else {
            (Vec::new(), Vec::new())
        };

        // Parse PARTITION BY
        let partition_by = if self.consume_keyword(Keyword::Partition)?.is_some() {
            self.expect_keyword(Keyword::By)?;
            self.parse_comma_separated(|p| p.parse_expression())?
        } else {
            Vec::new()
        };

        // Parse CLUSTER BY
        let cluster_by = if self.consume_keyword(Keyword::Cluster)?.is_some() {
            self.expect_keyword(Keyword::By)?;
            self.parse_comma_separated(|p| p.parse_expression())?
        } else {
            Vec::new()
        };

        // Parse OPTIONS
        let options = self.parse_options_clause()?;

        Ok(StatementKind::CreateTable(CreateTableStatement {
            or_replace,
            temporary,
            if_not_exists,
            name,
            columns,
            constraints,
            partition_by,
            cluster_by,
            options,
            as_query,
            like,
            clone,
        }))
    }

    /// Parse column definition.
    fn parse_column_def(&mut self) -> Result<ColumnDef> {
        let start = self.current_position();
        let name = self.parse_identifier()?;

        // Optional data type
        let data_type = if !self.check(&TokenKind::Comma)?
            && !self.check(&TokenKind::RightParen)?
            && !self.check_keyword(Keyword::Primary)?
            && !self.check_keyword(Keyword::Not)?
            && !self.check(&TokenKind::Null)?  // NULL is a special token
            && !self.check_keyword(Keyword::Default)?
            && !self.check_keyword(Keyword::Unique)?
            && !self.check_keyword(Keyword::Check)?
            && !self.check_keyword(Keyword::References)?
            && !self.check_keyword(Keyword::Generated)?
            && !self.check_keyword(Keyword::Hidden)?
            && !self.check_keyword(Keyword::Options)?
        {
            Some(self.parse_data_type()?)
        } else {
            None
        };

        // Parse column constraints
        let mut constraints = Vec::new();
        loop {
            if self.consume_keyword(Keyword::Not)?.is_some() {
                self.expect(&TokenKind::Null)?; // NULL is a special token
                constraints.push(ColumnConstraint::NotNull);
            } else if self.consume(&TokenKind::Null)?.is_some() {
                // NULL is a special token
                constraints.push(ColumnConstraint::Null);
            } else if self.consume_keyword(Keyword::Primary)?.is_some() {
                self.expect_keyword(Keyword::Key)?;
                constraints.push(ColumnConstraint::PrimaryKey);
            } else if self.consume_keyword(Keyword::Unique)?.is_some() {
                constraints.push(ColumnConstraint::Unique);
            } else if self.consume_keyword(Keyword::Default)?.is_some() {
                let expr = self.parse_expression()?;
                constraints.push(ColumnConstraint::Default(expr));
            } else if self.consume_keyword(Keyword::Check)?.is_some() {
                self.expect(&TokenKind::LeftParen)?;
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RightParen)?;
                constraints.push(ColumnConstraint::Check(expr));
            } else if self.consume_keyword(Keyword::References)?.is_some() {
                let table = self.parse_object_name()?;
                let columns = if self.consume(&TokenKind::LeftParen)?.is_some() {
                    let cols = self.parse_comma_separated(|p| p.parse_identifier())?;
                    self.expect(&TokenKind::RightParen)?;
                    cols
                } else {
                    Vec::new()
                };
                let on_delete = self.parse_referential_action(Keyword::Delete)?;
                let on_update = self.parse_referential_action(Keyword::Update)?;
                constraints.push(ColumnConstraint::References {
                    table,
                    columns,
                    on_delete,
                    on_update,
                });
            } else if self.consume_keyword(Keyword::Generated)?.is_some() {
                let always = if self.consume_keyword(Keyword::Always)?.is_some() {
                    true
                } else {
                    self.expect_keyword(Keyword::By)?;
                    self.expect_keyword(Keyword::Default)?;
                    false
                };
                self.expect_keyword(Keyword::As)?;
                self.expect(&TokenKind::LeftParen)?;
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RightParen)?;
                constraints.push(ColumnConstraint::Generated { expr, always });
            } else if self.consume_keyword(Keyword::Hidden)?.is_some() {
                constraints.push(ColumnConstraint::Hidden);
            } else {
                break;
            }
        }

        // Parse OPTIONS
        let options = self.parse_options_clause()?;

        let end = self.current_position();
        Ok(ColumnDef {
            name,
            data_type,
            constraints,
            options,
            span: Span::new(start, end),
        })
    }

    /// Parse referential action (ON DELETE/UPDATE).
    fn parse_referential_action(&mut self, keyword: Keyword) -> Result<Option<ReferentialAction>> {
        if self.consume_keyword(Keyword::On)?.is_some() {
            self.expect_keyword(keyword)?;
            let action = if self.consume_keyword(Keyword::No)?.is_some() {
                self.expect_keyword(Keyword::Action)?;
                ReferentialAction::NoAction
            } else if self.consume_keyword(Keyword::Restrict)?.is_some() {
                ReferentialAction::Restrict
            } else if self.consume_keyword(Keyword::Cascade)?.is_some() {
                ReferentialAction::Cascade
            } else if self.consume_keyword(Keyword::Set)?.is_some() {
                if self.consume(&TokenKind::Null)?.is_some() {
                    // NULL is a special token
                    ReferentialAction::SetNull
                } else {
                    self.expect_keyword(Keyword::Default)?;
                    ReferentialAction::SetDefault
                }
            } else {
                let token = self.peek()?;
                return Err(Error::unexpected_token(
                    "NO ACTION, RESTRICT, CASCADE, SET NULL, or SET DEFAULT",
                    format!("{}", token.kind),
                    token.span,
                ));
            };
            Ok(Some(action))
        } else {
            Ok(None)
        }
    }

    /// Parse table constraint.
    fn parse_table_constraint(&mut self) -> Result<TableConstraint> {
        // Optional constraint name
        let name = if self.consume_keyword(Keyword::Constraint)?.is_some() {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        if self.consume_keyword(Keyword::Primary)?.is_some() {
            self.expect_keyword(Keyword::Key)?;
            self.expect(&TokenKind::LeftParen)?;
            let columns = self.parse_comma_separated(|p| p.parse_sort_key())?;
            self.expect(&TokenKind::RightParen)?;
            let options = self.parse_options_clause()?;
            Ok(TableConstraint::PrimaryKey {
                name,
                columns,
                options,
            })
        } else if self.consume_keyword(Keyword::Unique)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let columns = self.parse_comma_separated(|p| p.parse_identifier())?;
            self.expect(&TokenKind::RightParen)?;
            Ok(TableConstraint::Unique { name, columns })
        } else if self.consume_keyword(Keyword::Foreign)?.is_some() {
            self.expect_keyword(Keyword::Key)?;
            self.expect(&TokenKind::LeftParen)?;
            let columns = self.parse_comma_separated(|p| p.parse_identifier())?;
            self.expect(&TokenKind::RightParen)?;
            self.expect_keyword(Keyword::References)?;
            let references_table = self.parse_object_name()?;
            self.expect(&TokenKind::LeftParen)?;
            let references_columns = self.parse_comma_separated(|p| p.parse_identifier())?;
            self.expect(&TokenKind::RightParen)?;
            let on_delete = self.parse_referential_action(Keyword::Delete)?;
            let on_update = self.parse_referential_action(Keyword::Update)?;
            Ok(TableConstraint::ForeignKey {
                name,
                columns,
                references_table,
                references_columns,
                on_delete,
                on_update,
            })
        } else if self.consume_keyword(Keyword::Check)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let expr = self.parse_expression()?;
            self.expect(&TokenKind::RightParen)?;
            let enforced = if self.consume_keyword(Keyword::Enforced)?.is_some() {
                Some(true)
            } else if self.consume_keyword(Keyword::Not)?.is_some() {
                self.expect_keyword(Keyword::Enforced)?;
                Some(false)
            } else {
                None
            };
            Ok(TableConstraint::Check {
                name,
                expr,
                enforced,
            })
        } else {
            let token = self.peek()?;
            Err(Error::unexpected_token(
                "PRIMARY, UNIQUE, FOREIGN, or CHECK",
                format!("{}", token.kind),
                token.span,
            ))
        }
    }

    /// Parse sort key for PRIMARY KEY.
    fn parse_sort_key(&mut self) -> Result<SortKey> {
        let column = self.parse_identifier()?;
        let order = if self.consume_keyword(Keyword::Asc)?.is_some() {
            Some(SortOrder::Asc)
        } else if self.consume_keyword(Keyword::Desc)?.is_some() {
            Some(SortOrder::Desc)
        } else {
            None
        };
        let nulls = None; // Not typically used in PRIMARY KEY
        Ok(SortKey {
            column,
            order,
            nulls,
        })
    }

    /// Parse OPTIONS clause.
    fn parse_options_clause(&mut self) -> Result<Vec<SqlOption>> {
        if self.consume_keyword(Keyword::Options)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let options = self.parse_comma_separated(|p| {
                let name = p.parse_identifier()?;
                p.expect(&TokenKind::Eq)?;
                let value = p.parse_expression()?;
                Ok(SqlOption { name, value })
            })?;
            self.expect(&TokenKind::RightParen)?;
            Ok(options)
        } else {
            Ok(Vec::new())
        }
    }

    /// Parse CREATE VIEW statement.
    fn parse_create_view(&mut self, or_replace: bool, materialized: bool) -> Result<StatementKind> {
        let if_not_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Not)?;
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        let name = self.parse_object_name()?;

        // Optional column list
        let columns = if self.consume(&TokenKind::LeftParen)?.is_some() {
            let cols = self.parse_comma_separated(|p| p.parse_identifier())?;
            self.expect(&TokenKind::RightParen)?;
            cols
        } else {
            Vec::new()
        };

        let options = self.parse_options_clause()?;

        self.expect_keyword(Keyword::As)?;
        let query = Box::new(self.parse_query()?);

        Ok(StatementKind::CreateView(CreateViewStatement {
            or_replace,
            materialized,
            if_not_exists,
            name,
            columns,
            query,
            options,
        }))
    }

    /// Parse CREATE INDEX statement.
    fn parse_create_index(&mut self, unique: bool) -> Result<StatementKind> {
        let if_not_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Not)?;
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        // Optional index name
        let name = if !self.check_keyword(Keyword::On)? {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.expect_keyword(Keyword::On)?;
        let table = self.parse_object_name()?;

        self.expect(&TokenKind::LeftParen)?;
        let columns = self.parse_comma_separated(|p| p.parse_sort_key())?;
        self.expect(&TokenKind::RightParen)?;

        let options = self.parse_options_clause()?;

        Ok(StatementKind::CreateIndex(CreateIndexStatement {
            unique,
            if_not_exists,
            name,
            table,
            columns,
            options,
        }))
    }

    /// Parse CREATE FUNCTION statement.
    fn parse_create_function(
        &mut self,
        or_replace: bool,
        temporary: bool,
    ) -> Result<StatementKind> {
        let if_not_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Not)?;
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        let name = self.parse_object_name()?;

        // Parse parameters
        self.expect(&TokenKind::LeftParen)?;
        let params = if self.check(&TokenKind::RightParen)? {
            Vec::new()
        } else {
            self.parse_comma_separated(|p| p.parse_function_param())?
        };
        self.expect(&TokenKind::RightParen)?;

        // Optional RETURNS clause
        let returns = if self.consume_keyword(Keyword::Returns)?.is_some() {
            Some(self.parse_data_type()?)
        } else {
            None
        };

        // Optional LANGUAGE clause
        let language = if self.consume_keyword(Keyword::Language)?.is_some() {
            let token = self.advance()?;
            match token.kind {
                TokenKind::Identifier(name) => Some(name),
                TokenKind::Keyword(_) => Some(token.text.to_string()),
                _ => {
                    return Err(Error::expected_identifier(token.span));
                }
            }
        } else {
            None
        };

        let options = self.parse_options_clause()?;

        // Parse body
        let body = if self.consume_keyword(Keyword::As)?.is_some() {
            if self.consume(&TokenKind::LeftParen)?.is_some() {
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RightParen)?;
                FunctionBody::Expr(expr)
            } else {
                let token = self.advance()?;
                match token.kind {
                    TokenKind::String(s) => FunctionBody::External(s),
                    _ => {
                        return Err(Error::unexpected_token(
                            "expression or string literal",
                            format!("{}", token.kind),
                            token.span,
                        ));
                    }
                }
            }
        } else {
            // No body - external function
            FunctionBody::External(String::new())
        };

        Ok(StatementKind::CreateFunction(CreateFunctionStatement {
            or_replace,
            temporary,
            if_not_exists,
            name,
            params,
            returns,
            language,
            body,
            options,
        }))
    }

    /// Parse function parameter.
    fn parse_function_param(&mut self) -> Result<FunctionParam> {
        // Optional parameter name
        let token = self.peek()?;
        let name = if matches!(
            &token.kind,
            TokenKind::Identifier(_) | TokenKind::QuotedIdentifier(_)
        ) {
            let next = self.peek_nth(1)?;
            if !matches!(next.kind, TokenKind::Comma | TokenKind::RightParen) {
                Some(self.parse_identifier()?)
            } else {
                None
            }
        } else {
            None
        };

        let data_type = self.parse_data_type()?;

        // Optional default value
        let default = if self.consume_keyword(Keyword::Default)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(FunctionParam {
            name,
            data_type,
            default,
        })
    }

    /// Parse CREATE PROCEDURE statement.
    fn parse_create_procedure(&mut self, or_replace: bool) -> Result<StatementKind> {
        let if_not_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Not)?;
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        let name = self.parse_object_name()?;

        // Parse parameters
        self.expect(&TokenKind::LeftParen)?;
        let params = if self.check(&TokenKind::RightParen)? {
            Vec::new()
        } else {
            self.parse_comma_separated(|p| p.parse_procedure_param())?
        };
        self.expect(&TokenKind::RightParen)?;

        let options = self.parse_options_clause()?;

        // Parse body (BEGIN ... END)
        self.expect_keyword(Keyword::Begin)?;
        let body = self.parse_statement_list()?;
        self.expect_keyword(Keyword::End)?;

        Ok(StatementKind::CreateProcedure(CreateProcedureStatement {
            or_replace,
            if_not_exists,
            name,
            params,
            body,
            options,
        }))
    }

    /// Parse procedure parameter.
    fn parse_procedure_param(&mut self) -> Result<ProcedureParam> {
        let mode = if self.consume_keyword(Keyword::In)?.is_some() {
            if self.consume_keyword(Keyword::Out)?.is_some() {
                ParamMode::InOut
            } else {
                ParamMode::In
            }
        } else if self.consume_keyword(Keyword::Out)?.is_some() {
            ParamMode::Out
        } else if self.consume_keyword(Keyword::Inout)?.is_some() {
            ParamMode::InOut
        } else {
            ParamMode::In
        };

        // Optional parameter name
        let name = {
            let next = self.peek_nth(1)?;
            if !matches!(next.kind, TokenKind::Comma | TokenKind::RightParen) {
                Some(self.parse_identifier()?)
            } else {
                None
            }
        };

        let data_type = self.parse_data_type()?;

        Ok(ProcedureParam {
            mode,
            name,
            data_type,
        })
    }

    /// Parse CREATE DATABASE statement.
    fn parse_create_database(&mut self) -> Result<StatementKind> {
        let if_not_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Not)?;
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        let name = self.parse_identifier()?;
        let options = self.parse_options_clause()?;

        Ok(StatementKind::CreateDatabase(CreateDatabaseStatement {
            name,
            if_not_exists,
            options,
        }))
    }

    /// Parse ALTER statement.
    pub fn parse_alter(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Alter)?;

        if self.consume_keyword(Keyword::Table)?.is_some() {
            self.parse_alter_table()
        } else if self.consume_keyword(Keyword::View)?.is_some() {
            self.parse_alter_view()
        } else {
            let token = self.peek()?;
            Err(Error::unexpected_token(
                "TABLE or VIEW",
                format!("{}", token.kind),
                token.span,
            ))
        }
    }

    /// Parse ALTER TABLE statement.
    fn parse_alter_table(&mut self) -> Result<StatementKind> {
        let if_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        let name = self.parse_object_name()?;
        let action = self.parse_alter_table_action()?;

        Ok(StatementKind::AlterTable(AlterTableStatement {
            if_exists,
            name,
            action,
        }))
    }

    /// Parse ALTER TABLE action.
    fn parse_alter_table_action(&mut self) -> Result<AlterTableAction> {
        if self.consume_keyword(Keyword::Add)?.is_some() {
            if self.consume_keyword(Keyword::Column)?.is_some() {
                let if_not_exists = if self.consume_keyword(Keyword::If)?.is_some() {
                    self.expect_keyword(Keyword::Not)?;
                    self.expect_keyword(Keyword::Exists)?;
                    true
                } else {
                    false
                };
                let column = self.parse_column_def()?;
                Ok(AlterTableAction::AddColumn {
                    if_not_exists,
                    column,
                })
            } else if self.check_keyword(Keyword::Constraint)?
                || self.check_keyword(Keyword::Primary)?
                || self.check_keyword(Keyword::Unique)?
                || self.check_keyword(Keyword::Foreign)?
                || self.check_keyword(Keyword::Check)?
            {
                let constraint = self.parse_table_constraint()?;
                Ok(AlterTableAction::AddConstraint(constraint))
            } else {
                let column = self.parse_column_def()?;
                Ok(AlterTableAction::AddColumn {
                    if_not_exists: false,
                    column,
                })
            }
        } else if self.consume_keyword(Keyword::Drop)?.is_some() {
            if self.consume_keyword(Keyword::Column)?.is_some() {
                let if_exists = if self.consume_keyword(Keyword::If)?.is_some() {
                    self.expect_keyword(Keyword::Exists)?;
                    true
                } else {
                    false
                };
                let column = self.parse_identifier()?;
                Ok(AlterTableAction::DropColumn { if_exists, column })
            } else if self.consume_keyword(Keyword::Constraint)?.is_some() {
                let if_exists = if self.consume_keyword(Keyword::If)?.is_some() {
                    self.expect_keyword(Keyword::Exists)?;
                    true
                } else {
                    false
                };
                let name = self.parse_identifier()?;
                Ok(AlterTableAction::DropConstraint { if_exists, name })
            } else {
                let token = self.peek()?;
                Err(Error::unexpected_token(
                    "COLUMN or CONSTRAINT",
                    format!("{}", token.kind),
                    token.span,
                ))
            }
        } else if self.consume_keyword(Keyword::Alter)?.is_some() {
            self.consume_keyword(Keyword::Column)?;
            let column = self.parse_identifier()?;
            let action = self.parse_alter_column_action()?;
            Ok(AlterTableAction::AlterColumn { column, action })
        } else if self.consume_keyword(Keyword::Rename)?.is_some() {
            if self.consume_keyword(Keyword::Column)?.is_some() {
                let from = self.parse_identifier()?;
                self.expect_keyword(Keyword::To)?;
                let to = self.parse_identifier()?;
                Ok(AlterTableAction::RenameColumn { from, to })
            } else if self.consume_keyword(Keyword::To)?.is_some() {
                let new_name = self.parse_object_name()?;
                Ok(AlterTableAction::RenameTable(new_name))
            } else {
                let token = self.peek()?;
                Err(Error::unexpected_token(
                    "COLUMN or TO",
                    format!("{}", token.kind),
                    token.span,
                ))
            }
        } else if self.consume_keyword(Keyword::Set)?.is_some() {
            self.expect_keyword(Keyword::Options)?;
            self.expect(&TokenKind::LeftParen)?;
            let options = self.parse_comma_separated(|p| {
                let name = p.parse_identifier()?;
                p.expect(&TokenKind::Eq)?;
                let value = p.parse_expression()?;
                Ok(SqlOption { name, value })
            })?;
            self.expect(&TokenKind::RightParen)?;
            Ok(AlterTableAction::SetOptions(options))
        } else {
            let token = self.peek()?;
            Err(Error::unexpected_token(
                "ADD, DROP, ALTER, RENAME, or SET",
                format!("{}", token.kind),
                token.span,
            ))
        }
    }

    /// Parse ALTER COLUMN action.
    fn parse_alter_column_action(&mut self) -> Result<AlterColumnAction> {
        if self.consume_keyword(Keyword::Set)?.is_some() {
            if self.consume_keyword(Keyword::Data)?.is_some() {
                self.expect_keyword(Keyword::Type)?;
                let data_type = self.parse_data_type()?;
                Ok(AlterColumnAction::SetDataType(data_type))
            } else if self.consume_keyword(Keyword::Default)?.is_some() {
                let expr = self.parse_expression()?;
                Ok(AlterColumnAction::SetDefault(expr))
            } else if self.consume_keyword(Keyword::Not)?.is_some() {
                self.expect(&TokenKind::Null)?; // NULL is a special token
                Ok(AlterColumnAction::SetNotNull)
            } else if self.consume_keyword(Keyword::Options)?.is_some() {
                self.expect(&TokenKind::LeftParen)?;
                let options = self.parse_comma_separated(|p| {
                    let name = p.parse_identifier()?;
                    p.expect(&TokenKind::Eq)?;
                    let value = p.parse_expression()?;
                    Ok(SqlOption { name, value })
                })?;
                self.expect(&TokenKind::RightParen)?;
                Ok(AlterColumnAction::SetOptions(options))
            } else {
                let token = self.peek()?;
                Err(Error::unexpected_token(
                    "DATA TYPE, DEFAULT, NOT NULL, or OPTIONS",
                    format!("{}", token.kind),
                    token.span,
                ))
            }
        } else if self.consume_keyword(Keyword::Drop)?.is_some() {
            if self.consume_keyword(Keyword::Default)?.is_some() {
                Ok(AlterColumnAction::DropDefault)
            } else if self.consume_keyword(Keyword::Not)?.is_some() {
                self.expect(&TokenKind::Null)?; // NULL is a special token
                Ok(AlterColumnAction::DropNotNull)
            } else {
                let token = self.peek()?;
                Err(Error::unexpected_token(
                    "DEFAULT or NOT NULL",
                    format!("{}", token.kind),
                    token.span,
                ))
            }
        } else if self.consume_keyword(Keyword::Type)?.is_some() {
            let data_type = self.parse_data_type()?;
            Ok(AlterColumnAction::SetDataType(data_type))
        } else {
            let token = self.peek()?;
            Err(Error::unexpected_token(
                "SET, DROP, or TYPE",
                format!("{}", token.kind),
                token.span,
            ))
        }
    }

    /// Parse ALTER VIEW statement.
    fn parse_alter_view(&mut self) -> Result<StatementKind> {
        let if_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        let name = self.parse_object_name()?;

        let action = if self.consume_keyword(Keyword::Set)?.is_some() {
            self.expect_keyword(Keyword::Options)?;
            self.expect(&TokenKind::LeftParen)?;
            let options = self.parse_comma_separated(|p| {
                let name = p.parse_identifier()?;
                p.expect(&TokenKind::Eq)?;
                let value = p.parse_expression()?;
                Ok(SqlOption { name, value })
            })?;
            self.expect(&TokenKind::RightParen)?;
            AlterViewAction::SetOptions(options)
        } else if self.consume_keyword(Keyword::As)?.is_some() {
            let query = Box::new(self.parse_query()?);
            AlterViewAction::SetQuery(query)
        } else {
            let token = self.peek()?;
            return Err(Error::unexpected_token(
                "SET or AS",
                format!("{}", token.kind),
                token.span,
            ));
        };

        Ok(StatementKind::AlterView(AlterViewStatement {
            if_exists,
            name,
            action,
        }))
    }

    /// Parse DROP statement.
    pub fn parse_drop(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Drop)?;

        let object_type = if self.consume_keyword(Keyword::Table)?.is_some() {
            ObjectType::Table
        } else if self.consume_keyword(Keyword::View)?.is_some() {
            ObjectType::View
        } else if self.consume_keyword(Keyword::Materialized)?.is_some() {
            self.expect_keyword(Keyword::View)?;
            ObjectType::MaterializedView
        } else if self.consume_keyword(Keyword::Index)?.is_some() {
            ObjectType::Index
        } else if self.consume_keyword(Keyword::Function)?.is_some() {
            ObjectType::Function
        } else if self.consume_keyword(Keyword::Procedure)?.is_some() {
            ObjectType::Procedure
        } else if self.consume_keyword(Keyword::Database)?.is_some() {
            ObjectType::Database
        } else if self.consume_keyword(Keyword::Schema)?.is_some() {
            ObjectType::Schema
        } else {
            let token = self.peek()?;
            return Err(Error::unexpected_token(
                "TABLE, VIEW, INDEX, FUNCTION, PROCEDURE, DATABASE, or SCHEMA",
                format!("{}", token.kind),
                token.span,
            ));
        };

        let if_exists = if self.consume_keyword(Keyword::If)?.is_some() {
            self.expect_keyword(Keyword::Exists)?;
            true
        } else {
            false
        };

        let names = self.parse_comma_separated(|p| p.parse_object_name())?;

        let cascade = self.consume_keyword(Keyword::Cascade)?.is_some();
        if !cascade {
            self.consume_keyword(Keyword::Restrict)?;
        }

        Ok(StatementKind::Drop(DropStatement {
            object_type,
            if_exists,
            names,
            cascade,
        }))
    }

    /// Parse TRUNCATE statement.
    pub fn parse_truncate(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Truncate)?;
        self.consume_keyword(Keyword::Table)?;
        let table = self.parse_object_name()?;
        Ok(StatementKind::Truncate(TruncateStatement { table }))
    }

    // ========================================================================
    // Transaction Control
    // ========================================================================

    /// Parse BEGIN statement.
    pub fn parse_begin(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Begin)?;

        // Check if this is BEGIN TRANSACTION or BEGIN ... END block
        if self.consume_keyword(Keyword::Transaction)?.is_some()
            || self.consume_keyword(Keyword::Work)?.is_some()
            || self.check_keyword(Keyword::Read)?
            || self.check(&TokenKind::Semicolon)?
            || self.check_eof()?
        {
            // Transaction
            let mode = if self.consume_keyword(Keyword::Read)?.is_some() {
                if self.consume_keyword(Keyword::Only)?.is_some() {
                    Some(TransactionMode::ReadOnly)
                } else {
                    self.expect_keyword(Keyword::Write)?;
                    Some(TransactionMode::ReadWrite)
                }
            } else {
                None
            };
            Ok(StatementKind::Begin(BeginStatement { mode }))
        } else {
            // BEGIN ... END block (for procedures)
            let _statements = self.parse_statement_list()?;
            self.expect_keyword(Keyword::End)?;
            // Return as simple begin for now
            Ok(StatementKind::Begin(BeginStatement { mode: None }))
        }
    }

    /// Parse ROLLBACK statement.
    pub fn parse_rollback(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Rollback)?;
        self.consume_keyword(Keyword::Transaction)?;
        self.consume_keyword(Keyword::Work)?;

        let savepoint = if self.consume_keyword(Keyword::To)?.is_some() {
            self.consume_keyword(Keyword::Savepoint)?;
            Some(self.parse_identifier()?)
        } else {
            None
        };

        Ok(StatementKind::Rollback(RollbackStatement { savepoint }))
    }

    // ========================================================================
    // Utility Statements
    // ========================================================================

    /// Parse EXPLAIN statement.
    pub fn parse_explain(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Explain)?;

        let analyze = self.consume_keyword(Keyword::Analyze)?.is_some();

        let format = if self.consume_keyword(Keyword::Format)?.is_some() {
            if self.consume_keyword(Keyword::Text)?.is_some() {
                Some(ExplainFormat::Text)
            } else if self.consume_keyword(Keyword::Json)?.is_some() {
                Some(ExplainFormat::Json)
            } else {
                let token = self.peek()?;
                return Err(Error::unexpected_token(
                    "TEXT or JSON",
                    format!("{}", token.kind),
                    token.span,
                ));
            }
        } else {
            None
        };

        let statement = Box::new(self.parse_statement()?);

        Ok(StatementKind::Explain(ExplainStatement {
            analyze,
            format,
            statement,
        }))
    }

    /// Parse DESCRIBE statement.
    pub fn parse_describe(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Describe)?;
        let object = self.parse_object_name()?;
        Ok(StatementKind::Describe(DescribeStatement { object }))
    }

    /// Parse SHOW statement.
    pub fn parse_show(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Show)?;

        let object_type = if self.consume_keyword(Keyword::Tables)?.is_some() {
            let from = if self.consume_keyword(Keyword::From)?.is_some()
                || self.consume_keyword(Keyword::In)?.is_some()
            {
                Some(self.parse_identifier()?)
            } else {
                None
            };
            ShowObjectType::Tables { from }
        } else if self.consume_keyword(Keyword::Databases)?.is_some() {
            ShowObjectType::Databases
        } else if self.consume_keyword(Keyword::Schemas)?.is_some() {
            let from = if self.consume_keyword(Keyword::From)?.is_some()
                || self.consume_keyword(Keyword::In)?.is_some()
            {
                Some(self.parse_identifier()?)
            } else {
                None
            };
            ShowObjectType::Schemas { from }
        } else if self.consume_keyword(Keyword::Columns)?.is_some() {
            self.expect_keyword(Keyword::From)?;
            let from = self.parse_object_name()?;
            ShowObjectType::Columns { from }
        } else if self.consume_keyword(Keyword::Functions)?.is_some() {
            let from = if self.consume_keyword(Keyword::From)?.is_some()
                || self.consume_keyword(Keyword::In)?.is_some()
            {
                Some(self.parse_identifier()?)
            } else {
                None
            };
            ShowObjectType::Functions { from }
        } else if self.consume_keyword(Keyword::Variables)?.is_some() {
            ShowObjectType::Variables
        } else {
            let token = self.peek()?;
            return Err(Error::unexpected_token(
                "TABLES, DATABASES, SCHEMAS, COLUMNS, FUNCTIONS, or VARIABLES",
                format!("{}", token.kind),
                token.span,
            ));
        };

        let filter = if self.consume_keyword(Keyword::Like)?.is_some() {
            let token = self.advance()?;
            match token.kind {
                TokenKind::String(s) => Some(ShowFilter::Like(s)),
                _ => {
                    return Err(Error::unexpected_token(
                        "string literal",
                        format!("{}", token.kind),
                        token.span,
                    ));
                }
            }
        } else if self.consume_keyword(Keyword::Where)?.is_some() {
            Some(ShowFilter::Where(self.parse_expression()?))
        } else {
            None
        };

        Ok(StatementKind::Show(ShowStatement {
            object_type,
            filter,
        }))
    }

    /// Parse SET statement.
    pub fn parse_set(&mut self) -> Result<StatementKind> {
        self.expect_keyword(Keyword::Set)?;

        let variable = self.parse_identifier()?;

        self.expect(&TokenKind::Eq)?;

        let value = if self.consume_keyword(Keyword::Default)?.is_some() {
            SetValue::Default
        } else {
            SetValue::Expr(self.parse_expression()?)
        };

        Ok(StatementKind::Set(SetStatement { variable, value }))
    }

    /// Parse a list of statements (for procedure bodies).
    fn parse_statement_list(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.check_keyword(Keyword::End)?
            && !self.check_keyword(Keyword::Exception)?
            && !self.check_eof()?
        {
            if self.consume(&TokenKind::Semicolon)?.is_some() {
                continue;
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.consume(&TokenKind::Semicolon)?;
        }

        Ok(statements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn parse_stmt(sql: &str) -> Statement {
        let mut parser = Parser::new(sql);
        parser.parse_statement().expect("Failed to parse")
    }

    #[test]
    fn test_insert_values() {
        let stmt = parse_stmt("INSERT INTO users (id, name) VALUES (1, 'Alice')");
        assert!(matches!(stmt.kind, StatementKind::Insert(_)));
    }

    #[test]
    fn test_insert_select() {
        let stmt = parse_stmt("INSERT INTO archive SELECT * FROM users WHERE active = false");
        assert!(matches!(stmt.kind, StatementKind::Insert(_)));
    }

    #[test]
    fn test_update() {
        let stmt = parse_stmt("UPDATE users SET name = 'Bob' WHERE id = 1");
        assert!(matches!(stmt.kind, StatementKind::Update(_)));
    }

    #[test]
    fn test_delete() {
        let stmt = parse_stmt("DELETE FROM users WHERE id = 1");
        assert!(matches!(stmt.kind, StatementKind::Delete(_)));
    }

    #[test]
    fn test_create_table() {
        let stmt = parse_stmt(
            "CREATE TABLE users (
                id INT64 NOT NULL PRIMARY KEY,
                name STRING,
                email STRING UNIQUE
            )",
        );
        assert!(matches!(stmt.kind, StatementKind::CreateTable(_)));
    }

    #[test]
    fn test_create_view() {
        let stmt =
            parse_stmt("CREATE VIEW active_users AS SELECT * FROM users WHERE active = true");
        assert!(matches!(stmt.kind, StatementKind::CreateView(_)));
    }

    #[test]
    fn test_drop_table() {
        let stmt = parse_stmt("DROP TABLE IF EXISTS users CASCADE");
        assert!(matches!(stmt.kind, StatementKind::Drop(_)));
    }

    #[test]
    fn test_alter_table() {
        let stmt = parse_stmt("ALTER TABLE users ADD COLUMN age INT64");
        assert!(matches!(stmt.kind, StatementKind::AlterTable(_)));
    }

    #[test]
    fn test_begin_commit() {
        let stmt = parse_stmt("BEGIN TRANSACTION");
        assert!(matches!(stmt.kind, StatementKind::Begin(_)));
    }

    #[test]
    fn test_rollback() {
        let stmt = parse_stmt("ROLLBACK");
        assert!(matches!(stmt.kind, StatementKind::Rollback(_)));
    }

    #[test]
    fn test_explain() {
        let stmt = parse_stmt("EXPLAIN SELECT * FROM users");
        assert!(matches!(stmt.kind, StatementKind::Explain(_)));
    }

    #[test]
    fn test_update_nested_field() {
        use crate::AssignmentTarget;

        // Test UPDATE with nested field assignment (e.g., struct.field = value)
        let stmt = parse_stmt("UPDATE users SET address.city = 'NYC' WHERE id = 1");
        if let StatementKind::Update(update) = &stmt.kind {
            assert_eq!(update.assignments.len(), 1);
            let assignment = &update.assignments[0];
            // Check that the target is a Path (compound identifier)
            if let AssignmentTarget::Path(parts) = &assignment.target {
                assert_eq!(parts.len(), 2);
                assert_eq!(parts[0].value, "address");
                assert_eq!(parts[1].value, "city");
            } else {
                panic!("Expected Path target for nested field");
            }
        } else {
            panic!("Expected UPDATE statement");
        }
    }

    #[test]
    fn test_update_deeply_nested_field() {
        use crate::AssignmentTarget;

        // Test UPDATE with deeply nested field assignment
        let stmt = parse_stmt("UPDATE users SET profile.contact.email = 'test@example.com'");
        if let StatementKind::Update(update) = &stmt.kind {
            assert_eq!(update.assignments.len(), 1);
            let assignment = &update.assignments[0];
            if let AssignmentTarget::Path(parts) = &assignment.target {
                assert_eq!(parts.len(), 3);
                assert_eq!(parts[0].value, "profile");
                assert_eq!(parts[1].value, "contact");
                assert_eq!(parts[2].value, "email");
            } else {
                panic!("Expected Path target for nested field");
            }
        } else {
            panic!("Expected UPDATE statement");
        }
    }

    #[test]
    fn test_update_simple_column() {
        use crate::AssignmentTarget;

        let stmt = parse_stmt("UPDATE users SET name = 'Bob'");
        if let StatementKind::Update(update) = &stmt.kind {
            assert_eq!(update.assignments.len(), 1);
            let assignment = &update.assignments[0];
            // Simple column assignment could be Column or Path with length 1
            match &assignment.target {
                AssignmentTarget::Column(ident) => {
                    assert_eq!(ident.value, "name");
                }
                AssignmentTarget::Path(parts) => {
                    assert_eq!(parts.len(), 1);
                    assert_eq!(parts[0].value, "name");
                }
            }
        } else {
            panic!("Expected UPDATE statement");
        }
    }

    #[test]
    fn test_update_multiple_assignments() {
        let stmt = parse_stmt("UPDATE users SET name = 'Bob', age = 30, active = true");
        if let StatementKind::Update(update) = &stmt.kind {
            assert_eq!(update.assignments.len(), 3);
        } else {
            panic!("Expected UPDATE statement");
        }
    }
}
