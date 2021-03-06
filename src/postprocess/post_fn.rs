use super::{
    data_plot_info::DataPlotInfo, get_snapshots, plot_info::PlotInfo, plot_params::PlotParams,
    snapshot::Snapshot,
};
use crate::{array_utils::FArray2, sim_params::SimParams, sim_set::SimSet};
use anyhow::Result;

pub enum PostFnKind {
    Snap,
    Sim,
    Set,
    NoPlotSet,
}

pub struct PostResult {
    pub params: PlotParams,
    pub data: Vec<FArray2>,
}

impl PostResult {
    pub fn new(params: PlotParams, data: Vec<FArray2>) -> Self {
        Self { params, data }
    }

    pub fn join(results: Vec<PostResult>) -> PostResult {
        let mut final_result = PostResult {
            params: PlotParams::default(),
            data: vec![],
        };
        for (i, result) in results.into_iter().enumerate() {
            final_result.data.extend(result.data);
            for (k, v) in result.params.0.iter() {
                final_result
                    .params
                    .0
                    .insert(format!("{}_{}", k, i), v.into());
            }
        }
        final_result
    }
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
    ) -> Result<PostResult>;

    fn run_post(
        &self,
        sim_set: &SimSet,
        plot_template_name: Option<&str>,
    ) -> Result<Vec<DataPlotInfo>> {
        match self.kind() {
            PostFnKind::Set => self.run_on_sim_set(sim_set, plot_template_name),
            PostFnKind::Sim => self.run_on_every_sim(sim_set, plot_template_name),
            PostFnKind::Snap => self.run_on_every_sim_and_snap(sim_set, plot_template_name),
            PostFnKind::NoPlotSet => self.run_on_sim_set_no_plot(sim_set),
        }
    }

    fn run_on_sim_set(
        &self,
        sim_set: &SimSet,
        plot_template_name: Option<&str>,
    ) -> Result<Vec<DataPlotInfo>> {
        let post_result = self.post(sim_set, None, None)?;
        Ok(vec![DataPlotInfo::new(
            self.get_plot_info(sim_set, None, None, plot_template_name)?,
            post_result,
        )])
    }

    fn run_on_sim_set_no_plot(&self, sim_set: &SimSet) -> Result<Vec<DataPlotInfo>> {
        self.post(sim_set, None, None)?;
        Ok(vec![])
    }

    fn run_on_every_sim(
        &self,
        sim_set: &SimSet,
        plot_template_name: Option<&str>,
    ) -> Result<Vec<DataPlotInfo>> {
        sim_set
            .iter()
            .map(|sim| {
                let post_result = self.post(sim_set, Some(sim), None)?;
                Ok(DataPlotInfo::new(
                    self.get_plot_info(sim_set, Some(sim), None, plot_template_name)?,
                    post_result,
                ))
            })
            .collect()
    }

    fn run_on_every_sim_and_snap(
        &self,
        sim_set: &SimSet,
        plot_template_name: Option<&str>,
    ) -> Result<Vec<DataPlotInfo>> {
        sim_set
            .iter()
            .map(|sim| {
                get_snapshots(sim)?
                    .map(|snap| {
                        let snap = snap?;
                        self.get_data_plot_info_for_sim_snap(
                            sim_set,
                            sim,
                            &snap,
                            plot_template_name,
                        )
                    })
                    .collect::<Result<Vec<DataPlotInfo>>>()
            })
            .flat_map(|res_vec| match res_vec {
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(err) => vec![Err(err)],
            })
            .collect()
    }

    fn get_data_plot_info_for_sim_snap(
        &self,
        sim_set: &SimSet,
        sim: &SimParams,
        snap: &Snapshot,
        plot_template_name: Option<&str>,
    ) -> Result<DataPlotInfo> {
        let res = self.post(sim_set, Some(sim), Some(&snap))?;
        Ok(DataPlotInfo::new(
            self.get_plot_info(sim_set, Some(sim), Some(&snap), plot_template_name)?,
            res,
        ))
    }

    fn get_plot_info(
        &self,
        sim_set: &SimSet,
        sim: Option<&SimParams>,
        snap: Option<&Snapshot>,
        plot_template_name: Option<&str>,
    ) -> Result<PlotInfo> {
        let name = match plot_template_name {
            None => self.name().into(),
            Some(name) => format!("{}_{}", self.name(), name),
        };
        let qualified_name = match plot_template_name {
            None => self.qualified_name().into(),
            Some(name) => format!("{}_{}", self.qualified_name(), name),
        };
        Ok(PlotInfo::new(
            &sim_set.get_folder()?,
            &name,
            &qualified_name,
            sim,
            snap,
        ))
    }
}
