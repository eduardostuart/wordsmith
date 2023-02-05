use lazy_static::lazy_static;
use regex::Regex;

use super::Component;

lazy_static! {
    /// Match tag util to include a reference to the assets path
    static ref REG_THEMES_PATH: Regex = Regex::new(r"@themes_path").unwrap();
}

#[derive(Debug, Clone)]
pub struct ThemesPath {
  themes_path: String,
}

impl ThemesPath {
  pub fn new(path: String) -> Self {
    Self { themes_path: path }
  }
}

impl Component for ThemesPath {
  fn compile(&self, input: &str) -> anyhow::Result<String> {
    log::info!(
      "compile themes_path: {:?} => {}",
      REG_THEMES_PATH.as_str(),
      &self.themes_path
    );
    Ok(
      REG_THEMES_PATH
        .replace_all(input, &self.themes_path)
        .to_string(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compile() {
    let themes_path = ThemesPath {
      themes_path: "/path/to/themes".to_owned(),
    };

    let input = "random text @themes_path here";
    let expected_output = "random text /path/to/themes here";

    let result = themes_path.compile(input);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_output);
  }
}
