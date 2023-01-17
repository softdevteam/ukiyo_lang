%%
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
func "FUNC"
while "WHILE"
let "LET"
print "PRINT"
if "IF"
else "ELSE"
return "RETURN"

[0-9]+ "INT"
[a-zA-Z_][a-zA-Z0-9_]*  "IDENTIFIER"
[ \t\n\r]+              ;
(//.*?$|/[*].*?[*]/)    ;