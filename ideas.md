# Ideas

This is a chronological collection of ideas I had/have about the language. That means that earlier ideas are potentially obsolete or have been reconsidered with later ideas.

(耳192, )


## Values

At any point in the program, we can have a floating value, e.g.

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

加:

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

问: Do we even need brackets that do not introduce a new local scope? Could we consider `()` as simply always introducing local scope, and leave `{}` for something else?


## Insert meta meaning into syntactic grammar

- Capitalization/Prefixing/etc. of identifiers has a meaning (e.g. Any "function" that operates on the type level is capitalized, any function that operates on the value level is not)
- Order of terms ? (e.g. a constructor has it's terms suffixed, and a function has it's terms prefixed)

And enforce these during compilation, and not just as coding guidelines.

另: Since the labels on the type level and the labels on the value level don't really interact (are the namespaces perhaps even completely separate?), we can probably reuse what different syntactic grammar means between the two situations.


## Match case anywhere

The way the `with` syntax works in Agda could in our case simply be possible at any point in the code, where we can pattern match on the current type in our function sequence. This is very much like imperative programming, but without introducing state.

Wait, can't we just integrate this with anonymous functions? Do we want pattern matching to be an operator or part of the function construction?

_Something like:_

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

問題: This is in a way similar or the opposite of constructors, we keep track of where our value came from, but contrary to constructors, we can still access the contained value, and don't have it wrapped in the constructor.


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

