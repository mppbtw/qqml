use crate::*;

#[test]
fn test_unterminated_literal() {
    let input = "
        hints 2;
        ask multichoice (1) 'where is the sun {
            * 'the moon' (1);
            * 'the earth' (0);
        };
        ";
    let output = parse(input);
    dbg!(&output);
    assert!(output.is_err());
    assert!(matches!(output.unwrap_err().errors.get(0).unwrap(), Error::UnterminatedLiteral(_)));
}

#[test]
fn test_max_marks_impossible() {
    let input = "
        ask multichoice (10) 'how many of your bases are belong to us' {
            * 'all of them' (3);
            * 'some of them';
        };
        ";
    let output = parse(input);
    assert_eq!(output.unwrap().warnings.len(), 1);
}
