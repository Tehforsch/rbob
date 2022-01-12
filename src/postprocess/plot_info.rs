use serde::Deserialize;
use serde::Serialize;

use super::snapshot::Snapshot;
use crate::sim_params::SimParams;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlotInfo {
    pub plot_name: String,
    pub name: String,
    pub qualified_name: String,
    pub plot_template_name: String,
}

impl PlotInfo {
    pub fn new(
        name: &str,
        qualified_name: &str,
        plot_template_name: Option<&str>,
        mb_sim: Option<&SimParams>,
        mb_snap: Option<&Snapshot>,
    ) -> PlotInfo {
        let plot_template_name = plot_template_name.unwrap_or(name).into();
        let name = format!("{}_{}", name, &plot_template_name);
        let qualified_name = format!("{}_{}", &qualified_name, &plot_template_name);
        let plot_name = match mb_sim {
            Some(sim) => {
                let sim_name = sim.folder.file_name().unwrap();
                match mb_snap {
                    None => format!("{}_{}", qualified_name, sim_name),
                    Some(snap) => {
                        format!("{}_{}_{}", qualified_name, sim_name, snap.get_name())
                    }
                }
            }
            None => qualified_name.clone(),
        };
        PlotInfo {
            plot_name,
            plot_template_name,
            name,
            qualified_name,
        }
    }
}
