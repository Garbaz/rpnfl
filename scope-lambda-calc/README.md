# Scope Lambda Calculus

This is basically just lambda calculus with De Bruijn indexing and built-in recursion. However, Application is written argument first - function second and is right-associative, and Abstraction is written as `{...}` instead of `λ(...)`. Furthermore, arguments are accessed with `$N`, starting with `$0` for the outermost argument. Similarly, recursion occurs with `%N`, starting again at `%0` with the innermost function. This means that every function is implicitly treated as recursive, and at any point, we can recurse back to any super-function.

## Grammar

```ebnf
Expr ::= Arg | Rec | Abstr | Appl

Arg ::= "$" <Number>

Rec ::= "%" <Number>

Abstr ::= "{" Expr "}"

Appl ::= Expr Expr
```

## Examples

identity: `{$0}`

infinite loop: `{$0 %0}`

true: `{{$0}}`

false: `{{$1}}`

if-then-else: `{{{$1 $2 $0}}}`

## TODO

- Allow for `(`/`)`

