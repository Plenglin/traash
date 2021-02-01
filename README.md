## The Language

```
SEMICOLON: ";"

```

```bnf
<text-array> ::= <text> <text-array> | ""
<single> ::= <text-array>
<sequential> ::= <command> ; <command>
<fork> ::= <command> & <command>
 
<command> ::= <single> | <sequential> | <fork>
```