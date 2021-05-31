use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::{config, util::expanduser};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub plot_template_folder: Utf8PathBuf,
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
        serde_yaml::from_str(&data).context("Reading config file contents")
    }

    fn default() -> ConfigFile {
        ConfigFile {
            plot_template_folder: Utf8Path::new("~/projects/phd/plotTemplates").into(),
        }
    }

    pub fn expanduser(&self) -> Result<ConfigFile> {
        let expanded = expanduser(&self.plot_template_folder).context(format!(
            "While reading plot template folder: {}",
            self.plot_template_folder
        ))?;
        Ok(ConfigFile {
            plot_template_folder: Utf8Path::new(&expanded).to_owned(),
        })
    }
}
