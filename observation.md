# Observations of Intuition

## Consistent syntax confuses

```lisp
(cons ...
    (t (...)))
```

Why is `t` here not a function? Usually `(some-symbol ...)` always is a function call in Lisp.

## The easier choice should be the right choice

Compared to `equal`, `eq` is easier to write, but when we mean "equality", we intuitively mean what `equal` does.

## We want to write first what we do first

```
(func (+ 5 7))
```

We want to do `5 + 7` first, but have to write `func` first.

Hence RPN...

