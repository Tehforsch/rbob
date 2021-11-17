use anyhow::Result;
use clap::Clap;

use super::data_plot_info::DataPlotInfo;
use super::post_compare::CompareFn;
use super::post_convergence::ConvergenceFn;
use super::post_expansion::DTypeExpansionFn;
use super::post_expansion::RTypeExpansionFn;
use super::post_scaling::ScalingFn;
use super::post_shadowing::ShadowingFn;
use super::post_slice::SliceFn;
use crate::sim_set::SimSet;

#[derive(Clap, Debug)]
pub enum PostFnName {
    RType(RTypeExpansionFn),
    DType(DTypeExpansionFn),
    Slice(SliceFn),
    Scaling(ScalingFn),
    Compare(CompareFn),
    Shadowing(ShadowingFn),
    Convergence(ConvergenceFn),
}

impl PostFnName {
    pub fn run(&self, sim_set: &SimSet, plot_template: Option<&str>) -> Vec<Result<DataPlotInfo>> {
        match self {
            PostFnName::Slice(slice) => SliceFn::run(slice, sim_set, plot_template),
            _ => unimplemented!(),
        }
    }
}
