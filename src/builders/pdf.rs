use std::{fs, path::PathBuf};

use anyhow::Result;
use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptions, LaunchOptionsBuilder};

use crate::config::Config;

#[derive(Debug)]
pub struct PdfGen<'a> {
  pub config: &'a Config,
}

impl<'a> PdfGen<'a> {
  pub fn new(config: &'a Config) -> Self {
    Self { config }
  }

  fn get_print_options(&self) -> PrintToPdfOptions {
    PrintToPdfOptions {
      landscape: Some(false),
      display_header_footer: Some(false),
      margin_left: Some(0.0),
      margin_top: Some(0.0),
      margin_right: Some(0.0),
      margin_bottom: Some(0.0),
      print_background: Some(true),
      prefer_css_page_size: Some(true),
      ..PrintToPdfOptions::default()
    }
  }

  fn get_browser_options(&self) -> Result<LaunchOptions> {
    Ok(
      LaunchOptionsBuilder::default()
        .disable_default_args(true)
        .headless(true)
        .build()?,
    )
  }

  /// Using chrome headless open an HTML file and generate a PDF file
  pub fn generate(&self, html_file: PathBuf, pdf_file: PathBuf) -> Result<()> {
    let browser_opts = self.get_browser_options()?;
    let browser = Browser::new(browser_opts)?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(&format!("file://{}", html_file.display()))?;

    let pdf = tab
      .wait_until_navigated()?
      .print_to_pdf(Some(self.get_print_options()))?;

    fs::write(pdf_file, pdf)?;

    Ok(())
  }
}
