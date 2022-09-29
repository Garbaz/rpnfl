# Compiler

## Parsing

- https://www.youtube.com/watch?v=vzfy4EKwG_Y
- https://www.youtube.com/watch?v=Jes3bD6P0To

Every expression is a series of expressions (e.g. `([1,2,3] {add1} map)`). All expressions are evaluated and the last expression is a function which takes all other expressions as arguments.


## Compiling

- https://www.youtube.com/watch?v=vzfy4EKwG_Y
- https://en.wikipedia.org/wiki/Semantics_(computer_science)#Approaches
- https://media.handmade-seattle.com/roc-lang/
- https://github.com/roc-lang/roc
- https://www.youtube.com/watch?v=-IsbsfV3jmw
- https://en.wikipedia.org/wiki/C--

Maybe compile to C or, perhaps even simpler, to Haskell or some Lisp (e.g. Scheme) to avoid hassle? Especially to begin with.


## Type system

- https://en.wikipedia.org/wiki/Substructural_type_system#Linear_type_systems
- https://pdfs.semanticscholar.org/01b5/1c8ebe7f7fc21878c9500e03ab77324172dd.pdf


## Ideas

### Configuration

```sh
compiler-name configure
```

Instead of requiring the user to specify configurations to the compiler as command line arguments or a configuration file, we provide an interactive configuration tool. This will generate/update a suitable configuration file for the current project or globally, depending on from where we start the configuration (or `--global`).

_As a more general principle:_ Provide an isomorphic but not equal interface for things to the user and the computer, instead of trying to find a format that suits both the user and the computer. If needed, the user can still edit configuration manually, but they should not have to.

This could build upon the bigger idea of having UI/file formats that are derived from the structure of data, instead of being constructed manually. We should not have to manually maintain a user interface and a config parser for the data structure that represents our compiler configuration. Instead, they should be derivable by some general principles.


### Type reduction

While code internally and for interfacing with other code has to retain type information, when we compile the code to a final binary, this information no longer is necessary. Therefore, in that case, types can be reduced for optimization without any loss of function. E.g. `Unit Maybe` can be reduced to an enumeration of two values, equivalent to `Bool`. Also, indirections can incorporated for all cases where the size of the indirect value is known.
