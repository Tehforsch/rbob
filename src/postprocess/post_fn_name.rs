use anyhow::Result;
use clap::Clap;

use super::data_plot_info::DataPlotInfo;
use super::post_compare::CompareFn;
use super::post_convergence::ConvergenceFn;
use super::post_expansion::ExpansionFn;
use super::post_scaling::ScalingFn;
use super::post_shadowing::ShadowingFn;
use super::post_slice::SliceFn;
use crate::sim_set::SimSet;

#[derive(Clap, Debug)]
pub enum PostFnName {
    Expansion(ExpansionFn),
    Slice(SliceFn),
    Scaling(ScalingFn),
    Compare(CompareFn),
    Shadowing(ShadowingFn),
    Convergence(ConvergenceFn),
}

impl PostFnName {
    pub fn run(&self, sim_set: &SimSet, plot_template: Option<&str>) -> Vec<Result<DataPlotInfo>> {
        use PostFnName::*;
        match self {
            Slice(slice) => SliceFn::run(slice, sim_set, plot_template),
            Compare(compare) => CompareFn::run(compare, sim_set),
            Expansion(expansion) => ExpansionFn::run(expansion, sim_set, plot_template),
            Scaling(scaling) => ScalingFn::run(scaling, sim_set, plot_template),
            Shadowing(shadowing) => ShadowingFn::run(shadowing, sim_set, plot_template),
            Convergence(convergence) => ConvergenceFn::run(convergence, sim_set, plot_template),
        }
    }
}
