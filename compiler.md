# Compiler

## 目:

### Parsing

- https://www.youtube.com/watch?v=vzfy4EKwG_Y
- https://www.youtube.com/watch?v=Jes3bD6P0To
- 

### Compiling

- https://www.youtube.com/watch?v=vzfy4EKwG_Y
- https://en.wikipedia.org/wiki/Semantics_(computer_science)#Approaches

### Type system

- https://en.wikipedia.org/wiki/Substructural_type_system#Linear_type_systems
- https://pdfs.semanticscholar.org/01b5/1c8ebe7f7fc21878c9500e03ab77324172dd.pdf
- 

## Ideas

### Configuration

```sh
compiler-name configure
```

Instead of requiring the user to specify configurations to the compiler as command line arguments or a configuration file, we provide an interactive configuration tool. This will generate/update a suitable configuration file for the current project or globally, depending on from where we start the configuration (or `--global`).

_As a more general principle:_ Provide an isomorphic but not equal interface for things for the user and the computer, instead of trying to find a format that suits both the user and the computer. If needed, the user can still edit configuration manually, but they should not have to.

This could build upon the bigger idea of having UI/file formats that are derived from the structure of data, instead of being constructed manually. We should not have to manually maintain a user interface and a config parser for the data structure that represents our compiler configuration. Instead, they should be derivable by some general principles.