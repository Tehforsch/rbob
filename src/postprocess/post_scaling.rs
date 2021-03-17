use std::collections::HashMap;

use super::{
    post_fn::{PostFn, PostFnKind},
    snapshot::Snapshot,
};
use crate::{array_utils::FArray2, sim_params::SimParams, sim_set::SimSet};
use anyhow::Result;
use clap::Clap;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]

#[derive(Serialize, Deserialize, Debug)]
pub struct ScalingDataPoint {
    num_cores: f64,
    run_time: f64,
}

impl ScalingDataPoint {
    pub fn from_sim(sim: &SimParams) -> Result<ScalingDataPoint> {
        Ok(ScalingDataPoint {
            num_cores: sim.get_num_cores()? as f64,
            run_time: sim.get_run_time()?,
        })
    }
}

#[derive(Clap, Debug)]
pub struct ScalingFn {}

impl PostFn for &ScalingFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "scaling"
    }

    fn qualified_name(&self) -> String {
        format!("{}", self.name())
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<(Vec<FArray2>, HashMap<String, String>)> {
        let mut res = FArray2::zeros((sim_set.len(), 2));
        for (i, sim) in sim_set.enumerate() {
            res[[*i, 0]] = sim.get_num_cores()? as f64;
            res[[*i, 1]] = sim.get_run_time()?;
        }

        Ok((vec![res], HashMap::new()))
    }

    // fn plot(&self, result: &Vec<Self::Output>, plot_info: &PlotInfo) -> Result<()> {
    //     run_plot(
    // }
}

impl ScalingFn {}
