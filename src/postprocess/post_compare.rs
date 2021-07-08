use crate::{sim_params::SimParams, sim_set::SimSet};

use super::{
    get_snapshots,
    plot_params::PlotParams,
    post_fn::{PostFn, PostResult},
};
use super::{post_fn::PostFnKind, snapshot::Snapshot};

use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;
use clap::Clap;
use itertools::Itertools;
use ndarray::{Array, Array1};
use ordered_float::OrderedFloat;

static MIN_VAL: f64 = 1e-24;
static EPSILON: f64 = 1e-9;

#[derive(Clap, Debug)]
pub struct CompareFn {
    pub reference: Utf8PathBuf,
    pub mean_error_treshold: Option<f64>,
}

impl PostFn for &CompareFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::NoPlotSet
    }

    fn name(&self) -> &'static str {
        "compare"
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
        let reference_sim_set = SimSet::from_output_folder(&self.reference)?;
        for either_or_both in sim_set.iter().zip_longest(reference_sim_set.iter()) {
            match either_or_both {
                itertools::EitherOrBoth::Both(sim, sim_reference) => {
                    self.compare_sims(sim, sim_reference)?;
                }
                itertools::EitherOrBoth::Left(sim) => {
                    panic!(
                        "Simulation {} available in new run but not in reference run!",
                        sim.get_name()
                    );
                }
                itertools::EitherOrBoth::Right(sim_reference) => {
                    panic!(
                        "Simulation {} available in reference run but not in new run!",
                        sim_reference.get_name()
                    );
                }
            }
        }
        Ok(PostResult::new(PlotParams::default(), vec![]))
    }
}

impl CompareFn {
    fn compare_sims(&self, sim: &SimParams, sim_reference: &SimParams) -> Result<()> {
        println!(
            "Comparing sim {} to {}",
            sim.get_name(),
            sim_reference.get_name()
        );
        for (key, _value) in sim.iter() {
            assert_eq!(
                sim[key], sim_reference[key],
                "Parameter values differ for {}",
                key
            );
        }
        for either_or_both in get_snapshots(sim)?.zip_longest(get_snapshots(sim_reference)?) {
            match either_or_both {
                itertools::EitherOrBoth::Both(snap, snap_reference) => {
                    let snap = snap?;
                    let snap_reference = snap_reference?;
                    self.compare_snaps(&snap, &snap_reference)?;
                }
                itertools::EitherOrBoth::Left(snap) => {
                    panic!(
                        "Snapshot {} available in simulation but not in reference!",
                        snap?.get_name()
                    );
                }
                itertools::EitherOrBoth::Right(snap_reference) => {
                    panic!(
                        "Snapshot {} available in reference but not in simulation!",
                        snap_reference?.get_name()
                    );
                }
            }
        }
        Ok(())
    }

    fn compare_snaps(&self, snap1: &Snapshot, snap2: &Snapshot) -> Result<()> {
        println!("  Comparing snap {} to {}", snap1, snap2,);
        self.check_is_close(snap1.coordinates()?, snap2.coordinates()?)?;
        self.check_is_close(snap1.h_plus_abundance()?, snap2.h_plus_abundance()?)?;
        Ok(())
    }

    fn check_is_close<D>(&self, arr1: Array<f64, D>, arr2: Array<f64, D>) -> Result<()>
    where
        D: ndarray::Dimension,
    {
        if !is_close(&arr1, &arr2) {
            let mean_diff = get_mean_relative_difference(&arr1, &arr2);
            let (val1, val2, diff) = get_max_relative_difference(arr1, arr2);
            if let Some(mean_error_treshold) = self.mean_error_treshold {
                if mean_diff < mean_error_treshold {
                    println!(
                        "    Found large errors:\n    Mean difference: {}\n    Max difference:{}",
                        mean_diff, diff
                    );
                    println!("    Ignoring these errors because they are below the given mean error treshold {}", mean_error_treshold);
                    return Ok(());
                }
            }
            return Err(anyhow!(
                "Relative difference too high:\nMean difference: {}\nMax difference between \n{}\n{}\n={}",
                mean_diff,
                val1,
                val2,
                diff
            ));
        }
        Ok(())
    }
}

fn is_close<D>(arr1: &Array<f64, D>, arr2: &Array<f64, D>) -> bool
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

fn get_mean_relative_difference<D>(arr1: &Array<f64, D>, arr2: &Array<f64, D>) -> f64
where
    D: ndarray::Dimension,
{
    arr1.iter()
        .zip(arr2.iter())
        .map(|(value1, value2)| get_relative_difference(*value1, *value2))
        .collect::<Array1<f64>>()
        .mean()
        .unwrap()
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
    (val1 - val2).abs() / (val1.abs() + val2.abs() + MIN_VAL)
}
