use std::{fs, path::Path};

use anyhow::Result;
use yaml_rust::yaml::Hash;
use yaml_rust::{Yaml, YamlLoader};

use crate::error::WordsmithError;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Config {
  /// The exported document title
  pub title: String,

  /// Document configurations - margins, dimension
  pub document: DocumentConfig,

  /// Cover page configuration
  pub cover: CoverConfig,

  /// List of authors
  pub authors: Option<Vec<String>>,
}

impl Config {
  pub fn new() -> Self {
    Self {
      title: Default::default(),
      document: DocumentConfig::default(),
      cover: CoverConfig::default(),
      authors: Default::default(),
    }
  }

  fn yaml_key(&self, key: &str) -> Yaml {
    Yaml::String(key.to_string())
  }

  /// Load project configuration file from a yaml file
  ///
  /// ```yaml
  ///  # Wordsmith configuration file
  ///  title: "Sample"
  ///  authors:
  ///    - Name <name@email.com>
  ///    - Name B <name-b@email.com>
  ///  document:
  ///    dimensions: [8.7, 11.69] #inches
  ///    margins:
  ///      left: 0.0
  ///      top: 0.0
  ///      right: 0.0
  ///      bottom: 0.0
  ///  cover:
  ///    file: "cover.jpg"
  ///    dimensions: [8.7, 11.69]
  ///    position:
  ///      left: 0.0
  ///      right: 0.0
  ///      top: 0.0
  ///      bottom: 0.0
  /// ```
  pub fn load_from_file(&mut self, file: &Path) -> Result<Self> {
    let source = fs::read_to_string(file)?;
    let doc = YamlLoader::load_from_str(&source)?;

    if doc.is_empty() {
      return Ok(Self::default());
    }

    let doc = &doc[0];
    let title = self.get_title_from_yaml(doc);
    let authors = self.get_authors_from_yaml(doc);
    let document = self.get_document_config_from_yaml(doc);
    let cover = self.get_cover_from_yaml(doc)?;

    Ok(Self {
      title,
      document,
      authors,
      cover,
    })
  }

  /// Extract the title from configuration file.
  ///
  /// The title will be used as an alternative text (alt) for cover image,
  /// pdf title and as a fallback if there is no cover image
  fn get_title_from_yaml(&self, doc: &Yaml) -> String {
    doc["title"].as_str().unwrap_or("Default title").to_string()
  }

  /// Get list of authors.
  ///
  /// The list of authors will be included in the pdf metadata
  fn get_authors_from_yaml(&self, doc: &Yaml) -> Option<Vec<String>> {
    let mut authors = Vec::<String>::new();
    for entry in doc["authors"].as_vec().unwrap() {
      authors.push(entry.as_str().unwrap().to_string());
    }
    Some(authors)
  }

  /// Get document configuration,
  /// which will be used to  determine the PDF's dimensions and margins
  fn get_document_config_from_yaml(&self, doc: &Yaml) -> DocumentConfig {
    if doc["document"].as_hash().is_none() {
      return DocumentConfig::default();
    }

    let doc = doc["document"].as_hash().unwrap();

    let margins = if let Some(m) = doc.get(&self.yaml_key("margins")) {
      self.get_position_values(&mut m.as_hash().unwrap().clone())
    } else {
      PositionValues::default()
    };

    let dimensions = if let Some(document_entry) = doc.get(&self.yaml_key("dimensions")) {
      self.get_dimension_from_vec(&mut document_entry.as_vec().unwrap().clone())
    } else {
      Dimensions::default()
    };

    DocumentConfig {
      dimensions,
      margins,
    }
  }

  /// Get list of position values (left, top, right, bottom)
  fn get_position_values(&self, doc: &mut Hash) -> PositionValues {
    let get_position_value = |pos: &str| match doc.get(&self.yaml_key(pos)) {
      Some(v) => v.as_f64().unwrap_or(0.0),
      None => 0.0,
    };

    PositionValues(
      get_position_value("left"),
      get_position_value("top"),
      get_position_value("right"),
      get_position_value("bottom"),
    )
  }

  fn get_dimension_from_vec(&self, doc: &mut [Yaml]) -> Dimensions {
    Dimensions(
      doc[0].as_f64().unwrap_or(21.0),
      doc[1].as_f64().unwrap_or(29.0),
    )
  }

  fn get_cover_from_yaml(&self, doc: &Yaml) -> Result<CoverConfig, WordsmithError> {
    if doc["cover"].as_hash().is_none() {
      return Err(WordsmithError::ConfigCoverNotDefined);
    }

    let cover = doc["cover"].as_hash().unwrap();

    let filename = if let Some(filename_entry) = cover.get(&self.yaml_key("file")) {
      filename_entry.as_str().unwrap().to_string()
    } else {
      return Err(WordsmithError::ConfigCoverFileIsInvalid);
    };

    let position = if let Some(position_entry) = cover.get(&self.yaml_key("position")) {
      self.get_position_values(&mut position_entry.as_hash().unwrap().clone())
    } else {
      PositionValues::default()
    };

    let dimension = if let Some(dimension_entry) = cover.get(&self.yaml_key("dimensions")) {
      self.get_dimension_from_vec(&mut dimension_entry.as_vec().unwrap().clone())
    } else {
      Dimensions::default()
    };

    Ok(CoverConfig {
      filename,
      dimension,
      position,
    })
  }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct PositionValues(f64, f64, f64, f64);

impl PositionValues {
  pub fn get_values(&self) -> (f64, f64, f64, f64) {
    (self.0, self.1, self.2, self.3)
  }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Dimensions(f64, f64);

impl Dimensions {
  pub fn get_values(&self) -> (f64, f64) {
    (self.0, self.1)
  }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct CoverConfig {
  /// Cover image filename
  pub filename: String,

  /// Cover dimension in inches
  pub dimension: Dimensions,

  /// Position of the cover
  pub position: PositionValues,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct DocumentConfig {
  /// Document dimensions
  pub dimensions: Dimensions,
  /// Document margins
  pub margins: PositionValues,
}
