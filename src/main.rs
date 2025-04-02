// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

fn line_parse(line: &str) {
    println!("{}", line);

    let chars = line.chars();
    for c in chars {
        println!("{}", c);
    }
}

fn main() {
    let line = "10 LET C = 4 + 2";
    line_parse(line);
}
