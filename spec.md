# c-minus (c-) language specification
1. program -> declaration-list  
2. declaration-list -> declaration-list | declaration  
3. declaration -> var-declaration | function-declaration  
4. var-declaration -> type-specifier **ID**; | type-specifier **ID** [**NUM**];  
5. type-specifier -> **int** | **VOID**  
6. fun-declaration -> type-specifier **ID** (params) compound-statement 
7. params -> param-list | **ε**
8. param-list -> param-list, param | param
9. param **ID** | type-specifier **ID** [ ]
10. compound-statement -> { local-declarations statement-list }
11. local-declarations -> local-declarations var-declaration | ε
12. statement-list -> statement-list statement | ε
13. statement -> expression-statement | compound-statement | selection-statement
| iteration-statement | return-statement
14. expression-statement -> expression ; | ;
15. selection-statement -> **if** (expression) statement | **if** (expression) **else** statement
16. iteration-statement -> **while** (expression) statement
17. return-statement -> **return** ; | **return** expression;
18. expression -> var **=** expression | simple-expression
19. var -> **ID** | **ID** [expression]
20. simple-expression -> additive-expression relop additive-expression | additive-expression
21. relop -> **<=** | **<** | **>=** | **>**| **==** | **!=** 
22. additive-expression -> additive-expression addop term | term
23. addop -> **+** | **-**
24. term -> term mulop term | factor
25. mulop  -> * | **/**
26. factor -> (expression) | var | call | NUM
27. call -> **ID** (args)
28. args -> arg-list | ε
29. arg-list -> arg-list expression | expression