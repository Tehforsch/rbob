use anyhow::Result;
use clap::Clap;

use super::data_plot_info::DataPlotInfo;
use super::post_compare::CompareFn;
use super::post_convergence::ConvergenceFn;
use super::post_expansion::ExpansionFn;
use super::post_ionization::IonizationFn;
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
    Ionization(IonizationFn),
}

impl PostFnName {
    pub fn run(
        &self,
        sim_set: &SimSet,
        plot_template: Option<&str>,
    ) -> Box<dyn Iterator<Item = Result<DataPlotInfo>>> {
        use PostFnName::*;
        match self {
            Slice(slice) => Box::new(SliceFn::run(slice, sim_set, plot_template)),
            Compare(compare) => Box::new(CompareFn::run(compare, sim_set)),
            Expansion(expansion) => Box::new(ExpansionFn::run(expansion, sim_set, plot_template)),
            Scaling(scaling) => Box::new(ScalingFn::run(scaling, sim_set, plot_template)),
            Shadowing(shadowing) => Box::new(ShadowingFn::run(shadowing, sim_set, plot_template)),
            Convergence(convergence) => {
                Box::new(ConvergenceFn::run(convergence, sim_set, plot_template))
            }
            Ionization(ionization) => {
                Box::new(IonizationFn::run(ionization, sim_set, plot_template))
            }
        }
    }
}
