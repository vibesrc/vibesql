//! Query (SELECT) statement parsing.

use super::Parser;
use crate::ast::*;
use crate::error::{Error, Result, Span};
use crate::lexer::{Keyword, TokenKind};

impl<'a> Parser<'a> {
    /// Parse a complete query (WITH clause, SELECT, set operations, ORDER BY, LIMIT).
    pub fn parse_query(&mut self) -> Result<Query> {
        let start = self.current_position();

        // Parse optional WITH clause
        let with = if self.check_keyword(Keyword::With)? {
            Some(self.parse_with_clause()?)
        } else {
            None
        };

        // Parse the query body
        let body = self.parse_query_body()?;

        // Parse ORDER BY
        let order_by = if self.consume_keyword(Keyword::Order)?.is_some() {
            self.expect_keyword(Keyword::By)?;
            self.parse_comma_separated(|p| p.parse_order_by_expr())?
        } else {
            Vec::new()
        };

        // Parse LIMIT/OFFSET
        let limit = self.parse_limit_clause()?;

        let end = self.current_position();
        Ok(Query {
            with,
            body,
            order_by,
            limit,
            span: Span::new(start, end),
        })
    }

    /// Parse WITH clause (Common Table Expressions).
    fn parse_with_clause(&mut self) -> Result<WithClause> {
        let start = self.current_position();
        self.expect_keyword(Keyword::With)?;

        let recursive = self.consume_keyword(Keyword::Recursive)?.is_some();
        let ctes = self.parse_comma_separated(|p| p.parse_cte())?;

        let end = self.current_position();
        Ok(WithClause {
            recursive,
            ctes,
            span: Span::new(start, end),
        })
    }

    /// Parse a single CTE.
    fn parse_cte(&mut self) -> Result<Cte> {
        let start = self.current_position();
        let name = self.parse_identifier()?;

        // Optional column list
        let columns = if self.consume(&TokenKind::LeftParen)?.is_some() {
            let cols = self.parse_comma_separated(|p| p.parse_identifier())?;
            self.expect(&TokenKind::RightParen)?;
            cols
        } else {
            Vec::new()
        };

        self.expect_keyword(Keyword::As)?;
        self.expect(&TokenKind::LeftParen)?;
        let query = Box::new(self.parse_query()?);
        self.expect(&TokenKind::RightParen)?;

        let end = self.current_position();
        Ok(Cte {
            name,
            columns,
            query,
            span: Span::new(start, end),
        })
    }

