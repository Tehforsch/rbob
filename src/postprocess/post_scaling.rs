use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Clap;

use super::plot_params::PlotParams;
use super::post_fn::PostFn;
use super::post_fn::PostFnKind;
use super::post_fn::PostResult;
use super::snapshot::Snapshot;
use crate::array_utils::FArray2;
use crate::postprocess::voronoi_swim::generate_all_grid_files;
use crate::postprocess::voronoi_swim::simulate_run_time;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;

#[derive(Clap, Debug)]
pub struct ScalingFn {
    quotient_parameter: Option<String>,
    #[clap(long)]
    voronoi_swim: Vec<Utf8PathBuf>,
}

impl PostFn for &ScalingFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "scaling"
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
        self.get_scaling_data(sim_set)
    }
}

impl ScalingFn {
    fn get_scaling_data(&self, sim_set: &SimSet) -> Result<PostResult> {
        let mut results = vec![];
        let sub_sim_sets = match self.quotient_parameter {
            Some(ref param) => sim_set.quotient(param),
            None => vec![sim_set.clone()],
        };
        let get_params = |res: FArray2| {
            let mut params = PlotParams::default();
            params
                .0
                .insert("referenceTime".into(), res[[0, 1]].to_string());
            params
                .0
                .insert("referenceCores".into(), res[[0, 0]].to_string());
            PostResult::new(params, vec![res])
        };
        for sub_sim_set in sub_sim_sets.iter() {
            let mut res = FArray2::zeros((sub_sim_set.len(), 2));
            for (i, sim) in sub_sim_set.enumerate() {
                res[[*i, 0]] = sim.get_num_cores()? as f64;
                res[[*i, 1]] = sim.get_rt_run_time()?;
            }
            results.push(get_params(res))
        }
        // Assume all sub sim sets have the same grids which is always
        // the case in scaling plots (in mine at least)
        for res in self.get_voronoi_swim_results(&sub_sim_sets[0])? {
            results.push(get_params(res))
        }
        Ok(PostResult::join(results))
    }

    fn get_voronoi_swim_results(&self, sub_sim_set: &SimSet) -> Result<Vec<FArray2>> {
        if !self.voronoi_swim.is_empty() {
            generate_all_grid_files(&sub_sim_set)?;
        }
        self.voronoi_swim.iter().map(|voronoi_swim_param_file| {
            let mut res = FArray2::zeros((sub_sim_set.len(), 2));
            for (i, sim) in sub_sim_set.enumerate() {
                res[[*i, 0]] = sim.get_num_cores()? as f64;
                res[[*i, 1]] = simulate_run_time(sim, voronoi_swim_param_file)? * sim.get_num_sweep_runs()? as f64;
            }
            Ok(res)
        }).collect::<Result<Vec<_>>>()
    }
}
