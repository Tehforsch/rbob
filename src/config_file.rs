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
    pub plot_template_folder: Utf8PathBuf,
    pub arepo_path: Utf8PathBuf,
    pub default_systype: String,
    pub bob_path: Utf8PathBuf,
    pub job_file_template: String,
    pub job_file_run_command: String,
    pub system_config: SystemConfiguration,
    pub make_command: String,
}

impl ConfigFile {
    pub fn read() -> Result<ConfigFile> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("bob").unwrap();
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
            plot_template_folder: Utf8Path::new(&expanduser(&self.plot_template_folder)?)
                .to_owned(),
            arepo_path: Utf8Path::new(&expanduser(&self.arepo_path)?).to_owned(),
            bob_path: Utf8Path::new(&expanduser(&self.bob_path)?).to_owned(),
            default_systype: self.default_systype.clone(),
            job_file_run_command: self.job_file_run_command,
            job_file_template: self.job_file_template,
            system_config: self.system_config,
            make_command: self.make_command,
        })
    }
}
