use std::{collections::HashMap, fmt};

use anyhow::Result;
use lazy_static::lazy_static;

use crate::error::WordsmithError;

use self::{
  assets_path::AssetsPath, break_tag::BreakTag, custom_block_tag::CustomBlock,
  themes_path::ThemesPath,
};

// I know.
// Regex is not the best for this. But, this should work for now (or forever muhahaha)
// Just kidding. Or not.
mod assets_path;
mod break_tag;
mod custom_block_tag;
mod themes_path;

lazy_static! {
    /// HTML used to create page breaks
    pub static ref BREAK_PAGE_HTML: &'static str = r#"<div style="page-break-after: always;"></div>"#;
}

pub trait Component: Clone + fmt::Debug {
  fn compile(&self, input: &str) -> anyhow::Result<String>;
}

#[derive(Debug, Clone)]
pub enum ComponentArg {
  String(String),
}

#[derive(Debug, Clone)]
pub struct Components<'a> {
  args: HashMap<String, ComponentArg>,
  components: Vec<&'a str>,
}

impl<'a> Components<'a> {
  pub fn new(args: HashMap<String, ComponentArg>) -> Self {
    Self {
      args,
      components: ["break", "custom_block", "assets_path", "themes_path"].to_vec(),
    }
  }

  /// Return string value from list of arguments
  /// If argument does not exist return empty string
  fn get_string_arg(&self, key: &str) -> String {
    if let Some(component) = self.args.get(key) {
      let ComponentArg::String(value) = component;
      return value.to_owned();
    }
    "".to_string()
  }

  pub fn compile_tag_themes_path(&self, input: &str) -> Result<String> {
    let path = self.get_string_arg("themes_path");
    ThemesPath::new(path).compile(input)
  }

  pub fn compile_tag_assets_path(&self, input: &str) -> Result<String> {
    let path = self.get_string_arg("assets_path");
    AssetsPath::new(path).compile(input)
  }

  pub fn compile_custom_block(&self, input: &str) -> Result<String> {
    CustomBlock::new().compile(input)
  }

  pub fn compile_break_tag(&self, input: &str) -> Result<String> {
    BreakTag::new().compile(input)
  }

  pub fn compile_all(&self, input: &str) -> Result<String> {
    let mut new_value = input.to_string();
    for name in self.components.clone() {
      new_value = match name {
        "break" => self.compile_break_tag(&new_value)?,
        "custom_block" => self.compile_custom_block(&new_value)?,
        "assets_path" => self.compile_tag_assets_path(&new_value)?,
        "themes_path" => self.compile_tag_themes_path(&new_value)?,
        t => return Err(WordsmithError::InvalidTag(t.to_string()).into()),
      }
      .to_string();
    }

    Ok(new_value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_string_arg() {
    let args = HashMap::from([
      (
        "themes_path".to_string(),
        ComponentArg::String("test".to_string()),
      ),
      (
        "assets_path".to_string(),
        ComponentArg::String("test".to_string()),
      ),
    ]);
    let components = Components::new(args);

    assert_eq!(components.get_string_arg("themes_path"), "test".to_string());
    assert_eq!(components.get_string_arg("assets_path"), "test".to_string());
    assert_eq!(components.get_string_arg("not_exist"), "".to_string());
  }

  #[test]
  fn test_compile_tag_themes_path() {
    let args = HashMap::from([(
      "themes_path".to_string(),
      ComponentArg::String("/themes".to_string()),
    )]);

    let components = Components::new(args);
    let result = components.compile_tag_themes_path("<img src=@themes_path/images/example.png>");

    assert_eq!(result.unwrap(), "<img src=/themes/images/example.png>");
  }
}
