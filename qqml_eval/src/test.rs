use super::utils::*;

#[test]
fn test_get_term_size() {
    // The only good way to test this function unfortunately.
    dbg!(get_term_size().unwrap().width);
    dbg!(get_term_size().unwrap().height);
}
