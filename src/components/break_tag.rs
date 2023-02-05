use lazy_static::lazy_static;
use regex::Regex;

use crate::components::BREAK_PAGE_HTML;

use super::Component;

lazy_static! {
    /// Match tag util to include page break
    static ref REG_BREAK: Regex = Regex::new(r"@break").unwrap();
}

#[derive(Debug, Clone)]
pub struct BreakTag;

impl BreakTag {
  pub fn new() -> Self {
    Self {}
  }
}

impl Component for BreakTag {
  fn compile(&self, input: &str) -> anyhow::Result<String> {
    log::info!("break_tag: {:?}", REG_BREAK.as_str());
    Ok(
      REG_BREAK
        .replace_all(input, BREAK_PAGE_HTML.to_string())
        .to_string(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compile() {
    let break_tag = BreakTag {};

    let input = "random text @break here";
    let expected_output = format!("random text {} here", *BREAK_PAGE_HTML);

    let result = break_tag.compile(input);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_output);
  }
}
