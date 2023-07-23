use std::process::exit;

mod section;
mod yarr;
mod section_play;

pub const TOPTEXT: &str = "\
Yet Another Revision Resource - The pointless and overkill homework project.
Yarr is free software, licensed under the MIT License and the source code is
available at https://github.com/MrPiggyPegasus/yarr.

";

pub fn print_help(command: HelpCommand) {
    let help_text = match command {
        HelpCommand::Yarr => yarr::HELP.to_owned(),
        HelpCommand::Section => section::HELP.to_owned(),
        HelpCommand::SectionPlay => section_play::HELP.to_owned(),
        _ => "abc".to_owned(),
    };
    println!("{}", help_text);
    exit(0);
}

pub enum HelpCommand {
    SectionPlay,
    Version,
    Yarr,
    Init,
    Section,
    Lint,
}
