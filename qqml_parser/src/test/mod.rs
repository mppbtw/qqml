use crate::*;

#[test]
fn test_parse_hints_directive() {
    let input1 = "
        hints 1;
        ".to_string();
    assert_eq!(parse(input1).unwrap().max_hints, 1);

    let input2 = "
        hints 2;
        ".to_string();
    assert_eq!(parse(input2).unwrap().max_hints, 2);
}
