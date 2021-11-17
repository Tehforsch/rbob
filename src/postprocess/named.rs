use anyhow::Result;

use super::plot_info::PlotInfo;
use super::snapshot::Snapshot;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;

pub trait Named {
    fn name(&self) -> &'static str;
    fn qualified_name(&self) -> String;
    fn get_plot_info(
        &self,
        sim_set: &SimSet,
        sim: Option<&SimParams>,
        snap: Option<&Snapshot>,
        plot_template_name: Option<&str>,
    ) -> Result<PlotInfo> {
        Ok(PlotInfo::new(
            &sim_set.get_folder()?,
            self.name(),
            &self.qualified_name(),
            plot_template_name,
            sim,
            snap,
        ))
    }
}
