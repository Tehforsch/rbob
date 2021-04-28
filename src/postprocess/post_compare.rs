use crate::{sim_params::SimParams, sim_set::SimSet};

use super::{
    get_snapshots,
    plot_params::PlotParams,
    post_fn::{PostFn, PostResult},
};
use super::{post_fn::PostFnKind, snapshot::Snapshot};

use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Clap;
use itertools::Itertools;
use ndarray::Array;
use ordered_float::OrderedFloat;

static EPSILON: f64 = 1e-10;
static MIN_VAL: f64 = 1e-24;

#[derive(Clap, Debug)]
pub struct CompareFn {
    pub reference: Utf8PathBuf,
}

impl PostFn for &CompareFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::NoPlotSet
    }

    fn name(&self) -> &'static str {
        "compare"
    }

    fn qualified_name(&self) -> String {
        format!("{}", self.name())
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        let reference_sim_set = SimSet::from_output_folder(&self.reference)?;
        for either_or_both in sim_set.iter().zip_longest(reference_sim_set.iter()) {
            match either_or_both {
                itertools::EitherOrBoth::Both(sim, sim_reference) => {
                    CompareFn::compare_sims(sim, sim_reference)?;
                }
                itertools::EitherOrBoth::Left(sim) => {
                    assert!(
                        false,
                        "Simulation {} available in new run but not in reference run!",
                        sim.get_name()
                    );
                }
                itertools::EitherOrBoth::Right(sim_reference) => {
                    assert!(
                        false,
                        "Simulation {} available in reference run but not in new run!",
                        sim_reference.get_name()
                    );
                }
            }
        }
        Ok(PostResult::new(PlotParams::new(), vec![]))
    }
}

impl CompareFn {
    fn compare_sims(sim: &SimParams, sim_reference: &SimParams) -> Result<()> {
        println!(
            "Comparing sim {} to {}",
            sim.get_name(),
            sim_reference.get_name()
        );
        for (key, _value) in sim.iter() {
            assert_eq!(sim[key], sim_reference[key]);
        }
        for either_or_both in get_snapshots(sim)?.zip_longest(get_snapshots(sim_reference)?) {
            match either_or_both {
                itertools::EitherOrBoth::Both(snap, snap_reference) => {
                    let snap = snap?;
                    let snap_reference = snap_reference?;
                    CompareFn::compare_snaps(&snap, &snap_reference)?;
                }
                itertools::EitherOrBoth::Left(snap) => {
                    assert!(
                        false,
                        "Snapshot {} available in simulation but not in reference!",
                        snap?.get_name()
                    );
                }
                itertools::EitherOrBoth::Right(snap_reference) => {
                    assert!(
                        false,
                        "Snapshot {} available in reference but not in simulation!",
                        snap_reference?.get_name()
                    );
                }
            }
        }
        Ok(())
    }

    fn compare_snaps(snap1: &Snapshot, snap2: &Snapshot) -> Result<()> {
        println!("  Comparing snap {} to {}", snap1, snap2,);
        assert!(is_close(snap1.coordinates()?, snap2.coordinates()?));
        if !is_close(snap1.h_plus_abundance()?, snap2.h_plus_abundance()?) {
            let (val1, val2, diff) =
                get_max_relative_difference(snap1.h_plus_abundance()?, snap2.h_plus_abundance()?);
            assert!(
                false,
                "Relative difference too high: max difference between \n{}\n{}\n={}",
                val1, val2, diff
            );
        }
        Ok(())
    }
}

fn is_close<D>(arr1: Array<f64, D>, arr2: Array<f64, D>) -> bool
where
    D: ndarray::Dimension,
{
    arr1.indexed_iter()
        .zip(arr2.indexed_iter())
        .all(|((indices1, value1), (indices2, value2))| {
            let relative_difference = get_relative_difference(*value1, *value2);
            indices1 == indices2 && relative_difference < EPSILON
        })
}

fn get_max_relative_difference<D>(arr1: Array<f64, D>, arr2: Array<f64, D>) -> (f64, f64, f64)
where
    D: ndarray::Dimension,
{
    let (val1, val2, diff) = arr1
        .iter()
        .zip(arr2.iter())
        .map(move |(value1, value2)| (value1, value2, get_relative_difference(*value1, *value2)))
        .max_by_key(|(_, _, diff)| OrderedFloat(*diff))
        .unwrap();
    (*val1, *val2, diff)
}

pub fn get_relative_difference(val1: f64, val2: f64) -> f64 {
    (val1 - val2) / (val1.abs() + val2.abs() + MIN_VAL)
}
