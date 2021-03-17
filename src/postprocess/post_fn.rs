use std::collections::HashMap;

use super::{data_plot_info::DataPlotInfo, get_snapshots, plot_info::PlotInfo, snapshot::Snapshot};
use crate::{array_utils::FArray2, sim_params::SimParams, sim_set::SimSet};
use anyhow::Result;

pub enum PostFnKind {
    Snap,
    Sim,
    Set,
    NoPlotSet,
}

pub trait PostFn {
    fn kind(&self) -> PostFnKind;
    fn name(&self) -> &'static str;
    fn qualified_name(&self) -> String;

    fn post(
        &self,
        sim_set: &SimSet,
        sim: Option<&SimParams>,
        snap: Option<&Snapshot>,
    ) -> Result<(Vec<FArray2>, HashMap<String, String>)>;

    fn run_post(&self, sim_set: &SimSet) -> Result<Vec<DataPlotInfo>> {
        match self.kind() {
            PostFnKind::Set => self.run_on_sim_set(sim_set),
            PostFnKind::Sim => self.run_on_every_sim(sim_set),
            PostFnKind::Snap => self.run_on_every_sim_and_snap(sim_set),
            PostFnKind::NoPlotSet => self.run_on_sim_set_no_plot(sim_set),
        }
    }

    fn run_on_sim_set(&self, sim_set: &SimSet) -> Result<Vec<DataPlotInfo>> {
        let post_result = self.post(sim_set, None, None)?;
        Ok(vec![DataPlotInfo {
            info: self.get_plot_info(sim_set, None, None)?,
            data: post_result.0,
            replacements: post_result.1,
        }])
    }

    fn run_on_sim_set_no_plot(&self, sim_set: &SimSet) -> Result<Vec<DataPlotInfo>> {
        self.post(sim_set, None, None)?;
        Ok(vec![])
    }

    fn run_on_every_sim(&self, sim_set: &SimSet) -> Result<Vec<DataPlotInfo>> {
        sim_set
            .iter()
            .map(|sim| {
                let post_result = self.post(sim_set, Some(sim), None)?;
                Ok(DataPlotInfo {
                    info: self.get_plot_info(sim_set, Some(sim), None)?,
                    data: post_result.0,
                    replacements: post_result.1,
                })
            })
            .collect()
    }

    fn run_on_every_sim_and_snap(&self, sim_set: &SimSet) -> Result<Vec<DataPlotInfo>> {
        sim_set
            .iter()
            .map(|sim| {
                Ok(get_snapshots(sim)?
                    .map(|snap| {
                        let snap = snap?;
                        self.get_data_plot_info_for_sim_snap(sim_set, sim, &snap)
                    })
                    .collect::<Result<Vec<DataPlotInfo>>>()?)
            })
            .flat_map(|res_vec| match res_vec {
                Ok(vec) => vec.into_iter().map(move |item| Ok(item)).collect(),
                Err(err) => vec![Err(err)],
            })
            .collect()
    }

    fn get_data_plot_info_for_sim_snap(
        &self,
        sim_set: &SimSet,
        sim: &SimParams,
        snap: &Snapshot,
    ) -> Result<DataPlotInfo> {
        let res = self.post(sim_set, Some(sim), Some(&snap))?;
        Ok(DataPlotInfo {
            info: self.get_plot_info(sim_set, Some(sim), Some(&snap))?,
            data: res.0,
            replacements: res.1,
        })
    }

    fn get_plot_info(
        &self,
        sim_set: &SimSet,
        sim: Option<&SimParams>,
        snap: Option<&Snapshot>,
    ) -> Result<PlotInfo> {
        Ok(PlotInfo::new(
            &sim_set.get_folder()?,
            &self.name(),
            &self.qualified_name(),
            sim,
            snap,
        ))
    }
}
