use crate::*;

#[test]
fn test_no_multichoice_answers() {
    let input = "
        hints 4;
        ask multichoice (0) 'fishsticks is good food?'{};
        ";
    let output = parse(input);
    dbg!(&output);

    // There is also an error for the max marks being zero
    assert_eq!(output.unwrap_err().errors.len(), 2);
}

#[test]
fn test_max_marks_is_zero() {
    let input = "
        hints 4;
        ask multichoice (0) 'fishsticks is good food?'{
            * 'yasss queeen slaaaaaaaayyyyyyyy' (1);
            * '*arms crossed and stern look* yes.';
        };
        ";
    let output = parse(input);
    dbg!(&output);

    // There is also an error for the max marks being zero
    assert_eq!(output.unwrap_err().errors.len(), 1);
}

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
    assert!(matches!(
        output.unwrap_err().errors.get(0).unwrap(),
        Error::UnterminatedLiteral(_)
    ));
}
#[test]
fn test_unterminated_literal_double_quotes() {
    let input = "
        hints 2;
        ask multichoice (1) \"where is the sun {
            * \"the moon\" (1);
            * \"the earth\" (0);
        };
        ";
    let output = parse(input);
    dbg!(&output);
    assert!(output.is_err());
    assert!(matches!(
        output.unwrap_err().errors.get(0).unwrap(),
        Error::UnterminatedLiteral(_)
    ));
}

#[test]
fn test_missing_bracket_for_multichoice_max_marks() {
    let input = "
        ask multichoice 1) 'how many of your bases are belong to us' {
            * 'all of them' (1);
            * 'som of them' (1);
        };
        ";
    let output = parse(input);
    let errors = output.unwrap_err().errors;
    dbg!(&errors);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_random_stuff_in_the_code() {
    let input = "
        ask multichoice (1) 'how many of your bases are belong to us' {
            * 'all of them'$ (1);
            * 'som of them' (1);
        };
        ";
    let output = parse(input);
    let errors = output.unwrap_err().errors;
}

#[test]
fn test_missing_comma_in_multichoice_hints() {
    let input = "

        hints 3;

        ask multichoice (1) 'how many of your bases are belong to us' {
            * 'all of them' (1);
            * 'som of them' (1);
            * 'unknown quantity of them';
        } hints 'hint one' 'hint two';
        ";

    let report = parse(input).unwrap_err();
    dbg!(&report.errors);
    assert_eq!(report.errors.len(), 1);
}

#[test]
fn test_impossible_max_marks() {
    let input = "

        hints 3;

        ask multichoice (10) 'how many of your bases are belong to us' {
            * 'all of them' (1);
            * 'som of them' (1);
            * 'unknown quantity of them';
        } hints 'hint one', 'hint two';
        ";
    let report = parse(input).unwrap_err();
    dbg!(&report.errors);
    assert_eq!(report.errors.len(), 1);
}
