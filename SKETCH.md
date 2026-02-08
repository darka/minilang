program     := { stmt } ;

stmt        := letStmt | assignStmt | ifStmt | whileStmt | forStmt
             | fnStmt | returnStmt | exprStmt ;

letStmt     := "let" IDENT "=" expr ;
assignStmt  := IDENT "=" expr | IDENT "[" expr "]" "=" expr ;

ifStmt      := "if" expr block [ "else" block ] ;
whileStmt   := "while" expr block ;
forStmt     := "for" IDENT "in" expr ".." expr block ;

fnStmt      := "fn" IDENT "(" [ params ] ")" block ;
params      := IDENT { "," IDENT } ;

returnStmt  := "return" [ expr ] ;
exprStmt    := expr ;

block       := "{" { stmt } "}" ;

expr        := logic ;
logic       := equality { ("and" | "or") equality } ;
equality    := compare { ("==" | "!=") compare } ;
compare     := term { ("<" | "<=" | ">" | ">=") term } ;
term        := factor { ("+" | "-") factor } ;
factor      := unary { ("*" | "/" | "%") unary } ;
unary       := ("not" | "-") unary | call ;
call        := primary { "(" [ args ] ")" | "[" expr "]" } ;
args        := expr { "," expr } ;

primary     := NUMBER | STRING | "true" | "false"
             | IDENT
             | "[" [ args ] "]"
             | "(" expr ")" ;
