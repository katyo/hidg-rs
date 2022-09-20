#[derive(clap::Parser)]
#[clap(
    name = "hidg",
    version,
    about,
    propagate_version = true,
    // Command::trailing_var_ar is required to use ValueHint::CommandWithArguments
    trailing_var_arg = true,
)]
pub struct Args {
    /// HIDG commands
    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Class {
    /// Keyboard
    #[clap(aliases = ["kbd", "k"])]
    Keyboard,

    /// Mouse
    #[clap(aliases = ["m"])]
    Mouse,
}

#[derive(clap::Subcommand)]
pub enum Cmd {
    /// Read-write reports in interactive mode
    Repl {
        #[clap(short, long, arg_enum, default_value = "keyboard")]
        class: Class,

        #[clap(value_parser, default_value = "hidg0")]
        path: std::path::PathBuf,
    },
}
