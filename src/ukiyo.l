%%
[0-9]+ "INT"

let "LET"
\( "LBRACK"
\) "RBRACK"
'(?:\\\\|\\'|[^'])*' "STRING"
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
[a-zA-Z_][a-zA-Z0-9_]*  "IDENTIFIER"

[ \t\n\r]+              ;
(//.*?$|/[*].*?[*]/)    ;