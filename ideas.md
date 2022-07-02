# Name

- Odwrotna ~> Odna? (("Reverse" in Polish in reference to RPN))
- Search for short low-search-result names using simple syllables
- Don't do a C/Rust/Julia/or like any other damn language with a name that's annoying to google

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

While backwards solving is generally more efficient at systematically finding a path that leads to the end goal, forward solving instead allows for a much more explorative approach to problem solving, which aligns much more with human cognition, and allows one to dynamically work towards sub-goals without having to clearly specify from the beginning what ones end-goal is.

## Natural language

Of all sentence structures in natural languages, SOV is the most common. It places the operands (the subject and object) first, and the operation (the verb) last, contrary to SVO as it appears in English and many other European languages, where the operator is placed between the operands. The syntax in most common programming languages models this SVO order of terms (or even arguably VSO, depending on what subject and object are in our metaphor), drawing both directly, and indirectly via common mathematical notation, from the prevalent grammatical structure of the languages spoken by their designers.

The programming language proposed here however, works analogous to the SOV word order.

It is of course however debatable whether at all or how strongly the alignment of ones native language with a syntax for programming or mathematics influences ones ability to solve problems through use of that syntax, or whether the abstract representation one develops of a problem is independent of ones native language's syntax [->linguistic relativity], or the syntax one uses to represent a problem [->?; Has this been researched?]



# Ideas

(耳192, )

## Values

At any point in the program, can have a floating value, e.g.

```
5
# Here we have a floating value of type Int
```

We have four options as to what to do with it:

- _Keep it floating_: It will become the right-most argument to the next function
- _Stuff it into a variable_: It will remain there until we invoke the variable, refloating it
- _Stack it_: It will not become an argument to the next function, but ???
- _Discard it_: It will not became an argument to any function (this can be simply seen as stuffing with `~> _`)

## Identical syntax for types and data

Perhaps we can consider a function (i.e. label) declaration to be analogous or even logically identical to function invocation??

## Allow `_` wherever we can

A `_` can stand in for any position we might put a label, but we are in the given context not required to do so for successful compilation.

