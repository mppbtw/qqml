use crate::Error;
use std::process::Command;

pub struct TermSize {
    pub width: usize,
    pub height: usize,
}

pub fn get_term_size() -> Result<TermSize, Error> {
    let width: usize;
    let height: usize;

    let height = match Command::new("tput").arg("lines").output() {
        Ok(o) => {
            match String::from_utf8(o.stdout) {
                Ok(s) => {
                    if s.ends_with('\n') {
                        match s.strip_suffix('\n').unwrap().parse::<usize>() {
                            Ok(u) => u,
                            Err(e) => return Err(Error::FailedToGetTermHeight(e.to_string())),
                        }
                    } else {
                        match s.parse::<usize>() {
                            Ok(u) => u,
                            Err(e) => return Err(Error::FailedToGetTermHeight(e.to_string())),
                        }
                    }
                },
                Err(e) => return Err(Error::FailedToGetTermHeight(e.to_string())),
            }
        }
        Err(e) => return Err(Error::FailedToGetTermHeight(e.to_string())),
    };

    let width = match Command::new("tput").arg("cols").output() {
        Ok(o) => {
            match String::from_utf8(o.stdout) {
                Ok(s) => {
                    if s.ends_with('\n') {
                        match s.strip_suffix('\n').unwrap().parse::<usize>() {
                            Ok(u) => u,
                            Err(e) => return Err(Error::FailedToGetTermHeight(e.to_string())),
                        }
                    } else {
                        match s.parse::<usize>() {
                            Ok(u) => u,
                            Err(e) => return Err(Error::FailedToGetTermHeight(e.to_string())),
                        }
                    }
                },
                Err(e) => return Err(Error::FailedToGetTermHeight(e.to_string())),
            }
        }
        Err(e) => return Err(Error::FailedToGetTermHeight(e.to_string())),
    };

    Ok(TermSize { width, height })
}
