use super::state::*;

#[test]
fn test_state_from_json() {
    let input = include_str!("./example.json").to_owned();

    let result = State::from_json(input).unwrap();

    assert!(true)
}
