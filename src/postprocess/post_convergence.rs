use anyhow::Result;
use clap::Clap;

use super::named::Named;
use super::plot_params::PlotParams;
use super::post_fn::PostResult;
use crate::array_utils::FArray2;
use crate::postprocess::DataPlotInfo;
use crate::set_function;
use crate::sim_set::SimSet;

#[derive(Clap, Debug)]
pub struct ConvergenceFn {}

impl ConvergenceFn {
    set_function!(convergence, { |sim_set| get_scaling_data(sim_set) });
}

impl Named for ConvergenceFn {
    fn name(&self) -> &'static str {
        "convergence"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }
}

fn get_scaling_data(sim_set: &SimSet) -> Result<PostResult> {
    let mut results = vec![];
    let sub_sim_sets = sim_set.quotient("SWEEP_NO_WARMSTARTING");
    for sub_sim_set in sub_sim_sets {
        let sim = sub_sim_set.iter().next().unwrap();
        let error_result = sim.get_log_file().get_convergence_errors()?;
        for errors in error_result.iter() {
            let mut res = FArray2::zeros((50, 2));
            for (j, error) in errors.iter().enumerate() {
                res[[j, 0]] = j as f64;
                res[[j, 1]] = *error;
            }
            let params = PlotParams::default();
            results.push(PostResult::new(params, vec![res]));
        }
    }
    Ok(PostResult::join(results))
}
