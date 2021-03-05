use super::{plot::PlotInfo, SetPostFn};
use crate::{sim_params::SimParams, sim_set::SimSet};
use anyhow::Result;
use clap::Clap;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]

#[derive(Serialize, Deserialize, Debug)]
pub struct ScalingDataPoint {
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
    type Output = ScalingDataPoint;

    fn post(&self, sim_set: &SimSet) -> Result<Vec<Vec<Self::Output>>> {
        let res = sim_set
            .iter()
            .map(|sim| ScalingDataPoint::from_sim(sim))
            .collect::<Result<Vec<ScalingDataPoint>>>()?;
        Ok(vec![res])
    }

    // fn plot(&self, result: &Vec<Self::Output>, plot_info: &PlotInfo) -> Result<()> {
    //     run_plot(
    // }
}

impl ScalingFn {}
