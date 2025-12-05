//! Expression parser.
//!
//! Implements expression parsing using a Pratt parser (precedence climbing)
//! for handling operator precedence correctly.

use crate::ast::*;
use crate::error::{Error, Result, Span};
use crate::lexer::{Keyword, TokenKind};

use super::Parser;

impl<'a> Parser<'a> {
    /// Parse an expression.
    pub fn parse_expression(&mut self) -> Result<Box<Expr>> {
        self.parse_expression_with_precedence(0)
    }

    /// Parse an expression with a minimum precedence.
    fn parse_expression_with_precedence(&mut self, min_precedence: u8) -> Result<Box<Expr>> {
        let mut left = self.parse_unary_expression()?;

        loop {
            // Clone the token kind to avoid borrow issues
            let token_kind = self.peek()?.kind.clone();

            // Check for binary operator
            if let Some((op, precedence)) = self.get_binary_op(&token_kind) {
                if precedence < min_precedence {
                    break;
                }

                self.advance()?;
                let next_precedence = if op.is_left_associative() {
                    precedence + 1
                } else {
                    precedence
                };

                let right = self.parse_expression_with_precedence(next_precedence)?;
                let span = left.span.merge(right.span);

                left = Expr::boxed(ExprKind::BinaryOp { op, left, right }, span);
            }
            // Check for AND
            else if self.check_keyword(Keyword::And)? {
                if BinaryOp::And.precedence() < min_precedence {
                    break;
                }
                self.advance()?;
                let right =
                    self.parse_expression_with_precedence(BinaryOp::And.precedence() + 1)?;
                let span = left.span.merge(right.span);
                left = Expr::boxed(
                    ExprKind::BinaryOp {
                        op: BinaryOp::And,
                        left,
                        right,
                    },
                    span,
                );
            }
            // Check for OR
            else if self.check_keyword(Keyword::Or)? {
                if BinaryOp::Or.precedence() < min_precedence {
                    break;
                }
                self.advance()?;
                let right = self.parse_expression_with_precedence(BinaryOp::Or.precedence() + 1)?;
                let span = left.span.merge(right.span);
                left = Expr::boxed(
                    ExprKind::BinaryOp {
                        op: BinaryOp::Or,
                        left,
                        right,
                    },
                    span,
                );
            }
            // Check for postfix operators and special expressions
            else if let Some(new_left) = self.try_parse_postfix_expression(left.clone())? {
                left = new_left;
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse a unary expression (NOT, -, +, ~).
    fn parse_unary_expression(&mut self) -> Result<Box<Expr>> {
        // Clone token info to avoid borrow issues
        let (start, token_kind) = {
            let token = self.peek()?;
            (token.span.start, token.kind.clone())
        };

        // NOT
        if self.check_keyword(Keyword::Not)? {
            self.advance()?;
            let expr = self.parse_unary_expression()?;
            let span = Span::new(start, expr.span.end);
            return Ok(Expr::boxed(
                ExprKind::UnaryOp {
                    op: UnaryOp::Not,
                    expr,
                },
                span,
            ));
        }

        // EXISTS
        if self.check_keyword(Keyword::Exists)? {
            return self.parse_exists_expression();
        }

        // CASE
        if self.check_keyword(Keyword::Case)? {
            return self.parse_case_expression();
        }

        // CAST / SAFE_CAST
        if self.check_keyword(Keyword::Cast)? || self.check_keyword(Keyword::SafeCast)? {
            return self.parse_cast_expression();
        }

        // EXTRACT
        if self.check_keyword(Keyword::Extract)? {
            return self.parse_extract_expression();
        }

        // INTERVAL
        if self.check_keyword(Keyword::Interval)? {
            return self.parse_interval_expression();
        }

        // ARRAY
        if self.check_keyword(Keyword::Array)? {
            return self.parse_array_expression();
        }

        // STRUCT
        if self.check_keyword(Keyword::Struct)? {
            return self.parse_struct_expression();
        }

        // IF
        if self.check_keyword(Keyword::If)? {
            return self.parse_if_expression();
        }

        // Unary + and -
        match &token_kind {
            TokenKind::Plus => {
                self.advance()?;
                let expr = self.parse_unary_expression()?;
                let span = Span::new(start, expr.span.end);
                return Ok(Expr::boxed(
                    ExprKind::UnaryOp {
                        op: UnaryOp::Plus,
                        expr,
                    },
                    span,
                ));
            }
            TokenKind::Minus => {
                self.advance()?;
                let expr = self.parse_unary_expression()?;
                let span = Span::new(start, expr.span.end);
                return Ok(Expr::boxed(
                    ExprKind::UnaryOp {
                        op: UnaryOp::Minus,
                        expr,
                    },
                    span,
                ));
            }
            TokenKind::Tilde => {
                self.advance()?;
                let expr = self.parse_unary_expression()?;
                let span = Span::new(start, expr.span.end);
                return Ok(Expr::boxed(
                    ExprKind::UnaryOp {
                        op: UnaryOp::BitwiseNot,
                        expr,
                    },
                    span,
                ));
            }
            _ => {}
        }

        self.parse_primary_expression()
    }

    /// Parse a primary expression (atoms: literals, identifiers, function calls, etc.).
    fn parse_primary_expression(&mut self) -> Result<Box<Expr>> {
        // Clone token info to avoid borrow conflicts
        let (span, token_kind) = {
            let token = self.peek()?;
            (token.span, token.kind.clone())
        };

        match token_kind {
            // Literals
            TokenKind::Null => {
                self.advance()?;
                Ok(Expr::boxed(ExprKind::Null, span))
            }
            TokenKind::Boolean(value) => {
                self.advance()?;
                Ok(Expr::boxed(ExprKind::Boolean(value), span))
            }
            TokenKind::Integer(value) => {
                self.advance()?;
                Ok(Expr::boxed(ExprKind::Integer(value), span))
            }
            TokenKind::Float(value) => {
                self.advance()?;
                Ok(Expr::boxed(ExprKind::Float(value), span))
            }
            TokenKind::String(value) => {
                self.advance()?;
                Ok(Expr::boxed(ExprKind::String(value), span))
            }
            TokenKind::Bytes(value) => {
                self.advance()?;
                Ok(Expr::boxed(ExprKind::Bytes(value), span))
            }

            // Typed literals (DATE, TIME, TIMESTAMP, JSON, etc.)
            TokenKind::Keyword(kw) => {
                if let Some(lit_type) = self.keyword_to_typed_literal(kw) {
                    self.advance()?;
                    let string_token = self.advance()?;
                    if let TokenKind::String(value) = string_token.kind {
                        let end_span = string_token.span;
                        return Ok(Expr::boxed(
                            ExprKind::TypedLiteral {
                                data_type: lit_type,
                                value,
                            },
                            Span::new(span.start, end_span.end),
                        ));
                    } else {
                        return Err(Error::unexpected_token(
                            "string literal",
                            format!("{}", string_token.kind),
                            string_token.span,
                        ));
                    }
                }

                // Function call or identifier
                if kw.is_reserved() {
                    return Err(Error::unexpected_token(
                        "expression",
                        format!("keyword {}", kw),
                        span,
                    ));
                }

                self.parse_identifier_or_function()
            }

            // Identifiers and function calls
            TokenKind::Identifier(_) | TokenKind::QuotedIdentifier(_) => {
                self.parse_identifier_or_function()
            }

            // Parameters
            TokenKind::At => {
                self.advance()?;
                let name_token = self.advance()?;
                let name = match name_token.kind {
                    TokenKind::Identifier(s) => s,
                    _ => return Err(Error::expected_identifier(name_token.span)),
                };
                let end_span = name_token.span;
                Ok(Expr::boxed(
                    ExprKind::Parameter(Parameter::Named(name)),
                    Span::new(span.start, end_span.end),
                ))
            }
            TokenKind::Question => {
                self.advance()?;
                Ok(Expr::boxed(
                    ExprKind::Parameter(Parameter::Positional(0)),
                    span,
                ))
            }

            // Parenthesized expression or subquery
            TokenKind::LeftParen => self.parse_parenthesized_expression(),

            // Array literal
            TokenKind::LeftBracket => self.parse_array_literal(),

            _ => Err(Error::expected_expression(span)),
        }
    }

    /// Try to parse postfix expressions (field access, array subscript, etc.).
    fn try_parse_postfix_expression(&mut self, left: Box<Expr>) -> Result<Option<Box<Expr>>> {
        // Clone token kind to avoid borrow issues
        let token_kind = self.peek()?.kind.clone();

        match token_kind {
            // Field access: expr.field
            TokenKind::Dot => {
                self.advance()?;
                let field = self.parse_identifier()?;
                let span = left.span.merge(field.span);
                Ok(Some(Expr::boxed(
                    ExprKind::FieldAccess { expr: left, field },
                    span,
                )))
            }

            // Array subscript: expr[index]
            TokenKind::LeftBracket => {
                self.advance()?;
                let index = self.parse_array_subscript_index()?;
                let end_token = self.expect(&TokenKind::RightBracket)?;
                let span = left.span.merge(end_token.span);
                Ok(Some(Expr::boxed(
                    ExprKind::ArraySubscript { array: left, index },
                    span,
                )))
            }

            // BETWEEN
            TokenKind::Keyword(Keyword::Between) => {
                self.advance()?;
                let low = self.parse_expression_with_precedence(10)?;
                self.expect_keyword(Keyword::And)?;
                let high = self.parse_expression_with_precedence(10)?;
                let span = left.span.merge(high.span);
                Ok(Some(Expr::boxed(
                    ExprKind::Between {
                        expr: left,
                        low,
                        high,
                        negated: false,
                    },
                    span,
                )))
            }

            // NOT BETWEEN
            TokenKind::Keyword(Keyword::Not) => {
                let next = self.peek_nth(1)?;
                if next.is_keyword(Keyword::Between) {
                    self.advance()?; // NOT
                    self.advance()?; // BETWEEN
                    let low = self.parse_expression_with_precedence(10)?;
                    self.expect_keyword(Keyword::And)?;
                    let high = self.parse_expression_with_precedence(10)?;
                    let span = left.span.merge(high.span);
                    return Ok(Some(Expr::boxed(
                        ExprKind::Between {
                            expr: left,
                            low,
                            high,
                            negated: true,
                        },
                        span,
                    )));
                }
                if next.is_keyword(Keyword::In) {
                    self.advance()?; // NOT
                    self.advance()?; // IN
                    return self.parse_in_expression(left, true);
                }
                if next.is_keyword(Keyword::Like) {
                    self.advance()?; // NOT
                    self.advance()?; // LIKE
                    return self.parse_like_expression(left, true);
                }
                Ok(None)
            }

            // IN
            TokenKind::Keyword(Keyword::In) => {
                self.advance()?;
                self.parse_in_expression(left, false)
            }

            // LIKE
            TokenKind::Keyword(Keyword::Like) => {
                self.advance()?;
                self.parse_like_expression(left, false)
            }

            // IS
            TokenKind::Keyword(Keyword::Is) => {
                self.advance()?;
                self.parse_is_expression(left)
            }

            _ => Ok(None),
        }
    }

    /// Parse an identifier or function call.
    fn parse_identifier_or_function(&mut self) -> Result<Box<Expr>> {
        let name = self.parse_object_name()?;

        // Check for function call
        if self.check(&TokenKind::LeftParen)? {
            self.parse_function_call(name)
        } else {
            // Simple identifier or qualified identifier
            let span = name.span;
            if name.parts.len() == 1 {
                Ok(Expr::boxed(
                    ExprKind::Identifier(name.parts.into_iter().next().unwrap()),
                    span,
                ))
            } else {
                Ok(Expr::boxed(ExprKind::CompoundIdentifier(name.parts), span))
            }
        }
    }

    /// Parse a function call.
    fn parse_function_call(&mut self, name: ObjectName) -> Result<Box<Expr>> {
        let start = name.span.start;
        self.expect(&TokenKind::LeftParen)?;

        // Check for COUNT(*) or similar
        let mut args = Vec::new();
        let mut distinct = false;

        if self.check(&TokenKind::Star)? {
            self.advance()?;
            args.push(FunctionArg::Star);
        } else if !self.check(&TokenKind::RightParen)? {
            // Check for DISTINCT
            if self.consume_keyword(Keyword::Distinct)?.is_some() {
                distinct = true;
            }

            // Parse arguments
            args = self.parse_comma_separated(|p| p.parse_function_arg())?;
        }

        // Check for ORDER BY in aggregate functions
        let order_by = if self.consume_keyword(Keyword::Order)?.is_some() {
            self.expect_keyword(Keyword::By)?;
            self.parse_comma_separated(|p| p.parse_order_by_expr())?
        } else {
            Vec::new()
        };

        // Check for LIMIT in aggregate functions
        let limit = if self.consume_keyword(Keyword::Limit)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        let end_token = self.expect(&TokenKind::RightParen)?;

        // Check for OVER clause (window function)
        if self.consume_keyword(Keyword::Over)?.is_some() {
            let window = self.parse_window_spec_or_ref()?;
            let span = Span::new(start, self.current_position());
            return Ok(Expr::boxed(
                ExprKind::WindowFunction(WindowFunctionCall {
                    function: FunctionCall {
                        name,
                        args,
                        distinct,
                        null_treatment: None,
                        order_by,
                        limit,
                    },
                    window,
                }),
                span,
            ));
        }

        let span = Span::new(start, end_token.span.end);
        Ok(Expr::boxed(
            ExprKind::Function(FunctionCall {
                name,
                args,
                distinct,
                null_treatment: None,
                order_by,
                limit,
            }),
            span,
        ))
    }

    /// Parse a window specification or reference.
    fn parse_window_spec_or_ref(&mut self) -> Result<WindowSpecOrRef> {
        if self.check(&TokenKind::LeftParen)? {
            self.expect(&TokenKind::LeftParen)?;
            let spec = self.parse_window_spec()?;
            self.expect(&TokenKind::RightParen)?;
            Ok(WindowSpecOrRef::Spec(spec))
        } else {
            let name = self.parse_identifier()?;
            Ok(WindowSpecOrRef::Ref(name))
        }
    }

    /// Parse a window specification.
    pub(super) fn parse_window_spec(&mut self) -> Result<WindowSpec> {
        let partition_by = if self.consume_keyword(Keyword::Partition)?.is_some() {
            self.expect_keyword(Keyword::By)?;
            self.parse_comma_separated(|p| p.parse_expression())?
        } else {
            Vec::new()
        };

        let order_by = if self.consume_keyword(Keyword::Order)?.is_some() {
            self.expect_keyword(Keyword::By)?;
            self.parse_comma_separated(|p| p.parse_order_by_expr())?
        } else {
            Vec::new()
        };

        let frame = self.parse_optional_window_frame()?;

        Ok(WindowSpec {
            partition_by,
            order_by,
            frame,
        })
    }

    /// Parse an optional window frame.
    fn parse_optional_window_frame(&mut self) -> Result<Option<WindowFrame>> {
        let unit = if self.consume_keyword(Keyword::Rows)?.is_some() {
            WindowFrameUnit::Rows
        } else if self.consume_keyword(Keyword::Range)?.is_some() {
            WindowFrameUnit::Range
        } else if self.consume_keyword(Keyword::Groups)?.is_some() {
            WindowFrameUnit::Groups
        } else {
            return Ok(None);
        };

        let (start, end) = if self.consume_keyword(Keyword::Between)?.is_some() {
            let start = self.parse_window_frame_bound()?;
            self.expect_keyword(Keyword::And)?;
            let end = self.parse_window_frame_bound()?;
            (start, Some(end))
        } else {
            (self.parse_window_frame_bound()?, None)
        };

        Ok(Some(WindowFrame { unit, start, end }))
    }

    /// Parse a window frame bound.
    fn parse_window_frame_bound(&mut self) -> Result<WindowFrameBound> {
        if self.consume_keyword(Keyword::Current)?.is_some() {
            self.expect_keyword(Keyword::Row)?;
            return Ok(WindowFrameBound::CurrentRow);
        }

        if self.consume_keyword(Keyword::Unbounded)?.is_some() {
            if self.consume_keyword(Keyword::Preceding)?.is_some() {
                return Ok(WindowFrameBound::Preceding(None));
            } else {
                self.expect_keyword(Keyword::Following)?;
                return Ok(WindowFrameBound::Following(None));
            }
        }

        let expr = self.parse_expression()?;

        if self.consume_keyword(Keyword::Preceding)?.is_some() {
            Ok(WindowFrameBound::Preceding(Some(expr)))
        } else {
            self.expect_keyword(Keyword::Following)?;
            Ok(WindowFrameBound::Following(Some(expr)))
        }
    }

    /// Parse an array subscript index.
    ///
    /// Supports:
    /// - Simple index: `array[0]`
    /// - OFFSET: `array[OFFSET(0)]`
    /// - ORDINAL: `array[ORDINAL(1)]`
    /// - SAFE_OFFSET: `array[SAFE_OFFSET(0)]`
    /// - SAFE_ORDINAL: `array[SAFE_ORDINAL(1)]`
    fn parse_array_subscript_index(&mut self) -> Result<ArraySubscriptKind> {
        // Check for OFFSET
        if self.consume_keyword(Keyword::Offset)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let idx = self.parse_expression()?;
            self.expect(&TokenKind::RightParen)?;
            return Ok(ArraySubscriptKind::Offset(idx));
        }

        // Check for ORDINAL
        if self.consume_keyword(Keyword::Ordinal)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let idx = self.parse_expression()?;
            self.expect(&TokenKind::RightParen)?;
            return Ok(ArraySubscriptKind::Ordinal(idx));
        }

        // Check for SAFE_OFFSET or SAFE_ORDINAL (as single keywords)
        if self.consume_keyword(Keyword::SafeOffset)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let idx = self.parse_expression()?;
            self.expect(&TokenKind::RightParen)?;
            return Ok(ArraySubscriptKind::SafeOffset(idx));
        }

        if self.consume_keyword(Keyword::SafeOrdinal)?.is_some() {
            self.expect(&TokenKind::LeftParen)?;
            let idx = self.parse_expression()?;
            self.expect(&TokenKind::RightParen)?;
            return Ok(ArraySubscriptKind::SafeOrdinal(idx));
        }

        // Simple index
        let idx = self.parse_expression()?;
        Ok(ArraySubscriptKind::Index(idx))
    }

