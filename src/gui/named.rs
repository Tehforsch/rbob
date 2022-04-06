use bob::postprocess::snapshot::Snapshot;
use bob::sim_params::SimParams;

pub trait Named {
    fn name(&self) -> &str;
}

impl Named for SimParams {
    fn name(&self) -> &str {
        &self.get_name()
    }
}

impl Named for Snapshot {
    fn name(&self) -> &str {
        &self.get_name()
    }
}
