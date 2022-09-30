# RPNFL (Reverse Polish Notation Functional programming Language; Working Title)

This is a project to design and implement a functional programming language which incorporates some ideas I have about how functional programming could be done differently.

The core ideas are:

- The language has no keywords as part of it's syntax. Program structure is represented with symbols, and anything that is defined, like functions/types/etc., is represented with alphanumerics.
- The arguments to a function come before the function, not after it (e.g. we write `x y f` instead of `f x y` or `f(x, y)` or `(f x y)`)
- A function can be "unordered", meaning that it's arguments can be given in any order (e.g. for a function `f:a->b->c` with `a != b`, `x y f` and `y x f` are equivalent; We can also curry on any argument: `x f:b->c` / `y f:a->c`)

For more see [why.md](/why.md) and [ideas.md](/ideas.md).