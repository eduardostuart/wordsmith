use anyhow::Result;
use colored::*;
use std::path::PathBuf;

use crate::{builders::HtmlGen, builders::PdfGen, config::Config, error::WordsmithError};

#[derive(Debug)]
pub struct Build {
  pub theme: Option<String>,
  pub path: PathBuf,
}

impl Build {
  pub fn new(path: PathBuf, theme: Option<String>) -> Self {
    Self { theme, path }
  }

  fn load_config(&self) -> Result<Config> {
    let config_file = &self.path.join("ws.yaml");
    if !config_file.exists() {
      return Ok(Config::default());
    }
    Config::new().load_from_file(config_file)
  }

  pub fn execute(&self) -> Result<()> {
    if !self.path.join(".ws-lock").exists() {
      return Err(WordsmithError::ProjectNotFound.into());
    }

    println!("{}", "Building...".yellow());

    log::debug!("Building...");

    let config = &self.load_config()?;

    let doc_builder = HtmlGen::new(config.clone(), self.path.clone(), self.theme.clone());
    let (html_file, _) = doc_builder.build()?;

    PdfGen::new(config).generate(html_file, self.path.join("output/pdf.pdf"))?;

    doc_builder.clean_after_build();

    println!("{}", "Done!".green());

    log::debug!("Build is complete");
    Ok(())
  }
}
