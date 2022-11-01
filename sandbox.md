
```
[3, 2, 5, 7, 4]
reverse
{inc} map
```

```
\x y z { z - (x * y) }
```

```
\\func x:Int y:Int z:Int -> Int
{
    z - (x * y)
}
```

```
{ Int Int Int -> Int

} ~> func

```

```
\\func Int Int Int -> Int
{

}
```

<hr>

The "purest" form of function definition would be something like this:

```
{~> x ~> y ~> z
    x + (y * z)
} ~> func
```

I.e. each `~>` expects an input and binds it to a label, and we assign the block to label `func`.

Advantage: Does not use any extra syntax

Disadvantage: The function name is at the end, which isn't ideal.

Idea: Introduce something like a topic marker, i.e. we are allowed to briefly invert the syntax in some manner.

Idea 2: Introduce `<~`, so that we can also assign backwards.

```
func <~
{
    ~> x ~> y ~> z
    x + (y * z)
}
```

Do we want this kind of inversion to be possible?

_Iteration 1:_

```
\x y z
{
    x + (y * z)
} ~> func
```

We introduce a `\` operator that takes a series of labels that will become inputs to the function. We still have to put the function name at the end though.

_Iteration 2:_

```
\func x y z
{
    x + (y * z)
}
```

We instead make the `\` operator assign the resulting function to the first given label in the outer scope. The label can be left blank `\_` to create an anonymous function.

<hr>

Type annotation could maybe look something like this:

```
\func x y z :: Int Int Int -> Int
{
    x + (y * z)
}
```

or 

```
\func x:Int y:Int z:Int -> Int
{
    x + (y * z)
}
```

<hr>

What about functions that have multiple results?

```
{
    "Hello World"
    " " split
    
    // Two `Str` are here in some form
}
```



<hr>

Data type declaration might look something like this, i.e. analogous to functions.

```
$type a -> n:Int m:Int x:a
{
    ???
}
```

Or maybe enum types instead?

How do we handle name spaces?

<hr>

```
\Plant {
    \Stem, Flower, Leaf : Blume {
        \Float'length, Float'width : Stem
        \Center, Petals : Flower {
            \Color : Center
            \Shape, Color : Petal
        }
        \Shape, Color : Leaf {
            ...
        }
    }

    \Trunk, Crown : Tree
}
```