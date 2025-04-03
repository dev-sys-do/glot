// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    KeywordLet, // LET

    // Variable
    // glot only supports single character variables
    Identifier(char),

    // Literals
    Number(u64),

    // Operators
    Equals, // assignment operator (not a comparator)
}

#[derive(Debug)]
pub enum Error {
    InvalidCharacter(char),
}

fn line_parse(line: &str) -> Result<Vec<Token>, Error> {
    println!("{}", line);

    let mut tokens = Vec::new();

    let chars = line.chars();
    for c in chars {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                println!("\"{}\"", c);
            }

            '+' | '-' | '*' | '/' => {
                println!("Operator {}", c);
            }

            '=' => {
                tokens.push(Token::Equals);
            }

            '0'..='9' => {
                println!("Digit: {}", c);
            }

            'A'..='Z' => {
                println!("Letter {}", c);
            }

            _ => {
                return Err(Error::InvalidCharacter(c));
            }
        }
    }

    Ok(tokens)
}

fn main() -> Result<(), Error> {
    let line = "10 LET C = 4 + 2";
    let tokens = line_parse(line)?;

    println!("Tokens: {:?}", tokens);

    Ok(())
}
