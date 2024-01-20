use clap::{
    Parser,
    ValueEnum,
};

/// Launch arguments
#[derive(Debug, Parser)]
#[command(author, about, version)]
pub struct Args {
    #[arg(long, value_enum, default_value_t=Mode::default())]
    pub mode: Mode,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Compatible with all ANSI terminals (so about any terminal)
    Ansi,

    /// Combine keys with modifiers, like Ctrl-Alt-b or Space-g,
    /// if the terminal supports the Kitty Keyboard Protocol
    #[default]
    Kitty,

    /// Combine keys with modifiers, like Ctrl-Alt-b or Space-g,
    /// but also without, like a-b-c.
    KittyMultiSimples,
}
