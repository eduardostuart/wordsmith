use thiserror::Error;

#[derive(Error, Debug)]
pub enum WordsmithError {
  #[error("Invalid tag {0}")]
  InvalidTag(String),

  #[error("Invalid closing tag {0}, {1}")]
  InvalidComponentClosingTag(String, String),

  #[error("Project not found")]
  ProjectNotFound,
  /// Represents a failure to create a new project
  #[error("Project already exists in {0} folder")]
  ProjectConflict(String),

  #[error("Cover configuration is missing or invalid")]
  ConfigCoverNotDefined,

  #[error("Cover configuration file is not defined or empty")]
  ConfigCoverFileIsInvalid,

  #[error("Theme {0} not found")]
  ThemeNotFound(String),

  /// Represents all other cases of `std::io::Error`.
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}
