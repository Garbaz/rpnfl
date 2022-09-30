# RPNFL Parser

## Grammar

```ebnf
Expr ::= "(" Expr {Expr} ")" 
       | "{" Expr {Expr} "}"
       | Refr 
       | Assign 
       | Tag 
       | Func 
       | Constr 
       | Module 
       | Destr 
       | Collct
       | Import 
       | Number 
       | Bespoke

Assign ::= "=" VarName
Tag    ::= "'" lowercase
Func   ::= "\" Types "->" Type ":" FuncName Expr
Constr ::= "\" Types ":" ConstrName
Module ::= "#" [Types ":"] ModuleName Expr
Destr  ::= "|" Pattern "=>" Expr
Collct ::= "[" (UnorderedCollct | OrderedCollct) "]"
Import ::= "@" ModuleRefr

Pattern ::= "(" Pattern ")" | {Pattern} TypeName | VarName | "_"

UnorderedCollct ::= Expr "," [UnorderedCollct | Expr]
OrderedCollct   ::= Expr ";" [OrderedCollct | Expr]

Type ::= "(" Type ")"
       | Type Tag
       | Types "->" Type
       | "[" Type "]"
       | TypeVarName
       | ConstrName
       | {Type} ModuleRefr
Types ::= UnorderedTypes | OrderedTypes
UnorderedTypes ::= Type | Type "," UnorderedTypes
OrderedTypes   ::= Type | Type ";" OrderedTypes

Refr ::= FuncRefr | ConstrRefr
FuncRefr   ::= Path FuncName
ConstrRefr ::= Path ConstrName
TypeRefr   ::= Path TypeName 
ModuleRefr ::= Path ModuleName
Path ::= {ModuleName "."}

VarName ::= SnakeLabel
FuncName ::= SnakeLabel
ConstrName ::= CamelLabel
ModuleName ::= CamelLabel
TypeVarName ::= SnakeLabel

SnakeLabel ::= <lowercase> {<lowercase> | "_" | <numeral>}
CamelLabel ::= <uppercase> {<uppercase> | <lowercase> | <numeral>}

Number ::= Integer | Float
Integer ::= ["-"] <digit> {<digit>}
Float   ::= ["-"] <digit> {<digit>} "." {<digit>}

Bespoke ::= Expr "+" Expr 
          | Expr "-" Expr 
          | Expr "*" Expr 
          | Expr "/" Expr 
          | Expr "&&" Expr 
          | Expr "||" Expr
          | "!" Expr
```

## TODO/Notes

- Parsing modules with arguments makes Lalrpop cry. What's the problem there? It doesn't seem to be the fact that the body is just tacked on without any separator?
- Update Grammar here to reflect what I have changed/actually implemented in Lalrpop
