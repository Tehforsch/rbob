use std::iter::once;

use anyhow::Result;

use super::data_plot_info::DataPlotInfo;
use super::get_snapshots;
use super::plot_info::PlotInfo;
use super::plot_params::PlotParams;
use super::snapshot::Snapshot;
use crate::array_utils::FArray2;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;

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

    fn run_post<'a>(
        &'a self,
        sim_set: &'a SimSet,
        plot_template_name: Option<&'a str>,
    ) -> Box<dyn Iterator<Item = Result<DataPlotInfo>> + 'a> {
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
    ) -> Box<dyn Iterator<Item = Result<DataPlotInfo>>> {
        let get_result = || {
            let data = self.post(sim_set, None, None)?;
            let info = self.get_plot_info(sim_set, None, None, plot_template_name)?;
            Ok(DataPlotInfo::new(info, data))
        };
        Box::new(once(get_result()))
    }

    fn run_on_sim_set_no_plot(
        &self,
        sim_set: &SimSet,
    ) -> Box<dyn Iterator<Item = Result<DataPlotInfo>> + '_> {
        let result = self.post(sim_set, None, None);
        if result.is_err() {
            Box::new(vec![Err(result.err().unwrap())].into_iter())
        } else {
            Box::new(vec![].into_iter())
        }
    }

    fn run_on_every_sim<'a, 'b>(
        &'a self,
        sim_set: &'a SimSet,
        plot_template_name: Option<&'a str>,
    ) -> Box<dyn Iterator<Item = Result<DataPlotInfo>> + 'a> {
        Box::new(sim_set.iter().map(move |sim| {
            let post_result = self.post(sim_set, Some(sim), None)?;
            Ok(DataPlotInfo::new(
                self.get_plot_info(sim_set, Some(sim), None, plot_template_name)?,
                post_result,
            ))
        }))
    }

    fn run_on_every_sim_and_snap<'a, 'b>(
        &'a self,
        sim_set: &'a SimSet,
        plot_template_name: Option<&'a str>,
    ) -> Box<dyn Iterator<Item = Result<DataPlotInfo>> + 'a> {
        Box::new(
            sim_set
                .iter()
                .map(move |sim| {
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
                }),
        )
    }

    fn get_data_plot_info_for_sim_snap(
        &self,
        sim_set: &SimSet,
        sim: &SimParams,
        snap: &Snapshot,
        plot_template_name: Option<&str>,
    ) -> Result<DataPlotInfo> {
        let res = self.post(sim_set, Some(sim), Some(snap))?;
        Ok(DataPlotInfo::new(
            self.get_plot_info(sim_set, Some(sim), Some(snap), plot_template_name)?,
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
