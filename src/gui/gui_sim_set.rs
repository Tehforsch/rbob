use bob::sim_params::SimParams;
use bob::sim_set::SimSet;
use bob::util::get_relative_path;
use camino::Utf8Path;
use camino::Utf8PathBuf;

use super::named::Named;

#[derive(PartialEq, Eq)]
pub struct GuiSimSet {
    pub top_path: Utf8PathBuf,
    pub path: Utf8PathBuf,
    pub name: String,
}

impl GuiSimSet {
    pub fn new(top_path: Utf8PathBuf, path: &Utf8Path) -> Self {
        let name = get_relative_path(&path, &top_path)
            .unwrap()
            .as_str()
            .to_owned();
        Self {
            top_path,
            path: path.to_owned(),
            name,
        }
    }
    pub fn get_sims(&self) -> impl Iterator<Item = SimParams> {
        SimSet::from_output_folder(&self.path)
            .unwrap()
            .into_iter()
            .map(|(_, s)| s)
    }
}

impl Named for GuiSimSet {
    fn name(&self) -> &str {
        &self.name
    }
}
