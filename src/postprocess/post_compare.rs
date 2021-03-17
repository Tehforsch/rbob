use std::collections::HashMap;

use crate::{array_utils::FArray2, sim_params::SimParams, sim_set::SimSet};

use super::{get_snapshots, post_fn::PostFn};
use super::{post_fn::PostFnKind, snapshot::Snapshot};

use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Clap;
use itertools::Itertools;
use ndarray::Array;

static EPSILON: f64 = 1e-50;
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
    ) -> Result<(Vec<FArray2>, HashMap<String, String>)> {
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
        Ok((vec![], HashMap::new()))
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
        assert!(CompareFn::is_close(
            snap1.coordinates()?,
            snap2.coordinates()?
        ));
        assert!(CompareFn::is_close(
            snap1.h_plus_abundance()?,
            snap2.h_plus_abundance()?
        ));
        Ok(())
    }

    fn is_close<D>(arr1: Array<f64, D>, arr2: Array<f64, D>) -> bool
    where
        D: ndarray::Dimension,
    {
        arr1.indexed_iter().zip(arr2.indexed_iter()).all(
            |((indices1, value1), (indices2, value2))| {
                indices1 == indices2 && CompareFn::same_within_epsilon(*value1, *value2)
            },
        )
    }

    pub fn same_within_epsilon(val1: f64, val2: f64) -> bool {
        (val1 - val2) / (val1.abs() + val2.abs() + MIN_VAL) < EPSILON
    }
}