    /// Parse a parenthesized expression or subquery.
    fn parse_parenthesized_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.expect(&TokenKind::LeftParen)?.span.start;

        // Check if this is a subquery
        if self.check_keyword(Keyword::Select)? || self.check_keyword(Keyword::With)? {
            let query = self.parse_query()?;
            let end = self.expect(&TokenKind::RightParen)?.span.end;
            return Ok(Expr::boxed(
                ExprKind::Subquery(Box::new(query)),
                Span::new(start, end),
            ));
        }

        let expr = self.parse_expression()?;

        // Check for tuple/struct: (expr1, expr2, ...)
        if self.check(&TokenKind::Comma)? {
            let mut fields = vec![StructField {
                name: None,
                value: expr,
            }];
            while self.consume(&TokenKind::Comma)?.is_some() {
                let value = self.parse_expression()?;
                fields.push(StructField { name: None, value });
            }
            let end = self.expect(&TokenKind::RightParen)?.span.end;
            return Ok(Expr::boxed(
                ExprKind::Struct { fields },
                Span::new(start, end),
            ));
        }

        let end = self.expect(&TokenKind::RightParen)?.span.end;
        Ok(Expr::boxed(
            ExprKind::Parenthesized(expr),
            Span::new(start, end),
        ))
    }

    /// Parse an array literal: [1, 2, 3].
    fn parse_array_literal(&mut self) -> Result<Box<Expr>> {
        let start = self.expect(&TokenKind::LeftBracket)?.span.start;

        let elements = if self.check(&TokenKind::RightBracket)? {
            Vec::new()
        } else {
            self.parse_comma_separated(|p| p.parse_expression())?
        };

        let end = self.expect(&TokenKind::RightBracket)?.span.end;

        Ok(Expr::boxed(
            ExprKind::Array {
                element_type: None,
                elements,
            },
            Span::new(start, end),
        ))
    }

    /// Parse an ARRAY expression: ARRAY[1, 2, 3] or ARRAY<type>[...].
    fn parse_array_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.expect_keyword(Keyword::Array)?.span.start;

        // Check for type parameter: ARRAY<type>
        let element_type = if self.consume(&TokenKind::Lt)?.is_some() {
            let data_type = self.parse_data_type()?;
            self.expect(&TokenKind::Gt)?;
            Some(Box::new(data_type))
        } else {
            None
        };

        // Parse array elements or subquery
        if self.check(&TokenKind::LeftParen)? {
            // ARRAY(subquery)
            self.advance()?;
            let query = self.parse_query()?;
            let end = self.expect(&TokenKind::RightParen)?.span.end;
            return Ok(Expr::boxed(
                ExprKind::Subquery(Box::new(query)),
                Span::new(start, end),
            ));
        }

        self.expect(&TokenKind::LeftBracket)?;
        let elements = if self.check(&TokenKind::RightBracket)? {
            Vec::new()
        } else {
            self.parse_comma_separated(|p| p.parse_expression())?
        };
        let end = self.expect(&TokenKind::RightBracket)?.span.end;

        Ok(Expr::boxed(
            ExprKind::Array {
                element_type,
                elements,
            },
            Span::new(start, end),
        ))
    }

    /// Parse a STRUCT expression.
    fn parse_struct_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.expect_keyword(Keyword::Struct)?.span.start;

        // Check for type parameter: STRUCT<...>
        if self.check(&TokenKind::Lt)? {
            // STRUCT<field_type, ...>(values)
            // For now, skip type parameters
            self.advance()?;
            let mut depth = 1;
            while depth > 0 {
                let token = self.advance()?;
                match token.kind {
                    TokenKind::Lt => depth += 1,
                    TokenKind::Gt => depth -= 1,
                    TokenKind::Eof => return Err(Error::unexpected_eof(token.span.start)),
                    _ => {}
                }
            }
        }

        self.expect(&TokenKind::LeftParen)?;

        let fields = if self.check(&TokenKind::RightParen)? {
            Vec::new()
        } else {
            self.parse_comma_separated(|p| p.parse_struct_field())?
        };

        let end = self.expect(&TokenKind::RightParen)?.span.end;

        Ok(Expr::boxed(
            ExprKind::Struct { fields },
            Span::new(start, end),
        ))
    }

    /// Parse a struct field: [name AS] expr.
    fn parse_struct_field(&mut self) -> Result<StructField> {
        let expr = self.parse_expression()?;

        // Check for AS alias
        if self.consume_keyword(Keyword::As)?.is_some() {
            let name = self.parse_identifier()?;
            return Ok(StructField {
                name: Some(name),
                value: expr,
            });
        }

        // No alias
        Ok(StructField {
            name: None,
            value: expr,
        })
    }

    /// Parse CASE expression.
    fn parse_case_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.expect_keyword(Keyword::Case)?.span.start;

        // Check for simple CASE (CASE expr WHEN ...)
        let operand = if !self.check_keyword(Keyword::When)? {
            Some(self.parse_expression()?)
        } else {
            None
        };

        let mut conditions = Vec::new();
        while self.consume_keyword(Keyword::When)?.is_some() {
            let condition = self.parse_expression()?;
            self.expect_keyword(Keyword::Then)?;
            let result = self.parse_expression()?;
            conditions.push((condition, result));
        }

        let else_result = if self.consume_keyword(Keyword::Else)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        let end = self.expect_keyword(Keyword::End)?.span.end;

        Ok(Expr::boxed(
            ExprKind::Case {
                operand,
                conditions,
                else_result,
            },
            Span::new(start, end),
        ))
    }

    /// Parse CAST or SAFE_CAST expression.
    fn parse_cast_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.peek()?.span.start;
        let safe = self.consume_keyword(Keyword::SafeCast)?.is_some();
        if !safe {
            self.expect_keyword(Keyword::Cast)?;
        }

        self.expect(&TokenKind::LeftParen)?;
        let expr = self.parse_expression()?;
        self.expect_keyword(Keyword::As)?;
        let data_type = self.parse_data_type()?;
        let end = self.expect(&TokenKind::RightParen)?.span.end;

        Ok(Expr::boxed(
            ExprKind::Cast {
                expr,
                data_type,
                safe,
            },
            Span::new(start, end),
        ))
    }

    /// Parse EXTRACT expression.
    fn parse_extract_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.expect_keyword(Keyword::Extract)?.span.start;
        self.expect(&TokenKind::LeftParen)?;

        let field_token = self.advance()?;
        let field = match DateTimePart::parse(&field_token.text) {
            Some(f) => f,
            None => {
                return Err(Error::invalid_syntax(
                    format!("invalid EXTRACT field: {}", field_token.text),
                    field_token.span,
                ))
            }
        };

        self.expect_keyword(Keyword::From)?;
        let from = self.parse_expression()?;
        let end = self.expect(&TokenKind::RightParen)?.span.end;

        Ok(Expr::boxed(
            ExprKind::Extract { field, from },
            Span::new(start, end),
        ))
    }

    /// Parse INTERVAL expression.
    fn parse_interval_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.expect_keyword(Keyword::Interval)?.span.start;

        let value = self.parse_expression_with_precedence(10)?;

        let unit_token = self.advance()?;
        let unit = match IntervalUnit::parse(&unit_token.text) {
            Some(u) => u,
            None => {
                return Err(Error::invalid_syntax(
                    format!("invalid INTERVAL unit: {}", unit_token.text),
                    unit_token.span,
                ))
            }
        };

        let span = Span::new(start, unit_token.span.end);
        Ok(Expr::boxed(ExprKind::Interval { value, unit }, span))
    }

    /// Parse IF expression.
    fn parse_if_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.expect_keyword(Keyword::If)?.span.start;
        self.expect(&TokenKind::LeftParen)?;

        let condition = self.parse_expression()?;
        self.expect(&TokenKind::Comma)?;
        let then_expr = self.parse_expression()?;
        self.expect(&TokenKind::Comma)?;
        let else_expr = self.parse_expression()?;

        let end = self.expect(&TokenKind::RightParen)?.span.end;

        Ok(Expr::boxed(
            ExprKind::If {
                condition,
                then_expr,
                else_expr,
            },
            Span::new(start, end),
        ))
    }

    /// Parse EXISTS expression.
    fn parse_exists_expression(&mut self) -> Result<Box<Expr>> {
        let start = self.expect_keyword(Keyword::Exists)?.span.start;
        self.expect(&TokenKind::LeftParen)?;
        let query = self.parse_query()?;
        let end = self.expect(&TokenKind::RightParen)?.span.end;

        Ok(Expr::boxed(
            ExprKind::Exists {
                subquery: Box::new(query),
                negated: false,
            },
            Span::new(start, end),
        ))
    }

    /// Parse IN expression.
    fn parse_in_expression(&mut self, left: Box<Expr>, negated: bool) -> Result<Option<Box<Expr>>> {
        self.expect(&TokenKind::LeftParen)?;

        let list = if self.check_keyword(Keyword::Select)? || self.check_keyword(Keyword::With)? {
            let query = self.parse_query()?;
            InList::Subquery(Box::new(query))
        } else {
            let values = self.parse_comma_separated(|p| p.parse_expression())?;
            InList::Values(values)
        };

        let end = self.expect(&TokenKind::RightParen)?.span.end;
        let span = left.span.merge(Span::new(end - 1, end));

        Ok(Some(Expr::boxed(
            ExprKind::In {
                expr: left,
                list,
                negated,
            },
            span,
        )))
    }

    /// Parse LIKE expression.
    fn parse_like_expression(
        &mut self,
        left: Box<Expr>,
        negated: bool,
    ) -> Result<Option<Box<Expr>>> {
        let pattern = self.parse_expression_with_precedence(10)?;

        let escape = if self.consume_keyword(Keyword::Escape)?.is_some() {
            Some(self.parse_expression_with_precedence(10)?)
        } else {
            None
        };

        let span = left.span.merge(pattern.span);

        Ok(Some(Expr::boxed(
            ExprKind::Like {
                expr: left,
                pattern,
                escape,
                negated,
            },
            span,
        )))
    }

    /// Parse IS expression (IS NULL, IS NOT NULL, IS TRUE, etc.).
    fn parse_is_expression(&mut self, left: Box<Expr>) -> Result<Option<Box<Expr>>> {
        let negated = self.consume_keyword(Keyword::Not)?.is_some();

        // Note: NULL, TRUE, FALSE are special tokens, not keywords
        let test = if self.consume(&TokenKind::Null)?.is_some() {
            IsTest::Null
        } else if self.consume(&TokenKind::Boolean(true))?.is_some() {
            IsTest::True
        } else if self.consume(&TokenKind::Boolean(false))?.is_some() {
            IsTest::False
        } else {
            let token = self.advance()?;
            return Err(Error::unexpected_token(
                "NULL, TRUE, FALSE, or UNKNOWN",
                format!("{}", token.kind),
                token.span,
            ));
        };

        let span = Span::new(left.span.start, self.current_position());

        Ok(Some(Expr::boxed(
            ExprKind::IsExpr {
                expr: left,
                test,
                negated,
            },
            span,
        )))
    }

    /// Get binary operator and its precedence from token.
    fn get_binary_op(&self, kind: &TokenKind) -> Option<(BinaryOp, u8)> {
        match kind {
            TokenKind::Plus => Some((BinaryOp::Plus, 7)),
            TokenKind::Minus => Some((BinaryOp::Minus, 7)),
            TokenKind::Star => Some((BinaryOp::Multiply, 8)),
            TokenKind::Slash => Some((BinaryOp::Divide, 8)),
            TokenKind::Percent => Some((BinaryOp::Modulo, 8)),
            TokenKind::Eq => Some((BinaryOp::Eq, 9)),
            TokenKind::NotEq | TokenKind::LtGt => Some((BinaryOp::NotEq, 9)),
            TokenKind::Lt => Some((BinaryOp::Lt, 9)),
            TokenKind::LtEq => Some((BinaryOp::LtEq, 9)),
            TokenKind::Gt => Some((BinaryOp::Gt, 9)),
            TokenKind::GtEq => Some((BinaryOp::GtEq, 9)),
            TokenKind::Ampersand => Some((BinaryOp::BitwiseAnd, 5)),
            TokenKind::Pipe => Some((BinaryOp::BitwiseOr, 3)),
            TokenKind::Caret => Some((BinaryOp::BitwiseXor, 4)),
            TokenKind::LeftShift => Some((BinaryOp::LeftShift, 6)),
            TokenKind::RightShift => Some((BinaryOp::RightShift, 6)),
            TokenKind::DoublePipe => Some((BinaryOp::Concat, 8)),
            _ => None,
        }
    }

    /// Convert keyword to typed literal type.
    fn keyword_to_typed_literal(&self, kw: Keyword) -> Option<TypedLiteralType> {
        match kw {
            Keyword::Date => Some(TypedLiteralType::Date),
            Keyword::Time => Some(TypedLiteralType::Time),
            Keyword::Timestamp => Some(TypedLiteralType::Timestamp),
            Keyword::Datetime => Some(TypedLiteralType::Datetime),
            Keyword::Json => Some(TypedLiteralType::Json),
            Keyword::Numeric | Keyword::Decimal => Some(TypedLiteralType::Numeric),
            _ => None,
        }
    }

    /// Parse a data type specification.
    pub fn parse_data_type(&mut self) -> Result<DataTypeSpec> {
        let token = self.advance()?;
        let start = token.span.start;

        let kind = match &token.kind {
            TokenKind::Keyword(kw) => self.parse_data_type_from_keyword(*kw, token.span)?,
            TokenKind::Identifier(name) => {
                // Named type or alias
                let upper = name.to_uppercase();
                self.parse_data_type_from_name(&upper, token.span)?
            }
            _ => {
                return Err(Error::unexpected_token(
                    "data type",
                    format!("{}", token.kind),
                    token.span,
                ))
            }
        };

        let end = self.current_position();
        Ok(DataTypeSpec::new(kind, Span::new(start, end)))
    }

    fn parse_data_type_from_keyword(&mut self, kw: Keyword, span: Span) -> Result<DataTypeKind> {
        match kw {
            // Boolean
            Keyword::Bool | Keyword::Boolean => Ok(DataTypeKind::Bool),

            // 32-bit integer (INTEGER, INT, INT32, SMALLINT)
            Keyword::Int32 | Keyword::Int | Keyword::Integer | Keyword::Smallint => {
                Ok(DataTypeKind::Int32)
            }

            // 64-bit integer (BIGINT, INT64)
            Keyword::Int64 | Keyword::Bigint => Ok(DataTypeKind::Int64),

            // Unsigned integers
            Keyword::Uint32 => Ok(DataTypeKind::Uint32),
            Keyword::Uint64 => Ok(DataTypeKind::Uint64),

            // 32-bit float (REAL, FLOAT32)
            Keyword::Float32 | Keyword::Real => Ok(DataTypeKind::Float32),

            // 64-bit float (DOUBLE, DOUBLE PRECISION, FLOAT, FLOAT64)
            Keyword::Float64 | Keyword::Float | Keyword::Double => Ok(DataTypeKind::Float64),

            // Fixed precision decimal (NUMERIC, DECIMAL)
            Keyword::Numeric | Keyword::Decimal => self.parse_numeric_type(),

            // String types (VARCHAR, TEXT, STRING, CHAR)
            Keyword::String | Keyword::Text | Keyword::Varchar | Keyword::Char => {
                self.parse_varchar_type()
            }

            // Binary types (VARBINARY, BYTES, BLOB, BINARY)
            Keyword::Bytes | Keyword::Varbinary | Keyword::Binary | Keyword::Blob => {
                self.parse_varbinary_type()
            }

            // Date/time
            Keyword::Date => Ok(DataTypeKind::Date),
            Keyword::Time => Ok(DataTypeKind::Time),
            Keyword::Datetime => Ok(DataTypeKind::Datetime),
            Keyword::Timestamp => Ok(DataTypeKind::Timestamp),
            Keyword::Interval => Ok(DataTypeKind::Interval),

            // Other types
            Keyword::Json => Ok(DataTypeKind::Json),
            Keyword::Uuid => Ok(DataTypeKind::Uuid),

            // Complex types
            Keyword::Array => self.parse_array_type(),
            Keyword::Struct => self.parse_struct_type(),
            Keyword::Range => self.parse_range_type(),

            _ => Err(Error::unexpected_token(
                "data type",
                format!("{}", kw),
                span,
            )),
        }
    }

    fn parse_data_type_from_name(&mut self, name: &str, span: Span) -> Result<DataTypeKind> {
        match name {
            // Boolean
            "BOOL" | "BOOLEAN" => Ok(DataTypeKind::Bool),

            // 32-bit integer
            "INT32" | "INT" | "INTEGER" | "SMALLINT" | "INT4" => Ok(DataTypeKind::Int32),

            // 64-bit integer
            "INT64" | "BIGINT" | "INT8" => Ok(DataTypeKind::Int64),

            // Unsigned integers
            "UINT32" | "UINTEGER" => Ok(DataTypeKind::Uint32),
            "UINT64" | "UBIGINT" => Ok(DataTypeKind::Uint64),

            // 32-bit float
            "FLOAT32" | "REAL" | "FLOAT4" => Ok(DataTypeKind::Float32),

            // 64-bit float
            "FLOAT64" | "FLOAT" | "DOUBLE" | "FLOAT8" => Ok(DataTypeKind::Float64),

            // Fixed precision decimal
            "NUMERIC" | "DECIMAL" | "DEC" => self.parse_numeric_type(),

            // String types
            "STRING" | "TEXT" | "VARCHAR" | "CHAR" | "CHARACTER" | "NVARCHAR" | "NCHAR" => {
                self.parse_varchar_type()
            }

            // Binary types
            "BYTES" | "VARBINARY" | "BYTEA" | "BLOB" | "BINARY" => self.parse_varbinary_type(),

            // Date/time
            "DATE" => Ok(DataTypeKind::Date),
            "TIME" => Ok(DataTypeKind::Time),
            "DATETIME" => Ok(DataTypeKind::Datetime),
            "TIMESTAMP" => Ok(DataTypeKind::Timestamp),
            "INTERVAL" => Ok(DataTypeKind::Interval),

            // Other types
            "JSON" | "JSONB" => Ok(DataTypeKind::Json),
            "UUID" => Ok(DataTypeKind::Uuid),

            _ => Err(Error::unexpected_token("data type", name, span)),
        }
    }

    fn parse_numeric_type(&mut self) -> Result<DataTypeKind> {
        let (precision, scale) = if self.consume(&TokenKind::LeftParen)?.is_some() {
            let p = self.parse_integer_literal()? as u8;
            let s = if self.consume(&TokenKind::Comma)?.is_some() {
                Some(self.parse_integer_literal()? as u8)
            } else {
                None
            };
            self.expect(&TokenKind::RightParen)?;
            (Some(p), s)
        } else {
            (None, None)
        };

        Ok(DataTypeKind::Numeric { precision, scale })
    }

    fn parse_varchar_type(&mut self) -> Result<DataTypeKind> {
        let max_length = if self.consume(&TokenKind::LeftParen)?.is_some() {
            let len = self.parse_integer_literal()? as u64;
            self.expect(&TokenKind::RightParen)?;
            Some(len)
        } else {
            None
        };
        Ok(DataTypeKind::Varchar { max_length })
    }

    fn parse_varbinary_type(&mut self) -> Result<DataTypeKind> {
        let max_length = if self.consume(&TokenKind::LeftParen)?.is_some() {
            let len = self.parse_integer_literal()? as u64;
            self.expect(&TokenKind::RightParen)?;
            Some(len)
        } else {
            None
        };
        Ok(DataTypeKind::Varbinary { max_length })
    }

    fn parse_array_type(&mut self) -> Result<DataTypeKind> {
        self.expect(&TokenKind::Lt)?;
        let element_type = self.parse_data_type()?;
        self.expect(&TokenKind::Gt)?;
        Ok(DataTypeKind::Array(Box::new(element_type)))
    }

    fn parse_struct_type(&mut self) -> Result<DataTypeKind> {
        self.expect(&TokenKind::Lt)?;

        let fields = if self.check(&TokenKind::Gt)? {
            Vec::new()
        } else {
            self.parse_comma_separated(|p| p.parse_struct_type_field())?
        };

        self.expect(&TokenKind::Gt)?;
        Ok(DataTypeKind::Struct(fields))
    }

    fn parse_struct_type_field(&mut self) -> Result<TypeStructField> {
        // Check if there's a name before the type
        let token = self.peek()?;
        let has_name = matches!(
            &token.kind,
            TokenKind::Identifier(_) | TokenKind::QuotedIdentifier(_)
        );

        let (name, data_type) = if has_name {
            let maybe_name = self.parse_identifier()?;
            // Check if this was actually a type name
            if self.check(&TokenKind::Comma)? || self.check(&TokenKind::Gt)? {
                // It was a type, not a field name
                // Need to reparse as a type - for simplicity, treat as named type
                let dt = DataTypeSpec::new(
                    DataTypeKind::Named(vec![maybe_name.clone()]),
                    maybe_name.span,
                );
                (None, dt)
            } else {
                let dt = self.parse_data_type()?;
                (Some(maybe_name), dt)
            }
        } else {
            let dt = self.parse_data_type()?;
            (None, dt)
        };

        Ok(TypeStructField { name, data_type })
    }

    fn parse_range_type(&mut self) -> Result<DataTypeKind> {
        self.expect(&TokenKind::Lt)?;
        let element_type = self.parse_data_type()?;
        self.expect(&TokenKind::Gt)?;
        Ok(DataTypeKind::Range(Box::new(element_type)))
    }

    fn parse_integer_literal(&mut self) -> Result<i64> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::Integer(n) => Ok(n),
            _ => Err(Error::unexpected_token(
                "integer",
                format!("{}", token.kind),
                token.span,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expr(sql: &str) -> Box<Expr> {
        let mut parser = Parser::new(sql);
        parser
            .parse_expression()
            .expect("Failed to parse expression")
    }

    #[test]
    fn test_literals() {
        let expr = parse_expr("42");
        assert!(matches!(expr.kind, ExprKind::Integer(42)));

        let expr = parse_expr("3.14");
        assert!(matches!(expr.kind, ExprKind::Float(f) if (f - 3.14).abs() < 0.001));

        let expr = parse_expr("'hello'");
        assert!(matches!(expr.kind, ExprKind::String(s) if s == "hello"));

        let expr = parse_expr("TRUE");
        assert!(matches!(expr.kind, ExprKind::Boolean(true)));

        let expr = parse_expr("NULL");
        assert!(matches!(expr.kind, ExprKind::Null));
    }

    #[test]
    fn test_binary_operations() {
        let expr = parse_expr("1 + 2");
        assert!(matches!(
            expr.kind,
            ExprKind::BinaryOp {
                op: BinaryOp::Plus,
                ..
            }
        ));

        let expr = parse_expr("1 + 2 * 3");
        // Should parse as 1 + (2 * 3) due to precedence
        if let ExprKind::BinaryOp { op, left: _, right } = &expr.kind {
            assert_eq!(*op, BinaryOp::Plus);
            assert!(matches!(
                right.kind,
                ExprKind::BinaryOp {
                    op: BinaryOp::Multiply,
                    ..
                }
            ));
        } else {
            panic!("Expected BinaryOp");
        }
    }

    #[test]
    fn test_comparison() {
        let expr = parse_expr("a > b");
        assert!(matches!(
            expr.kind,
            ExprKind::BinaryOp {
                op: BinaryOp::Gt,
                ..
            }
        ));

        let expr = parse_expr("x = 10");
        assert!(matches!(
            expr.kind,
            ExprKind::BinaryOp {
                op: BinaryOp::Eq,
                ..
            }
        ));
    }

    #[test]
    fn test_logical_operations() {
        let expr = parse_expr("a AND b");
        assert!(matches!(
            expr.kind,
            ExprKind::BinaryOp {
                op: BinaryOp::And,
                ..
            }
        ));

        let expr = parse_expr("a OR b");
        assert!(matches!(
            expr.kind,
            ExprKind::BinaryOp {
                op: BinaryOp::Or,
                ..
            }
        ));

        let expr = parse_expr("NOT a");
        assert!(matches!(
            expr.kind,
            ExprKind::UnaryOp {
                op: UnaryOp::Not,
                ..
            }
        ));
    }

    #[test]
    fn test_function_call() {
        let expr = parse_expr("COUNT(*)");
        if let ExprKind::Function(f) = &expr.kind {
            assert_eq!(f.name.parts.len(), 1);
            assert!(matches!(&f.args[0], FunctionArg::Star));
        } else {
            panic!("Expected Function");
        }

        let expr = parse_expr("UPPER('hello')");
        assert!(matches!(expr.kind, ExprKind::Function(_)));
    }

    #[test]
    fn test_case_expression() {
        let expr = parse_expr("CASE WHEN x > 0 THEN 'positive' ELSE 'non-positive' END");
        assert!(matches!(expr.kind, ExprKind::Case { .. }));
    }

    #[test]
    fn test_cast() {
        let expr = parse_expr("CAST(x AS INT64)");
        if let ExprKind::Cast { safe, .. } = &expr.kind {
            assert!(!safe);
        } else {
            panic!("Expected Cast");
        }
    }

    #[test]
    fn test_between() {
        let expr = parse_expr("x BETWEEN 1 AND 10");
        assert!(matches!(
            expr.kind,
            ExprKind::Between { negated: false, .. }
        ));

        let expr = parse_expr("x NOT BETWEEN 1 AND 10");
        assert!(matches!(expr.kind, ExprKind::Between { negated: true, .. }));
    }

    #[test]
    fn test_in_list() {
        let expr = parse_expr("x IN (1, 2, 3)");
        assert!(matches!(expr.kind, ExprKind::In { negated: false, .. }));
    }

    #[test]
    fn test_like() {
        let expr = parse_expr("name LIKE 'John%'");
        assert!(matches!(expr.kind, ExprKind::Like { negated: false, .. }));
    }

    #[test]
    fn test_is_null() {
        let expr = parse_expr("x IS NULL");
        assert!(matches!(
            expr.kind,
            ExprKind::IsExpr {
                test: IsTest::Null,
                negated: false,
                ..
            }
        ));

        let expr = parse_expr("x IS NOT NULL");
        assert!(matches!(
            expr.kind,
            ExprKind::IsExpr {
                test: IsTest::Null,
                negated: true,
                ..
            }
        ));
    }

    #[test]
    fn test_array_literal() {
        let expr = parse_expr("[1, 2, 3]");
        if let ExprKind::Array { elements, .. } = &expr.kind {
            assert_eq!(elements.len(), 3);
        } else {
            panic!("Expected Array");
        }
    }

    #[test]
    fn test_parenthesized() {
        let expr = parse_expr("(1 + 2) * 3");
        if let ExprKind::BinaryOp {
            op: BinaryOp::Multiply,
            left,
            ..
        } = &expr.kind
        {
            assert!(matches!(left.kind, ExprKind::Parenthesized(_)));
        } else {
            panic!("Expected BinaryOp");
        }
    }
}
