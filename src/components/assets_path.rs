use lazy_static::lazy_static;
use regex::Regex;

use super::Component;

lazy_static! {
    /// Match tag util to include a reference to the assets path
    static ref REG_ASSETS_PATH: Regex = Regex::new(r"@assets_path").unwrap();
}

#[derive(Debug, Clone)]
pub struct AssetsPath {
  assets_path: String,
}

impl AssetsPath {
  pub fn new(path: String) -> Self {
    Self { assets_path: path }
  }
}

impl Component for AssetsPath {
  fn compile(&self, input: &str) -> anyhow::Result<String> {
    log::info!(
      "compile_tag_assets_path: {:?} => {}",
      REG_ASSETS_PATH.as_str(),
      &self.assets_path
    );
    Ok(
      REG_ASSETS_PATH
        .replace_all(input, &self.assets_path)
        .to_string(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compile() {
    let assets_path = AssetsPath {
      assets_path: "/path/to/assets".to_owned(),
    };

    let input = "random text @assets_path here";
    let expected_output = "random text /path/to/assets here";

    let result = assets_path.compile(input);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_output);
  }
}
