// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

fn main() {
    let line = "10 LET C = 4 + 2";
    let chars = line.chars();

    println!("{}", line);
    for c in chars {
        println!("{}", c);
    }
}
