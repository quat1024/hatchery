# i-am-very-good-at-mastermind

Utilities for the "mm" game on vazkii's discord, which is like Mastermind but instead of guessing colors you guess four-letter words

Warning this code is really scuffed.

## frequency table

Frequency of the letter being in each position (letter 1, 2, 3, or 4), and "anywhere" is the frequency of it appearing in the word at all.

```console
letter  1       2       3       4       anywhere
a       100     493     231     97      921
b       187     11      58      45      301
c       166     26      95      16      303
d       149     15      77      149     390
e       56      326     219     367     968
f       136     3       49      46      234
g       130     12      70      68      280
h       128     65      8       68      269
i       31      317     143     29      520
j       53      1       7       1       62
k       47      15      50      136     248
l       144     125     196     131     596
m       142     21      103     80      346
n       67      44      206     122     439
o       62      479     167     79      787
p       189     29      75      96      389
q       6       1       0       0       7
r       136     127     218     76      557
s       245     9       145     453     852
t       170     23      136     237     566
u       17      277     91      16      401
v       50      11      39      3       103
w       78      26      46      42      192
x       0       12      12      15      39
y       6       30      32      115     183
z       4       1       26      12      43

letter 1 frequency order:       spbtcdlmrfghawnoejvkiuyqzx
letter 2 frequency order:       aoeiurlhnypwctmkdxgvbsfzqj
letter 3 frequency order:       aernlositmcudpgbkfwvyzxhjq
letter 4 frequency order:       setdklnyapmorhgfbwiucxzvjq
all letters frequency order:    easoltrinudpmcbghkfwyvjzxq
```

## repeated letters

* "doubles": the letter appears exactly
* "adjdubs": the letter appears twice in a row
* "triples": the letter appears three times

```console
letter  doubles adjdubs triples
a       39      1       0
b       14      2       1       bibb
c       5       0       0
d       14      3       0
e       99      72      1       epee
f       22      20      0
g       10      2       0
h       5       0       0
i       6       0       0
j       2       1       0
k       3       0       0
l       52      52      2       loll, lull
m       10      3       0
n       15      4       0
o       92      66      0
p       22      2       0
q       0       0       0
r       8       5       0
s       55      25      2       sass, suss
t       29      8       0
u       6       0       0
v       1       0       0
w       0       0       0
x       0       0       0
y       0       0       0
z       5       5       0
```