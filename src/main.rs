// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    KeywordLet, // LET

    // Variable
    // glot only supports single character variables
    Identifier(String),

    // Literals
    Number(u64),

    // Operators
    Equals, // assignment operator (not a comparator)
}

#[derive(Debug)]
pub enum Error {
    InvalidCharacter(char),
    InvalidIdentifier(String),
}

fn line_parse(line: &str) -> Result<Vec<Token>, Error> {
    println!("{}", line);

    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                println!("\"{}\"", c);

                // Move the iterator forward
                chars.next();
            }

            '+' | '-' | '*' | '/' => {
                println!("Operator {}", c);

                // Move the iterator forward
                chars.next();
            }

            '=' => {
                tokens.push(Token::Equals);

                // Move the iterator forward
                chars.next();
            }

            '0'..='9' => {
                println!("Digit: {}", c);

                // Move the iterator forward
                chars.next();
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
                    _ => {
                        // If not a keyword, check if it's a valid single-char variable
                        if ident.len() == 1 {
                            tokens.push(Token::Identifier(ident));
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

    Ok(tokens)
}

fn main() -> Result<(), Error> {
    let line = "10 LET C = 4 + 2";
    let tokens = line_parse(line)?;

    println!("Tokens: {:?}", tokens);

    Ok(())
}
