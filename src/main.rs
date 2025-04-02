// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

fn line_parse(line: &str) {
    println!("{}", line);

    let chars = line.chars();
    for c in chars {
        if c.is_ascii_digit() {
            println!("Digit: {}", c);
        } else if c.is_ascii_uppercase() {
            println!("Letter {}", c);
        } else if c == '=' {
            println!("Assign {}", c);
        } else if c == '+' || c == '-' {
            println!("Operator {}", c);
        } else {
            println!("\"{}\"", c);
        }
    }
}

fn main() {
    let line = "10 LET C = 4 + 2";
    line_parse(line);
}
