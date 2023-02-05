use std::path::PathBuf;

use clap::{ColorChoice, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "Eduardo Stuart <e@s.tuart.me>", 
    version,
    about,
    long_about = None,
    arg_required_else_help(true),
    color = ColorChoice::Always
)]
pub struct Cli {
  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  pub config: Option<PathBuf>,

  // The number of occurrences of the `v/verbose` flag
  /// Verbose mode (-v, -vv, -vvv, etc.)
  /// Default value will show only errors
  #[arg(short, long, action = clap::ArgAction::Count, default_value_t = 0)]
  pub verbose: u8,

  #[command(subcommand)]
  pub commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Initialize a new project in the directory
  Init {
    /// Folder to be created. If not defined, use current directory
    folder: Option<String>,
  },
  /// Build the project and generate a PDF file
  Build {
    /// Which theme should be used: light, dark or something else ?
    theme: Option<String>,
    /// File output name
    output: Option<String>,
  },
}
