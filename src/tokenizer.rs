// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

use crate::Error;

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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GlotLine {
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

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::tokenizer::GlotLine;
    use crate::tokenizer::Token;

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
}
