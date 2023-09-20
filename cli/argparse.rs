/// This is a pretty simple argparsing solution inspired by Go's Cobra library
pub struct Command {
    children: Vec<Command>,
    usage: String,
    long: String,
    short: String,
}

impl Command {
    pub fn new(cmd_info: &CommandBuilder) -> Self {
        Self {
            children: vec![],
            usage: cmd_info.usage,
            long: cmd_info.long,
            short: cmd_info.short,
        }
    }
}

pub struct CommandBuilder {
    usage: String,
    long: String,
    short: String,
}
