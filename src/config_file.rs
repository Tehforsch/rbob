use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::config;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub plot_template_folder: PathBuf,
}

impl ConfigFile {
    pub fn read() -> Result<ConfigFile> {
        let package_name = std::env::var("CARGO_PKG_NAME")?;
        let xdg_dirs = xdg::BaseDirectories::with_prefix(package_name).unwrap();
        let config_path = xdg_dirs.find_config_file(config::CONFIG_FILE_NAME);
        match config_path {
            Some(path) => ConfigFile::from_file(&path),
            None => Ok(ConfigFile::default()),
        }
    }

    fn from_file(file: &Path) -> Result<ConfigFile> {
        let data = fs::read_to_string(file)
            .context(format!("While reading config file at {:?}", file,))?;
        Ok(serde_yaml::from_str(&data).context("Reading config file contents")?)
    }

    fn default() -> ConfigFile {
        ConfigFile {
            plot_template_folder: "~/projects/bob/plotTemplates".into(),
        }
    }
}
