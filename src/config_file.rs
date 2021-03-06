use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{config, util::expanduser};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub plot_template_folder: PathBuf,
}

impl ConfigFile {
    pub fn read() -> Result<ConfigFile> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("bob").unwrap();
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

    pub fn expanduser(&self) -> Result<ConfigFile> {
        Ok(ConfigFile {
            plot_template_folder: expanduser(&self.plot_template_folder)?,
        })
    }
}
