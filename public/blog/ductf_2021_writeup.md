---
title: DUCTF 2021 Writeup
description: Writeup for a maths-based crypto cipher in the Down Under CTF 2021.
image: /public/blog_media/hello_world/title.png
publish_date: 2021-09-21 20:30
tags:
- programming
---

> [!note]
> This is a writeup for the Substitution II `crypto` problem in the Down Under CTF 2021, which can be found [here](https://github.com/DownUnderCTF/Challenges_2021_Public/tree/main/crypto/substitution-cipher-ii).

# Substitution Cipher II (100 points)

## Explanation:

Here is the Sage code that was provided:

```sage
from string import ascii_lowercase, digits
CHARSET = "DUCTF{}_!?'" + ascii_lowercase + digits
n = len(CHARSET)

def encrypt(msg, f):
    ct = ''
    for c in msg:
        ct += CHARSET[f.substitute(CHARSET.index(c))]
    return ct

P.<x> = PolynomialRing(GF(n))
f = P.random_element(6)

FLAG = open('./flag.txt', 'r').read().strip()

enc = encrypt(FLAG, f)
print(enc)
```

`encrypt(msg, f)` is fairly straightforward, it maps each character in our message to another character in `CHARSET`. `CHARSET` itself consists of `DUCTF{}_!?`, plus all lowercase characters and digits, making up a total of 47 characters.

The next two lines, however, are very interesting. `P.<x> = PolynomialRing(GF(n))` defines a Galois field of size 47, otherwise known as `GF(47)`. From Wikipedia (emphasis mine):

> In mathematics, a Galois field is a field that contains a **finite number of elements**. A Galois field is a set on which the operations of **multiplication, addition, subtraction and division are defined** and satisfy certain basic rules. The most common examples of finite fields are given by the **integers mod p when p is a prime number**.

Because 47 is prime, `GF(47)` consists of the set of integers `{0, 1, 2... 46}`, where addition, subtraction, multiplication and division are all `mod 47`. Thus, `6 * 8` in the field of real numbers would be `48`, but in `GF(47)` it would be `48 mod 47 = 1`.

`f = P.random_element(6)` defines a random polynomial of degree 6 - in other words, `f = ax^6 + bx^5 + cx^4 + dx^3 + ex^2 + fx + g`, where `a` to `g` are all integers between 0 and 46 inclusive. Thus, it is our job to figure out those 7 variables in order to reverse-engineer the encryption.

To solve a system of 7 unknown variables, we need 7 independent equations. Luckily, we know what exactly 7 of the characters in our encrypted flag map to because of the standard flag format!

```
encrypted: Ujyw5dnFofaou0au3nx3Cn84
format:    DUCTF{.................}
```

Because the cipher is one-to-one, `D` maps to `U`, `U` maps to `j`, and so on. Based on the indices of these characters in `CHARSET`, we know that `f(0) = 1`, `f(1) = 20`... `f(6) = 41`. We now have our 7 simultaneous equations:

```
                                 g =  1 (1)
  a +   b +   c +  d +  e +  f + g = 20 (2)
64a + 32b + 16c + 8d + 4e + 2f + g = 35 (3)
...
```

We can create a matrix representation of these simultaneous equations, and now we need to solve for `x`:

```
[  0  0  0 0 0 0 1 ]     [  1 ]
[  1  1  1 1 1 1 1 ] x = [ 20 ]
[ 64 32 16 8 4 2 1 ]     [ 35 ]
...                      ...
```

Plugging the matrix and column vector into Maple and solving for `x`, `x` is equal to the column vector `<37/80, -633/80, 2437/48, -7229/48, 5963/30, -4349/60, 1>`, which is equivalent to `<41, 15, 40, 9, 28, 27, 1>` mod 47.

Thus, our random polynomial, `f`, is `41x^6 + 15x^5 + 40x^4 + 9x^3 + 28x^2 + 27x + 1`.

## Final solution:

```python
from string import ascii_lowercase, digits

CHARSET = "DUCTF{}_!?'" + ascii_lowercase + digits
# Encrypted flag
FLAG = "Ujyw5dnFofaou0au3nx3Cn84"

# Given a number between 0 and 47, applies the random polynomial in Sage
# to that number mod 47
def polynomial_g47(x):
    poly = 41 * x ** 6 + 15 * x ** 5 + 40 * x ** 4 + 9 * x ** 3 + 28 * x ** 2 + 27 * x + 1
    return poly % 47

# Maps each possible input character (D, U, C, T, F and so on) to
# a list of characters it might be translated from, so mapping[1] = "D"
mapping = [""] * 47
for i in range(47):
    mapping[polynomial_g47(i)] += CHARSET[i]

print(mapping)

# Prints out all possible combinations of characters that
# form the encrypted flag
result = ""
for char in FLAG:
    index = CHARSET.index(char)
    current = mapping[index]

    if len(current) > 1:
        result += f"[{current}]"
    else:
        result += current

print(result)
```

Running this program returns the output `D[U!]C[Tt]F{go0d_0l'_l4gr4[fnp]g[38]}`.
Choosing the right characters in each square bracket, we get the flag
`DUCTF{go0d_0l'_l4gr4ng3}`.
