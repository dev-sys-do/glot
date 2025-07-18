// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

use std::iter::Peekable;
use std::path::PathBuf;
use std::vec::IntoIter;

pub mod parser;
pub mod tokenizer;

use tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCharacter(char),
    InvalidIdentifier(String),
    InvalidNumber(String),
    InvalidOperatorToken(Token),
    InvalidSourceFile(PathBuf),
    InvalidValueToken(Token),
    UnexpectedToken(Token),
    EndOfInput,
}

// Helper to consume next token or return error
fn consume_token(tokens_iter: &mut Peekable<IntoIter<Token>>) -> Result<Token, Error> {
    tokens_iter.next().ok_or(Error::EndOfInput)
}
