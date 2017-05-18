# Examples

```
a = 3 * (8.24 + 2.62)
sqrt(8 * 8)
fn add(a1, a2) -> a1 + a2
```

# Grammar

## Lexical elements

### Numeric literal
A sequence of one or more characters in the range of [0-9].
Can optionally have a "fractional" part, which begins with a '.' character.
Oh, and it must also be able to be parsed by Rust's `f64::from_str`. *shrugs*

### Identifier
Consists of one or more unicode scalar values that have the "alphabetic" property.
Oh, and they can also contain _.
Oh, and they can also contain [0-9], but not begin with them.
Also not a Keyword.

### Keyword
Same rules as identifier, except it's a predefined name that has a special meaning.
The keywords are: `fn`.

### Operator
Either of `+`, `-`, `/`, `*`, `^`, `(`, `)`, `,`, `=`, `->`.
