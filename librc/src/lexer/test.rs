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
