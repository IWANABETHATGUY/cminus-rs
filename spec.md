# c-minus (c-) language specification
- program -> declaration-list  
- declaration-list -> declaration-list | declaration  
- declaration -> var-declaration | function-declaration  
- var-declaration -> type-specifier **ID** initializer; | type-specifier **ID** [**NUM**] array-initializer;  
- initializer -> = expression | ε
- array-initializer -> = {array-init-list} | ε
- array-init-list -> array-init-list, expression | expression
- type-specifier -> **int** | **VOID** | **bool** 
- fun-declaration -> type-specifier **ID** (params) compound-statement 
- params -> param-list | **ε**
- param-list -> param-list, param | param
- param -> type-specifier **ID** | type-specifier **ID** [ ]
- compound-statement -> { local-declarations statement-list }
- local-declarations -> local-declarations var-declaration | ε
- statement-list -> statement-list statement | ε
- statement -> expression-statement | compound-statement | selection-statement
-iteration-statement | return-statement
- expression-statement -> expression ; | ;
- selection-statement -> **if** (expression) statement | **if** (expression) **else** statement
- iteration-statement -> **while** (expression) statement
- return-statement -> **return** ; | **return** expression;
- expression -> var **=** expression | or-expression 
- var -> **ID** | **ID** [expression]
- simple-expression -> additive-expression relop additive-expression | aditive-expression
- or-expression -> and-expression | or-expression || and-expression
- and-expression -> simple-expression | and-expression && simple-expression
- relop -> **<=** | **<** | **>=** | **>**| **==** | **!=** 
- additive-expression -> additive-expression addop term | (unaryOp)? term
- addop -> **+** | **-**
- term -> term mulop term | factor
- mulop  -> * | **/**
- factor -> (expression) | var | call | NUM | **BoolLiteral**
- call -> **ID** (args)
- args -> arg-list | ε
- arg-list -> arg-list expression | expression
- BoolLiteral -> true | false
- unaryOp -> '-' | '+'




