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

## digraph table

```console
left: first letter, top: second letter
  a   b   c   d   e   f   g   h   i   j   k   l   m   n   o   p   q   r   s   t   u   v   w   x   y   z
a 1   26  39  40  1   15  36  5   42  3   28  68  59  72  2   39  1   112 52  63  13  22  28  14  30  13
b 43  2   0   0   42  0   0   0   19  0   0   15  0   0   44  0   0   16  35  1   32  0   0   0   7   0
c 49  0   0   0   24  0   0   33  5   0   43  14  1   1   59  0   0   16  9   6   22  0   0   0   4   1
d 27  0   0   3   49  0   2   2   27  0   0   2   0   0   43  0   0   11  36  0   27  0   0   0   11  1
e 77  5   13  47  72  13  9   0   5   0   10  45  23  48  6   18  0   53  61  30  3   6   31  12  11  3
f 29  0   0   0   24  20  0   0   21  0   0   26  0   0   26  0   0   9   2   15  14  0   0   0   2   0
g 31  0   0   0   29  0   2   5   16  0   0   16  0   5   29  0   0   18  38  0   15  0   0   0   8   0
h 51  0   0   0   41  0   0   0   27  0   0   1   0   1   43  0   0   2   0   1   29  0   0   0   5   0
i 9   15  29  33  22  14  16  0   0   0   11  65  27  83  9   25  0   31  35  51  0   10  1   1   1   3
j 16  0   0   0   9   0   0   0   9   1   0   0   0   0   14  0   0   0   0   0   12  0   0   0   0   0
k 11  0   0   0   42  0   0   4   26  0   0   0   0   6   9   0   0   1   6   0   3   0   0   0   4   0
l 76  1   1   17  76  6   1   0   44  0   11  52  8   2   76  5   0   0   14  26  35  0   0   0   14  0
m 51  9   0   0   47  0   0   0   35  0   0   0   3   3   40  25  0   0   28  0   21  0   0   0   4   0
n 29  0   4   28  54  1   31  0   12  0   30  1   0   4   31  0   0   0   35  32  13  1   0   3   8   0
o 34  26  20  32  20  11  27  3   16  3   19  58  29  62  66  42  0   57  38  48  29  11  39  4   8   6
p 47  0   0   1   57  0   0   2   33  0   0   15  0   0   45  2   0   16  37  4   30  0   0   0   4   0
q 0   0   0   0   0   0   0   0   0   0   0   0   0   0   0   0   0   0   0   0   7   0   0   0   0   0
r 76  9   5   18  81  5   5   1   53  0   18  8   16  18  65  6   0   5   18  21  34  1   0   0   17  1
s 34  0   11  0   51  0   0   50  22  0   22  23  4   10  36  23  0   0   25  52  19  0   10  0   7   0
t 48  0   2   0   57  0   0   33  27  0   0   0   0   0   52  0   0   12  48  8   24  0   5   0   10  3
u 7   21  13  21  19  13  21  0   6   2   5   31  32  43  3   9   0   52  44  33  0   2   0   3   2   3
v 22  0   0   0   45  0   0   0   19  0   0   0   0   0   6   0   0   0   1   0   1   0   0   0   6   0
w 43  0   0   1   24  0   0   3   19  0   2   9   0   12  10  1   0   2   21  1   1   0   0   0   1   0
x 2   0   0   0   4   0   0   0   5   0   0   1   0   0   2   1   0   0   0   2   0   0   0   0   7   0
y 7   0   0   0   9   0   0   0   0   0   2   2   2   2   4   4   0   8   24  2   0   0   0   2   0   0
z 1   0   0   0   13  0   0   0   1   0   0   0   0   0   5   0   0   0   0   0   0   0   0   0   6   5
```