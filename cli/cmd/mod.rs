mod run;

use crate::argparse::Command;

pub fn init(parent: &mut Command) {
    run::init(parent);
}
