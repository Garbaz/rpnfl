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

## Structure vs concepts

While not as explicit in a language like English, there is a fundamental difference between the concepts represented in a sentence, and the sentence's structure. And structure here is not just grammar in the sense of word order, but also small words or conjugation, parts of a sentence that don't ultimately represent any new thing, but rather specifies the relation of the things we are talking about. A concept is anything that introduces a new idea, whereas structure simply relates the arrangement of these ideas.

In many natural languages, including the English language, we generally do not make much of a distinction between these two. Other than basic sentence structure (word order, punctuation), everything else is represented via words or conjugation. However, in some languages, most visibly perhaps Japanese, these differences are very apparent. While not strictly separated into two distinct alphabets, generally, if one looks at a Japanese sentence, the structure of the sentence is immediately apparent, due to particles and conjugation being consistently written in one alphabet that stands out. For example:

```
ネズミは走り、ネコはそれを追った。
￣￣￣　￣　　￣￣　　　　￣
```

Everything underlined here directly represents the core concepts of the sentence (Mouse, to run, Cat, to chase), and also uses different alphabets than the the particles and conjugation that dictate the relation between these core concepts and the overall structure of the sentence.

While it is perhaps arguable that given natural language generally is primarily spoken, and therefore usually at least partially comprehended via subvocalization, this kind of structure is, though useful, not entirely necessary in a writing system. I would argue however that for a programming language, which is primarily, if not entirely, comprehended via direct parsing of it's structure, having such a very clear distinction between structure and concepts makes a lot of sense.

Specifically, what I would consider a concept is programming, is anything that should invoke us to consider ideas beyond the immediate lines of code in front of us, like some specific algorithm or data structure. Just like with a natural language, it should be things that introduce something new, rather than simply specifying the structure of how the things we do have are arranged.

However, across almost all programming languages, we rarely make this distinction between concepts and structure in our choice of syntax, very commonly using keywords for structure (e.g. `class`, `while`, `fn`, `where`), which are indifferentiable from function or variable name. Though using symbols for concepts is rare outside of notation that has been inherited from mathematics (e.g. `+`, `==`). While we try our best to bring this distinction back in by differentiating any keywords of a programming language via colour or even font weight, for this language, I would rather directly and more clearly make this distinction in the design of the language itself. All alphanumeric words represent concepts (variables, functions, data types), while all symbols represent structure (perhaps making an exception for mathematical notation, since that's pretty baked in to our minds).

