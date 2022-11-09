%%
;   SEMICOLON
,   COMMA
=   EQ
-   MINUS
\+  PLUS
==  EQEQ
<=  LTEQ
>=  GTEQ
<   LT
>   GT
let     LET

[0-9]+                  INT
[ \t\n\r]+              ;
(//.*?$|/[*].*?[*]/)    ;
[a-zA-Z_][a-zA-Z0-9_]*  IDENTIFIER