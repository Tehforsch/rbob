use std::fs;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use serde::Deserialize;
use serde::Serialize;

use crate::config;
use crate::job_params::SystemConfiguration;
use crate::util::expanduser;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub subsweep_path: Utf8PathBuf,
    pub subsweep_build_path: Utf8PathBuf,
    pub job_file_template: String,
    pub job_file_run_command: String,
    pub default_features: Vec<String>,
    pub default_profile: Option<String>,
    pub system_config: SystemConfiguration,
}

impl ConfigFile {
    pub fn read() -> Result<ConfigFile> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("rbob").unwrap();
        let config_path = xdg_dirs.find_config_file(config::CONFIG_FILE_NAME);
        if let Some(path) = config_path {
            ConfigFile::from_file(&path)
        } else {
            Err(anyhow!("No config file present"))
        }
    }

    fn from_file(file: &Path) -> Result<ConfigFile> {
        let data = fs::read_to_string(file)
            .context(format!("While reading config file at {:?}", file,))?;
        serde_yaml::from_str(&data).context("Reading config file contents")
    }

    pub fn expanduser(self) -> Result<ConfigFile> {
        Ok(ConfigFile {
            subsweep_path: Utf8Path::new(&expanduser(&self.subsweep_path)?).to_owned(),
            subsweep_build_path: Utf8Path::new(&expanduser(&self.subsweep_build_path)?).to_owned(),
            job_file_run_command: self.job_file_run_command,
            job_file_template: self.job_file_template,
            default_features: self.default_features,
            default_profile: self.default_profile,
            system_config: self.system_config,
        })
    }
}
