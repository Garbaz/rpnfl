# Why

## Forward solving

Because we want to think forward, i.e., we know what we have, and what we can do with it, and start building ourselves a path towards what we want.

Also, we can at any point in time while we are coding autocomplete exactly those functions that fit the types we currently have dangling, i.e.

```
[1, 2, 3, 4]
## Ctrl+SPACE
## => Any function in scope that can take an `Int List` as an argument, e.g. sum
```

```
[1, 2, 3, 4]
'inc
## Ctrl+SPACE
## => Any function in scope that can take an `Int List` and a `Int -> Int` as arguments, e.g. map
```

While backwards solving is generally more efficient at systematically finding a path that leads to the end goal, forward solving instead allows for an explorative approach to problem solving, which aligns much more with human cognition, and allows one to dynamically work towards sub-goals without having to clearly specify from the beginning what ones end-goal is.

## Natural language

Of all sentence structures in natural languages, SOV is the most common. It places the operands (the subject and object) first, and the operation (the verb) last, contrary to SVO as it appears in English, where the operator is placed between the operands. The syntax in most common programming languages models this SVO order of terms (or even arguably VSO, depending on what subject and object are in our metaphor), drawing both directly, and indirectly via common mathematical notation, from the prevalent grammatical structure of the languages spoken by their designers.

The programming language proposed here however, works analogous to the SOV word order.

It is of course debatable whether at all or how strongly the alignment of ones native language with a syntax for programming or mathematics influences ones ability to solve problems through use of that syntax, or whether the abstract representation one develops of a problem is independent of ones native language's syntax [->linguistic relativity], or the syntax one uses to represent a problem [->?; Has this been researched?]