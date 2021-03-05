use super::{data_plot_info::DataPlotInfo, plot::PlotInfo, snapshot::Snapshot};
use crate::{
    array_utils::FArray2, config_file::ConfigFile, sim_params::SimParams, sim_set::SimSet,
};
use anyhow::Result;

pub enum PostFnKind {
    Snap,
    Sim,
    Set,
}

pub trait PostFn {
    const KIND: PostFnKind;
    const NAME: &'static str;

    fn post(
        sim_set: &SimSet,
        sim: Option<&SimParams>,
        snap: Option<&Snapshot>,
    ) -> Result<Vec<FArray2>>;

    fn run_post(&self, config_file: &ConfigFile, sim_set: &SimSet) -> Result<Vec<DataPlotInfo>> {
        match Self::KIND {
            PostFnKind::Set => Self::run_on_sim_set(config_file, sim_set),
            PostFnKind::Sim => {
                todo!()
            }
            // sim_set
            //     .iter()
            //     .map(self.post(config_file, sim_set, Some(sim), None))
            //     .collect()?,
            PostFnKind::Snap => {
                todo!()
            }
        }
    }

    fn run_on_sim_set(_config_file: &ConfigFile, sim_set: &SimSet) -> Result<Vec<DataPlotInfo>> {
        let post_result = Self::post(sim_set, None, None)?;
        let plot_info = PlotInfo::new(&sim_set.get_folder()?, Self::NAME, None, None);
        Ok(vec![DataPlotInfo {
            info: plot_info,
            data: post_result,
        }])
    }
}

// fn run_on_sim_snap(
//     &self,
//     config_file: &ConfigFile,
//     sim: &SimParams,
//     snap: &Snapshot,
//     plot_info: &PlotInfo,
// ) -> Result<()> {
//     let res = self.post(sim, snap)?;
//     let filenames = write_results(&plot_info.data_folder, &res)?;
//     plot::run_plot(config_file, &plot_info, &filenames)
// }

// fn run_on_sim(
//     &self,
//     config_file: &ConfigFile,
//     sim_set: &SimSet,
//     plot_info: &PlotInfo,
// ) -> Result<()> {
//     let res = self.post(sim_set)?;
//     let filenames = write_results(&plot_info.data_folder, &res)?;
//     plot::run_plot(config_file, &plot_info, &filenames)
// }
