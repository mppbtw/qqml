use super::utils::*;

#[test]
fn do_the_roar() {
    // The only good way to test this function unfortunately.
    dbg!(get_term_size().unwrap().width);
    dbg!(get_term_size().unwrap().height);
}
