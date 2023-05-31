# Finite Field Definition

A finite field is defined as a finite set of numbers and two operations `+` (addition) and `⋅` (multiplication) that satisfy the following:

1. If `a` and `b` are in the set, `a + b` and `a ⋅ b` are in the set. We call this property _closed_.
2. `0` exists and has the property `a + 0 = a`. We call this the _additive identity_.
3. `1` exists and has the property `a ⋅ 1 = a`. We call this the _multiplicative identity_.
4. If a is in the set, `–a` is in the set, which is defined as the value that makes `a + (–a) = 0`. This is what we call the _additive inverse_.
5. If a is in the set and is not `0`, `a^–1` is in the set, which is defined as the value that makes `a ⋅ a^–1 = 1`. This is what we call the _multiplicative inverse_.

- because the set is finite, we can designate a number `p`, which is how big the set is. This is what we call the _order_ of the set.
    - fields must have an order that is a power of a prime
    - the finite fields whose order is prime are the ones we’re interested in
- we have to define addition and multiplication in a way that ensures the results stay in the set.
- and 1 are in the set.
- if a is in the set, `–a` is in the set
- if a is in the set, `a^–1` is in the set

We want to represent each finite field element.

We can define addition on the finite set using modulo arithmetic.

`n^(p–1)` is always 1 for every p that is prime and every n > 0. 

Fermat’s little theorem. `n^(p–1)%p = 1` where `p` is prime.