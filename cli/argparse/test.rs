use super::utils::separate_lines;

#[test]
pub fn test_separate_lines() {
    let input: Vec<Vec<String>> = vec![vec!["a".into(), "b".into()], vec!["cc".into(), "dd".into()]];
    let expected = "a   b\ncc  dd";
    let result = separate_lines(input, 2).unwrap();
    dbg!(&result);
    assert_eq!(result, expected);
}
