%%
[0-9]+ "INT"
while "WHILE"
let "LET"
print "PRINT"

\( "LBRACK"
\) "RBRACK"
\{ "LBRACE"
\} "RBRACE"
"(?:\\"|[^"\n])*" "STRING"

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