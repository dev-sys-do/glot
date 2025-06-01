// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Peekable;
use std::path::Path;
use std::path::PathBuf;
use std::vec::IntoIter;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// glot source file
    #[arg(short, long, value_name = "FILE")]
    source: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    KeywordLet,   // LET
    KeywordPrint, // PRINT
    KeywordEnd,   // END

    // Variable
    // glot only supports single character variables
    Identifier(char),

    // Literals
    Number(u64),

    // Operators
    Equals, // assignment operator (not a comparator)
    OperatorPlus,
    OperatorMinus,
    OperatorMultiply,
    OperatorDivide,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCharacter(char),
    InvalidIdentifier(String),
    InvalidNumber(String),
    InvalidOperatorToken(Token),
    InvalidSourceFile(PathBuf),
    InvalidValueToken(Token),
    EndOfInput,
}

// Helper to consume next token or return error
fn consume_token(tokens_iter: &mut Peekable<IntoIter<Token>>) -> Result<Token, Error> {
    tokens_iter.next().ok_or(Error::EndOfInput)
}

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
    pub fn new(tokens_iter: &mut Peekable<IntoIter<Token>>) -> Result<Self, Error> {
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
    pub fn new(tokens_iter: &mut Peekable<IntoIter<Token>>) -> Result<Self, Error> {
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
    pub fn new(tokens_iter: &mut Peekable<IntoIter<Token>>) -> Result<Self, Error> {
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct GlotLine {
    tokens: Vec<Token>,
}

impl GlotLine {
    pub fn new(line: &str) -> Result<Self, Error> {
        let mut tokens = Vec::new();
        let mut chars = line.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    // Move the iterator forward
                    chars.next();
                }

                '+' => {
                    tokens.push(Token::OperatorPlus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::OperatorMinus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::OperatorMultiply);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::OperatorDivide);
                    chars.next();
                }
                '=' => {
                    tokens.push(Token::Equals);
                    chars.next();
                }

                '0'..='9' => {
                    // Build the string representing the number
                    let mut num_str = String::new();

                    // Start peeking into the character stream
                    while let Some(&ch) = chars.peek() {
                        // Exit the loop as soon as the next character is *not* a digit
                        if !ch.is_ascii_digit() {
                            break;
                        }

                        // Accumulate digits into the number string
                        num_str.push(ch);
                        chars.next();
                    }

                    // Check that this is a valid number
                    match num_str.parse::<u64>() {
                        Ok(num) => tokens.push(Token::Number(num)),
                        Err(_) => return Err(Error::InvalidNumber(num_str)),
                    }
                }

                'A'..='Z' => {
                    // Identifier for keyword (LET, etc) or variable
                    let mut ident = String::new();

                    // Start peeking into the character stream
                    while let Some(&ch) = chars.peek() {
                        // Exit the loop as soon as the next character is *not*
                        // an upper case letter
                        if !ch.is_ascii_uppercase() {
                            break;
                        }

                        // Accumulate upper case letters into the identifier
                        ident.push(ch);

                        // Move the iterator forward
                        chars.next();
                    }

                    // Check if it's a keyword or variable
                    match ident.as_str() {
                        "LET" => tokens.push(Token::KeywordLet),
                        "PRINT" => tokens.push(Token::KeywordPrint),
                        "END" => tokens.push(Token::KeywordEnd),

                        _ => {
                            // If not a keyword, check if it's a valid single-char variable
                            if ident.len() == 1 {
                                tokens.push(Token::Identifier(
                                    ident
                                        .chars()
                                        .next()
                                        .ok_or(Error::InvalidIdentifier(ident))?,
                                ));
                            } else {
                                // Multi-char variable is an error
                                return Err(Error::InvalidIdentifier(ident));
                            }
                        }
                    }
                }

                _ => {
                    return Err(Error::InvalidCharacter(c));
                }
            }
        }

        Ok(GlotLine { tokens })
    }
}

#[derive(Debug, Clone)]
struct Glotter {
    source: PathBuf,
    lines: Vec<GlotLine>,
}

impl Glotter {
    pub fn new_from_file(source_path: &Path) -> Result<Self, Error> {
        Ok(Glotter {
            source: source_path.to_path_buf(),
            lines: Vec::new(),
        })
    }

    pub fn tokenize(&mut self) -> Result<(), Error> {
        let source_file = File::open(self.source.clone())
            .map_err(|_| Error::InvalidSourceFile(self.source.clone()))?;
        let source = BufReader::new(source_file);

        for line in source.lines() {
            let line = line.unwrap();
            self.lines.push(GlotLine::new(&line)?);
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mut glotter = Glotter::new_from_file(&cli.source)?;
    glotter.tokenize()?;

    for line in glotter.lines {
        println!("{:?}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{BinaryOperator, Error, Expression, ExpressionItem, GlotLine, Term, Token};

    #[test]
    fn test_tokenizer_print_var() -> Result<(), Error> {
        let line = "10 PRINT G";
        let expected_tokens = [
            Token::Number(10),
            Token::KeywordPrint,
            Token::Identifier('G'),
        ];

        let glot_line = GlotLine::new(&line)?;
        assert_eq!(glot_line.tokens, expected_tokens);

        Ok(())
    }

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
        let expression = Expression::new(&mut glot_line.tokens.into_iter().peekable())?;

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
        let expression = Expression::new(&mut glot_line.tokens.into_iter().peekable())?;

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
            Expression::new(&mut glot_line.tokens.into_iter().peekable()),
            Err(Error::InvalidOperatorToken(Token::Equals))
        );

        Ok(())
    }

    #[test]
    fn test_invalid_expression_statement() -> Result<(), Error> {
        let line = "LET A = 5";

        let glot_line = GlotLine::new(&line)?;
        assert_eq!(
            Expression::new(&mut glot_line.tokens.into_iter().peekable()),
            Err(Error::InvalidValueToken(Token::KeywordLet))
        );

        Ok(())
    }

    #[test]
    fn test_invalid_expression_keyword() -> Result<(), Error> {
        let line = "PRINT";

        let glot_line = GlotLine::new(&line)?;
        assert_eq!(
            Expression::new(&mut glot_line.tokens.into_iter().peekable()),
            Err(Error::InvalidValueToken(Token::KeywordPrint))
        );

        Ok(())
    }
}
