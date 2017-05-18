use self::TokenKind::*;
use super::*;

macro_rules! test {
    ($name:ident, $text:expr, $($token:expr),*) => {
        #[test]
        fn $name() {
            assert_eq!(lex($text), Ok(vec![$($token),*]))
        }
    };
}

test!(empty, "",);
test!(
    numeric_literal_no_frac,
    "245",
    Token {
        kind: NumLiteral(245.),
        span: (0, 3),
    }
);
test!(
    numeric_literal_with_frac,
    "9234.3247",
    Token {
        kind: NumLiteral(9234.3247),
        span: (0, 9),
    }
);
test!(
    ident,
    "foo_bar_932",
    Token {
        kind: Identifier,
        span: (0, 11),
    }
);
test!(
    arith,
    "foo+ 2.4    *(3 - bar  )",
    Token {
        kind: Identifier,
        span: (0, 3),
    },
    Token {
        kind: Plus,
        span: (3, 4),
    },
    Token {
        kind: NumLiteral(2.4),
        span: (5, 8),
    },
    Token {
        kind: Asterisk,
        span: (12, 13),
    },
    Token {
        kind: LParen,
        span: (13, 14),
    },
    Token {
        kind: NumLiteral(3.),
        span: (14, 15),
    },
    Token {
        kind: Minus,
        span: (16, 17),
    },
    Token {
        kind: Identifier,
        span: (18, 21),
    },
    Token {
        kind: Rparen,
        span: (23, 24),
    }
);
