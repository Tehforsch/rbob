use super::{
    plot_params::PlotParams,
    post_fn::{PostFn, PostFnKind, PostResult},
    snapshot::Snapshot,
};
use crate::{array_utils::FArray2, sim_params::SimParams, sim_set::SimSet};
use anyhow::Result;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct WeakScalingFn {}

impl PostFn for &WeakScalingFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "weak_scaling"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        get_scaling_data(sim_set)
    }
}

#[derive(Clap, Debug)]
pub struct StrongScalingFn {}

impl PostFn for &StrongScalingFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "strong_scaling"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        get_scaling_data(sim_set)
    }
}

#[derive(Clap, Debug)]
pub struct StrongScalingRuntimeFn {}

impl PostFn for &StrongScalingRuntimeFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "strong_scaling_runtime"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        get_scaling_data(sim_set)
    }
}

#[derive(Clap, Debug)]
pub struct WeakScalingRuntimeFn {}

impl PostFn for &WeakScalingRuntimeFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "weak_scaling_runtime"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        get_scaling_data(sim_set)
    }
}

fn get_scaling_data(sim_set: &SimSet) -> Result<PostResult> {
    let mut results = vec![];
    let sub_sim_sets = sim_set.quotient("SWEEP");
    for sub_sim_set in sub_sim_sets {
        let mut res = FArray2::zeros((sub_sim_set.len(), 2));
        for (i, sim) in sub_sim_set.enumerate() {
            res[[*i, 0]] = sim.get_num_cores()? as f64;
            res[[*i, 1]] = sim.get_run_time()?;
        }
        let mut params = PlotParams::default();
        params
            .0
            .insert("referenceTime".into(), res[[0, 1]].to_string());
        results.push(PostResult::new(params, vec![res]));
    }
    Ok(PostResult::join(results))
}
