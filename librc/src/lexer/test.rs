use self::TokenKind::*;
use super::*;

macro_rules! test {
    ($name:ident: $text:tt == $($kind:expr, $beg:expr, $end:expr),*) => {
        #[test]
        fn $name() {
            assert_eq!(lex($text), Ok(vec![$(Token { kind: $kind, span: ($beg, $end) }),*]))
        }
    };
}

test!(empty: "" ==);
test!(numeric_literal_no_frac: "245" == NumLiteral(245.), 0, 3);
test!(numeric_literal_with_frac: "9234.3247" == NumLiteral(9234.3247), 0, 9);
test!(ident: "foo_bar_932" == Identifier, 0, 11);
test!(arith: "foo+ 2.4    *(3 - bar  )" ==
Identifier, 0, 3,
Plus, 3, 4,
NumLiteral(2.4), 5, 8,
Asterisk, 12, 13,
LParen, 13, 14,
NumLiteral(3.), 14, 15,
Minus, 16, 17,
Identifier, 18, 21,
RParen, 23, 24
);
test!(assign: "a = 3 * (8.24 + 2.62)" ==
Identifier, 0, 1,
EqualsSign, 2, 3,
NumLiteral(3.), 4, 5,
Asterisk, 6, 7,
LParen, 8, 9,
NumLiteral(8.24), 9, 13,
Plus, 14, 15,
NumLiteral(2.62), 16, 20,
RParen, 20, 21
);
test!(fncall: "sqrt(8 * 8)" ==
Identifier, 0, 4,
LParen, 4, 5,
NumLiteral(8.), 5, 6,
Asterisk, 7, 8,
NumLiteral(8.), 9, 10,
RParen, 10, 11
);
test!(fndef: "fn add(a1, a2) -> a1 + a2" ==
FnKeyword, 0, 2,
Identifier, 3, 6,
LParen, 6, 7,
Identifier, 7, 9,
Comma, 9, 10,
Identifier, 11, 13,
RParen, 13, 14,
RightArrow, 15, 17,
Identifier, 18, 20,
Plus, 21, 22,
Identifier, 23, 25
);
