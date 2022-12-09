%%
[0-9]+ "INT"

let "LET"
print "PRINT"

\( "LBRACK"
\) "RBRACK"
\{ "LBRACE"
\} "RBRACE"
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
[ \t\n\r]+ while [ \t\n\r]+ "WHILE"

[ \t\n\r]+              ;
(//.*?$|/[*].*?[*]/)    ;