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
       | Cond 
       | Collct 
       | Numeral 
       | Bespoke

Assign ::= "=" VarName
Tag ::= "'" lowercase
Import ::= "@" ModuleRefr
Func ::= "\" Type ":" FuncName Expr
Constr ::= "\" Types ":" ConstrName
Module ::= "#" [Types ":"] ModuleName Expr
Destr ::= "|" Pattern "=>" Expr
Collct ::= "[" (UnorderedCollct | OrderedCollct) "]"

Pattern ::= "(" Pattern ")" | {Pattern} TypeName | VarName

UnorderedCollct ::= Expr "," [UnorderedCollct | Expr]
OrderedCollct ::= Expr ";" [OrderedCollct | Expr]

Type ::= "(" Type ")" | (Types "->" Type) | "[" Types "]" | TypeVarName | ConstrName | {Type} ModuleRefr | Type Tag
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

Numeral ::= IntegerNumeral | FloatNumeral
IntegerNumeral ::= ["-"] <digit> {<digit>}
FloatNumeral ::= ["-"] <digit> {<digit>} "." {<digit>}

 Bespoke ::= Expr "+" Expr 
 | Expr "-" Expr 
 | Expr "*" Expr 
 | Expr "/" Expr 
 | Expr "&&" Expr 
 | Expr "||" Expr
 | "!" Expr
```