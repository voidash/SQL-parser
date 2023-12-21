हामीले सपोर्ट गर्ने keyword लिस्ट : https://www.w3schools.com/sql/sql_ref_keywords.asp

### If Anurag types

```
select beautiful_girls from computer_engineering where SEM = 1;
```

Then the scanner will have 

```
[
        Symbol { name: "select", len: 6, token: Select, group: Keyword },
        Symbol { name: "beautiful_girls", len: 15, token: Identifier, group: Identifier },
        Symbol { name: "from", len: 4, token: From, group: Keyword },
        Symbol { name: "computer_engineering", len: 20, token: Identifier, group: Identifier },
        Symbol { name: "where", len: 5, token:j Where, group: Keyword },
        Symbol { name: "sem", len: 3, token: Identifier, group: Identifier },
        Symbol { name: "=", len: 1, token: EQ, group: Operator },
        Symbol { name: "1", len: 1, token: Identifier, group: Identifier },
        Symbol { name: ";", len: 1, token: Semicolon, group: Delimiter }
]
```

### Demo

```
CREATE TABLE ComputerEngineering (
    Batch varchar(255),
    number_of_students int,
);
```
![](./assets/example.png)

### Steps to Build

```
wasm-pack build --no-typescript --release --target web 

python -m http.server 80 
```

### Grammar

> Aru garna alchi lagyo

EBNF of Select
```
<select-statement> -> "SELECT" ["DISTINCT"] ["TOP" <u32>] ["(" <get-id-list> ")" | <get-id-list> ] "FROM" <get-id-list> [ <join-type> <identifier> "ON" <identifier> ] ;

<join-type> ->   "INNER JOIN" | "FULL OUTER JOIN" | "LEFT JOIN" | "RIGHT JOIN" 

// curly brackets is a Kleene-star which can repeat one or more times
<get-id-list> -> <identifier>  {"," <identifier>} 

<identifier> ->  '*' | <alpha> { <alphanumeric> | "_" | "-" }  

<alpha> ->  
    "a" | "b" | ... | "z" | "A" | "B" | ... | "Z"

<alphanumeric> -> 
    <alpha> | <digit>

<digit> ::=
    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
```