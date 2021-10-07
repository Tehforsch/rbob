use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;

use crate::config::DEFAULT_PLOT_EXTENSION;
use crate::config::PLOT_TEMPLATE_FOLDER;
use crate::util::read_file_contents;
use crate::util::write_file;

#[derive(Debug)]
pub struct PlotTemplate {
    path: Utf8PathBuf,
}

impl PlotTemplate {
    pub fn new(name: &str) -> Result<PlotTemplate> {
        let path = PLOT_TEMPLATE_FOLDER.join(format!("{}.{}", name, DEFAULT_PLOT_EXTENSION));
        Ok(PlotTemplate { path })
    }

    pub fn write_to(&self, target: &Utf8Path) -> Result<()> {
        // Rewrite this to just copy (this used to contain replacements but now it doesnt make sense anymore)
        let contents = read_file_contents(&self.path)
            .context(format!("While reading plot template file {:?}", self.path))?;
        write_file(target, &contents)
    }
}
