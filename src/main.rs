use anyhow::Result;
use clap::Parser;
use log::Level;
use wordsmith::{Build, Cli, Commands, Init};

fn main() -> Result<()> {
  // Parse cli commands
  let args = Cli::parse();

  simple_logger::init_with_level(match args.verbose {
    1 => Level::Warn,
    2 => Level::Info,
    3 => Level::Debug,
    4 => Level::Trace,
    _ => Level::Error,
  })?;

  let current_path = std::env::current_dir()?;
  log::debug!("Running on {}", current_path.display());

  match args.commands.unwrap() {
    Commands::Init { folder } => {
      log::debug!("Init command triggered");
      Init::new(current_path, folder).execute()?;
    }
    Commands::Build { theme, output } => {
      log::debug!("Build command triggered");
      log::debug!("Building args: {:?}, {:?}", theme, output);
      Build::new(current_path, theme).execute()?;
    }
  };

  Ok(())
}
