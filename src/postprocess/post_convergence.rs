use super::{
    plot_params::PlotParams,
    post_fn::{PostFn, PostFnKind, PostResult},
    snapshot::Snapshot,
};
use crate::{array_utils::FArray2, sim_params::SimParams, sim_set::SimSet};
use anyhow::Result;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct ConvergenceFn {}

impl PostFn for &ConvergenceFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "convergence"
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
