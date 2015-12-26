# Day 1: Not Quite Lisp

## Part A

Santa is trying to deliver presents in a large apartment building, but he can't
find the right floor - the directions he got are a little confusing. He starts
on the ground floor (floor 0) and then follows the instructions one character at
a time.

An opening parenthesis `(` means he should go up one floor, and a closing
parenthesis `)` means he should go down one floor.

The apartment building is very tall, and the basement is very deep; he will
never find the top or bottom floors.

For example:

1. `(())` and `()()` both result in floor 0.
2. `(((` and `(()(()(` both result in floor 3.
3. `))(((((` also results in floor 3.
4. `())` and `))(` both result in floor -1 (the first basement level).
5. `)))` and `)())())` both result in floor -3.
