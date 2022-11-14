%%

[0-9]+ "INT"
[a-zA-Z_][a-zA-Z0-9_]*  "IDENTIFIER"

let "LET"
\( "LBRACK"
\) "RBRACK"

;   "SEMICOLON"
,   "COMMA"
=   "EQ"
-   "MINUS"
\+  "PLUS"
==  "EQEQ"
<=  "LTEQ"
>=  "GTEQ"
<   "LT"
>   "GT"

[ \t\n\r]+              ;
(//.*?$|/[*].*?[*]/)    ;