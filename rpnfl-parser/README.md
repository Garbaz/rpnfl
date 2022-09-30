# RPNFL Parser

## Grammar

```ebnf
Expr ::= "(" Expr {Expr} ")" 
       | "{" Expr {Expr} "}"
       | FuncRefr 
       | Assign 
       | Tag 
       | Import 
       | Func 
       | Constr 
       | Module 
       | Destr 
       | Collct
       | Number 
       | Bespoke

Assign ::= "=" VarName
Tag ::= "'" lowercase
Import ::= "@" ModuleRefr
Func ::= "\" Types "->" Type ":" FuncName Expr
Constr ::= "\" Types ":" ConstrName
Module ::= "#" [Types ":"] ModuleName Expr
Destr ::= "|" Pattern "=>" Expr
Collct ::= "[" (UnorderedCollct | OrderedCollct) "]"

Pattern ::= "(" Pattern ")" | {Pattern} TypeName | VarName

UnorderedCollct ::= Expr "," [UnorderedCollct | Expr]
OrderedCollct ::= Expr ";" [OrderedCollct | Expr]

Type ::= "(" Type ")"
       | Type Tag
       | (Types "->" Type)
       | "[" Type "]"
       | TypeVarName
       | ConstrName
       | {Type} ModuleRefr
Types ::= UnorderedTypes | OrderedTypes
UnorderedTypes ::= Type | (Type "," UnorderedTypes)
OrderedTypes ::= Type | (Type ";" OrderedTypes)

FuncRefr ::= FuncName Path
TypeRefr ::= TypeName Path 
ModuleRefr ::= ModuleName Path
Path ::= {"." ModuleName}

VarName ::= Label
FuncName ::= Label
ConstrName ::= CapLabel
ModuleName ::= CapLabel
TypeVarName ::= Label

Label ::= <lowercase> {<lowercase> | "_" | <numeral>}
CapLabel ::= <uppercase> {<uppercase> | <lowercase> | "_" | <numeral>}

Number ::= Integer | Float
Integer ::= ["-"] <digit> {<digit>}
Float ::= ["-"] <digit> {<digit>} "." {<digit>}

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
