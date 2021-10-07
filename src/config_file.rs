use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;

use crate::config;
use crate::util::expanduser;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub plot_template_folder: Utf8PathBuf,
    pub arepo_path: Utf8PathBuf,
    pub default_systype: String,
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
            arepo_path: Utf8Path::new("~/projects/arepo").into(),
            default_systype: "Manjaro".into(),
        }
    }

    pub fn expanduser(&self) -> Result<ConfigFile> {
        Ok(ConfigFile {
            plot_template_folder: Utf8Path::new(&expanduser(&self.plot_template_folder)?)
                .to_owned(),
            arepo_path: Utf8Path::new(&expanduser(&self.arepo_path)?).to_owned(),
            default_systype: self.default_systype.clone(),
        })
    }
}
