# Span

The lexer should have span info available for tokens.

For example:

```
foobarvar24 = kakkafaggotcake
              ^^^^^^^^^^^^^^^ UNDEFINED VARIABLE MOTHERFUCKER, DO YOU SPEAK IT?

a . 2
  ^ UNEXPECTED TOKEN, WHAT THE FUCK IS THIS SHIT SUPPOSED TO BE?
```

To achieve this, it must know where a token begins and ends, and store it.

A span is left-inclusive, right-exclusive.
e.g. `245` is `0..3`.
