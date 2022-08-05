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

`param0_type param1_type param2_type \_ result_type` <-- 👍

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


## How do we treat a label existing

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

((`~` probably is the best choice here. In most cases, we will likely just have something like `(...)~argument_name`, or rather, the auto-formatter should align things like that. That doesn't look all too bad.))

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


### 目:

- https://www.youtube.com/watch?v=UkDSL0U9ndQ


## Namespaces, types and constructors

A constructor is a function, which instead of returning something, simply wraps it's arguments in it's name. It is differentiated visually by a `'` at the beginning of the name (or maybe we should use `` ` `` instead?) E.g.

```
\Int : 'wrapped

1729'wrapped ~> a_wrapped_int
```

A `struct`-style representation of data is a single constructor. To assert that we want a specific constructor (i.e. a certain `struct`) in a type signature, we specify the constructor name. E.g.

```
\Float -> Float -> Float : 'vec3

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

問: How do we handle importing a namespace? In the above example I assume that I can just write the `'nothing` and `'exists` constructors without specifying from which namespace they come from, i.e. I assume implicit importing, which isn't really what we want (looking at you Haskell). But we also probably don't want to write something like `Option.'exists`, or perhaps sugared a bit `'Option.exists` (which is arguably pretty ugly, but allows for stuff like `x'Option.exists`, instead of `x Option.'exists`...).

答: The left-branching-normativity has struck me once more. Instead of writing `Option.'exists`, we could write something like `'exists@Option`. Or we could simply not have a syntax as such, but rather allow to open a Namespace in local scope, i.e. something like `(%Option x'exists)`, with the intended usage being that we open namespaces in a local, but larger scope, instead of referring via Some kind of `A.B.C.d` syntax to functions. Of course we permit renaming as well, to allow for using two conflicting namespaces simultaneously. Maybe we could use the `@` for that instead.

_Addendum:_

It might be that the whole idea of prefixing constructors with `'` isn't so good after all. Instead we could use `'a` as type variables in type signatures, and constructors simply look the same as functions (?). Depends on how unclean it looks in the end to have `'` strewn everywhere.

_Addendum 2:_

Maybe instead of using `\\` for namespaces, we should use it for constructors?


### Namespaces?

Maybe not a good name, what could we call it instead? A "module"? Though that already has certain connotations from other languages.

As in the above example, we want to give type arguments to our namespaces. A namespace's name is part of the type system and can be specified as a type in a type signature, which means that we ask for any constructor that is specified in that namespace.

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

反: Instead of thinking of pattern matches as different branches of one match, we could (at least semantically), consider them as guards, i.e., either a value passes a pattern match, going into it's body, or it fails, going past the pattern match.

反反: That will ruin typing though if the body's output does not have the same type as the type we are matching against.

反反反: Polymorphic typing, maybe? That smells like unintended behaviour. Though arguably, there is no difference other than semantics between treating modules as types and having any arbitrary combination of constructors as a type. 

To simply destruct a single `struct`-style constructor:

```
\'vec3 -> 'vec3 -> 'vec3 : add
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


## Combine Boolean and Option

Instead of having separate types for boolean logic and option logic (-> monads), we combine the two into one unified system of logic. E.g.

```
\a -> a -> a Option : (==)
{
    ~> x ~> y
    x y equal
    | 'true => x'exists
    | _     => 'nothing
}

\a Option -> a Option -> a Option : (&&)
{
    ~> x ~> y
    (x, y)
    | (_'exists, _'exists) => y'exists
    | _ => 'nothing
}

\a Option -> a Option -> a Option : (||)
{
    ~> x ~> y
    (x, y)
    | (_'exists, _) => x
    | ('nothing, y'exists) => y
    | ('nothing, 'nothing) => 'nothing
}

\a Option -> (a -> b) -> b Option : (??)
{
    ~> x ~> f
    x
    | v'exists => (v f)'exists
    | 'nothing => 'nothing
}
```

We can combine `??` with `||` to get a ternary operator:

```
condition ?? (then do this with value) || (else do that)
```

问: For this ternary operator to make much sense would require lazy evaluation though, otherwise both the "then" and the "else" branch will be evaluated.

答: A wild idea, but maybe we could consider `()` as eagerly evaluated and `{}` as lazily evaluated (??). Not sure if that makes sense, but might be worth a consideration. Or is what I called "escaping" before simply equivalent to lazy evaluation?


## Currying & Manipulating the argument stack

Functions curry by default, i.e. if in a code block, there are inputs to a function dangling, then they become inputs of the entire code block.

If we have a function from which we want to return multiple things, we could either support simply having multiple results directly, i.e. calling the function will cause multiple floating values to appear. Not sure how clean that would be.\
Or we could have a built-in operator that transforms a floating tuple into multiple floating values.

More generally, it might make sense to have bespoke operators that allow us to manipulate the argument stack to avoid having to do too much binding, e.g. swapping things around, etc.


## Polymorphisms

Polymorphic functions, i.e. functions that take some arbitrary type parameter (with possibly type-class/trait restrictions) and implement some general code 👍. Same for polymorphic modules & constructors (i.e. enum-style data structures and struct-style data structures) 👍.

Records are simply constructors with named arguments. Do we want row polymorphism for constructors? Should we extend that to functions, i.e. we can specify as an argument's type a function that takes a minimum of certain arguments. What happens to the rest? Curry, but what to? That's nonsense.


## Function Type Isomorphism

As an explicit opt-in perhaps with some operator, we could allow for a function's arguments to be scrambled to fit whatever input is available, e.g.

```
/// f : Int -> Bool -> String

"hello" 1 @f
```

_Alternative:_ Instead of making in an operator we put in place at the call location, we could have functions be divided into ordered functions and non ordered functions, with the latter having either be unambiguous about the input types (like `f` above), or be symmetric (like e.g. `+` or `Set::union`). I.e. a non-ordered function can take it's arguments in any order. (<- 👍 _better than operator_)

We could simply do this via type annotation, e.g.:

```
\Int; String -> Bool : ordered_func
\Int, String -> Bool : unordered_func
```

General idea: ordered terms <-> `;` || unordered terms <-> `,`. Or some other pair of operators &->> List vs. Map


## List, Set, Map ; Tuple, Bunch, Record

A **list** is an unbounded ordered collection of values.

```
[;]
[x;]
[x; y; z;]
```

A **set** is an unbounded unordered collection of values.

```
[,]
[x,]
[x, y, z,]
```

A **map** is an unbounded unordered collection of value-key pairs.

```
[=]
[x=a]
[x=a, y=b, z=c,]
```

Analogously for **tuple**/**bunch**/**record**, but with different brackets.

_Question:_

How exactly does e.g. a record type differ from a "namespace type", i.e. an interface / signature. Both a namespace and a record are simply a structure that has labels inside it that refer to certain values (or functions).

-> First class namespaces (really should call these modules or something more descriptive) = records

**Wait a second:**

Or aren't records/tuples simply anonymous constructors?!

And isn't a "namespace" (i.e. really a module) also just a record, i.e. a constructor.

Can we unify all these things into one neat syntax?


## Big Syntax Shuffle

We prefix a (named argument) tag with `'`, and that `'` remains part of the name, separating their namespace from the namespace of variables:

```
\Int'x , Int'y -> Int : func
{
    'x + 'y
}
```

Variable & named argument assignment with `=`:

```
{
    [1,2,3,4] =l
    1729 ='x
}
```


## The dot syntax

If we have any kind of namespace (explicit namespace, implicit namespace, record), we can use the `.` to look inside the namespace.


## Arguments <=> Tuples+Records

Just as we can uncurry a function taking multiple positional arguments to take a tuple of arguments, we can uncurry a function taking named arguments to take a record of arguments. Same with unordered arguments and bunch.

However, if we allow mixed use of named and unnamed arguments, how do we handle uncurrying such a function?

Would it make sense to unify the static data types (tuple, bunch, record) into one? Or maybe at least combine bunch with record into one ordered data type and bunch with record into one unordered data type?

_Inverse idea:_ Take the current floating value stack and bundle it up into a static data structure. E.g.

```
1
2
3
4
*bundle up*
/// => (1;2;3;4)
/// or => (1,2,3,4)
```

or

```
1 ='a
2 ='b
*bundle up*
/// => (1='a, 2='b)
```


## Refer to super-scope

Allow to refer to any arbitrary super-scope from any point. This is a superset of recursion (& stuff like `break` in imperative languages). 
```
{ $0 } = inf_loop
{
    =y
    | 0 => y
    | suc x => x y $0 suc
} = add
```

_Note:_ This is probably not a good feature for this language, but maybe interesting to explore in itself. Maybe something like: A lambda calculus-style minimal programming language where $n refers to super-scopes and %n to arguments. to the current scope. Or perhaps rather we refer the m-th argument of the n-th scope with $n.m? (目[scope-lambda-calc](/scope-lambda-calc))


## Constants as functions

Would it make sense to consider e.g. `7` a function? It takes no arguments and returns a certain `Int`. Or perhaps an infinite sequence of identical `Int`s?

Or even allow numerals to be polymorphic functions, e.g. to use `0` or `1` also in other situations (probably not pretty)?

It would make more sense to rather consider numerals as a special case of constructors.


## A clean way to handle multiple results

There are situations, especially in pure functional programming, that we want to return multiple values as our result. However, often times these results are not of equal utility. Rather, we might want to return the actual result we wanted to achieve, together with some metadata that we need for recursion or for some meta use like logging. However, generally there is no clean way to separate this main result from our meta results, especially not in a way where we can simply act as if the meta results do not exist for most purposes and then access them when actually needed.

How about, just like with ordered/unordered/named arguments, we have ordered/unordered/named return values?

Something where we can pick out the return values that we care about and ignore the rest (for later, or completely), as long as it is unambiguous.

How does this interact with the arguments to a function?

Why shouldn't we just be able to write a function like this:

```
\Int'x -> Int , Bool : f
{
    'x even
    'x + 1
}
```

I.e. everything that get's left hanging around at the end of our function becomes a result? And for ordered, maybe like this:

```
\Int'x -> Int ; Bool : g
{
    'x + 1 ;
    'x even
}
```

Though what happens when we call a function like this to the value stack? Are they just pushed on in order? Then what's the difference between having ordered and unordered results?

## Consider tuple/bunch/record as equivalent to each other and to function arguments

Could we, at least in an opt-in manner, act as if we are simply handing around tuples/bunches/records, or rather even a common fixed-size collection type, instead of having a separate but equivalent semantic for these collections and the arguments (^and perhaps results) of functions?

Also, introduce a semantic for tuples/bunches/records being coercible, i.e. we can always consider a collection that has more as one that has less.

_Idea:_ How about we make `a,b,c` and `(a,b,c)` equal to each other, but not equal to `[a,b,c]`. This would make for a neat general semantic idea that `(`/`)` have no meaning other than to define precedence in parsing, whereas `[`/`]` introduce an orthogonal dimension. So `(a,b),c == a,(b,c)`, but `[a,b],c != a,[b,c]`. So `,` becomes a simple associative infix operator again that works with `(`/`)` just like any other infix operator.


## Type annotation for variables

It makes sense for modules and functions/constructors to have the type annotation on the left, since there is where the arguments will appear later, but how do we do type annotation for variables? Do we even need the option, or can we just assume that we always can infer the type?

```
blorbo =x 
blarf  = Int -> Int : y /// This doesn't look very sexy
```

## Dealing with missing type class implementations

A situation that comes up in languages that use type classes like Rust or Haskell, is that we have some external type that we want to use, and some external type class that we want that type to fulfil for some usage, but the creator of that type did not implement that type class. Especially when the implementation we seek is simply the derived one, this can be frustrating. How can we do this better?

Would it make sense to introduce some way that allows us either generally define the implementation of an external type class for an external type? This is something Rust explicitly forbids, likely for good reason.

Alternatively, would it make sense to at least allow us to ask the compiler "on the fly" to use the derived implementation of a type class for a type, if possible?

## Type classes

From ^, I notice we don't have a syntax for declaring a type class. Would it make sense to use `\\` for a type class instead of for a module? What is a type class really for the purposes of this language?

## Do notation

Haskell provides a nice `do ...` notation for working with monads. This is very similar to how code looks like here. Could we provide perhaps an extension to our syntax like `#{...}` that causes the code block to automatically handle monads? So we can effectively write the same code, but instead of function composition, we have monad composition. Or can we make this even more general? Or as a macro instead of bespoke?

## Infix operator precedence

Assuming we do allow arbitrary custom defined infix operators, one option would be to allow providing their precedence and association somehow in the code. I would argue against this option, since

1: This would mean that our source code would suddenly talk back to the parser, which I find quite ugly

2: While operator precedence is well known for common mathematical operators, when custom operators are combined, if we do not bracket the expressions, the interpretation of the code by the user becomes unclear.

Instead, I would rather implement general sensible rules that specify operator precedence across the language. Ideas for that would be:

- Infix operators over functions, e.g. `x f <|> y g` should be parsed as `(x f) <|> (y g)`
- Long operator over short operator, e.g. `a + b <|> c` should be parsed as `(a + b) <|> c`, since `<|>` consists of 3 characters, whereas `+` only of 1 (How would this interplay with standard infix operators? What about something like `a + b // c`, where `//` is supposed to be cast-down-division?)
- Bespoke precedence for built-ins (Or maybe not? Like, how often is it really an annoyance to have to bracket e.g. `a + b * c` as `a + (b * c)`. Might that actually be helpful to reduce mistakes?)

Though the question remains how we would allow for permitting both left and right associative operators. Just enforce one? Make it depend on the operator or it's function somehow?