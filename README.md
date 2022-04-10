# cminus-rs
cminus-lang , which is a subset of c implemented by rust
## preview
[online preview](https://iwanabethatguy.github.io/cminus-rs/)
## SPEC

```ebnf
Program = { Declaration } ;
Declaration = VarDeclaration 
            | FunctionDeclaration;
VarDeclaration = TypeSpecifier Identifier [ '=' Expression ] ';'
              |  TypeSpecifier Identifier '['Number']' ['=' Expression {',' Expression} ]';';
TypeSpecifier = 'int' | 'void' | 'bool';
FunctionDeclaration = TypeSpecifier Identifier '(' params ')' CompoundStatement;
Params = Param {',' Param };
Param = TypeSpecifier Identifier |
      | TypeSpecifier Identifier '[' ']';
CompoundStatement = '{' LocalDeclarations  StatementList '}'
LocalDeclarations = { VarDeclaration };
StatementList = { Statement };
Statement = ExpressionStatement 
          | CompoundStatement 
          | SelectionStatement
          | IterationStatement
          | ReturnStatement;
ExpressionStatement = { Expression ';' };
SelectionStatement = 'if' '(' Expression ')' Statement
                   | 'if' '(' Expression ')' 'else' Statement;
iterationStatement = 'while' '(' Expression ')' Statement;
ReturnStatement = 'return' [ Expression ] ';';
Expression = Var '=' Expression
           | OrExpression;
Var = Identifier | Identifier '[' Expression ']';
SimpleExpression = AdditiveExpression RelOp AdditiveExpression | AdditiveExpression;
OrExpression = AndExpression | OrExpression '||' AndExpression;
AndExpression = SimpleExpression | AndExpression  '&&' SimpleExpression;
RelOp = '<=' | '<' | '>=' | '>' '==' | '!=';
AdditiveExpression = AdditiveExpression AddOp Term | [UnaryOp] Term;
AddOp = '+' | '-';
Term = Term MulOp Term | Factor;
MulOp = '*' | '/';
Factor = '(' Expression')' | Var | Call | Number | Bool;
Call = Identifier '(' [Expression {',' Expression}] ')';
Bool = 'true' | 'false';
UnaryOp = '-' | '+';

Identifier = 'a'-'z' {'a'-'z'Number};
Number = Digit { Digit };
Digit = 0 | 1 | 2 | 3 | 4 | 5 | 6| 7 | 8 | 9;
```