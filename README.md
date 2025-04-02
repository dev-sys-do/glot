# `glot`

`glot` is a basic [BASIC](https://en.wikipedia.org/wiki/BASIC) language interpreter, written in Rust.

`glot` supports a subset of the `BASIC`language:

* Only the `PRINT`, `LET`, `ÌF..THEN..ELSE`, `GOTO` and `END` keywords are supported.
* Variables are identified with a single character (`LET A = 42`).
* Literals are either integers (`4278`) or double-quoted strings (`"A string"`).
* Expressions are evaluated from left to right (`4 + 2 * 8` evaluates to 48, not 20).

## Examples

### Counting Loop

``` purebasic
10 LET C = 4 + 2
20 PRINT C
30 LET I = 1
40 PRINT "Counting:"
50 PRINT I
60 LET I = I + 1
70 IF I < C THEN GOTO 130
80 PRINT "Done!"
90 END
```

## Language Grammar


``` ebnf

program         ::= { statement }
statement       ::= print_statement
                    | assignment_statement
                    | if_statement
                    | goto_statement
                    | end_statement

print_statement ::= "PRINT" expression

assignment_statement ::= "LET" identifier "=" expression

if_statement    ::= "IF" condition "THEN" statement [ "ELSE" statement ]

goto_statement  ::= "GOTO" integer_literal

end_statement   ::= "END"

condition       ::= expression comparison_operator expression

comparison_operator ::= "=" | "!=" | "<" | ">" | "<=" | ">="

expression      ::= term { ( "+" | "-" | "*" | "/" ) term }

term            ::= identifier
                    | literal

literal         ::= integer_literal
                    | string_literal

integer_literal ::= digit { digit }

string_literal  ::= "\"" character { character } "\""

identifier      ::= letter

digit           ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

letter          ::= "A" | "B" | ... | "Z" | "a" | "b" | ... | "z"
```
##
