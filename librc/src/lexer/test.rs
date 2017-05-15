use self::TokenKind::*;
use super::*;

#[test]
fn empty() {
    assert_eq!(lex(""), Ok(vec![]));
}

#[test]
fn numeric_literal_no_frac() {
    assert_eq!(
        lex("245"),
        Ok(
            vec![
                Token {
                    kind: NumLiteral(245.),
                    span: (0, 3),
                },
            ]
        )
    );
}

#[test]
fn numeric_literal_with_frac() {
    assert_eq!(
        lex("9234.3247"),
        Ok(
            vec![
                Token {
                    kind: NumLiteral(9234.3247),
                    span: (0, 9),
                },
            ]
        )
    )
}
