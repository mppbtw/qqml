pub fn print_help(command: HelpCommand) {}

pub enum HelpCommand {
    Play,
    Version,
    Yarr,
    Init,
    Sections,
    Lint,
}
