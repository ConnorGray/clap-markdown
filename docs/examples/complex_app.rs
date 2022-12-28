use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// An example command-line tool
#[derive(Parser)]
#[command(name = "complex-app")]
pub struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(long, default_value = "local")]
    target: Target,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
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
