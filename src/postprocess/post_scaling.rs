use anyhow::Result;
use clap::Clap;

use super::data_plot_info::DataPlotInfo;
use super::named::Named;
use super::plot_params::PlotParams;
use super::post_fn::PostResult;
use crate::array_utils::FArray2;
use crate::set_function;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;

#[derive(Clap, Debug)]
pub struct ScalingFn {
    quotient_parameter: Option<String>,
    #[clap(long)]
    ignore_failed: bool,
}

impl Named for ScalingFn {
    fn name(&self) -> &'static str {
        "scaling"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }
}

impl ScalingFn {
    set_function!(scaling, { |sim_set| scaling.get_scaling_data(sim_set) });
}

impl ScalingFn {
    fn get_scaling_data(&self, sim_set: &SimSet) -> Result<PostResult> {
        let mut results = vec![];
        let mut sub_sim_sets = match self.quotient_parameter {
            Some(ref param) => sim_set.quotient(param),
            None => vec![sim_set.clone()],
        };
        sub_sim_sets.sort_by_key(|set| set.iter().map(|sim| sim.get_num_cores().unwrap()).min());
        let get_params = |first_sim: &SimParams, res: FArray2| {
            let mut params = PlotParams::default();
            params.add("referenceTime".into(), res[[0, 1]]);
            params.add("referenceCores".into(), res[[0, 0]]);
            params.add(
                "NDir".into(),
                first_sim.get("SX_NDIR").unwrap().unwrap_i64(),
            );
            params.add("NFreq".into(), crate::config::SX_NFREQ);
            PostResult::new(params, vec![res])
        };
        for sub_sim_set in sub_sim_sets.iter() {
            let mut res = FArray2::zeros((sub_sim_set.len(), 2));
            for (i, sim) in sub_sim_set.enumerate() {
                res[[*i, 0]] = sim.get_num_cores()? as f64;
                res[[*i, 1]] = if self.ignore_failed {
                    match sim.get_rt_run_time() {
                        Ok(time) => time,
                        Err(_) => 0.0,
                    }
                } else {
                    sim.get_rt_run_time()?
                };
            }
            results.push(get_params(sub_sim_set.iter().next().unwrap(), res))
        }
        Ok(PostResult::join(results))
    }
}
