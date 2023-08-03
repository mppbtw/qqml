#[derive(Debug)]
pub enum Error {
    FailedToGetTermWidth(String),
    FailedToGetTermHeight(String),
    TerminalTooSmall,
}
