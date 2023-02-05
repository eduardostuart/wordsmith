use anyhow::Result;
use colored::Colorize;
use std::{
  fs::{self, create_dir_all, File},
  path::{Path, PathBuf},
};

use crate::{assets::Asset, error::WordsmithError};

#[derive(Debug)]
pub struct Init {
  path: PathBuf,
  folder_to_create: Option<String>,
}

impl Init {
  /// Creates [Init] command with provided path
  pub fn new(path: PathBuf, folder_to_create: Option<String>) -> Self {
    Self {
      path,
      folder_to_create,
    }
  }

  fn get_project_folder(&self) -> PathBuf {
    let path = &self.path;
    if let Some(folder) = &self.folder_to_create {
      path.join(folder)
    } else {
      path.to_path_buf()
    }
  }

  fn get_lock_file(&self) -> PathBuf {
    self.get_project_folder().join(".ws-lock")
  }

  fn create_dir(&self, entry: &Path) -> Result<()> {
    if entry.parent().is_none() {
      return Ok(());
    }

    let path = &self.get_project_folder().join(entry.parent().unwrap());
    if !path.exists() {
      log::info!("Creating directory: {:?}", &path);
      create_dir_all(path)?;
    }

    Ok(())
  }

  fn create_project_structure(&self) -> Result<()> {
    for embed_entry in Asset::iter() {
      let entry = embed_entry.to_string();
      self.create_dir(Path::new(&entry))?;

      let file_full_path = self.get_project_folder().join(&entry);
      log::info!("Creating file: {}", &file_full_path.display());

      let asset = Asset::get(&entry).unwrap();
      fs::write(&file_full_path, asset.data)?;
    }

    File::create(self.get_lock_file())?;
    Ok(())
  }

  pub fn execute(&self) -> Result<()> {
    println!(
      "\n{}\n",
      r#"
░█░█░█▀█░█▀▄░█▀▄░█▀▀░█▄█░▀█▀░▀█▀░█░█
░█▄█░█░█░█▀▄░█░█░▀▀█░█░█░░█░░░█░░█▀█
░▀░▀░▀▀▀░▀░▀░▀▀░░▀▀▀░▀░▀░▀▀▀░░▀░░▀░▀
      "#
      .purple()
    );

    let project_dir = &self.get_project_folder();

    if !project_dir.exists() {
      fs::create_dir(project_dir)?;
    }

    if self.get_lock_file().exists() {
      return Err(
        WordsmithError::ProjectConflict(format!("{}", &self.get_project_folder().display())).into(),
      );
    }

    self.create_project_structure()?;

    println!("{}", "\nDone!\n".green());

    Ok(())
  }
}