Alternatively, we could consider building the syntax such that any label is optional (though I'm not sure whether that would be pretty).

## Function definition

`param0_type param1_type param2_type \function_name result_type`

or anonymously:

`param0_type param1_type param2_type \ result_type`

or maybe to be less spacing sensitive:

`param0_type param1_type param2_type \\ result_type`

or maybe once more reusing `_`:

`param0_type param1_type param2_type \_ result_type` <-- *:thumbs_up:*

(Maybe `-> result_type` instead?)

## Function invocation

A label stated in the blank stands for invocation of the underlying function, it's inputs being any outstanding outputs.

```
give_me_a_number
inc
```

To turn a label into a value representing what it stands for, we have to escape it.

```
give_me_a_bunch_of_numbers
'inc map
```

(Maybe escaping could instead be something like `\{inc map}`, i.e. an anonymous function containing the escaped code)


## Named arguments

Allow for arguments to optionally be referred to by name, for cleaner partial application. I.e.:

`( increment@func [1 2 3 4 5]@list map )`

is equivalent to

`( [1 2 3 4 5] increment map )`

or maybe in OCaml-style (though kinda prefixy...)

`( ~list:[1 2 3 4 5] map )`

is a function that is awaiting one function as an argument.

(syntax TBD)

_Addendum:_

As part of a functions type annotation we can give arguments names, so that we can introduce them in the function body at an arbitrary location, instead of in order.

```
Int:x Int:y \subr -> Int
{
    y x sub
}
```

## Scope

Allow opening a new local scope at any time in some manner. Like `let ... in ...` in OCaml.

An option would be to consider `()` and `{}` as equivalent, with the distinction being that the latter opens a new local scope inside it, whereas the former does not. If we then also permit for a scope to be named, we would have namespaces covered as well. Naming of scopes could perhaps be aligned or even joined with function syntax (?).

_Question:_ Do we even need brackets that do not introduce a new local scope? Could we consider `()` as simply always introducing local scope, and leave `{}` for something else?

## Insert meta meaning into syntactic grammar

- Capitalization/Prefixing/etc. of identifiers has a meaning (e.g. Any "function" that operates on the type level is capitalized, any function that operates on the value level is not)
- Order of terms ? (e.g. a constructor has it's terms suffixed, and a function has it's terms prefixed)

And enforce these during compilation, and not just as coding guidelines.

_Alternative:_

Since the labels on the type level and the labels on the value level don't really interact (are the namespaces perhaps even completely separate?), we can probably reuse what different syntactic grammar means between the two situations.

## Match case anywhere

The way the `with` syntax works in Agda could in our case simply be possible at any point in the code, where we can pattern match on the current type in our function sequence. This is very much like imperative programming, but without introducing state.

Wait, can't we just integrate this with anonymous functions? Do we want pattern matching to be an operator or part of the function construction?

Something like:

```
[4,3,2,1]
match {
    hd :: tl => hd
    []       => 1
}
invert
print
```

(Syntax tbd, maybe something in the style of the ternary operator?)

## Mark exception-throwing / maybe-returning function

A function that can throw a runtime exception is marked with a `!` at the end of it's label (?) and a function that returns a `Maybe` is marked with a `?` at the end of it's label (???).

Alternatively, we could include information about a function potentially throwing a runtime exception in the function's type signature (return type?).

## Type tags

Following from the above point about exceptions, maybe it would be useful to have a system that allows one to add tags to values in the type system that provide additional information about the origins of the value. These are not like traits in Rust, i.e. not part of the type itself, but rather additional information about that specific value of the type, e.g. that the value might potentially be error throwing.

These tags would not restrict the usage of the value, unless deliberately specified by a function. For example a function could have a condition that it does not take a function as it's argument that would potentially throw and exception. E.g.:

```
(/) : Int -> Int -> (excpt) Int
critical : a, (!excpt) b => (a -> b) -> b
//// ==> We cannot call `{/ 0} critical`
```

問題: What else other than exceptions would we use this system for however?

問題: This is in a way similar or the opposite of constructors, we keep track of where our value came from, but contrary to constructors, we can still access the contained value, and don't have to wrapped in the constructor.

## Traits/Classes

If it's a good idea, why not just steal it?

Define a type category that requires certain functions to exist for a type, and allow for the type signature of a function or type definition to be able to require for a type parameter to satisfy certain class constraints.

(Syntax tbd)

## Uniform use of symbols

Reuse of symbols should correspond with similarity of concepts. For example, we should not reuse `{}` for e.g. sets, if it is used for declaring a code block.

## How to chain multiple inputs

```
{
    [1, 2, 3, 4 , 5]
    \{ inc toString }
    concat
}
```

## How to we treat a label existing

Given a label, there are two ways to interpret it, either as a reference to itself, or something to be evaluated.

Since most of the time we are interested in evaluation of a label, not in referencing it, the default behaviour should be evaluation, with reference requiring explicit escaping.

E.g.

```
toString
```

means that we expect some input, and give as `String` as output.

```
\toString
```

means that we expect no input, and give a `a -> String` as output. 

## Monad / Applicative

Some of the ideas pursued here are similar to the concepts behind the Monad..

## Constructor annotation

(Physical unit / grammatical case / etc.)-style annotations to values as syntax for constructors of a type.

These constructors can either be enumerations of a value or alternative constructors

```
enum <a> Option {
    'nothing
    a 'exists
}

data (Int) Time {
    h 'hours { (60 * h) 'minutes }
    m 'minutes { (60 * m) 'seconds }
    s 'seconds { s Time }
}

x :: Int Option = 17'exists
t :: Time = 18'hours + 30'minutes + 10'seconds
```

_Alternative:_ Introduce prefix `'` as simply indicating a function to be higher binding than anything that does not have the prefix, i.e. in the above example `hours` etc. are simply ordinary functions, which normally would bind lower than `+`, but by prefixing them with `'`, we are able to write them without the need for brackets. However, this already is the default behaviour in languages like Haskell, where a prefix function binds stronger than an infix function. Will it be the same here? In which case, we could just enforce that all constructors start with `'`, which would also align with the idea of _[Insert meta meaning into syntactic grammar](#insert-meta-meaning-into-syntactic-grammar)_.

## (Named) arguments

A function can take named arguments (which can be either mandatory or optional), which are assigned in an equivalent manner to variables, but do not share a namespace to prevent accidentally passing unindented optional arguments to a function.

E.g. A function `splitlines` has optional argument `keep_lineends`;

```
"Hello\nWorld!"
true => keep_lineends
splitlines
```

(Syntax tbd; Maybe the `~>` Syntax used so far for normal variable assignment would make more sense to be used for this, with variable assignment getting a new style of arrow, like e.g. `=>` or `->` or `|>` (??). Alternatively, maybe assignment to a named argument could be with `~`, and `~>` remains as normal variable assignment)

_Addendum:_

A function's unnamed arguments (which perhaps could be limited to one), could simply be on it's argument stack at the onset of the function body, and additional named arguments can be introduced at any time (either as unmet labels or perhaps with some marker that we are referring to an argument, not a label in the current scope).

In terms of inferring the type of a function, this would permit us to simply consider as it's arguments any unmet inputs, and any named arguments that are introduced throughout the function body. We would not have to specify a signature providing names and types for arguments unless desired (and perhaps needed in some edge-cases).

## Constructors and functions

The only difference between a constructor (in the Rust->enum sense) and a function is that for the former we remember where we came from, whereas for the latter we do not. And with constructors we commonly unify multiple of them under a type, knowing always that if that type appears, it has to have come from one of these constructors.

What even is the difference between a namespace and a GADT?

## Escaping / Bracket evaluation

How about `()` is evaluated, i.e. it's contents are executed upon being encountered and it's value is the result of this execution, whereas `{}` is not executed upon being encountered, and it's value is the function the block stands for.

## Forward and backward solving

At any point in writing a program, we know what types we currently have available on the argument stack. And if there is a type signature present, or the result of the current code block already is promised as input to a function, we also know the type we have to work towards. Both of these informations should be presented to the programmer, dynamically giving them the implicit task of "This is what we have, and this is where we want to go". We can also provide highly tailored autocomplete based on this information (perhaps even have a shortcut for autocompleting functions that give the desired result?)

Perhaps it could be highlighted in the code from which unmet outputs our currently dangling values come from.

This structure of information can also be interesting in explorative code generation, since it always is precisely clear what can be done.

Under ideal circumstances, the programmer's workflow should consist mostly of having to pick the correct function from a provided few that match the current argument stack, and only really have to provide any introductions (i.e. Pulling something out of the hat, like a constant or new function definition). Though it has to be considered that if we make extensive use of named arguments the default, that this wouldn't work as well, or rather, we should probably ignore named arguments (that are not present) for autocomplete, and perhaps dynamically introduce them for the programmer upon them picking a function (問: How much would this widen the search/result space and impact practicability?)

_More ideas for IDE features:_

- Highlight somehow which arguments end up as inputs to which functions.
- 

## Namespaces, types and constructors

A constructor is a function, which instead of returning something, simply wraps it's arguments in it's name. It is differentiated visually by a `'` at the beginning of the name (or maybe we should use `` ` `` instead?) E.g.

```
\Int : 'wrapped

1729'wrapped ~> a_wrapped_int
```

A `struct`-style representation of data is a single constructor. To assert that we want a specific constructor (i.e. a certain `struct`) in a type signature, we specify the constructor name. E.g.

```
\Float Float Float : 'vec3

1.0 1.0 0.5 'vec3 ~> 'vec3 : a_3d_vector

\'vec3 : length_sq
{
    | x y z 'vec3 => (
        x*x + y*y + z*z
    )
}

a_3d_vector length_sq ~> Float : the_length_of_the_vector
```

An `enum`-style representation of data is a namespace containing multiple constructors. To assert that we want any constructor from a certain namespace (i.e. a certain `enum`) in a type signature, we specify the name of the namespace E.g.

```
\\a : Option
{
    \'nothing
    \a : 'exists
}

\Int Option -> Int : default
{
    | 'nothing => 0
    | x 'exists => x
}

(1729'exists default) ~> Int : exists_gives_int
('nothing default) 0 ~> Int : nothing_also_gives_int
```

問: How do we handle importing a namespace? In the above example I assume that I can just write the `'nothing` and `'exists` constructors without specifying from which namespace they come from, i.e. I assume implicit importing, which isn't really what we want. But we also probably don't want to write something like `Option.'exists`, or perhaps sugared a bit `'Option.exists` (which is arguably pretty ugly, but allows for stuff like `x'Option.exists`, instead of `x Option.'exists`...).

### Namespaces?

Maybe not a good name, what could we call it instead? A "module"? Though that already has certain connotations from other languages.

So like in the above example, we want to give type arguments to our namespaces. A namespace's name is part of the type system and can be specified as a type in a type signature, which means that we ask for any constructor that is specified in that namespace.

_What do we do with sub-namespaces?_

Perhaps it would make sense to allow both, either permitting only constructors defined in the namespace itself (default), e.g. `A -> A : function`, or also allow any sub-namespaces, e.g. `A* -> A* : function`.

## Pattern matching

To destruct constructed values and to differentiate between different possible inputs (-> `enum`-style use of namespaces), we want to we able to pattern match on a value. What would be a clean syntax for this?

_One option would be:_

```
{
    | x 'exists => (...)
    | 'nothing => (...)
}
```

This is relatively clean, however, if there is a lot of code in the branches, then all that keeps things tidy is indentation, which is perhaps not ideal. Also, it isn't visually too clear that these different pattern matches are linked.

To simply destruct a single `struct`-style constructor:

```
\'vec3 'vec3 -> 'vec3 : add
{
    | x1 y1 z1 'vec3
    | x2 y2 z2 'vec3
    (x1 + x2) (y1 + y2) (z1 + z2) 'vec3
}
```

I.e. we either introduce different if-then branches with `=>`, or we simply introduce the used names into the current namespace without `=>`.

An if-then-else style of branching can be achieved with:

```
{
    | 'true => (...)
    | _ => (...)
}
```

Maybe instead if the `|`, something involving the `?` would also be usable.