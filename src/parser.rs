// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

use crate::Error;
use crate::consume_token;
use crate::tokenizer::Token;

use std::iter::Peekable;
use std::slice::Iter;

// glot expressions.
// A glot expression can be assigned to a variable, or used as an operand.
//   A, A + B pr A + B * 10 are valid expressions.
//
// The grammatical definition of an expression is:
//   expression      ::= term { ( "+" | "-" | "*" | "/" ) term }
// A glot expression always starts with a `term` (a variable or a number), followed by
// a series of (`binary operator`, `term`) couples.

// A number or a variable in an expression.
// A, 10 and B in `A + 10 * B`
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Number(u64),
    Variable(char),
}

impl Term {
    pub fn new(tokens_iter: &mut Peekable<Iter<Token>>) -> Result<Self, Error> {
        match consume_token(tokens_iter)? {
            Token::Number(n) => Ok(Term::Number(n)),
            Token::Identifier(v) => Ok(Term::Variable(v)),
            t => Err(Error::InvalidValueToken(t)),
        }
    }
}

// Operators used within expressions
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperator {
    pub fn new(tokens_iter: &mut Peekable<Iter<Token>>) -> Result<Self, Error> {
        match consume_token(tokens_iter)? {
            Token::OperatorPlus => Ok(BinaryOperator::Add),
            Token::OperatorMinus => Ok(BinaryOperator::Subtract),
            Token::OperatorMultiply => Ok(BinaryOperator::Multiply),
            Token::OperatorDivide => Ok(BinaryOperator::Divide),
            t => Err(Error::InvalidOperatorToken(t)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ExpressionItem {
    Term(Term),
    Operator(BinaryOperator),
}

// A glot expression.
// Example: `A + 10 * B` -> [Value(A), Operator(Add), Value(10), Operator(Multiply), Value(B)]
#[derive(Debug, Clone, PartialEq)]
struct Expression {
    items: Vec<ExpressionItem>,
}

impl Expression {
    pub fn new(tokens_iter: &mut Peekable<Iter<Token>>) -> Result<Self, Error> {
        let mut items = Vec::new();

        // First item must be a term
        let first_term = Term::new(tokens_iter)?;
        items.push(ExpressionItem::Term(first_term));

        loop {
            if let Some(_token) = tokens_iter.peek() {
                let operator = BinaryOperator::new(tokens_iter)?;
                let term = Term::new(tokens_iter)?;

                items.push(ExpressionItem::Operator(operator));
                items.push(ExpressionItem::Term(term));
            } else {
                break;
            }
        }

        Ok(Expression { items })
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::parser::BinaryOperator;
    use crate::parser::Expression;
    use crate::parser::ExpressionItem;
    use crate::parser::Term;
    use crate::tokenizer::GlotLine;
    use crate::tokenizer::Token;

    #[test]
    fn test_expression_arithmetic() -> Result<(), Error> {
        let line = "A + 10 * B";
        let expected_items = [
            ExpressionItem::Term(Term::Variable('A')),
            ExpressionItem::Operator(BinaryOperator::Add),
            ExpressionItem::Term(Term::Number(10)),
            ExpressionItem::Operator(BinaryOperator::Multiply),
            ExpressionItem::Term(Term::Variable('B')),
        ];

        let glot_line = GlotLine::new(&line)?;
        let expression = Expression::new(&mut glot_line.tokens().iter().peekable())?;

        assert_eq!(
            expression,
            Expression {
                items: expected_items.to_vec()
            }
        );

        Ok(())
    }

    #[test]
    fn test_expression_variable() -> Result<(), Error> {
        let line = "A";
        let expected_items = [ExpressionItem::Term(Term::Variable('A'))];

        let glot_line = GlotLine::new(&line)?;
        let expression = Expression::new(&mut glot_line.tokens().iter().peekable())?;

        assert_eq!(
            expression,
            Expression {
                items: expected_items.to_vec()
            }
        );

        Ok(())
    }

    #[test]
    fn test_invalid_expression_assign() -> Result<(), Error> {
        let line = "A = 5";

        let glot_line = GlotLine::new(&line)?;
        assert_eq!(
            Expression::new(&mut glot_line.tokens().iter().peekable()),
            Err(Error::InvalidOperatorToken(Token::Equals))
        );

        Ok(())
    }

    #[test]
    fn test_invalid_expression_statement() -> Result<(), Error> {
        let line = "LET A = 5";

        let glot_line = GlotLine::new(&line)?;
        assert_eq!(
            Expression::new(&mut glot_line.tokens().iter().peekable()),
            Err(Error::InvalidValueToken(Token::KeywordLet))
        );

        Ok(())
    }

    #[test]
    fn test_invalid_expression_keyword() -> Result<(), Error> {
        let line = "PRINT";

        let glot_line = GlotLine::new(&line)?;
        assert_eq!(
            Expression::new(&mut glot_line.tokens().iter().peekable()),
            Err(Error::InvalidValueToken(Token::KeywordPrint))
        );

        Ok(())
    }
}