    /// Parse query body (SELECT, set operations, or parenthesized query).
    fn parse_query_body(&mut self) -> Result<QueryBody> {
        let mut left = self.parse_query_primary()?;

        // Check for set operations
        loop {
            let op = if self.consume_keyword(Keyword::Union)?.is_some() {
                Some(SetOperator::Union)
            } else if self.consume_keyword(Keyword::Intersect)?.is_some() {
                Some(SetOperator::Intersect)
            } else if self.consume_keyword(Keyword::Except)?.is_some() {
                Some(SetOperator::Except)
            } else {
                None
            };

            if let Some(op) = op {
                let all = self.consume_keyword(Keyword::All)?.is_some();
                if !all {
                    self.consume_keyword(Keyword::Distinct)?;
                }
                let right = self.parse_query_primary()?;
                left = QueryBody::SetOperation {
                    op,
                    all,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse a primary query (SELECT or parenthesized query).
    fn parse_query_primary(&mut self) -> Result<QueryBody> {
        if self.consume(&TokenKind::LeftParen)?.is_some() {
            let query = self.parse_query()?;
            self.expect(&TokenKind::RightParen)?;
            Ok(QueryBody::Parenthesized(Box::new(query)))
        } else {
            let select = self.parse_select()?;
            Ok(QueryBody::Select(Box::new(select)))
        }
    }

    /// Parse a SELECT statement.
    fn parse_select(&mut self) -> Result<Select> {
        let start = self.current_position();
        self.expect_keyword(Keyword::Select)?;

        // Parse SELECT AS STRUCT/VALUE (value table syntax)
        let select_as = self.parse_select_as()?;

        // Parse DISTINCT/ALL
        let distinct = if self.consume_keyword(Keyword::Distinct)?.is_some() {
            Some(Distinct::Distinct)
        } else if self.consume_keyword(Keyword::All)?.is_some() {
            Some(Distinct::All)
        } else {
            None
        };

        // Parse projection list
        let projection = self.parse_comma_separated(|p| p.parse_select_item())?;

        // Parse FROM clause
        let from = if self.consume_keyword(Keyword::From)?.is_some() {
            Some(self.parse_from_clause()?)
        } else {
            None
        };

        // Parse WHERE clause
        let where_clause = if self.consume_keyword(Keyword::Where)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Parse GROUP BY clause
        let group_by = if self.consume_keyword(Keyword::Group)?.is_some() {
            self.expect_keyword(Keyword::By)?;
            Some(self.parse_group_by_clause()?)
        } else {
            None
        };

        // Parse HAVING clause
        let having = if self.consume_keyword(Keyword::Having)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Parse QUALIFY clause (window function filter)
        let qualify = if self.consume_keyword(Keyword::Qualify)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Parse WINDOW clause
        let window = if self.consume_keyword(Keyword::Window)?.is_some() {
            self.parse_comma_separated(|p| p.parse_named_window_def())?
        } else {
            Vec::new()
        };

        let end = self.current_position();
        Ok(Select {
            distinct,
            select_as,
            projection,
            from,
            where_clause,
            group_by,
            having,
            qualify,
            window,
            span: Span::new(start, end),
        })
    }

    /// Parse SELECT AS STRUCT/VALUE modifier.
    fn parse_select_as(&mut self) -> Result<Option<SelectAs>> {
        // Check if we have 'AS' next (but not as part of an alias - lookahead for STRUCT/VALUE)
        if self.check_keyword(Keyword::As)? {
            // Peek at the token after AS
            let next_token = self.peek_nth(1)?;
            match &next_token.kind {
                TokenKind::Keyword(Keyword::Struct) => {
                    self.consume_keyword(Keyword::As)?;
                    self.consume_keyword(Keyword::Struct)?;
                    return Ok(Some(SelectAs::Struct));
                }
                TokenKind::Keyword(Keyword::Value) => {
                    self.consume_keyword(Keyword::As)?;
                    self.consume_keyword(Keyword::Value)?;
                    return Ok(Some(SelectAs::Value));
                }
                // Could be a type name like `SELECT AS myproto.Message`
                TokenKind::Identifier(_) | TokenKind::QuotedIdentifier(_) => {
                    self.consume_keyword(Keyword::As)?;
                    let type_name = self.parse_object_name()?;
                    return Ok(Some(SelectAs::TypeName(type_name)));
                }
                _ => {}
            }
        }
        Ok(None)
    }

    /// Parse a SELECT item.
    pub(super) fn parse_select_item(&mut self) -> Result<SelectItem> {
        // Check for wildcard
        if self.consume(&TokenKind::Star)?.is_some() {
            // Check for EXCEPT or REPLACE
            if self.consume_keyword(Keyword::Except)?.is_some() {
                self.expect(&TokenKind::LeftParen)?;
                let except = self.parse_comma_separated(|p| p.parse_identifier())?;
                self.expect(&TokenKind::RightParen)?;
                return Ok(SelectItem::WildcardExcept {
                    qualifier: None,
                    except,
                });
            } else if self.consume_keyword(Keyword::Replace)?.is_some() {
                self.expect(&TokenKind::LeftParen)?;
                let replace = self.parse_comma_separated(|p| {
                    let expr = p.parse_expression()?;
                    p.expect_keyword(Keyword::As)?;
                    let alias = p.parse_identifier()?;
                    Ok((expr, alias))
                })?;
                self.expect(&TokenKind::RightParen)?;
                return Ok(SelectItem::WildcardReplace {
                    qualifier: None,
                    replace,
                });
            }
            return Ok(SelectItem::Wildcard);
        }

        // Parse expression
        let expr = self.parse_expression()?;

        // Check for qualified wildcard: expr.*
        if let ExprKind::Identifier(ref ident) = expr.kind {
            if self.consume(&TokenKind::Dot)?.is_some() && self.consume(&TokenKind::Star)?.is_some()
            {
                let qualifier = ObjectName::simple(ident.clone());
                // Check for EXCEPT or REPLACE
                if self.consume_keyword(Keyword::Except)?.is_some() {
                    self.expect(&TokenKind::LeftParen)?;
                    let except = self.parse_comma_separated(|p| p.parse_identifier())?;
                    self.expect(&TokenKind::RightParen)?;
                    return Ok(SelectItem::WildcardExcept {
                        qualifier: Some(qualifier),
                        except,
                    });
                } else if self.consume_keyword(Keyword::Replace)?.is_some() {
                    self.expect(&TokenKind::LeftParen)?;
                    let replace = self.parse_comma_separated(|p| {
                        let expr = p.parse_expression()?;
                        p.expect_keyword(Keyword::As)?;
                        let alias = p.parse_identifier()?;
                        Ok((expr, alias))
                    })?;
                    self.expect(&TokenKind::RightParen)?;
                    return Ok(SelectItem::WildcardReplace {
                        qualifier: Some(qualifier),
                        replace,
                    });
                }
                return Ok(SelectItem::QualifiedWildcard { qualifier });
            }
        }

        // Parse optional alias
        let alias = self.parse_optional_alias()?;

        Ok(SelectItem::Expr { expr, alias })
    }

    /// Parse FROM clause.
    fn parse_from_clause(&mut self) -> Result<FromClause> {
        let tables = self.parse_comma_separated(|p| p.parse_table_ref())?;
        Ok(FromClause { tables })
    }

    /// Parse a table reference.
    pub(super) fn parse_table_ref(&mut self) -> Result<TableRef> {
        let mut left = self.parse_table_primary()?;

        // Parse joins
        loop {
            let join_type = self.parse_join_type()?;
            if let Some(jt) = join_type {
                let right = self.parse_table_primary()?;
                let condition = self.parse_join_condition(jt)?;
                left = TableRef::Join {
                    left: Box::new(left),
                    right: Box::new(right),
                    join_type: jt,
                    condition,
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse a primary table reference.
    fn parse_table_primary(&mut self) -> Result<TableRef> {
        // Check for parenthesized table ref or subquery
        if self.consume(&TokenKind::LeftParen)?.is_some() {
            // Could be subquery or parenthesized table ref
            if self.check_keyword(Keyword::Select)? || self.check_keyword(Keyword::With)? {
                let query = self.parse_query()?;
                self.expect(&TokenKind::RightParen)?;
                let alias = self.parse_optional_table_alias()?;
                return Ok(TableRef::Subquery {
                    query: Box::new(query),
                    alias,
                });
            } else {
                let inner = self.parse_table_ref()?;
                self.expect(&TokenKind::RightParen)?;
                return Ok(TableRef::Parenthesized(Box::new(inner)));
            }
        }

        // Check for UNNEST
        if self.consume_keyword(Keyword::Unnest)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let expr = self.parse_expression()?;
            self.expect(&TokenKind::RightParen)?;
            let alias = self.parse_optional_table_alias()?;
            let (with_offset, offset_alias) = if self.consume_keyword(Keyword::With)?.is_some() {
                self.expect_keyword(Keyword::Offset)?;
                let offset_alias = if self.consume_keyword(Keyword::As)?.is_some() {
                    Some(self.parse_identifier()?)
                } else {
                    None
                };
                (true, offset_alias)
            } else {
                (false, None)
            };
            return Ok(TableRef::Unnest {
                expr,
                alias,
                with_offset,
                offset_alias,
            });
        }

        // Simple table reference
        let name = self.parse_object_name()?;

        // Check if this is a table function call
        if self.consume(&TokenKind::LeftParen)?.is_some() {
            let args = if self.check(&TokenKind::RightParen)? {
                Vec::new()
            } else {
                self.parse_comma_separated(|p| p.parse_function_arg())?
            };
            self.expect(&TokenKind::RightParen)?;
            let alias = self.parse_optional_table_alias()?;
            return Ok(TableRef::TableFunction { name, args, alias });
        }

        // Parse optional hints
        let hints = if self.consume(&TokenKind::At)?.is_some() {
            self.expect(&TokenKind::LeftBrace)?;
            let opts = self.parse_comma_separated(|p| p.parse_sql_option())?;
            self.expect(&TokenKind::RightBrace)?;
            opts
        } else {
            Vec::new()
        };

        let alias = self.parse_optional_table_alias()?;
        Ok(TableRef::Table { name, alias, hints })
    }

    /// Parse SQL option (key = value).
    fn parse_sql_option(&mut self) -> Result<SqlOption> {
        let name = self.parse_identifier()?;
        self.expect(&TokenKind::Eq)?;
        let value = self.parse_expression()?;
        Ok(SqlOption { name, value })
    }

    /// Parse a function argument.
    pub(super) fn parse_function_arg(&mut self) -> Result<FunctionArg> {
        // Check for named argument
        let token = self.peek()?;
        if let TokenKind::Identifier(_) = &token.kind {
            let next = self.peek_nth(1)?;
            if matches!(next.kind, TokenKind::FatArrow) {
                let name = self.parse_identifier()?;
                self.advance()?; // consume =>
                let expr = self.parse_expression()?;
                return Ok(FunctionArg::Named { name, value: expr });
            }
        }

        let expr = self.parse_expression()?;
        Ok(FunctionArg::Unnamed(expr))
    }

    /// Parse JOIN type.
    fn parse_join_type(&mut self) -> Result<Option<JoinType>> {
        if self.consume_keyword(Keyword::Cross)?.is_some() {
            self.expect_keyword(Keyword::Join)?;
            return Ok(Some(JoinType::Cross));
        }

        if self.consume_keyword(Keyword::Natural)?.is_some() {
            self.consume_keyword(Keyword::Inner)?;
            self.expect_keyword(Keyword::Join)?;
            return Ok(Some(JoinType::Natural));
        }

        let join_type = if self.consume_keyword(Keyword::Inner)?.is_some() {
            Some(JoinType::Inner)
        } else if self.consume_keyword(Keyword::Left)?.is_some() {
            if self.consume_keyword(Keyword::Semi)?.is_some() {
                Some(JoinType::LeftSemi)
            } else if self.consume_keyword(Keyword::Anti)?.is_some() {
                Some(JoinType::LeftAnti)
            } else {
                self.consume_keyword(Keyword::Outer)?;
                Some(JoinType::Left)
            }
        } else if self.consume_keyword(Keyword::Right)?.is_some() {
            if self.consume_keyword(Keyword::Semi)?.is_some() {
                Some(JoinType::RightSemi)
            } else if self.consume_keyword(Keyword::Anti)?.is_some() {
                Some(JoinType::RightAnti)
            } else {
                self.consume_keyword(Keyword::Outer)?;
                Some(JoinType::Right)
            }
        } else if self.consume_keyword(Keyword::Full)?.is_some() {
            self.consume_keyword(Keyword::Outer)?;
            Some(JoinType::Full)
        } else {
            None
        };

        if let Some(jt) = join_type {
            self.expect_keyword(Keyword::Join)?;
            return Ok(Some(jt));
        }

        // Plain JOIN is INNER JOIN
        if self.consume_keyword(Keyword::Join)?.is_some() {
            return Ok(Some(JoinType::Inner));
        }

        Ok(None)
    }

    /// Parse JOIN condition.
    fn parse_join_condition(&mut self, join_type: JoinType) -> Result<Option<JoinCondition>> {
        match join_type {
            JoinType::Cross | JoinType::Natural => Ok(None),
            _ => {
                if self.consume_keyword(Keyword::On)?.is_some() {
                    let expr = self.parse_expression()?;
                    Ok(Some(JoinCondition::On(expr)))
                } else if self.consume_keyword(Keyword::Using)?.is_some() {
                    self.expect(&TokenKind::LeftParen)?;
                    let columns = self.parse_comma_separated(|p| p.parse_identifier())?;
                    self.expect(&TokenKind::RightParen)?;
                    Ok(Some(JoinCondition::Using(columns)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Parse GROUP BY clause.
    fn parse_group_by_clause(&mut self) -> Result<GroupByClause> {
        let items = self.parse_comma_separated(|p| p.parse_group_by_item())?;
        Ok(GroupByClause { items })
    }

    /// Parse a GROUP BY item.
    fn parse_group_by_item(&mut self) -> Result<GroupByItem> {
        if self.consume_keyword(Keyword::Rollup)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let exprs = self.parse_comma_separated(|p| p.parse_expression())?;
            self.expect(&TokenKind::RightParen)?;
            return Ok(GroupByItem::Rollup(exprs));
        }

        if self.consume_keyword(Keyword::Cube)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let exprs = self.parse_comma_separated(|p| p.parse_expression())?;
            self.expect(&TokenKind::RightParen)?;
            return Ok(GroupByItem::Cube(exprs));
        }

        if self.consume_keyword(Keyword::Grouping)?.is_some() {
            self.expect_keyword(Keyword::Sets)?;
            self.expect(&TokenKind::LeftParen)?;
            let sets = self.parse_comma_separated(|p| {
                if p.consume(&TokenKind::LeftParen)?.is_some() {
                    let exprs = p.parse_comma_separated(|p2| p2.parse_expression())?;
                    p.expect(&TokenKind::RightParen)?;
                    Ok(exprs)
                } else {
                    Ok(vec![p.parse_expression()?])
                }
            })?;
            self.expect(&TokenKind::RightParen)?;
            return Ok(GroupByItem::GroupingSets(sets));
        }

        let expr = self.parse_expression()?;
        Ok(GroupByItem::Expr(expr))
    }

    /// Parse ORDER BY expression.
    pub(super) fn parse_order_by_expr(&mut self) -> Result<OrderByExpr> {
        let expr = self.parse_expression()?;

        let order = if self.consume_keyword(Keyword::Asc)?.is_some() {
            Some(SortOrder::Asc)
        } else if self.consume_keyword(Keyword::Desc)?.is_some() {
            Some(SortOrder::Desc)
        } else {
            None
        };

        let nulls = if self.consume_keyword(Keyword::Nulls)?.is_some() {
            if self.consume_keyword(Keyword::First)?.is_some() {
                Some(NullsOrder::First)
            } else if self.consume_keyword(Keyword::Last)?.is_some() {
                Some(NullsOrder::Last)
            } else {
                let token = self.peek()?;
                return Err(Error::unexpected_token(
                    "FIRST or LAST",
                    format!("{}", token.kind),
                    token.span,
                ));
            }
        } else {
            None
        };

        Ok(OrderByExpr { expr, order, nulls })
    }

    /// Parse LIMIT/OFFSET clause.
    fn parse_limit_clause(&mut self) -> Result<Option<LimitClause>> {
        if self.consume_keyword(Keyword::Limit)?.is_some() {
            let count = if self.consume_keyword(Keyword::All)?.is_some() {
                None
            } else {
                Some(self.parse_expression()?)
            };

            let offset = if self.consume_keyword(Keyword::Offset)?.is_some() {
                Some(self.parse_expression()?)
            } else {
                None
            };

            Ok(Some(LimitClause { count, offset }))
        } else if self.consume_keyword(Keyword::Offset)?.is_some() {
            let offset = Some(self.parse_expression()?);

            let count = if self.consume_keyword(Keyword::Limit)?.is_some() {
                if self.consume_keyword(Keyword::All)?.is_some() {
                    None
                } else {
                    Some(self.parse_expression()?)
                }
            } else {
                None
            };

            Ok(Some(LimitClause { count, offset }))
        } else {
            Ok(None)
        }
    }

    /// Parse a named window definition.
    fn parse_named_window_def(&mut self) -> Result<WindowDef> {
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::As)?;
        let spec = self.parse_window_spec()?;
        Ok(WindowDef { name, spec })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn parse_query(sql: &str) -> Query {
        let mut parser = Parser::new(sql);
        parser.parse_query().expect("Failed to parse query")
    }

    #[test]
    fn test_simple_select() {
        let query = parse_query("SELECT 1");
        assert!(matches!(query.body, QueryBody::Select(_)));
    }

    #[test]
    fn test_select_from() {
        let query = parse_query("SELECT * FROM users");
        if let QueryBody::Select(select) = query.body {
            assert!(select.from.is_some());
            assert_eq!(select.projection.len(), 1);
        } else {
            panic!("Expected SELECT");
        }
    }

    #[test]
    fn test_select_where() {
        let query = parse_query("SELECT id FROM users WHERE active = true");
        if let QueryBody::Select(select) = query.body {
            assert!(select.where_clause.is_some());
        } else {
            panic!("Expected SELECT");
        }
    }

    #[test]
    fn test_select_join() {
        let query = parse_query("SELECT * FROM users u JOIN orders o ON u.id = o.user_id");
        if let QueryBody::Select(select) = query.body {
            assert!(select.from.is_some());
        } else {
            panic!("Expected SELECT");
        }
    }

    #[test]
    fn test_select_group_by() {
        let query = parse_query("SELECT department, COUNT(*) FROM employees GROUP BY department");
        if let QueryBody::Select(select) = query.body {
            assert!(select.group_by.is_some());
        } else {
            panic!("Expected SELECT");
        }
    }

    #[test]
    fn test_union() {
        let query = parse_query("SELECT 1 UNION ALL SELECT 2");
        assert!(matches!(query.body, QueryBody::SetOperation { .. }));
    }

    #[test]
    fn test_with_clause() {
        let query = parse_query("WITH cte AS (SELECT 1 AS x) SELECT * FROM cte");
        assert!(query.with.is_some());
    }

    #[test]
    fn test_order_by_limit() {
        let query = parse_query("SELECT * FROM t ORDER BY id DESC LIMIT 10 OFFSET 5");
        assert_eq!(query.order_by.len(), 1);
        assert!(query.limit.is_some());
    }
}
