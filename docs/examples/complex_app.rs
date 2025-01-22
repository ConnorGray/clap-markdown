use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// An example command-line tool
#[derive(Parser)]
#[command(name = "complex-app")]
pub struct Cli {
    /// Optional name to operate on
    ///
    /// Longer description
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", visible_alias = "configuration")]
    config: Option<PathBuf>,

    #[arg(long, default_value = "local")]
    target: Target,

    #[arg(long, visible_alias = "vv", visible_alias = "vvv")]
    very_very_verbose: bool,

    /// Turn debugging information on
    ///
    /// Repeat this option to see more and more debug information.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[arg(short, long, hide = true)]
    secret_arg: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    #[command(visible_alias = "tester")]
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Demo that `Options` is not printed if all options are hidden
    OnlyHiddenOptions {
        #[arg(short, long, hide = true)]
        secret: bool,
    },
}

#[derive(clap::ValueEnum)]
#[derive(Clone)]
enum Target {
    /// Do the operation locally
    Local,
    // Intentionally undocumented.
    Remote,
}
