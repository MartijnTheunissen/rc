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
