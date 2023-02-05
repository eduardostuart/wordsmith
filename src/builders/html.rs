use crate::{
  components::{ComponentArg, Components, BREAK_PAGE_HTML},
  config::Config,
  error::WordsmithError,
};
use anyhow::Result;
use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};
use lazy_static::lazy_static;
use std::{
  collections::HashMap,
  fs::{self, create_dir_all, read_dir, read_to_string, remove_dir_all},
  path::PathBuf,
};

lazy_static! {
    /// Default theme
    static ref DEFAULT_THEME: &'static str = "light";
}

#[derive(Debug, Clone)]
pub struct HtmlGen<'a> {
  pub theme: Option<String>,
  pub config: Config,
  pub path: PathBuf,
  components: Components<'a>,
}

impl<'a> HtmlGen<'a> {
  /// Creates a [Builder] with provided theme, document configurations,
  /// and the target path to build the project
  pub fn new(config: Config, path: PathBuf, theme: Option<String>) -> Self {
    // Register list of components
    let components = Components::new(HashMap::from([
      (
        "assets_path".to_string(),
        ComponentArg::String(path.join("assets").display().to_string()),
      ),
      (
        "themes_path".to_string(),
        ComponentArg::String(path.join("themes").display().to_string()),
      ),
    ]));

    Self {
      theme,
      config,
      path,
      components,
    }
  }

  /// Return the full path relative to the current working directory.
  fn get_path(&self, path: &str) -> PathBuf {
    self.path.join(path)
  }

  /// Return the folder that should contain all project builds/generated files.
  fn get_output_path(&self) -> PathBuf {
    self.get_path("output")
  }

  fn get_output_file(&self, file: &str) -> PathBuf {
    self.get_output_path().join(file)
  }

  /// Return the defined theme or a default theme
  fn get_theme(&self) -> String {
    if let Some(theme) = &self.theme {
      return theme.to_owned();
    }
    DEFAULT_THEME.to_string()
  }

  /// Check if output path exists and remove its contents
  fn clean_output_folder(&self) -> Result<()> {
    log::debug!("Cleaning output folder");
    if self.get_output_path().exists() {
      remove_dir_all(self.get_output_path())?;
    }
    Ok(())
  }

  /// Transform markdown content into HTML
  fn transform_md_to_html(&self, markdown: &str) -> String {
    let options = &ComrakOptions {
      extension: ComrakExtensionOptions {
        strikethrough: true,
        tagfilter: false,
        table: true,
        autolink: true,
        tasklist: true,
        superscript: true,
        header_ids: Some("header-id-".to_string()),
        footnotes: true,
        description_lists: true,
        ..ComrakExtensionOptions::default()
      },
      render: ComrakRenderOptions {
        hardbreaks: false,
        github_pre_lang: true,
        ..ComrakRenderOptions::default()
      },
      ..ComrakOptions::default()
    };
    markdown_to_html(markdown, options)
  }

  /// Load theme HTML from themes folder
  ///
  /// Return [WordsmithError::ThemeNotFound] error if theme does not exist
  pub fn get_theme_html(&self) -> Result<String> {
    let theme_path = self.get_path(&format!("themes/{}.html", self.get_theme()));
    if !&theme_path.exists() {
      return Err(WordsmithError::ThemeNotFound(format!("{}", &theme_path.display())).into());
    }
    Ok(read_to_string(theme_path)?)
  }

  /// Build cover image HTML
  ///
  /// If there's no cover image the document title will be used
  /// as a fallback.
  pub fn get_cover_html(&self) -> String {
    log::debug!("Building cover");

    let image_src = self
      .get_path("assets/images/")
      .join(&self.config.cover.filename);

    if !image_src.exists() {
      return format!(r#"<h1>{}</1>"#, &self.config.title);
    }

    // Get cover dimensions
    let (width, height) = &self.config.cover.dimension.get_values();

    format!(
      r#"
      <div style="width:{w}mm;height:{h}mm;" class="cover">
        <img src="{src}" style="width:{w}mm;height:{h}mm;" alt="{alt}" />
      </div>
    "#,
      src = &image_src.display(),
      alt = &self.config.title,
      w = &width,
      h = &height
    )
  }

  /// Turn all your markdown files into HTML and concatenate
  /// them into one HTML response.
  pub fn get_content_html(&self) -> Result<String> {
    let mut content: Vec<String> = Vec::new();

    let mut paths: Vec<_> = read_dir(self.get_path("content"))?
      .map(|d| d.unwrap())
      .collect();

    paths.sort_by_key(|d| d.path());

    for entry in paths {
      let metadata = &entry.metadata()?;
      let path = &entry.path();
      let file_extension = &path.extension();

      // Skip directories and files that are not markdown
      if file_extension.is_none() || !metadata.is_file() {
        continue;
      }
      if !matches!(file_extension.unwrap().to_str(), Some("md")) {
        continue;
      }

      let raw_content = read_to_string(path)?;
      let compiled_content = self.components.compile_tag_assets_path(&raw_content)?;

      content.push(self.transform_md_to_html(&compiled_content))
    }

    Ok(content.join(" "))
  }

  /// Remove generated files
  #[allow(dead_code)]
  pub fn clean_after_build(&self) {
    if fs::remove_file(self.get_output_file("html.html")).is_ok() {
      log::debug!("Generated HTML file removed");
    }
  }

  fn get_theme_partial_file_html(&self, file: &str) -> Result<String> {
    Ok(read_to_string(self.get_path("themes/").join(file))?)
  }

  pub fn get_document_margin_style(&self) -> String {
    let (doc_w, doc_h) = self.config.document.dimensions.get_values();
    let (ml, mt, mr, mb) = self.config.document.margins.get_values();

    format!(
      r#"
      <style>
        @page {{
          size: {doc_w}mm {doc_h}mm;
        }}

        body {{
          padding-left: {ml}mm !important;
          padding-right: {mr}mm !important;
          padding-top: {mt}mm !important;
          padding-bottom: {mb}mm !important;
        }}

        h1 {{
          padding-top: {mt}mm !important;
        }}
      </style>
    "#
    )
  }

  pub fn generate_html_file_content(&self) -> Result<String> {
    log::debug!("Generating HTML file content");
    let mut html = String::new();

    html.push_str(r#"<!DOCTYPE html><head><meta charset="utf-8">"#);
    html.push_str(self.get_document_margin_style().as_str());
    html.push_str(
      self
        .get_theme_partial_file_html("__base-head.html")?
        .as_str(),
    );
    html.push_str(self.get_theme_html()?.as_str());
    html.push_str(r#"</head><body>"#);
    html.push_str(self.get_cover_html().as_str());
    html.push_str(BREAK_PAGE_HTML.to_string().as_str());
    html.push_str(self.get_content_html()?.as_str());
    html.push_str(r#"</body></html>"#);

    self.components.compile_all(&html)
  }

  pub fn build(&self) -> Result<(PathBuf, String)> {
    log::debug!("Building doc");
    // Prepare output folder
    self.clean_output_folder()?;

    log::debug!("Creating directories");
    create_dir_all(self.get_output_path())?;

    let html_file = self.get_output_file("html.html");
    let html = self.generate_html_file_content()?;

    log::debug!("Generating {} file", &html_file.display());
    fs::write(&html_file, &html)?;

    Ok((html_file, html))
  }
}
