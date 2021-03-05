use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::{
    config,
    config_file::ConfigFile,
    util::{expanduser, read_file_contents, write_file},
};

pub struct PlotTemplate {
    path: PathBuf,
}

impl PlotTemplate {
    pub fn new(config_file: &ConfigFile, name: &str) -> Result<PlotTemplate> {
        let path = &config_file.plot_template_folder.join(format!(
            "{}.{}",
            name,
            config::DEFAULT_PLOT_EXTENSION
        ));
        Ok(PlotTemplate {
            path: path.to_owned(),
        })
    }

    pub fn write_to(&self, target: &Path) -> Result<()> {
        // Rewrite this to just copy (this used to contain replacements but now it doesnt make sense anymore)
        let contents = read_file_contents(&self.path)
            .context(format!("While reading plot template file {:?}", self.path))?;
        write_file(target, &contents)
    }
}
