use bob::postprocess::snapshot::Snapshot;
use bob::sim_params::SimParams;
use bob::util::get_shell_command_output;

use super::gui_sim_set::GuiSimSet;
use crate::gui::named::Named;

pub trait Plot {
    fn get_command(&self) -> Vec<&str>;
    fn run_plot(
        &self,
        sim_sets: Vec<&GuiSimSet>,
        sims: Vec<&SimParams>,
        snaps: Vec<&Snapshot>,
    ) -> String {
        let mut args = vec![];
        args.extend(sim_sets.iter().map(|set| set.name()));
        args.push("-s");
        args.extend(sims.iter().map(|sim| sim.name()));
        args.push("--snaps");
        args.extend(snaps.iter().map(|snap| snap.name()));
        args.extend(&self.get_command());
        get_shell_command_output("pybob", &args, None, false).stdout
    }
}

pub struct Slice {}
impl Plot for Slice {
    fn get_command(&self) -> Vec<&str> {
        vec!["slice", "--axis", "z", "--field", "Temperature"]
    }
}
