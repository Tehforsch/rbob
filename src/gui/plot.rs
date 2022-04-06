use bob::postprocess::snapshot::Snapshot;
use bob::sim_params::SimParams;
use bob::util::get_shell_command_output;
use camino::Utf8Path;
use strum::EnumIter;
use strum::IntoStaticStr;

use super::config;
use super::gui_sim_set::GuiSimSet;
use crate::gui::named::Named;

#[derive(IntoStaticStr, EnumIter)]
pub enum Plot {
    Slice,
}

impl Plot {
    fn get_command(&self) -> Vec<&str> {
        match self {
            Plot::Slice => vec!["slice", "--axis", "z", "--field", "Temperature"],
        }
    }

    pub fn run_plot(
        &self,
        folder: &Utf8Path,
        sim_sets: Vec<&GuiSimSet>,
        sims: Vec<&SimParams>,
        snaps: Vec<&Snapshot>,
    ) -> String {
        let mut args = vec![];
        args.push(config::PYBOB_PATH);
        args.extend(sim_sets.iter().map(|set| set.name()));
        args.push("-s");
        args.extend(sims.iter().map(|sim| sim.name()));
        args.push("--snaps");
        args.extend(snaps.iter().map(|snap| snap.name()));
        args.extend(&self.get_command());
        println!("{} {}", "pybob", args[1..].join(" "));
        get_shell_command_output("python3", &args, Some(folder), true).stdout
    }
}

impl Named for Plot {
    fn name(&self) -> &str {
        match self {
            Plot::Slice => "slice",
        }
    }
}