另: Introduce prefix `'` as simply indicating a function to be higher binding than anything that does not have the prefix, i.e. in the above example `hours` etc. are simply ordinary functions, which normally would bind lower than `+`, but by prefixing them with `'`, we are able to write them without the need for brackets. However, this already is the default behaviour in languages like Haskell, where a prefix function binds stronger than an infix function. Will it be the same here? In which case, we could just enforce that all constructors start with `'`, which would also align with the idea of _[Insert meta meaning into syntactic grammar](#insert-meta-meaning-into-syntactic-grammar)_.


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

加:

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

More ideas for IDE features:

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

Addendum:

It might be that the whole idea of prefixing constructors with `'` isn't so good after all. Instead we could use `'a` as type variables in type signatures, and constructors simply look the same as functions (?). Depends on how unclean it looks in the end to have `'` strewn everywhere.

Addendum 2:

Maybe instead of using `\\` for namespaces, we should use it for constructors?


### Namespaces?

Maybe not a good name, what could we call it instead? A "module"? Though that already has certain connotations from other languages.

As in the above example, we want to give type arguments to our namespaces. A namespace's name is part of the type system and can be specified as a type in a type signature, which means that we ask for any constructor that is specified in that namespace.

_What do we do with sub-namespaces?_

Perhaps it would make sense to allow both, either permitting only constructors defined in the namespace itself (default), e.g. `A -> A : function`, or also allow any sub-namespaces, e.g. `A* -> A* : function`.


## Pattern matching

To destruct constructed values and to differentiate between different possible inputs (-> `enum`-style use of namespaces), we want to we able to pattern match on a value. What would be a clean syntax for this?

One option would be:

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

Alternative: Instead of making in an operator we put in place at the call location, we could have functions be divided into ordered functions and non ordered functions, with the latter having either be unambiguous about the input types (like `f` above), or be symmetric (like e.g. `+` or `Set::union`). I.e. a non-ordered function can take it's arguments in any order. (<- 👍 _better than operator_)

We could simply do this via type annotation, e.g.:

```
\Int; String -> Bool : ordered_func
\Int, String -> Bool : unordered_func
```

_General idea:_ ordered terms <-> `;` || unordered terms <-> `,`. Or some other pair of operators &->> List vs. Map


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

问:

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

观: This is probably not a good feature for this language, but maybe interesting to explore in itself. Maybe something like: A lambda calculus-style minimal programming language where $n refers to super-scopes and %n to arguments. to the current scope. Or perhaps rather we refer the m-th argument of the n-th scope with $n.m? (目[scope-lambda-calc](/scope-lambda-calc))


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

议: How about we make `a,b,c` and `(a,b,c)` equal to each other, but not equal to `[a,b,c]`. This would make for a neat general semantic idea that `(`/`)` have no meaning other than to define precedence in parsing, whereas `[`/`]` introduce an orthogonal dimension. So `(a,b),c == a,(b,c)`, but `[a,b],c != a,[b,c]`. So `,` becomes a simple associative infix operator again that works with `(`/`)` just like any other infix operator. <-- 👍


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


## First class functors / monads

Not sure entirely what this would entail. Vaguely: Make working with them as convenient as possible.


## Infix operator precedence

Assuming we do allow arbitrary custom defined infix operators, one option would be to allow providing their precedence and association somehow in the code. I would argue against this option, since

_1:_ This would mean that our source code would suddenly talk back to the parser, which I find quite ugly

_2:_ While operator precedence is well known for common mathematical operators, when custom operators are combined, if we do not bracket the expressions, the interpretation of the code by the user becomes unclear.

Instead, I would rather implement general sensible rules that specify operator precedence across the language. Ideas for that would be:

- Infix operators over functions, e.g. `x f <|> y g` should be parsed as `(x f) <|> (y g)`
- Long operator over short operator, e.g. `a + b <|> c` should be parsed as `(a + b) <|> c`, since `<|>` consists of 3 characters, whereas `+` only of 1 (How would this interplay with standard infix operators? What about something like `a + b // c`, where `//` is supposed to be cast-down-division?)
- Bespoke precedence for built-ins (Or maybe not? Like, how often is it really an annoyance to have to bracket e.g. `a + b * c` as `a + (b * c)`. Might that actually be helpful to reduce mistakes?)

Though the question remains how we would allow for permitting both left and right associative operators. Just enforce one? Make it depend on the operator or it's function somehow?

另: We could also simply not permit custom infix operators. The idea of the language after all is that symbols represent structure, and words represent concepts. If there is some larger concept behind an operator, then it might just be better off as a function. However, this would mean that for any actually structural operators we might want in our language, we do have to bake them in, instead of being able to extend them as fit like we can in Haskell. Another argument against custom operators is that, while they make the code _elegant_, in the long term, they don't make the code _readable_. It only makes sense for something to be represented as an operator, if it either is part of the core code structure, or represents something that ultimately is not relevant to follow what the code is doing. We should not have to look up the definition (and especially not the precedence) of infix operators when reading a piece of code.


## Monadic IO

目: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/07/mark.pdf


## Variants, once more

议: Everything is functions. A constructor is a function that does not compute, but simply encapsulates it's arguments. A sum (/variant/enum) type is a set of constructors. For type inference / checking, this would mean that we have to simply figure out the set of constructors that a certain argument or result might be.

This might be utilizable for optimization (you don't need to compile a code path for a constructor of a variant that never is used), or directly in the semantics of the program. A module as a type simply is a convenient shorthand for the set of constructors it contains. 

另: Can we generalize to "A type is either a constructor, or a set of types"? (What for though?)


## Differentiate ordered / unordered by brackets

Code is an ordered collection of functions. A module is an unordered collection of functions. Should they be differentiated by brackets? However, which? `{}` are cool for either, but neither `()` nor `[]` really look great if opened over an entire block like `{}`...


## Alternative syntaxes for function

```
eq = \Int; Int -> Bool
///or
\eq : Int; Int -> Bool
```

In a syntax like `Int'x -> Int , Bool : f`, the function name being at the end might not be great for general usability. When we are in search of a specific function by name, it is visually difficult to always have to find the end of each function signature. And it just semantically is generally the case that if we define a named function, it is the function name that we have in mind as it's primary identifier.

反: This however does break analogy to the application of a function, e.g. `\eq : Int, Int -> Bool` is applied like `0 1 eq`.


## Considerations about constructors/variants

A constructor in essence describes either the definition of an atomic thing, or more generally a thing that is made of other things. Notably, a constructor therefore always represents a thing, not an action. In alignment to the practice observed in e.g. the grammar of the natural language German, we therefore capitalize a constructor to clarify that it is a thing `\Float ; Float : Vec`, not an action `\Float , Float -> Float : add {}`. This gives us product types.

To represent sum types, we need to extend this to the ability of having a type that can be any of a set of constructors. Therefore we define a variant, which is just a parametrized collection of constructors, like

```
\\a : Maybe {
    \a : Exists
    \Nothing
    \a Maybe , (a -> b Maybe)'f -> b Maybe : chain {
        | Nothing => Nothing
        | x Exists => x f
    }
}
```

观: We are getting quite a lot of `\`s in the code. The question might be, do we really need them? Especially the `\\` is kinda ugly, and should perhaps be replaced with something like `#`.

议: How about we only introduce untagged/unnamed arguments at the beginning of the function, with any tagged arguments being introduced with the tag they were given in the signature.

To work with polymorphic types, we would like to be able to constrain a general type variable in some form, that the types that might be substituted for it have to satisfy certain conditions. In particular, we want to ensure that certain functions are defined for a type, such that we can legally use these inside a function body.

```
\[Eq : a] => a , a -> Bool : neq {
    (==)
    | False => True
    | True  => False
}
```

另: We might also consider a type class and a variant as one thing, a collection of constructors and functions that we promise to be present for the argument that is provided. The constructors say how the argument might have been constructed, the functions say how the argument might be used. This introduces a notion of subtying, where we consider one collection of constructors and functions a subtype of another, if the former has **at most** the constructors and **at least** the functions of the later. This means that inside the definition of a function which takes an argument by the later specification, we can provide an argument that satisfies the former specification, and are still guaranteed that the function definition is still legal, i.e. that any pattern matching (destruction) will capture any possible value of the argument, and any function application will be well defined. <-- 👍?

^ Therefore, instead of having type classes that certain type arguments have to comply with, we can directly use a type class as a type. E.g.:

```
\Show -> String : greet {
    show
    ("Hello " ++)
}
```

However, we still want to be able to ensure that two arguments are of the same type, instead of two types that fit a provided type (so that their implementations of functions fit):

```
\[Eq < a] a , a -> Bool : neq {
    (==)
    not
}
```

And importantly, not just can we consider an argument satisfying one type, but a union of types (i.e. the argument can be either type's constructor, and we can only use functions that both implement)

```
\Int | Vec : times-two {
    =x
    x + x
}
```

观: It would not really make sense to define the intersection of two types, since that would not longer clearly specify what functions can be used inside the function.

However, a consideration should be made how we define what constitutes one type being a subtype of another. For example, if we just define a function `show` in our type declaration, does that automatically mean we are a subtype of `Show` since we have all the functions declared that `Show` specifies? Or do we have to explicitly say that our type's `show` actually is supposed to be the `show` that `Show` defines, not just some random function that happens to have the same name. Also, `show` has to in some manner be declared as abstract in `Show` (e.g. simply leaving away the body `{}`). If that's the case, how about we simply refer to the type `Show` in our definition, either by it's path `\Blorbo -> String : Show.show {...}`, or by opening the Show namespace inside our type declaration.

Another consideration would be the option to extend a type locally. E.g. implementing some type signature we have come up with for our own project for some generic type elsewhere. E.g.:

```
#+Int {
    \Int -> String : Show.show {...}
}
```


## Marking symmetric functions

There are many function which are symmetric, like `\Float , Float -> Float : add {}` or `\a , a -> Bool : eq {}`. However, unaware of the symmetry, these will be considered ambiguous if defined with unordered arguments. Therefore, we would either define them as ordered or with differently named arguments. However, both of these options are somewhat antithetical to them being symmetric. Therefore, it might be of interest to introduce either an option to mark a function as symmetric, which in essence simply causes the compiler to choose one arbitrary order to the arguments in application, or introduce actually an option to proof the symmetry of the function on a meta level to ensure that we are not erroneously deeming functions as symmetric when in fact they are not.


## Ordered arguments

Are ordered arguments in the first place a good idea? Would it make sense for the language to simply not have ordered arguments at all?

反: For infix operations like `>=`, we do need ordered arguments. However, this can be considered a special case.

议: We only allow ordered arguments for infix operators. Perhaps a syntax like this:

```
\a List : ++ : a List -> a List {

}
```

or something less crazy.


## Meta-data for data

A common occurrence in constructing data types is that we end up bundling some extra data with the data we actually care about for special occasions/recursion, requiring us to always destruct the data and discard the specialized part to get at the data we care about.

One possibility in resolving this is to allow meta-data to be attached to data. In essence, our data just becomes a pair consisting of the type of the meta data and the data, however, when we access a variable of such a type, it is automatically destructed to only the data, unless we explicitly access the meta data. This does not extend the expressibility of our language, but allows for more convenient usage.

One very clear application would be to handle errors that we would not like to explicitly represent with a `Maybe`/etc., like division by 0, but still be catchable at runtime. E.g.:

```
\Int , Int -> Int$Error : div {
    =x
    | 0 => 0$("division by zero" Error)
    | y => (x / y)$(NoError)
}
```

(syntax TBD)

This would mean that we can treat the result as if it simply were an `Int`, but at the same time carry with us when something went wrong. How this is handled externally is a consideration left open. One possibility would be to consider all computation as inherently monadic, with the context simply being always carried through implicitly instead of requiring explicit acknowledgement of it. <-- ❓

另: This general idea could also be considered as a variant on named arguments, where we can take parts of a type that are only considered part of the type if accessed directly via name.

```
\String, Int'm : Blam

"Hello World" 1729'm Blam -> b

(b == "Hello World") assert
(b'm == 1729) assert
```

(syntax TBD)

观: More generally, we want to be able to have multiple perspectives on the same data such that we don't have to be overly verbose in working with it. This includes namespaces (the data of all declared variables) & named arguments for functions / constructors, but could just as well be expanded arbitrarily. The idea hereby being that, just as we work with abstract concepts in our mind, we can write programs where we only consider the parts of our data that we care about at this moment, without having to consider the entire extend of details our data contains. We should be able to specify and carry the details through our program, but have them for our own syntactic understanding separated from the major parts we care about.


## Type Tag as data wrapper

The purpose of type tags is to differentiate types such that the type signature of an unordered function is unambiguous. In essence however, a type tag is not really different from a wrapper type (i.e. a constructor of a single polymorphic argument). E.g.:

```
\a : W
\Int W , Int -> Int : f {
    |(x W) => x =x
    =y
    x + y
}

/// achieves the same effect as

\Int'w , Int -> Int : f {
    (w +)
}
```

This way, a type tag can be considered simply a special kind of constructor that

- is implicitly defined when it appears inside the type signature of a function
- has special syntax for constructing (e.g.`0'w`)
- is automatically destructed and it's contents assigned to a variable inside the function


## Functions as incomplete data

Instead of looking at a function as transforming data, we can also consider it as a piece of data that has yet to be fully determined


## Implicit arguments and secondary scope

In a language like Agda there are implicit arguments which for most situation do not have to be accessed. But if we do, it's always a bit of a chase to get to them. And more generally, we might want to have values present but not cluttering up the main scope. Therefore, instead of having things completely out of scope that then have to be brought into scope, we could have a "secondary" scope which simply is differentiated by being somewhat less direct to access, e.g. requiring a prefix or something like that. This also goes with the "Meta-data" idea above. 


## Alternative approach to syntax of application

How about: Given `f:a->z` and `g:b->z` means that `h = f && g` is a valid expression and equivalent to  `h = \x y -> (x f) && (y g)`, resulting in `h:a->b->z`. I.e. Functions are implicitly open, and missing arguments are handed up to the parent expression. I.e. a function `a->b` does not require what is given as it's argument syntactically to be complete expression of type `a`, but can be any abstraction with a result type of `a`, with the abstraction being pulled over the application. I.e. "Functions as incomplete data".

E.g. If `f:p->q->z`, `g:a->p` and `h:b->q`, then `g h f` is the same as `\x y -> (x g) (y h) f`. It's not the same as `(g h) f`, but the same as `g (h f)`, with `h f:p->b->z` and therefore `g (h f):a->b->z`.

E.g. `{inc} map concat` is okay, because `concat:a List->a`, `map:a List->(a->b)->b List`, so `map concat:a List->(a->b)->b`, and so `{inc} map concat:a List->b`.

Is this in the end the same as I have proposed so far? Or does this differ in some way? Which is more intuitive?


## Reconsidering paths

As it stands, I am defining paths to be right branching like everything else in the language. There are two things that speak against this:
- It's unnatural given the rest of the language's grammar (-> Japanese genitive) <-- Not very relevant, but linguistically interesting
- It's inconvenient for autocomplete (e.g. `Module.[CTRL+SPACE]` allows us to look through everything defined in `Module`, whereas the same is not possible with `???.Module`)

=> Redefine paths as left branching <- 👍

想: Autocomplete in programming is the same as predicting ahead in language comprehension. We want for the context to most narrow what could come next.


## Patterns

The simplest kind of deconstruction pattern is to simply write out a constructor, with the components assigned to variables.

```
v | x y z C => (...x...y...z...)
```

For this to be executed, we have to simply check that `v` is a `C` construct and unravel it. If `v` is of type `C`, this can be directly ensured at compile time. However, if `v` is of a module type `M` containing `C`, information has to be retained at runtime with which constructor in `M` `v` has been constructed with.

For more complex hierarchical patterns, this process simply has to be repeated recursively. If e.g. `x` is not a variable, but `p q D`:

```
v | (p q D) y z C => (...p...q...y...z...)
```

Then this can be treated as/reformed into:

```
v | x y z C => (x | p q D => (...p...q...y...z...))
```


## Unordered Constructors

Just like with functions, we could allow constructors to be defined unordered. However, different to functions, where we only ever have to put in arguments to the function, constructors also can be destructed again. E.g.:

```
\Int , String : Index
1 "Hello" Index =p
p | x y Pair => {...x...y...}
```

However, the question is: Is `x == 1 && y == "Hello"` or `x == "Hello" && y == 1` in the destructor's body?

_Option 1:_ Constructors are simply ordered in patterns, the order being the order given in the constructors definition. While philosophically this isn't super clean, it would be much easier to implement. With the main reason for wanting unordered arguments is to allow for currying in application, this wouldn't really be a problem. If we don't care about a certain component of the data, we can simply give it a `_` and ignore it.

_Option 2:_ Instead of having `_` to ignore arguments, we do allow for constructors to appear in patterns only partially applied and unordered. The main problem with this is that we have to somehow figure out from the usage of the variables that we do capture which component of the data it is referring to. This might not always be possible. Specifically:

When we define an unordered function or constructor, what matters is that the it's arguments could under no circumstances take the same value. I.e. For a function of type `A , B -> C`, there can not exist a type `T` such that `T <: A` and `T <: B`, or in other words, if we have any possible type in the language, we have to be able to exclusively decide whether it fits `A`, it fits `B`, or it fits neither.

However, when we want to destruct an unordered constructor in an unordered fashion, what matters instead is that it's components could under no circumstance be used the same. I.e. For a constructor of type `A, B`, there can not exist a type `T` such that `A <: T` and `B <: T`, so that we can infer uniquely from the usage of the variable the component was assigned to which component of the constructor it was. Though it is unclear whether we can always infer the right type even if this is not the case (Does Hindley–Milner cover this?).


## Destructors & Data Flow

Returning to one of the primary motivations for this language, for the structure of a program's code to represent the flow of it's data with it's structure in an intuitive way, the syntax for the deconstructor should perhaps be reconsidered or extended. Also, the idea about allowing a function to have multiple results might also not be such a bad idea.

To note is that the deconstructor really has two different modes of operation: Checking whether the given data fits a certain pattern, and deconstructing the data in accordance to that pattern. However, the first mode is only needed for enum data, i.e. if the given data can be one of multiple constructors at runtime. Otherwise, the deconstructor really only serves to disassemble, which requires no branching.

A possible new deconstructor pattern could be: `|PATTERN|` with the pattern allowing for three kinds of holes: `_` ("ignore") to drop a certain part of the data, `%` ("pass"; Maybe `#`?) for the part to become a new floating value, and `[:snake_label:]` ("variable") to assign the part to a label.

How we cleanly do branching however, I'm still not sure. Something like this would be pretty:

```
| x _ _ A | x
|   p q B | p + q
|       C | 1729
```

However, how precedence and nesting is handled with this, I'm not sure. Maybe:

```
| x _ _ A | x     ;
|   p q B | p + q ;
|       C | 1729  ;
```


## Concerning Records, Variants, Symbols and Enums

At it's core, what we mean with a "data type" is to give a name to a given form of packaging data. This is what we call a "struct" or "record".\
However, it is often times convenient to extend this definition of a type to allow not just for a type name to refer to a single form of packaging data, but also for a type to stand for one of multiple forms. This is what we call a "variant" or "sum type" or "union".\
Also, we sometimes want a static symbol in our language without any further data attached to it. This is what we call a "symbol".\
Just like with variants, we can also allow for a type name to stand for one of multiple symbols. This is what we call an "enum".

Ultimately, these four concepts are captured by two features:

A _constructor_ is a named data form (->product). It can represent a record:

```
\Float ; Float : Vec2
```

Or it can represent a symbol (->unit):

```
\Tag
```

A _module_ (name TBD) is a named collection of types (->sum). It can represent a variant:

```
\\Tree {
    \Leaf
    \Tree ; Tree : Node
}
```

Or it can represent an enum:

```
\\Color {
    \Red
    \Orange
    \Yellow
    \Green
    \Blue
    \Purple
}
```

However, a module can not only contain constructors, but also functions, and importantly, also further modules ("submodules").

```
\\Collection {
    \\v List {
        \Empty
        \v ; v List : Ext
    }
    \\k v Map {
        \Empty
        \k ; v ; v Map : Ext
        
        \(k;v) List -> k v Map : fromList {...}
    }
    \\v Set {
        \Empty
        \v ; v Set : Ext

        \v List -> v Set : fromList {...}
    }
}
```

Both records and modules are entirely equivalent on the type level. However, when it comes to deconstructing them, their behaviour differs:

A constructor can simply be deconstructed into it's components:

```
\Vec2 -> Float : x {
    | x y Vec2 | x
}
```

A module however instead deconstructs into one of multiple types:

```
\Color -> Int : hex {
    | Red    | 0xE50000
    | Orange | 0xFF8D00
    | Yellow | 0xFFEE00
    | Green  | 0x028121
    | Blue   | 0x004CFF
    | Purple | 0x770088
}
```

```
\Collection -> Boolean : isEmpty {
    | List.Empty | True
    |  Map.Empty | True
    |  Set.Empty | True
    |          _ | False
}
```

问: How does this interact with polymorphism however? E.g.:

```
\Collection -> ??? List : toList {
    ???
}
``` 


## Reconsidering syntax for ordered/unordered listings

So far I have considered `,` for unordered listings and `;` for ordered listings. It might be preferable to reverse these. Or alternatively, use `.` instead of `;` for one of the two. The primary reason for this suggestion is that `;` simply looks quite ugly.

议: Ordered listings are written with `,`. Unordered listings are written with `.`.

因: `,` is not symmetric, suggesting that there is something essentially different between it's left and right side, i.e. the presence of horizontal order. `.` is symmetric, suggesting the left and right side are equivalent, i.e. the absence of horizontal order.

例:

```
\Float , Float : Vec2
\String . Vec2 : Location
```

问: However, what syntax would we then use for paths?

## Notationally concise and informationally verbose

Languages like Lisp are very notationally concise, meaning that the representation of the program's core structure takes up little space in it's syntax. However Lisp also is informationally concise, it doesn't tell you much about the meaning of a program without you having to parse and interpret the code.

A language like Haskell on the other hand is (in places) notationally verbose with it's annotations and keywords, however, Haskell also is informationally verbose. You can read what a Haskell program does without having to parse and interpret the code inside it's functions, you just have to look at the type annotation.

A goal with this language is be notationally concise, while also being informationally verbose.

## Probprog

Under the hypothesis that probabilistic programming is a good approach to producing useful programs, probprog should be an intrinsic part of this language. What this looks like however, I'm not entirely certain about. If we want to be able to support approaches like MCMC, this would have to be included in the implementation structure of the language itself (and maybe in the syntax in some way as well?). Under the additional hypothesis of "rejection sampling is all you need", we also would have to include some way for distributions to be hooked into from the outside for bias learning.

For any type `A`, instead of providing a explicit value, we should be able to write something like `?`, which will uniformly (by some choice of the meaning of "uniform") sample a possible value of type `A`.

Additionally, a mechanism should exist to bias this choice. For example, we should be able to bias the choice of a `Float` such that we get samples in accordance to a normal distribution. That is not to say that "the normal distribution" is a primitive of our language, rather a general mechanism should exist to bias a sample in any arbitrary way.

The most straightforward of course is to define a distribution as simply an ordinary function utilizing `?`, requiring no additional primitive beside the `?` operator.

## Probprog 2

Functions have two kinds of arguments, and these arguments have two possible states.

The first kind of argument is a "user argument", it is to be provided by the user that uses the function.\
The second kind of argument is an "oracle argument", it is to be provided by the probabilistic machinery of the computer, the oracle. 

Both user arguments and oracle arguments are either "undetermined" or "determined". A user argument is determined by the user specifying a value (either as a constant or as the result of further computation); An oracle argument is determined by the oracle according to some probabilistic process (uniform choice of possible values, a learned distribution, etc.).

With each determined argument, a function can be reduced, and a function with no undetermined arguments remaining is a "constant".\
A function with undetermined oracle arguments is "non-deterministic".

A function can principally both be curried by a user argument or an oracle argument.

## Uniquely inferable untagged union

When defining a sum type (aka. tagged unions), one sometimes has a situation as such:

```rust
//rust
struct A(...);
struct B(...);
enum C {
    A(A),
    B(B),
}
```

Semantically, an untagged union would be sufficient here. So, just like with untagged unordered function arguments, we could allow for untagged union types. Something like:

```
#C {
    \... : A
    \... : B
}
```

## Files are variables

A file containing a bunch of code is the same as if that code was written in some block which then is then assigned to that variable name, with the only difference being that to access the namespace of file "variables", we prefix their name. How do we handle paths?

One idea for this would also be that e.g. a block `x := {...}` can either be evaluated `y := x`, or "opened" `y := #x` (syntax tbd). In the second case, the assignments that are made in the block are exported, and either dumped into the current namespace, `#x`, or attached to a new sub-namespace `y := #x`.