use bob::sim_params::SimParams;
use bob::sim_set::SimSet;
use camino::Utf8PathBuf;

#[derive(PartialEq, Eq)]
pub struct GuiSimSet {
    pub path: Utf8PathBuf,
}

impl GuiSimSet {
    pub fn name(&self) -> &str {
        self.path.file_name().unwrap()
    }

    pub fn get_sims(&self) -> impl Iterator<Item = SimParams> {
        SimSet::from_output_folder(&self.path)
            .unwrap()
            .into_iter()
            .map(|(_, s)| s)
    }
}
