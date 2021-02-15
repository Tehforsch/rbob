use super::{plot::PlotInfo, SetPostFn};
use crate::{sim_params::SimParams, sim_set::SimSet};
use anyhow::Result;
use clap::Clap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScalingResult {
    data: Vec<ScalingDataPoint>,
}

#[derive(Serialize, Deserialize)]
struct ScalingDataPoint {
    num_cores: i64,
    run_time: f64,
}

impl ScalingDataPoint {
    pub fn from_sim(sim: &SimParams) -> Result<ScalingDataPoint> {
        Ok(ScalingDataPoint {
            num_cores: sim.get_num_cores()?,
            run_time: sim.get_run_time()?,
        })
    }
}

#[derive(Clap, Debug)]
pub struct ScalingFn {}

impl SetPostFn for &ScalingFn {
    type Output = ScalingResult;

    fn post(&self, sim_set: &SimSet) -> Result<Vec<Self::Output>> {
        let res = ScalingResult {
            data: sim_set
                .iter()
                .map(|sim| ScalingDataPoint::from_sim(sim))
                .collect::<Result<Vec<ScalingDataPoint>>>()?,
        };
        Ok(vec![res])
    }

    fn plot(&self, result: &Vec<Self::Output>, plot_info: &PlotInfo) -> Result<()> {
        Ok(())
    }
}

impl ScalingFn {}
