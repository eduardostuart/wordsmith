use lazy_static::lazy_static;
use regex::Regex;

use crate::error::WordsmithError;

use super::Component;

lazy_static! {
    /// Custom tags block
    ///
    /// Rust does not support look ahead/behind/around.
    /// To validate the end tag, we use captures and the naming group reference
    ///
    /// Expect the capture naming groups:
    /// - t: for the tag
    /// - c: for the content
    /// - e: for the end tag
    static ref REG_TAG_BLOCK: Regex = Regex::new(r"(@(?P<t>(info|warn|danger|quote)))(?s)(?P<c>.*?)(?s)(?P<e>@end(info|warn|danger|quote)?)").unwrap();
}

#[derive(Debug, Clone)]
pub struct CustomBlock;

impl CustomBlock {
  pub fn new() -> Self {
    Self {}
  }
}

impl Component for CustomBlock {
  fn compile(&self, input: &str) -> anyhow::Result<String> {
    log::info!("break_tag: {:?}", REG_TAG_BLOCK.as_str());

    // Check for invalid tags
    for m in REG_TAG_BLOCK.captures_iter(input) {
      let tag = &m["t"];
      let end_tag = &m["e"];

      log::warn!("Validating tag block: {} = {}?", tag, end_tag);

      if !end_tag.ends_with(tag) {
        return Err(
          WordsmithError::InvalidComponentClosingTag(tag.to_string(), end_tag.to_string()).into(),
        );
      }
    }

    Ok(
      REG_TAG_BLOCK
        .replace_all(input, r#"<blockquote class="$t-block">$c</blockquote>"#)
        .to_string(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_valid_block(input: &str, expected_output: &str) {
    let custom_block = CustomBlock::new();

    let result = custom_block.compile(input);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_output);
  }

  #[test]
  fn test_compile_valid_info_block() {
    let input = "@info This is some text inside a info block @endinfo";
    let expected_output =
      r#"<blockquote class="info-block"> This is some text inside a info block </blockquote>"#;
    test_valid_block(input, expected_output);
  }

  #[test]
  fn test_compile_valid_warn_block() {
    let input = "@warn This is some text inside a warn block @endwarn";
    let expected_output =
      r#"<blockquote class="warn-block"> This is some text inside a warn block </blockquote>"#;
    test_valid_block(input, expected_output);
  }

  #[test]
  fn test_compile_valid_danger_block() {
    let input = "@danger This is some text inside a danger block @enddanger";
    let expected_output =
      r#"<blockquote class="danger-block"> This is some text inside a danger block </blockquote>"#;
    test_valid_block(input, expected_output);
  }

  #[test]
  fn test_compile_valid_quote_block() {
    let input = "@quote This is some text inside a quote block @endquote";
    let expected_output =
      r#"<blockquote class="quote-block"> This is some text inside a quote block </blockquote>"#;
    test_valid_block(input, expected_output);
  }

  #[test]
  fn test_compile_invalid_input() {
    let custom_block = CustomBlock::new();

    let input = "@info This is some text inside a info block @endwarn";
    let expected_error =
      WordsmithError::InvalidComponentClosingTag("info".to_owned(), "@endwarn".to_owned());

    let result = custom_block.compile(input);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
  }
}
